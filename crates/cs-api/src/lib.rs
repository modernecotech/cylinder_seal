//! CylinderSeal REST API.
//!
//! Operator/admin-facing surface plus server-to-server invoice endpoints
//! for `business_electronic` accounts. Devices use gRPC (`cs-sync`).

pub mod admin;
pub mod admin_ui;
pub mod beneficial_owners;
pub mod business;
pub mod compliance;
pub mod emergency_directive;
pub mod handlers;
pub mod invoices;
pub mod iraq_admin;
pub mod middleware;
pub mod otp;
pub mod producer;
pub mod routes;
pub mod rule_governance;
pub mod travel_rule;
pub mod wallets;
pub mod webhooks;

pub use handlers::ApiState;
pub use middleware::{
    require_admin, require_api_key, AdminAuthState, AdminPrincipal, AuthState, BusinessPrincipal,
};
pub use otp::{LogOnlyOtpSender, OtpSender, OtpState, OTP_TTL_SECONDS};
pub use routes::{create_router, RouterDeps};
pub use webhooks::WebhookDispatcher;
