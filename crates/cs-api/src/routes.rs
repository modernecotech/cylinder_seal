// API route definitions

use axum::Router;

pub fn create_router() -> Router {
    Router::new()
        // TODO: add routes for:
        // - POST /v1/onramp/deposit (PayPal/Flutterwave webhook)
        // - POST /v1/kyc/callback (KYC provider callback)
        // - GET /v1/admin/health
        // - GET /v1/admin/stats
}
