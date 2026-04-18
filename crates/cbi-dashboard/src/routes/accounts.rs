//! Account management routes

use axum::{extract::{State, Path, Query}, response::IntoResponse, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use crate::state::AppState;

#[derive(Serialize)]
pub struct UserDetail {
    pub user_id: Uuid,
    pub display_name: String,
    pub phone_number: Option<String>,
    pub kyc_tier: String,
    pub account_type: String,
    pub balance_owc: i64,
    pub credit_score: Option<f64>,
    pub region: Option<String>,
    pub account_status: String,
    pub created_at: String,
}

#[derive(Deserialize)]
pub struct UserSearchQuery {
    pub phone: Option<String>,
    pub name: Option<String>,
}

#[derive(Deserialize)]
pub struct FreezeAccountRequest {
    pub reason: String,
}

#[derive(Serialize)]
pub struct UserSearchResult {
    pub users: Vec<UserDetail>,
    pub total: i32,
}

/// GET /api/accounts/search
/// Search users by phone or name
pub async fn search_users(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<UserSearchQuery>,
) -> Result<Json<UserSearchResult>, StatusCode> {
    let query = if let Some(phone) = params.phone {
        sqlx::query!(
            r#"
            SELECT user_id, display_name, phone_number, kyc_tier, account_type,
                   balance_owc, credit_score, region, account_status, created_at
            FROM users
            WHERE phone_number = $1
            LIMIT 10
            "#,
            phone
        )
        .fetch_all(&app_state.db_pool)
        .await
    } else if let Some(name) = params.name {
        sqlx::query!(
            r#"
            SELECT user_id, display_name, phone_number, kyc_tier, account_type,
                   balance_owc, credit_score, region, account_status, created_at
            FROM users
            WHERE display_name ILIKE $1
            LIMIT 10
            "#,
            format!("%{}%", name)
        )
        .fetch_all(&app_state.db_pool)
        .await
    } else {
        return Err(StatusCode::BAD_REQUEST);
    }
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let users: Vec<_> = query
        .into_iter()
        .map(|row| UserDetail {
            user_id: row.user_id,
            display_name: row.display_name,
            phone_number: row.phone_number,
            kyc_tier: row.kyc_tier,
            account_type: row.account_type,
            balance_owc: row.balance_owc,
            credit_score: row.credit_score.map(|d| d.to_string().parse().unwrap_or(0.0)),
            region: row.region,
            account_status: row.account_status,
            created_at: row.created_at.to_rfc3339(),
        })
        .collect();

    let total = users.len() as i32;

    Ok(Json(UserSearchResult { users, total }))
}

/// GET /api/accounts/:user_id
/// Get user detail
pub async fn get_user(
    State(app_state): State<Arc<AppState>>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<UserDetail>, StatusCode> {
    let user = sqlx::query!(
        r#"
        SELECT user_id, display_name, phone_number, kyc_tier, account_type,
               balance_owc, credit_score, region, account_status, created_at
        FROM users
        WHERE user_id = $1
        "#,
        user_id
    )
    .fetch_optional(&app_state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(UserDetail {
        user_id: user.user_id,
        display_name: user.display_name,
        phone_number: user.phone_number,
        kyc_tier: user.kyc_tier,
        account_type: user.account_type,
        balance_owc: user.balance_owc,
        credit_score: user.credit_score.map(|d| d.to_string().parse().unwrap_or(0.0)),
        region: user.region,
        account_status: user.account_status,
        created_at: user.created_at.to_rfc3339(),
    }))
}

/// POST /api/accounts/:user_id/freeze
/// Freeze a user account
pub async fn freeze_account(
    State(app_state): State<Arc<AppState>>,
    Path(user_id): Path<Uuid>,
    Json(req): Json<FreezeAccountRequest>,
) -> Result<StatusCode, StatusCode> {
    sqlx::query!(
        r#"
        UPDATE users
        SET account_status = 'frozen', account_status_reason = $1, account_status_changed_at = NOW()
        WHERE user_id = $2
        "#,
        req.reason,
        user_id
    )
    .execute(&app_state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    sqlx::query!(
        r#"
        INSERT INTO account_status_log (user_id, previous_status, new_status, reason, source, actor_operator_id, changed_at)
        VALUES ($1, 'active', 'frozen', $2, 'admin', NULL, NOW())
        "#,
        user_id,
        req.reason
    )
    .execute(&app_state.db_pool)
    .await
    .ok();

    Ok(StatusCode::OK)
}

/// POST /api/accounts/:user_id/unfreeze
/// Unfreeze a user account
pub async fn unfreeze_account(
    State(app_state): State<Arc<AppState>>,
    Path(user_id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    sqlx::query!(
        r#"
        UPDATE users
        SET account_status = 'active', account_status_reason = NULL, account_status_changed_at = NOW()
        WHERE user_id = $1
        "#,
        user_id
    )
    .execute(&app_state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}
