//! Axum router assembly.

use axum::middleware as axum_mw;
use axum::routing::{delete, get, post};
use axum::Router;
use std::sync::Arc;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;

use cs_storage::repository::{
    ApiKeyRepository, BusinessProfileRepository, InvoiceRepository, JournalRepository,
    UserRepository,
};

use crate::business;
use crate::compliance;
use crate::handlers::{
    get_balance, health, kyc_callback, list_entries, readiness, stats, ApiState,
};
use crate::invoices;
use crate::middleware::{require_api_key, AuthState};

pub fn create_router(
    users: Arc<dyn UserRepository>,
    journal: Arc<dyn JournalRepository>,
    business_profiles: Arc<dyn BusinessProfileRepository>,
    api_keys: Arc<dyn ApiKeyRepository>,
    invoices_repo: Arc<dyn InvoiceRepository>,
    node_id: String,
) -> Router {
    let state = ApiState {
        users,
        journal,
        business_profiles,
        api_keys: api_keys.clone(),
        invoices: invoices_repo,
        node_id,
        started_at: chrono::Utc::now(),
    };

    // Public router (no auth required).
    let public = Router::new()
        .route("/v1/admin/health", get(health))
        .route("/v1/admin/readiness", get(readiness))
        .route("/v1/admin/stats", get(stats))
        .route("/v1/users/:user_id/balance", get(get_balance))
        .route("/v1/users/:user_id/entries", get(list_entries))
        .route("/v1/kyc/callback", post(kyc_callback))
        // Business registration is public but gated downstream by manual
        // CBI approval.
        .route("/v1/businesses", post(business::register_business))
        .route("/v1/businesses/:user_id", get(business::get_business))
        // Ops-only endpoints (TODO: wrap with admin-JWT middleware).
        .route("/v1/businesses/:user_id/approve", post(business::approve_business))
        .route("/v1/businesses/:user_id/edd", post(business::mark_edd_cleared))
        .route(
            "/v1/businesses/:user_id/api-keys",
            post(business::issue_api_key).get(business::list_api_keys),
        )
        .route(
            "/v1/businesses/:user_id/api-keys/:key_id",
            delete(business::revoke_api_key),
        )
        // Compliance / risk management endpoints (TODO: admin-JWT gate).
        .route("/v1/compliance/dashboard", get(compliance::dashboard))
        .route("/v1/compliance/rules", get(compliance::list_rules))
        .route("/v1/compliance/rules/:code", get(compliance::get_rule))
        .route("/v1/compliance/evaluate", post(compliance::evaluate_transaction))
        .route("/v1/compliance/users/:user_id/risk", get(compliance::get_user_risk))
        .route("/v1/compliance/exchange-rates", get(compliance::exchange_rates));

    // API-key-gated router (server-to-server calls for business_electronic).
    let authed = Router::new()
        .route(
            "/v1/invoices",
            post(invoices::create_invoice).get(invoices::list_open_invoices),
        )
        .route("/v1/invoices/:invoice_id", get(invoices::get_invoice))
        .route(
            "/v1/invoices/:invoice_id/cancel",
            post(invoices::cancel_invoice),
        )
        .layer(axum_mw::from_fn_with_state(
            AuthState { api_keys },
            require_api_key,
        ));

    public
        .merge(authed)
        .layer(TraceLayer::new_for_http())
        .layer(TimeoutLayer::new(std::time::Duration::from_secs(30)))
        .with_state(state)
}
