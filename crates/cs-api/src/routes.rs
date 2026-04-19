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
use cs_storage::iraq_phase2::{
    CbiPegRepository, DeviceBindingRepository, EmergencyDirectiveRepository, OtpRepository,
    UserRegionRepository, WalletBalanceRepository,
};
use cs_storage::producer_repo::{
    DocRepository, IndividualProducerRepository, ProducerRepository, RestrictedCategoryRepository,
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
use crate::emergency_directive::{self, EmergencyDirectiveState};
use crate::invoices;
use crate::iraq_admin::{self, IraqAdminState};
use crate::middleware::{
    require_admin, require_api_key, AdminAuthState, AuthState,
};
use crate::otp::{self, OtpSender, OtpState};
use crate::producer::{self, ProducerApiState};
use crate::rule_governance::{self, RuleGovernanceState};
use crate::travel_rule::{self, TravelRuleState};
use crate::wallets::{self, WalletState};

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
    pub user_regions: Arc<dyn UserRegionRepository>,
    pub device_bindings: Arc<dyn DeviceBindingRepository>,
    pub emergency_directives: Arc<dyn EmergencyDirectiveRepository>,
    pub wallet_balances: Arc<dyn WalletBalanceRepository>,
    pub cbi_peg: Arc<dyn CbiPegRepository>,
    pub otp_repo: Arc<dyn OtpRepository>,
    /// Sender for outbound OTP codes. In dev: [`LogOnlyOtpSender`]. In
    /// production: an Asiacell/Zain/Korek HTTPS client.
    pub otp_sender: Arc<dyn OtpSender>,
    /// Per-deployment pepper combined with the OTP digits before hashing.
    /// Loaded from runtime config; rotating it invalidates all in-flight
    /// challenges (acceptable since TTL is 10 minutes).
    pub otp_pepper: Arc<Vec<u8>>,
    pub producers: Arc<dyn ProducerRepository>,
    pub docs: Arc<dyn DocRepository>,
    pub individual_producers: Arc<dyn IndividualProducerRepository>,
    pub restricted_categories: Arc<dyn RestrictedCategoryRepository>,
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
    let iraq_admin_state = IraqAdminState {
        region_repo: deps.user_regions.clone(),
        device_binding_repo: deps.device_bindings.clone(),
    };
    let emergency_directive_state = EmergencyDirectiveState {
        repo: deps.emergency_directives.clone(),
    };
    let wallet_state = WalletState {
        balances: deps.wallet_balances.clone(),
        peg: deps.cbi_peg.clone(),
    };
    let otp_state = OtpState {
        repo: deps.otp_repo.clone(),
        users: deps.users.clone(),
        sender: deps.otp_sender.clone(),
        pepper: deps.otp_pepper.clone(),
    };

    let producer_state = ProducerApiState {
        producers: deps.producers.clone(),
        docs: deps.docs.clone(),
        ips: deps.individual_producers.clone(),
        restricted: deps.restricted_categories.clone(),
    };
    let producer_citizen_routes = producer::citizen_routes(producer_state.clone());
    let producer_admin_routes = producer::admin_routes(producer_state).layer(
        axum_mw::from_fn_with_state(admin_auth_state.clone(), require_admin),
    );

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

    // OTP issue/verify is unauthenticated (the OTP itself is the proof)
    // but rate-limited at the gateway. Promotes anonymous → phone_verified.
    let otp_routes = Router::new()
        .route("/v1/otp/issue", post(otp::issue))
        .route("/v1/otp/verify", post(otp::verify))
        .with_state(otp_state);

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

    let iraq_admin_routes = Router::new()
        .route(
            "/v1/admin/users/:user_id/region",
            axum::routing::get(iraq_admin::get_user_region)
                .post(iraq_admin::set_user_region),
        )
        .route(
            "/v1/admin/users/:user_id/device-binding",
            axum::routing::get(iraq_admin::get_device_binding)
                .post(iraq_admin::set_device_binding),
        )
        .with_state(iraq_admin_state)
        .layer(axum_mw::from_fn_with_state(
            admin_auth_state.clone(),
            require_admin,
        ));

    let emergency_directive_routes = Router::new()
        .route(
            "/v1/admin/emergency-directives",
            axum::routing::get(emergency_directive::list_active_directives)
                .post(emergency_directive::issue_directive),
        )
        .route(
            "/v1/admin/emergency-directives/:directive_id",
            axum::routing::delete(emergency_directive::revoke_directive),
        )
        .with_state(emergency_directive_state)
        .layer(axum_mw::from_fn_with_state(
            admin_auth_state.clone(),
            require_admin,
        ));

    // Wallet + peg routes. Per-user balance reads are admin-gated (operator
    // dashboard); the peg + conversion endpoints are public so the mobile
    // app can fetch them without an auth round-trip.
    let wallet_admin_routes = Router::new()
        .route("/v1/admin/users/:user_id/wallets", get(wallets::list_wallets))
        .route(
            "/v1/admin/users/:user_id/wallets/:currency",
            get(wallets::get_wallet),
        )
        .with_state(wallet_state.clone())
        .layer(axum_mw::from_fn_with_state(
            admin_auth_state.clone(),
            require_admin,
        ));
    let peg_public_routes = Router::new()
        .route("/v1/peg/current", get(wallets::current_peg))
        .route("/v1/peg/convert", get(wallets::convert))
        .with_state(wallet_state);

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
        .route(
            "/v1/invoices/:invoice_id/fiscal-receipt",
            post(invoices::set_fiscal_receipt),
        )
        .with_state(api_state)
        .layer(axum_mw::from_fn_with_state(
            AuthState {
                api_keys: deps.api_keys.clone(),
            },
            require_api_key,
        ));

    public
        .merge(otp_routes)
        .merge(producer_citizen_routes)
        .merge(producer_admin_routes)
        .merge(admin_login)
        .merge(admin_self)
        .merge(admin_business)
        .merge(compliance_routes)
        .merge(travel_rule_routes)
        .merge(ubo_routes)
        .merge(iraq_admin_routes)
        .merge(emergency_directive_routes)
        .merge(wallet_admin_routes)
        .merge(peg_public_routes)
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
