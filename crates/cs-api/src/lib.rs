//! CylinderSeal REST API.
//!
//! Operator/admin-facing surface plus server-to-server invoice endpoints
//! for `business_electronic` accounts. Devices use gRPC (`cs-sync`).

pub mod business;
pub mod compliance;
pub mod handlers;
pub mod invoices;
pub mod middleware;
pub mod routes;
pub mod webhooks;

pub use handlers::ApiState;
pub use middleware::{BusinessPrincipal, require_api_key, AuthState};
pub use routes::create_router;
pub use webhooks::WebhookDispatcher;
