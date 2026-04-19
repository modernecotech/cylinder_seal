//! Producer registry, DOC, and Individual Producer (IP) dashboard routes.
//!
//! Read-only views for CBI analysts:
//! - `/api/producers`       — list formal producers
//! - `/api/docs`            — list recently-issued DOCs
//! - `/api/ip`              — list recent IP registrations
//! - `/api/ip/by-category`  — aggregated IP counts by category
//! - `/api/restricted`      — current CBI restricted categories

use axum::{extract::State, http::StatusCode, Json};
use serde::Serialize;
use sqlx::Row;
use std::sync::Arc;

use crate::state::AppState;

#[derive(Serialize)]
pub struct ProducerRow {
    pub producer_id: String,
    pub legal_name: String,
    pub tier: String,
    pub verification_status: String,
    pub governorate: String,
    pub employment_count: Option<i32>,
}

pub async fn list_producers(
    State(app): State<Arc<AppState>>,
) -> Result<Json<Vec<ProducerRow>>, StatusCode> {
    let rows = sqlx::query(
        "SELECT producer_id, legal_name, tier, verification_status, governorate, employment_count
         FROM producer_registry
         ORDER BY created_at DESC
         LIMIT 200",
    )
    .fetch_all(&app.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(
        rows.iter()
            .map(|r| ProducerRow {
                producer_id: r
                    .try_get::<uuid::Uuid, _>("producer_id")
                    .map(|v| v.to_string())
                    .unwrap_or_default(),
                legal_name: r.try_get("legal_name").unwrap_or_default(),
                tier: r.try_get("tier").unwrap_or_default(),
                verification_status: r.try_get("verification_status").unwrap_or_default(),
                governorate: r.try_get("governorate").unwrap_or_default(),
                employment_count: r.try_get("employment_count").ok(),
            })
            .collect(),
    ))
}

#[derive(Serialize)]
pub struct DocRow {
    pub doc_id: String,
    pub producer_id: String,
    pub sku: String,
    pub product_name: String,
    pub iraqi_content_pct: i16,
    pub status: String,
}

pub async fn list_docs(
    State(app): State<Arc<AppState>>,
) -> Result<Json<Vec<DocRow>>, StatusCode> {
    let rows = sqlx::query(
        "SELECT doc_id, producer_id, sku, product_name, iraqi_content_pct, status
         FROM domestic_origin_certificates
         ORDER BY issued_at DESC
         LIMIT 200",
    )
    .fetch_all(&app.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(
        rows.iter()
            .map(|r| DocRow {
                doc_id: r
                    .try_get::<uuid::Uuid, _>("doc_id")
                    .map(|v| v.to_string())
                    .unwrap_or_default(),
                producer_id: r
                    .try_get::<uuid::Uuid, _>("producer_id")
                    .map(|v| v.to_string())
                    .unwrap_or_default(),
                sku: r.try_get("sku").unwrap_or_default(),
                product_name: r.try_get("product_name").unwrap_or_default(),
                iraqi_content_pct: r.try_get("iraqi_content_pct").unwrap_or(0),
                status: r.try_get("status").unwrap_or_default(),
            })
            .collect(),
    ))
}

#[derive(Serialize)]
pub struct IpRow {
    pub ip_id: String,
    pub user_id: String,
    pub category: String,
    pub governorate: String,
    pub display_name: String,
    pub monthly_cap_iqd: i64,
    pub registered_at: String,
}

pub async fn list_ip(
    State(app): State<Arc<AppState>>,
) -> Result<Json<Vec<IpRow>>, StatusCode> {
    let rows = sqlx::query(
        "SELECT ip_id, user_id, category, governorate, display_name, monthly_cap_iqd,
                registered_at
         FROM individual_producers
         ORDER BY registered_at DESC
         LIMIT 200",
    )
    .fetch_all(&app.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(
        rows.iter()
            .map(|r| IpRow {
                ip_id: r
                    .try_get::<uuid::Uuid, _>("ip_id")
                    .map(|v| v.to_string())
                    .unwrap_or_default(),
                user_id: r
                    .try_get::<uuid::Uuid, _>("user_id")
                    .map(|v| v.to_string())
                    .unwrap_or_default(),
                category: r.try_get("category").unwrap_or_default(),
                governorate: r.try_get("governorate").unwrap_or_default(),
                display_name: r.try_get("display_name").unwrap_or_default(),
                monthly_cap_iqd: r.try_get("monthly_cap_iqd").unwrap_or(0),
                registered_at: r
                    .try_get::<chrono::DateTime<chrono::Utc>, _>("registered_at")
                    .map(|d| d.to_rfc3339())
                    .unwrap_or_default(),
            })
            .collect(),
    ))
}

#[derive(Serialize)]
pub struct IpCategoryCount {
    pub category: String,
    pub count: i64,
}

pub async fn ip_by_category(
    State(app): State<Arc<AppState>>,
) -> Result<Json<Vec<IpCategoryCount>>, StatusCode> {
    let rows = sqlx::query(
        "SELECT category, COUNT(*) AS cnt
         FROM individual_producers
         WHERE status = 'active'
         GROUP BY category
         ORDER BY cnt DESC",
    )
    .fetch_all(&app.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(
        rows.iter()
            .map(|r| IpCategoryCount {
                category: r.try_get("category").unwrap_or_default(),
                count: r.try_get("cnt").unwrap_or(0),
            })
            .collect(),
    ))
}

#[derive(Serialize)]
pub struct RestrictedRow {
    pub category: String,
    pub effective_from: String,
    pub max_allowed_tier: i16,
    pub cbi_circular_ref: Option<String>,
    pub is_active: bool,
}

pub async fn list_restricted(
    State(app): State<Arc<AppState>>,
) -> Result<Json<Vec<RestrictedRow>>, StatusCode> {
    let rows = sqlx::query(
        "SELECT category, effective_from, max_allowed_tier, cbi_circular_ref, is_active
         FROM restricted_categories
         ORDER BY effective_from ASC, category ASC",
    )
    .fetch_all(&app.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(
        rows.iter()
            .map(|r| RestrictedRow {
                category: r.try_get("category").unwrap_or_default(),
                effective_from: r
                    .try_get::<chrono::NaiveDate, _>("effective_from")
                    .map(|d| d.to_string())
                    .unwrap_or_default(),
                max_allowed_tier: r.try_get("max_allowed_tier").unwrap_or(0),
                cbi_circular_ref: r.try_get("cbi_circular_ref").ok(),
                is_active: r.try_get("is_active").unwrap_or(false),
            })
            .collect(),
    ))
}
