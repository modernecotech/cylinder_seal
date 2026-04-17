//! Invoice endpoints for business-electronic accounts.
//!
//! All endpoints here require a valid API-key bearer token — wire them
//! under the `require_api_key` middleware in [`routes.rs`]. Handlers read
//! the authenticated [`BusinessPrincipal`] from request extensions.
//!
//! Routes:
//! - `POST /v1/invoices` — create a new invoice; response includes the
//!   `CS1:INV:<hex>` payment URI the customer scans.
//! - `GET /v1/invoices/:invoice_id` — poll status (alternative to webhook).
//! - `GET /v1/invoices` — list open invoices for the calling business.
//! - `POST /v1/invoices/:invoice_id/cancel` — cancel an open invoice.

use axum::extract::{Extension, Path, State};
use axum::http::StatusCode;
use axum::Json;
use chrono::{Duration, Utc};
use cs_storage::models::InvoiceRecord;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::handlers::ApiState;
use crate::middleware::BusinessPrincipal;

#[derive(Deserialize)]
pub struct CreateInvoiceRequest {
    pub amount_owc: i64,
    pub currency: String,
    pub description: Option<String>,
    pub external_reference: Option<String>,
    /// Optional webhook URL. If set, the super-peer POSTs a JSON payload
    /// when the invoice transitions to `paid` or `expired`.
    pub webhook_url: Option<String>,
    /// Time-to-live in seconds. Default 1 hour; max 7 days.
    #[serde(default = "default_ttl")]
    pub ttl_seconds: i64,
}

fn default_ttl() -> i64 {
    3600
}

#[derive(Serialize)]
pub struct CreateInvoiceResponse {
    pub invoice_id: Uuid,
    /// `CS1:INV:<hex>` URI — encode as QR for the customer to scan.
    pub payment_uri: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

pub async fn create_invoice(
    State(state): State<ApiState>,
    Extension(principal): Extension<BusinessPrincipal>,
    Json(req): Json<CreateInvoiceRequest>,
) -> Result<Json<CreateInvoiceResponse>, (StatusCode, String)> {
    if !principal.has_scope("invoice.create") && !principal.scopes.is_empty() {
        return Err((StatusCode::FORBIDDEN, "missing scope: invoice.create".into()));
    }
    if req.amount_owc <= 0 {
        return Err((StatusCode::BAD_REQUEST, "amount_owc must be > 0".into()));
    }
    let ttl = req.ttl_seconds.clamp(60, 7 * 24 * 3600);

    let now = Utc::now();
    let invoice_id = Uuid::now_v7();
    let expires_at = now + Duration::seconds(ttl);

    let record = InvoiceRecord {
        invoice_id,
        user_id: principal.user_id,
        amount_owc: req.amount_owc,
        currency: req.currency.clone(),
        description: req.description,
        external_reference: req.external_reference,
        status: "open".to_string(),
        paid_by_user_id: None,
        paid_by_transaction_id: None,
        webhook_url: req.webhook_url,
        webhook_delivered_at: None,
        created_at: now,
        expires_at,
        paid_at: None,
    };
    state
        .invoices
        .create(&record)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let payment_uri = format!("CS1:INV:{}", hex::encode_upper(invoice_id.as_bytes()));
    Ok(Json(CreateInvoiceResponse {
        invoice_id,
        payment_uri,
        expires_at,
    }))
}

#[derive(Serialize)]
pub struct InvoiceStatusDto {
    pub invoice_id: Uuid,
    pub status: String,
    pub amount_owc: i64,
    pub currency: String,
    pub description: Option<String>,
    pub external_reference: Option<String>,
    pub paid_by_user_id: Option<Uuid>,
    pub paid_by_transaction_id: Option<Uuid>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub paid_at: Option<chrono::DateTime<chrono::Utc>>,
}

pub async fn get_invoice(
    State(state): State<ApiState>,
    Extension(principal): Extension<BusinessPrincipal>,
    Path(invoice_id): Path<Uuid>,
) -> Result<Json<InvoiceStatusDto>, (StatusCode, String)> {
    let inv = state
        .invoices
        .get(invoice_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let Some(inv) = inv else {
        return Err((StatusCode::NOT_FOUND, "invoice not found".into()));
    };
    if inv.user_id != principal.user_id {
        return Err((StatusCode::NOT_FOUND, "invoice not found".into()));
    }
    Ok(Json(InvoiceStatusDto {
        invoice_id: inv.invoice_id,
        status: inv.status,
        amount_owc: inv.amount_owc,
        currency: inv.currency,
        description: inv.description,
        external_reference: inv.external_reference,
        paid_by_user_id: inv.paid_by_user_id,
        paid_by_transaction_id: inv.paid_by_transaction_id,
        created_at: inv.created_at,
        expires_at: inv.expires_at,
        paid_at: inv.paid_at,
    }))
}

pub async fn list_open_invoices(
    State(state): State<ApiState>,
    Extension(principal): Extension<BusinessPrincipal>,
) -> Result<Json<Vec<InvoiceStatusDto>>, (StatusCode, String)> {
    let rows = state
        .invoices
        .list_open_for_user(principal.user_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(
        rows.into_iter()
            .map(|inv| InvoiceStatusDto {
                invoice_id: inv.invoice_id,
                status: inv.status,
                amount_owc: inv.amount_owc,
                currency: inv.currency,
                description: inv.description,
                external_reference: inv.external_reference,
                paid_by_user_id: inv.paid_by_user_id,
                paid_by_transaction_id: inv.paid_by_transaction_id,
                created_at: inv.created_at,
                expires_at: inv.expires_at,
                paid_at: inv.paid_at,
            })
            .collect(),
    ))
}

pub async fn cancel_invoice(
    State(state): State<ApiState>,
    Extension(principal): Extension<BusinessPrincipal>,
    Path(invoice_id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    let inv = state
        .invoices
        .get(invoice_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    match inv {
        Some(inv) if inv.user_id == principal.user_id => {
            state
                .invoices
                .cancel(invoice_id)
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            Ok(StatusCode::NO_CONTENT)
        }
        _ => Err((StatusCode::NOT_FOUND, "invoice not found".into())),
    }
}
