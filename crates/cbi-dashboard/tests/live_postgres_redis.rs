use axum::{
    body::{to_bytes, Body},
    http::{header, Request, StatusCode},
    Router,
};
use cbi_dashboard::{
    auth::hash_password,
    build_app,
    state::{AppState, PostgresAuditRecorder, RedisSessionStore},
};
use redis::AsyncCommands;
use serde_json::{json, Value};
use sqlx::{postgres::PgPoolOptions, PgPool, Row};
use std::sync::Arc;
use tower::ServiceExt;
use uuid::Uuid;

const PASSWORD: &str = "live-test-password";

struct LiveHarness {
    router: Router,
    db_pool: PgPool,
    redis_pool: deadpool_redis::Pool,
    supervisor_username: String,
    auditor_username: String,
}

#[tokio::test]
async fn live_postgres_redis_auth_sessions_audit_and_role_gates(
) -> Result<(), Box<dyn std::error::Error>> {
    if !live_tests_enabled() {
        eprintln!("skipping live PostgreSQL/Redis test; set CBI_DASHBOARD_LIVE_TESTS=1 to run");
        return Ok(());
    }

    let harness = LiveHarness::new().await?;

    let response = harness
        .router
        .clone()
        .oneshot(json_request(
            "POST",
            "/auth/login",
            json!({ "username": harness.supervisor_username, "password": PASSWORD }),
            None,
        )?)
        .await?;
    assert_eq!(response.status(), StatusCode::OK);
    let supervisor_login = json_body(response).await?;
    let supervisor_token = token_from_login(&supervisor_login);

    let mut redis_conn = harness.redis_pool.get().await?;
    let session_json: String = redis_conn
        .get(format!("session:{supervisor_token}"))
        .await?;
    assert!(session_json.contains(&harness.supervisor_username));
    assert!(session_json.contains("supervisor"));

    let response = harness
        .router
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/projects")
                .body(Body::empty())?,
        )
        .await?;
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    let response = harness
        .router
        .clone()
        .oneshot(bearer_request("GET", "/api/projects", &supervisor_token)?)
        .await?;
    assert_eq!(response.status(), StatusCode::OK);

    let response = harness
        .router
        .clone()
        .oneshot(json_request(
            "POST",
            "/api/audit/directives",
            json!({
                "directive_type": "velocity_cap",
                "description": "live integration directive"
            }),
            Some(&supervisor_token),
        )?)
        .await?;
    assert_eq!(response.status(), StatusCode::OK);
    assert_audit_result(
        &harness.db_pool,
        &harness.supervisor_username,
        "directive.create",
        "ok",
    )
    .await?;

    let response = harness
        .router
        .clone()
        .oneshot(json_request(
            "POST",
            "/auth/login",
            json!({ "username": harness.auditor_username, "password": PASSWORD }),
            None,
        )?)
        .await?;
    assert_eq!(response.status(), StatusCode::OK);
    let auditor_login = json_body(response).await?;
    let auditor_token = token_from_login(&auditor_login);

    let response = harness
        .router
        .clone()
        .oneshot(json_request(
            "POST",
            "/api/audit/directives",
            json!({
                "directive_type": "emergency_pause",
                "description": "auditor should be denied"
            }),
            Some(&auditor_token),
        )?)
        .await?;
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
    assert_audit_result(
        &harness.db_pool,
        &harness.auditor_username,
        "directive.create",
        "denied",
    )
    .await?;

    let account_id = Uuid::new_v4();
    let response = harness
        .router
        .clone()
        .oneshot(json_request(
            "POST",
            &format!("/api/accounts/{account_id}/freeze"),
            json!({ "reason": "live integration freeze" }),
            Some(&supervisor_token),
        )?)
        .await?;
    assert_eq!(response.status(), StatusCode::OK);
    assert_audit_result(
        &harness.db_pool,
        &harness.supervisor_username,
        "account.freeze",
        "ok",
    )
    .await?;

    let response = harness
        .router
        .clone()
        .oneshot(bearer_request("POST", "/auth/logout", &supervisor_token)?)
        .await?;
    assert_eq!(response.status(), StatusCode::OK);
    let still_exists: bool = redis_conn
        .exists(format!("session:{supervisor_token}"))
        .await?;
    assert!(!still_exists);

    let response = harness
        .router
        .clone()
        .oneshot(bearer_request("GET", "/api/projects", &supervisor_token)?)
        .await?;
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    harness.cleanup(&[supervisor_token, auditor_token]).await?;
    Ok(())
}

impl LiveHarness {
    async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgresql://postgres:change-me-dev-only@localhost:5432/cylinder_seal".to_string()
        });
        let redis_url =
            std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string());

        let db_pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await?;
        sqlx::migrate!("../../migrations").run(&db_pool).await?;

        let redis_pool = deadpool_redis::Config::from_url(redis_url)
            .create_pool(Some(deadpool_redis::Runtime::Tokio1))?;

        let suffix = Uuid::new_v4().simple().to_string();
        let supervisor_username = format!("live_supervisor_{suffix}");
        let auditor_username = format!("live_auditor_{suffix}");

        seed_operator(&db_pool, &supervisor_username, "supervisor").await?;
        seed_operator(&db_pool, &auditor_username, "auditor").await?;

        let state = Arc::new(
            AppState::new(
                db_pool.clone(),
                Arc::new(RedisSessionStore::new(redis_pool.clone())),
                Arc::new(PostgresAuditRecorder::new(db_pool.clone())),
                3_600,
            )
            .await?,
        );

        Ok(Self {
            router: build_app(state),
            db_pool,
            redis_pool,
            supervisor_username,
            auditor_username,
        })
    }

    async fn cleanup(&self, tokens: &[String]) -> Result<(), Box<dyn std::error::Error>> {
        let mut redis_conn = self.redis_pool.get().await?;
        for token in tokens {
            let _: i64 = redis_conn.del(format!("session:{token}")).await?;
        }

        sqlx::query(
            "DELETE FROM admin_audit_log WHERE operator_username = $1 OR operator_username = $2",
        )
        .bind(&self.supervisor_username)
        .bind(&self.auditor_username)
        .execute(&self.db_pool)
        .await?;

        sqlx::query("DELETE FROM admin_operators WHERE username = $1 OR username = $2")
            .bind(&self.supervisor_username)
            .bind(&self.auditor_username)
            .execute(&self.db_pool)
            .await?;

        Ok(())
    }
}

async fn seed_operator(
    db_pool: &PgPool,
    username: &str,
    role: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let password_hash = hash_password(PASSWORD).map_err(|err| {
        std::io::Error::other(format!("failed to hash live-test password: {err}"))
    })?;
    let email = format!("{username}@example.invalid");

    sqlx::query(
        r#"
        INSERT INTO admin_operators (
            operator_id,
            username,
            display_name,
            email,
            password_hash,
            role,
            active
        )
        VALUES ($1, $2, $3, $4, $5, $6, TRUE)
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(username)
    .bind(format!("Live test {role}"))
    .bind(email)
    .bind(password_hash)
    .bind(role)
    .execute(db_pool)
    .await?;

    Ok(())
}

async fn assert_audit_result(
    db_pool: &PgPool,
    username: &str,
    action: &str,
    result: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let row = sqlx::query(
        r#"
        SELECT action, result
        FROM admin_audit_log
        WHERE operator_username = $1 AND action = $2
        ORDER BY created_at DESC
        LIMIT 1
        "#,
    )
    .bind(username)
    .bind(action)
    .fetch_one(db_pool)
    .await?;

    let stored_action: String = row.get("action");
    let stored_result: String = row.get("result");
    assert_eq!(stored_action, action);
    assert_eq!(stored_result, result);
    Ok(())
}

fn live_tests_enabled() -> bool {
    matches!(
        std::env::var("CBI_DASHBOARD_LIVE_TESTS").as_deref(),
        Ok("1" | "true" | "TRUE" | "yes" | "YES")
    )
}

fn bearer_request(
    method: &str,
    uri: &str,
    token: &str,
) -> Result<Request<Body>, Box<dyn std::error::Error>> {
    Ok(Request::builder()
        .method(method)
        .uri(uri)
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .body(Body::empty())?)
}

fn json_request(
    method: &str,
    uri: &str,
    body: Value,
    token: Option<&str>,
) -> Result<Request<Body>, Box<dyn std::error::Error>> {
    let mut builder = Request::builder()
        .method(method)
        .uri(uri)
        .header(header::CONTENT_TYPE, "application/json");
    if let Some(token) = token {
        builder = builder.header(header::AUTHORIZATION, format!("Bearer {token}"));
    }
    Ok(builder.body(Body::from(serde_json::to_vec(&body)?))?)
}

async fn json_body(
    response: axum::response::Response,
) -> Result<Value, Box<dyn std::error::Error>> {
    let bytes = to_bytes(response.into_body(), usize::MAX).await?;
    Ok(serde_json::from_slice(&bytes)?)
}

fn token_from_login(login: &Value) -> String {
    login
        .get("token")
        .and_then(Value::as_str)
        .expect("login response includes token")
        .to_string()
}
