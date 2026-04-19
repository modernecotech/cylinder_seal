//! REST endpoints for Producer Registry, Domestic Origin Certificates,
//! Individual Producer (IP) track, and restricted-category lookups.
//!
//! Split into two surface layers:
//!
//! ## Citizen-facing (no admin auth)
//! - `POST /v1/ip/register`      — register caller as Individual Producer
//! - `GET  /v1/ip/me`            — fetch own IP record
//! - `GET  /v1/ip/me/rollups`    — monthly rollup history
//! - `GET  /v1/restricted-categories` — read the CBI restricted list
//! - `GET  /v1/doc/lookup?producer_id=&sku=` — validate a DOC scan
//!
//! ## Admin-facing (require_admin)
//! - `POST   /v1/admin/producers`            — upsert a formal producer
//! - `PATCH  /v1/admin/producers/:id/verify` — verify or suspend
//! - `POST   /v1/admin/docs`                 — issue a DOC
//! - `POST   /v1/admin/docs/:id/revoke`      — revoke
//! - `POST   /v1/admin/restricted-categories`— upsert category
//! - `GET    /v1/admin/ip/list`              — recent IP registrations

use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use chrono::{NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use cs_core::producer::{
    DocStatus, DomesticOriginCertificate, IpCategory, Producer, ProducerTier, RestrictedCategory,
    VerificationStatus,
};
use cs_policy::individual_producer as ip_policy;
use cs_storage::producer_repo::{
    DocRepository, IndividualProducerRepository, ProducerRepository, RestrictedCategoryRepository,
};

#[derive(Clone)]
pub struct ProducerApiState {
    pub producers: Arc<dyn ProducerRepository>,
    pub docs: Arc<dyn DocRepository>,
    pub ips: Arc<dyn IndividualProducerRepository>,
    pub restricted: Arc<dyn RestrictedCategoryRepository>,
}

// ---------------------------------------------------------------------------
// Request/response shapes
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
pub struct IpRegisterRequest {
    pub user_id: Uuid,
    pub category: String,
    pub governorate: String,
    pub district: Option<String>,
    pub display_name: String,
    pub attestation_text: String,
}

#[derive(Serialize)]
pub struct IpRegisterResponse {
    pub ip_id: Uuid,
    pub category: String,
    pub monthly_cap_iqd: i64,
    pub status: String,
    pub ddpb_badge_ref: String,
}

#[derive(Serialize)]
pub struct RestrictedCategoryView {
    pub category: String,
    pub effective_from: NaiveDate,
    pub max_allowed_tier: u8,
    pub cbi_circular_ref: Option<String>,
    pub is_active: bool,
    pub notes: Option<String>,
}

impl From<RestrictedCategory> for RestrictedCategoryView {
    fn from(c: RestrictedCategory) -> Self {
        Self {
            category: c.category,
            effective_from: c.effective_from,
            max_allowed_tier: c.max_allowed_tier,
            cbi_circular_ref: c.cbi_circular_ref,
            is_active: c.is_active,
            notes: c.notes,
        }
    }
}

#[derive(Deserialize)]
pub struct DocLookupQuery {
    pub producer_id: Uuid,
    pub sku: String,
}

#[derive(Serialize)]
pub struct DocView {
    pub doc_id: Uuid,
    pub producer_id: Uuid,
    pub sku: String,
    pub product_name: String,
    pub iraqi_content_pct: u8,
    pub status: String,
    pub is_valid: bool,
    pub expires_at: chrono::DateTime<Utc>,
}

impl DocView {
    fn from_doc(d: DomesticOriginCertificate) -> Self {
        let now = Utc::now();
        let is_valid = matches!(d.status, DocStatus::Active) && d.expires_at > now;
        Self {
            doc_id: d.doc_id,
            producer_id: d.producer_id,
            sku: d.sku,
            product_name: d.product_name,
            iraqi_content_pct: d.iraqi_content_pct,
            status: match d.status {
                DocStatus::Active => "active",
                DocStatus::Expired => "expired",
                DocStatus::Revoked => "revoked",
            }
            .to_string(),
            is_valid,
            expires_at: d.expires_at,
        }
    }
}

#[derive(Deserialize)]
pub struct ProducerUpsertRequest {
    pub producer_id: Option<Uuid>,
    pub legal_name: String,
    pub ministry_trade_id: Option<String>,
    pub business_user_id: Option<Uuid>,
    pub tier: String, // "micro" | "sme" | "industrial" | "state_owned"
    pub governorate: String,
    pub employment_count: Option<i32>,
    pub annual_revenue_iqd: Option<i64>,
}

#[derive(Serialize)]
pub struct ProducerView {
    pub producer_id: Uuid,
    pub legal_name: String,
    pub tier: String,
    pub verification_status: String,
    pub governorate: String,
    pub employment_count: Option<i32>,
}

impl From<Producer> for ProducerView {
    fn from(p: Producer) -> Self {
        Self {
            producer_id: p.producer_id,
            legal_name: p.legal_name,
            tier: match p.tier {
                ProducerTier::Micro => "micro",
                ProducerTier::Sme => "sme",
                ProducerTier::Industrial => "industrial",
                ProducerTier::StateOwned => "state_owned",
            }
            .into(),
            verification_status: match p.verification_status {
                VerificationStatus::Pending => "pending",
                VerificationStatus::Verified => "verified",
                VerificationStatus::Suspended => "suspended",
                VerificationStatus::Revoked => "revoked",
            }
            .into(),
            governorate: p.governorate,
            employment_count: p.employment_count,
        }
    }
}

#[derive(Deserialize)]
pub struct VerifyProducerRequest {
    pub status: String, // "verified" | "suspended" | "revoked"
    pub verified_by: Option<Uuid>,
}

#[derive(Deserialize)]
pub struct DocIssueRequest {
    pub producer_id: Uuid,
    pub sku: String,
    pub product_name: String,
    pub iraqi_content_pct: u8,
    pub bill_of_materials: serde_json::Value,
    pub expires_at: chrono::DateTime<Utc>,
    pub issued_by: Option<Uuid>,
}

#[derive(Deserialize)]
pub struct DocRevokeRequest {
    pub reason: String,
}

#[derive(Deserialize)]
pub struct RestrictedCategoryUpsertRequest {
    pub category: String,
    pub effective_from: NaiveDate,
    pub max_allowed_tier: u8,
    pub cbi_circular_ref: Option<String>,
    pub is_active: bool,
    pub notes: Option<String>,
}

// ---------------------------------------------------------------------------
// Citizen-facing handlers
// ---------------------------------------------------------------------------

fn parse_ip_category(s: &str) -> Result<IpCategory, (StatusCode, String)> {
    match s {
        "food" => Ok(IpCategory::Food),
        "crafts" => Ok(IpCategory::Crafts),
        "textiles" => Ok(IpCategory::Textiles),
        "repair" => Ok(IpCategory::Repair),
        "agriculture" => Ok(IpCategory::Agriculture),
        "services" => Ok(IpCategory::Services),
        "construction" => Ok(IpCategory::Construction),
        "transport" => Ok(IpCategory::Transport),
        other => Err((
            StatusCode::BAD_REQUEST,
            format!("unknown IP category: {other}"),
        )),
    }
}

fn parse_producer_tier(s: &str) -> Result<ProducerTier, (StatusCode, String)> {
    match s {
        "micro" => Ok(ProducerTier::Micro),
        "sme" => Ok(ProducerTier::Sme),
        "industrial" => Ok(ProducerTier::Industrial),
        "state_owned" => Ok(ProducerTier::StateOwned),
        other => Err((
            StatusCode::BAD_REQUEST,
            format!("unknown producer tier: {other}"),
        )),
    }
}

fn parse_verification(s: &str) -> Result<VerificationStatus, (StatusCode, String)> {
    match s {
        "pending" => Ok(VerificationStatus::Pending),
        "verified" => Ok(VerificationStatus::Verified),
        "suspended" => Ok(VerificationStatus::Suspended),
        "revoked" => Ok(VerificationStatus::Revoked),
        other => Err((
            StatusCode::BAD_REQUEST,
            format!("unknown verification status: {other}"),
        )),
    }
}

pub async fn register_ip(
    State(state): State<ProducerApiState>,
    Json(req): Json<IpRegisterRequest>,
) -> Result<Json<IpRegisterResponse>, (StatusCode, String)> {
    let category = parse_ip_category(&req.category)?;

    if let Some(existing) = state
        .ips
        .get_by_user(req.user_id)
        .await
        .map_err(internal)?
    {
        return Ok(Json(IpRegisterResponse {
            ip_id: existing.ip_id,
            category: existing.category.as_str().into(),
            monthly_cap_iqd: existing.monthly_cap_iqd,
            status: "already_registered".into(),
            ddpb_badge_ref: format!("ddpb:{}", existing.ip_id),
        }));
    }

    let ip = ip_policy::new_ip_registration(
        req.user_id,
        category,
        req.governorate,
        req.district,
        req.display_name,
        req.attestation_text,
    );
    state.ips.register(&ip).await.map_err(internal)?;

    Ok(Json(IpRegisterResponse {
        ip_id: ip.ip_id,
        category: ip.category.as_str().into(),
        monthly_cap_iqd: ip.monthly_cap_iqd,
        status: "active".into(),
        ddpb_badge_ref: format!("ddpb:{}", ip.ip_id),
    }))
}

#[derive(Deserialize)]
pub struct MeQuery {
    pub user_id: Uuid,
}

pub async fn get_my_ip(
    State(state): State<ProducerApiState>,
    Query(q): Query<MeQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let ip = state
        .ips
        .get_by_user(q.user_id)
        .await
        .map_err(internal)?
        .ok_or((StatusCode::NOT_FOUND, "not registered as IP".into()))?;
    Ok(Json(serde_json::json!({
        "ip_id": ip.ip_id,
        "category": ip.category.as_str(),
        "governorate": ip.governorate,
        "display_name": ip.display_name,
        "monthly_cap_iqd": ip.monthly_cap_iqd,
        "registered_at": ip.registered_at,
        "ddpb_badge_ref": format!("ddpb:{}", ip.ip_id),
    })))
}

pub async fn get_my_rollups(
    State(state): State<ProducerApiState>,
    Query(q): Query<MeQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let ip = state
        .ips
        .get_by_user(q.user_id)
        .await
        .map_err(internal)?
        .ok_or((StatusCode::NOT_FOUND, "not registered as IP".into()))?;
    let rollups = state.ips.list_rollups(ip.ip_id).await.map_err(internal)?;
    let items: Vec<_> = rollups
        .iter()
        .map(|r| {
            serde_json::json!({
                "period": r.period,
                "gross_iqd": r.gross_iqd,
                "tx_count": r.tx_count,
                "micro_tax_withheld_owc": r.micro_tax_withheld_owc,
                "over_cap_volume_iqd": r.over_cap_volume_iqd,
            })
        })
        .collect();
    Ok(Json(serde_json::json!({ "ip_id": ip.ip_id, "rollups": items })))
}

pub async fn list_restricted_categories(
    State(state): State<ProducerApiState>,
) -> Result<Json<Vec<RestrictedCategoryView>>, (StatusCode, String)> {
    let cats = state.restricted.list_all().await.map_err(internal)?;
    Ok(Json(cats.into_iter().map(Into::into).collect()))
}

pub async fn lookup_doc(
    State(state): State<ProducerApiState>,
    Query(q): Query<DocLookupQuery>,
) -> Result<Json<DocView>, (StatusCode, String)> {
    let doc = state
        .docs
        .get_by_sku(q.producer_id, &q.sku)
        .await
        .map_err(internal)?
        .ok_or((StatusCode::NOT_FOUND, "DOC not found".into()))?;
    Ok(Json(DocView::from_doc(doc)))
}

// ---------------------------------------------------------------------------
// Admin-facing handlers
// ---------------------------------------------------------------------------

pub async fn admin_upsert_producer(
    State(state): State<ProducerApiState>,
    Json(req): Json<ProducerUpsertRequest>,
) -> Result<Json<ProducerView>, (StatusCode, String)> {
    let tier = parse_producer_tier(&req.tier)?;
    let producer = Producer {
        producer_id: req.producer_id.unwrap_or_else(Uuid::new_v4),
        legal_name: req.legal_name,
        ministry_trade_id: req.ministry_trade_id,
        business_user_id: req.business_user_id,
        tier,
        verification_status: VerificationStatus::Pending,
        governorate: req.governorate,
        employment_count: req.employment_count,
        annual_revenue_iqd: req.annual_revenue_iqd,
        verified_at: None,
        verified_by: None,
        created_at: Utc::now(),
    };
    state.producers.upsert(&producer).await.map_err(internal)?;
    Ok(Json(ProducerView::from(producer)))
}

pub async fn admin_verify_producer(
    State(state): State<ProducerApiState>,
    Path(id): Path<Uuid>,
    Json(req): Json<VerifyProducerRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    let status = parse_verification(&req.status)?;
    state
        .producers
        .set_verification(id, status, req.verified_by)
        .await
        .map_err(internal)?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn admin_issue_doc(
    State(state): State<ProducerApiState>,
    Json(req): Json<DocIssueRequest>,
) -> Result<Json<DocView>, (StatusCode, String)> {
    if req.iraqi_content_pct > 100 {
        return Err((
            StatusCode::BAD_REQUEST,
            "iraqi_content_pct must be 0-100".into(),
        ));
    }
    let doc = DomesticOriginCertificate {
        doc_id: Uuid::new_v4(),
        producer_id: req.producer_id,
        sku: req.sku,
        product_name: req.product_name,
        iraqi_content_pct: req.iraqi_content_pct,
        bill_of_materials: req.bill_of_materials,
        issued_at: Utc::now(),
        expires_at: req.expires_at,
        issued_by: req.issued_by,
        status: DocStatus::Active,
        revocation_reason: None,
    };
    state.docs.upsert(&doc).await.map_err(internal)?;
    Ok(Json(DocView::from_doc(doc)))
}

pub async fn admin_revoke_doc(
    State(state): State<ProducerApiState>,
    Path(id): Path<Uuid>,
    Json(req): Json<DocRevokeRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    state.docs.revoke(id, &req.reason).await.map_err(internal)?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn admin_upsert_restricted_category(
    State(state): State<ProducerApiState>,
    Json(req): Json<RestrictedCategoryUpsertRequest>,
) -> Result<Json<RestrictedCategoryView>, (StatusCode, String)> {
    let cat = RestrictedCategory {
        category: req.category,
        effective_from: req.effective_from,
        max_allowed_tier: req.max_allowed_tier,
        cbi_circular_ref: req.cbi_circular_ref,
        is_active: req.is_active,
        notes: req.notes,
    };
    state.restricted.upsert(&cat).await.map_err(internal)?;
    Ok(Json(RestrictedCategoryView::from(cat)))
}

#[derive(Deserialize)]
pub struct AdminIpListQuery {
    pub limit: Option<i64>,
}

pub async fn admin_list_ips(
    State(state): State<ProducerApiState>,
    Query(q): Query<AdminIpListQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let limit = q.limit.unwrap_or(100).clamp(1, 500);
    let ips = state.ips.list_recent(limit).await.map_err(internal)?;
    let items: Vec<_> = ips
        .iter()
        .map(|ip| {
            serde_json::json!({
                "ip_id": ip.ip_id,
                "user_id": ip.user_id,
                "category": ip.category.as_str(),
                "governorate": ip.governorate,
                "display_name": ip.display_name,
                "monthly_cap_iqd": ip.monthly_cap_iqd,
                "registered_at": ip.registered_at,
            })
        })
        .collect();
    Ok(Json(serde_json::json!({ "items": items })))
}

// ---------------------------------------------------------------------------
// Router assembly
// ---------------------------------------------------------------------------

pub fn citizen_routes(state: ProducerApiState) -> axum::Router {
    use axum::routing::{get, post};
    axum::Router::new()
        .route("/v1/ip/register", post(register_ip))
        .route("/v1/ip/me", get(get_my_ip))
        .route("/v1/ip/me/rollups", get(get_my_rollups))
        .route("/v1/restricted-categories", get(list_restricted_categories))
        .route("/v1/doc/lookup", get(lookup_doc))
        .with_state(state)
}

pub fn admin_routes(state: ProducerApiState) -> axum::Router {
    use axum::routing::{get, patch, post};
    axum::Router::new()
        .route("/v1/admin/producers", post(admin_upsert_producer))
        .route(
            "/v1/admin/producers/:id/verify",
            patch(admin_verify_producer),
        )
        .route("/v1/admin/docs", post(admin_issue_doc))
        .route("/v1/admin/docs/:id/revoke", post(admin_revoke_doc))
        .route(
            "/v1/admin/restricted-categories",
            post(admin_upsert_restricted_category),
        )
        .route("/v1/admin/ip/list", get(admin_list_ips))
        .with_state(state)
}

fn internal<E: std::fmt::Display>(e: E) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
}
