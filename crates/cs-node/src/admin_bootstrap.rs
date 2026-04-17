//! `cylinder-seal-node admin bootstrap` subcommand.
//!
//! Creates the first supervisor operator. The migration intentionally
//! does NOT seed an admin user (so we don't ship a hash that any reader
//! of the codebase would know). Operators run this CLI on first install,
//! receive a one-time generated password on stdout, and are expected to
//! change it on first login.

use anyhow::{bail, Context, Result};
use argon2::password_hash::{rand_core::OsRng, PasswordHasher, SaltString};
use argon2::Argon2;
use chrono::Utc;
use cs_storage::compliance::{
    AdminOperator, AdminOperatorRepository, PgAdminOperatorRepository,
};
use cs_storage::postgres::PostgresConfig;
use rand::RngCore;
use std::sync::Arc;
use uuid::Uuid;

use crate::config::Config;
use crate::AdminCommand;

pub async fn dispatch(cmd: AdminCommand, cfg: &Config) -> Result<()> {
    match cmd {
        AdminCommand::Bootstrap {
            username,
            email,
            display_name,
            reset_password,
        } => bootstrap(cfg, &username, &email, &display_name, reset_password).await,
    }
}

async fn bootstrap(
    cfg: &Config,
    username: &str,
    email: &str,
    display_name: &str,
    reset_password: bool,
) -> Result<()> {
    let pg_cfg = PostgresConfig {
        host: cfg.database.host.clone(),
        port: cfg.database.port,
        database: cfg.database.name.clone(),
        username: cfg.database.user.clone(),
        password: cfg.database.password.clone(),
        max_connections: 4,
    };
    let pool = cs_storage::postgres::connect(&pg_cfg)
        .await
        .context("connect postgres")?;
    let repo: Arc<dyn AdminOperatorRepository> =
        Arc::new(PgAdminOperatorRepository::new(pool.clone()));

    let existing = repo
        .find_by_username(username)
        .await
        .context("lookup operator")?;

    let plain_password = generate_password();
    let hash = hash_password(&plain_password).context("hash password")?;

    match existing {
        Some(op) if !reset_password => {
            bail!(
                "operator '{}' already exists (operator_id={}). Pass --reset-password to rotate.",
                op.username,
                op.operator_id
            );
        }
        Some(op) => {
            sqlx::query(
                "UPDATE admin_operators SET password_hash = $1, active = true, updated_at = now() WHERE operator_id = $2",
            )
            .bind(&hash)
            .bind(op.operator_id)
            .execute(&pool)
            .await
            .context("rotate password")?;
            println!("Operator '{}' updated.", op.username);
            println!("New password: {}", plain_password);
            println!("Role: {}", op.role);
        }
        None => {
            let new_op = AdminOperator {
                operator_id: Uuid::nil(),
                username: username.to_string(),
                display_name: display_name.to_string(),
                email: email.to_string(),
                password_hash: hash,
                role: "supervisor".into(),
                active: true,
                mfa_secret: None,
                created_at: Utc::now(),
                last_login_at: None,
            };
            let id = repo.create(&new_op).await.context("create operator")?;
            println!("Operator '{}' created (id={}).", username, id);
            println!("Role: supervisor");
            println!("Password: {}", plain_password);
            println!("\nLog in via POST /v1/admin/auth/login with that password,");
            println!("then immediately use POST /v1/admin/operators to create role-specific accounts");
            println!("for the rest of the team. Rotate this bootstrap password ASAP.");
        }
    }
    Ok(())
}

fn generate_password() -> String {
    // 16 bytes of entropy -> 32 chars hex. Not as compact as base64, but
    // avoids pulling base64 in; entropy is still 128 bits.
    let mut bytes = [0u8; 16];
    OsRng.fill_bytes(&mut bytes);
    hex::encode(bytes)
}

fn hash_password(password: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|h| h.to_string())
        .map_err(|e| anyhow::anyhow!("argon2: {e}"))
}
