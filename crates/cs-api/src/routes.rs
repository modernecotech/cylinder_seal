//! Axum router assembly.

use axum::middleware as axum_mw;
use axum::routing::{delete, get, post};
use axum::Router;
use std::sync::Arc;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;

use cs_storage::compliance::{
    AdminAuditRepository, AdminOperatorRepository, AdminSessionStore,
    BeneficialOwnerRepository, FeedRunRepository, RiskSnapshotRepository,
    RuleVersionRepository, TransactionEvaluationRepository, TravelRuleRepository,
};
use cs_storage::repository::{
    ApiKeyRepository, BusinessProfileRepository, InvoiceRepository, JournalRepository,
    UserRepository,
};

use crate::admin::{self, AdminApiState};
use crate::admin_ui;
use crate::beneficial_owners::{self, BeneficialOwnerState};
use crate::business;
use crate::compliance::{self, ComplianceState};
use crate::handlers::{
    get_balance, health, kyc_callback, list_entries, readiness, stats, ApiState,
};
use crate::invoices;
use crate::middleware::{
    require_admin, require_api_key, AdminAuthState, AuthState,
};
use crate::rule_governance::{self, RuleGovernanceState};
use crate::travel_rule::{self, TravelRuleState};

pub struct RouterDeps {
    pub users: Arc<dyn UserRepository>,
    pub journal: Arc<dyn JournalRepository>,
    pub business_profiles: Arc<dyn BusinessProfileRepository>,
    pub api_keys: Arc<dyn ApiKeyRepository>,
    pub invoices: Arc<dyn InvoiceRepository>,
    pub admin_operators: Arc<dyn AdminOperatorRepository>,
    pub admin_sessions: Arc<dyn AdminSessionStore>,
    pub admin_audit: Arc<dyn AdminAuditRepository>,
    pub evaluations: Arc<dyn TransactionEvaluationRepository>,
    pub snapshots: Arc<dyn RiskSnapshotRepository>,
    pub rule_versions: Arc<dyn RuleVersionRepository>,
    pub travel_rule: Arc<dyn TravelRuleRepository>,
    pub beneficial_owners: Arc<dyn BeneficialOwnerRepository>,
    pub feed_runs: Arc<dyn FeedRunRepository>,
    pub node_id: String,
    pub admin_session_ttl_hours: u32,
}

pub fn create_router(deps: RouterDeps) -> Router {
    let api_state = ApiState {
        users: deps.users.clone(),
        journal: deps.journal.clone(),
        business_profiles: deps.business_profiles.clone(),
        api_keys: deps.api_keys.clone(),
        invoices: deps.invoices.clone(),
        node_id: deps.node_id.clone(),
        started_at: chrono::Utc::now(),
    };

    let admin_state = AdminApiState {
        operators: deps.admin_operators.clone(),
        sessions: deps.admin_sessions.clone(),
        audit: deps.admin_audit.clone(),
        session_ttl_hours: deps.admin_session_ttl_hours,
    };

    let admin_auth_state = AdminAuthState {
        sessions: deps.admin_sessions.clone(),
        audit: deps.admin_audit.clone(),
    };

    let compliance_state = ComplianceState {
        api: api_state.clone(),
        evaluations: deps.evaluations.clone(),
        snapshots: deps.snapshots.clone(),
        feed_runs: deps.feed_runs.clone(),
    };

    // Server-rendered admin UI (HTMX). Login page is public; dashboard
    // sits behind require_admin like the JSON endpoints do.
    let admin_ui_public = Router::new()
        .route("/admin/login", get(admin_ui::login_page));
    let admin_ui_dashboard = Router::new()
        .route("/admin/", get(admin_ui::index))
        .with_state(compliance_state.clone())
        .layer(axum_mw::from_fn_with_state(
            admin_auth_state.clone(),
            require_admin,
        ));

    let travel_rule_state = TravelRuleState {
        repo: deps.travel_rule.clone(),
    };
    let owner_state = BeneficialOwnerState {
        repo: deps.beneficial_owners.clone(),
    };
    let governance_state = RuleGovernanceState {
        repo: deps.rule_versions.clone(),
    };

    // Server-rendered admin UI pages for governance / UBO / Travel Rule.
    // Each one shares its underlying state with the JSON API router.
    let admin_ui_governance = Router::new()
        .route("/admin/rules/proposals", get(admin_ui::rule_proposals_page))
        .with_state(governance_state.clone())
        .layer(axum_mw::from_fn_with_state(
            admin_auth_state.clone(),
            require_admin,
        ));
    let admin_ui_owners = Router::new()
        .route(
            "/admin/businesses/:user_id/owners",
            get(admin_ui::ubo_page),
        )
        .with_state(owner_state.clone())
        .layer(axum_mw::from_fn_with_state(
            admin_auth_state.clone(),
            require_admin,
        ));
    let admin_ui_travel_rule = Router::new()
        .route("/admin/travel-rule/:tx_id", get(admin_ui::travel_rule_page))
        .with_state(travel_rule_state.clone())
        .layer(axum_mw::from_fn_with_state(
            admin_auth_state.clone(),
            require_admin,
        ));

    // Public routes (no auth).
    let public = Router::new()
        .route("/v1/admin/health", get(health))
        .route("/v1/admin/readiness", get(readiness))
        .route("/v1/admin/stats", get(stats))
        .route("/v1/users/:user_id/balance", get(get_balance))
        .route("/v1/users/:user_id/entries", get(list_entries))
        .route("/v1/kyc/callback", post(kyc_callback))
        .route("/v1/businesses", post(business::register_business))
        .route("/v1/businesses/:user_id", get(business::get_business))
        .with_state(api_state.clone());

    // Admin login (public — that's where you GET your session token).
    let admin_login = Router::new()
        .route("/v1/admin/auth/login", post(admin::login))
        .route("/v1/admin/auth/logout", post(admin::logout))
        .with_state(admin_state.clone());

    // Admin-gated routes (analyst+ unless handler asserts a higher role).
    let admin_business = Router::new()
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
        .with_state(api_state.clone())
        .layer(axum_mw::from_fn_with_state(
            admin_auth_state.clone(),
            require_admin,
        ));

    let admin_self = Router::new()
        .route("/v1/admin/auth/whoami", get(admin::whoami))
        .route("/v1/admin/operators", post(admin::create_operator))
        .with_state(admin_state.clone())
        .layer(axum_mw::from_fn_with_state(
            admin_auth_state.clone(),
            require_admin,
        ));

    let compliance_routes = Router::new()
        .route("/v1/compliance/dashboard", get(compliance::dashboard))
        .route("/v1/compliance/rules", get(compliance::list_rules))
        .route("/v1/compliance/rules/:code", get(compliance::get_rule))
        .route("/v1/compliance/evaluate", post(compliance::evaluate_transaction))
        .route(
            "/v1/compliance/users/:user_id/risk",
            get(compliance::get_user_risk),
        )
        .route(
            "/v1/compliance/users/:user_id/explanations",
            get(compliance::user_transaction_explanations),
        )
        .route("/v1/compliance/exchange-rates", get(compliance::exchange_rates))
        .with_state(compliance_state)
        .layer(axum_mw::from_fn_with_state(
            admin_auth_state.clone(),
            require_admin,
        ));

    let travel_rule_routes = Router::new()
        .route("/v1/travel-rule", post(travel_rule::submit_payload))
        .route(
            "/v1/travel-rule/:transaction_id",
            get(travel_rule::get_payload),
        )
        .with_state(travel_rule_state)
        .layer(axum_mw::from_fn_with_state(
            admin_auth_state.clone(),
            require_admin,
        ));

    let ubo_routes = Router::new()
        .route(
            "/v1/businesses/:user_id/beneficial-owners",
            post(beneficial_owners::add_owner).get(beneficial_owners::list_owners),
        )
        .route(
            "/v1/businesses/:user_id/beneficial-owners/:owner_id/verify",
            post(beneficial_owners::verify_owner),
        )
        .with_state(owner_state)
        .layer(axum_mw::from_fn_with_state(
            admin_auth_state.clone(),
            require_admin,
        ));

    let governance_routes = Router::new()
        .route("/v1/governance/rules/proposals", post(rule_governance::propose_rule))
        .route("/v1/governance/rules/proposals", get(rule_governance::list_pending))
        .route(
            "/v1/governance/rules/proposals/:version_id/approve",
            post(rule_governance::approve_rule),
        )
        .route(
            "/v1/governance/rules/proposals/:version_id/reject",
            post(rule_governance::reject_rule),
        )
        .route(
            "/v1/governance/rules/:rule_code/history",
            get(rule_governance::rule_history),
        )
        .with_state(governance_state)
        .layer(axum_mw::from_fn_with_state(
            admin_auth_state,
            require_admin,
        ));

    // API-key-gated router (server-to-server for business_electronic).
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
        .with_state(api_state)
        .layer(axum_mw::from_fn_with_state(
            AuthState {
                api_keys: deps.api_keys.clone(),
            },
            require_api_key,
        ));

    public
        .merge(admin_login)
        .merge(admin_self)
        .merge(admin_business)
        .merge(compliance_routes)
        .merge(travel_rule_routes)
        .merge(ubo_routes)
        .merge(governance_routes)
        .merge(authed)
        .merge(admin_ui_public)
        .merge(admin_ui_dashboard)
        .merge(admin_ui_governance)
        .merge(admin_ui_owners)
        .merge(admin_ui_travel_rule)
        .layer(TraceLayer::new_for_http())
        .layer(TimeoutLayer::new(std::time::Duration::from_secs(30)))
}
