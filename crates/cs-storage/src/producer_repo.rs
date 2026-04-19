//! Repositories for producer registry, Domestic Origin Certificates,
//! Individual Producer track, restricted categories, and tier transaction log.
//!
//! Schema lives in `migrations/20260420000001_producer_ip_doc.sql`.

use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
use serde_json::Value as JsonValue;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use cs_core::error::{CylinderSealError, Result};
use cs_core::producer::{
    DocStatus, DomesticOriginCertificate, IndividualProducer, IpCategory, IpFlag, IpFlagSeverity,
    IpFlagSource, IpMonthlyRollup, IpStatus, Producer, ProducerTier, RestrictedCategory,
    VerificationStatus,
};

fn db_err(e: sqlx::Error) -> CylinderSealError {
    CylinderSealError::DatabaseError(e.to_string())
}

// ---------------------------------------------------------------------------
// Enum helpers
// ---------------------------------------------------------------------------

fn producer_tier_to_str(t: ProducerTier) -> &'static str {
    match t {
        ProducerTier::Micro => "micro",
        ProducerTier::Sme => "sme",
        ProducerTier::Industrial => "industrial",
        ProducerTier::StateOwned => "state_owned",
    }
}
fn producer_tier_from_str(s: &str) -> ProducerTier {
    match s {
        "micro" => ProducerTier::Micro,
        "sme" => ProducerTier::Sme,
        "industrial" => ProducerTier::Industrial,
        "state_owned" => ProducerTier::StateOwned,
        _ => ProducerTier::Micro,
    }
}

fn verif_to_str(v: VerificationStatus) -> &'static str {
    match v {
        VerificationStatus::Pending => "pending",
        VerificationStatus::Verified => "verified",
        VerificationStatus::Suspended => "suspended",
        VerificationStatus::Revoked => "revoked",
    }
}
fn verif_from_str(s: &str) -> VerificationStatus {
    match s {
        "pending" => VerificationStatus::Pending,
        "verified" => VerificationStatus::Verified,
        "suspended" => VerificationStatus::Suspended,
        "revoked" => VerificationStatus::Revoked,
        _ => VerificationStatus::Pending,
    }
}

fn doc_status_to_str(s: DocStatus) -> &'static str {
    match s {
        DocStatus::Active => "active",
        DocStatus::Expired => "expired",
        DocStatus::Revoked => "revoked",
    }
}
fn doc_status_from_str(s: &str) -> DocStatus {
    match s {
        "active" => DocStatus::Active,
        "expired" => DocStatus::Expired,
        "revoked" => DocStatus::Revoked,
        _ => DocStatus::Active,
    }
}

fn ip_category_from_str(s: &str) -> IpCategory {
    match s {
        "food" => IpCategory::Food,
        "crafts" => IpCategory::Crafts,
        "textiles" => IpCategory::Textiles,
        "repair" => IpCategory::Repair,
        "agriculture" => IpCategory::Agriculture,
        "services" => IpCategory::Services,
        "construction" => IpCategory::Construction,
        "transport" => IpCategory::Transport,
        _ => IpCategory::Services,
    }
}

fn ip_status_to_str(s: IpStatus) -> &'static str {
    match s {
        IpStatus::Active => "active",
        IpStatus::Suspended => "suspended",
        IpStatus::Graduated => "graduated",
        IpStatus::Inactive => "inactive",
    }
}
fn ip_status_from_str(s: &str) -> IpStatus {
    match s {
        "active" => IpStatus::Active,
        "suspended" => IpStatus::Suspended,
        "graduated" => IpStatus::Graduated,
        "inactive" => IpStatus::Inactive,
        _ => IpStatus::Active,
    }
}

fn ip_flag_source_to_str(s: IpFlagSource) -> &'static str {
    match s {
        IpFlagSource::PatternEngine => "pattern_engine",
        IpFlagSource::PeerReport => "peer_report",
        IpFlagSource::Inspector => "inspector",
        IpFlagSource::CustomsMismatch => "customs_mismatch",
    }
}
fn ip_flag_source_from_str(s: &str) -> IpFlagSource {
    match s {
        "pattern_engine" => IpFlagSource::PatternEngine,
        "peer_report" => IpFlagSource::PeerReport,
        "inspector" => IpFlagSource::Inspector,
        "customs_mismatch" => IpFlagSource::CustomsMismatch,
        _ => IpFlagSource::PatternEngine,
    }
}

fn ip_flag_severity_to_str(s: IpFlagSeverity) -> &'static str {
    match s {
        IpFlagSeverity::Low => "low",
        IpFlagSeverity::Medium => "medium",
        IpFlagSeverity::High => "high",
    }
}
fn ip_flag_severity_from_str(s: &str) -> IpFlagSeverity {
    match s {
        "low" => IpFlagSeverity::Low,
        "medium" => IpFlagSeverity::Medium,
        "high" => IpFlagSeverity::High,
        _ => IpFlagSeverity::Low,
    }
}

// ---------------------------------------------------------------------------
// Restricted categories repo
// ---------------------------------------------------------------------------

#[async_trait]
pub trait RestrictedCategoryRepository: Send + Sync {
    /// Fetch all categories (active or not). Sorted by effective_from.
    async fn list_all(&self) -> Result<Vec<RestrictedCategory>>;

    /// Active restrictions whose effective_from <= today.
    async fn list_active_on(&self, on: NaiveDate) -> Result<Vec<RestrictedCategory>>;

    /// Lookup a single category — used by the hard-restriction gate.
    async fn get(&self, category: &str) -> Result<Option<RestrictedCategory>>;

    /// Upsert (CBI circular additions / expansions).
    async fn upsert(&self, cat: &RestrictedCategory) -> Result<()>;
}

pub struct PgRestrictedCategoryRepository {
    pool: PgPool,
}

impl PgRestrictedCategoryRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

fn row_to_restricted_category(row: &sqlx::postgres::PgRow) -> RestrictedCategory {
    RestrictedCategory {
        category: row.get("category"),
        effective_from: row.get("effective_from"),
        max_allowed_tier: row.get::<i16, _>("max_allowed_tier") as u8,
        cbi_circular_ref: row.get("cbi_circular_ref"),
        is_active: row.get("is_active"),
        notes: row.get("notes"),
    }
}

#[async_trait]
impl RestrictedCategoryRepository for PgRestrictedCategoryRepository {
    async fn list_all(&self) -> Result<Vec<RestrictedCategory>> {
        let rows = sqlx::query(
            "SELECT category, effective_from, max_allowed_tier, cbi_circular_ref, is_active, notes
             FROM restricted_categories ORDER BY effective_from ASC, category ASC",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(rows.iter().map(row_to_restricted_category).collect())
    }

    async fn list_active_on(&self, on: NaiveDate) -> Result<Vec<RestrictedCategory>> {
        let rows = sqlx::query(
            "SELECT category, effective_from, max_allowed_tier, cbi_circular_ref, is_active, notes
             FROM restricted_categories
             WHERE is_active = TRUE AND effective_from <= $1
             ORDER BY category ASC",
        )
        .bind(on)
        .fetch_all(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(rows.iter().map(row_to_restricted_category).collect())
    }

    async fn get(&self, category: &str) -> Result<Option<RestrictedCategory>> {
        let row = sqlx::query(
            "SELECT category, effective_from, max_allowed_tier, cbi_circular_ref, is_active, notes
             FROM restricted_categories WHERE category = $1",
        )
        .bind(category)
        .fetch_optional(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(row.as_ref().map(row_to_restricted_category))
    }

    async fn upsert(&self, cat: &RestrictedCategory) -> Result<()> {
        sqlx::query(
            "INSERT INTO restricted_categories
               (category, effective_from, max_allowed_tier, cbi_circular_ref, is_active, notes)
             VALUES ($1,$2,$3,$4,$5,$6)
             ON CONFLICT (category) DO UPDATE SET
               effective_from = EXCLUDED.effective_from,
               max_allowed_tier = EXCLUDED.max_allowed_tier,
               cbi_circular_ref = EXCLUDED.cbi_circular_ref,
               is_active = EXCLUDED.is_active,
               notes = EXCLUDED.notes,
               updated_at = NOW()",
        )
        .bind(&cat.category)
        .bind(cat.effective_from)
        .bind(cat.max_allowed_tier as i16)
        .bind(&cat.cbi_circular_ref)
        .bind(cat.is_active)
        .bind(&cat.notes)
        .execute(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Producer registry
// ---------------------------------------------------------------------------

#[async_trait]
pub trait ProducerRepository: Send + Sync {
    async fn get(&self, id: Uuid) -> Result<Option<Producer>>;
    async fn get_by_business_user(&self, user_id: Uuid) -> Result<Option<Producer>>;
    async fn list_by_verification(&self, status: VerificationStatus) -> Result<Vec<Producer>>;
    async fn upsert(&self, p: &Producer) -> Result<()>;
    async fn set_verification(
        &self,
        id: Uuid,
        status: VerificationStatus,
        verified_by: Option<Uuid>,
    ) -> Result<()>;
}

pub struct PgProducerRepository {
    pool: PgPool,
}

impl PgProducerRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

fn row_to_producer(row: &sqlx::postgres::PgRow) -> Producer {
    Producer {
        producer_id: row.get("producer_id"),
        legal_name: row.get("legal_name"),
        ministry_trade_id: row.get("ministry_trade_id"),
        business_user_id: row.get("business_user_id"),
        tier: producer_tier_from_str(row.get::<String, _>("tier").as_str()),
        verification_status: verif_from_str(row.get::<String, _>("verification_status").as_str()),
        governorate: row.get("governorate"),
        employment_count: row.get("employment_count"),
        annual_revenue_iqd: row.get("annual_revenue_iqd"),
        verified_at: row.get("verified_at"),
        verified_by: row.get("verified_by"),
        created_at: row.get("created_at"),
    }
}

#[async_trait]
impl ProducerRepository for PgProducerRepository {
    async fn get(&self, id: Uuid) -> Result<Option<Producer>> {
        let row = sqlx::query(
            "SELECT producer_id, legal_name, ministry_trade_id, business_user_id, tier,
                    verification_status, governorate, employment_count, annual_revenue_iqd,
                    verified_at, verified_by, created_at
             FROM producer_registry WHERE producer_id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(row.as_ref().map(row_to_producer))
    }

    async fn get_by_business_user(&self, user_id: Uuid) -> Result<Option<Producer>> {
        let row = sqlx::query(
            "SELECT producer_id, legal_name, ministry_trade_id, business_user_id, tier,
                    verification_status, governorate, employment_count, annual_revenue_iqd,
                    verified_at, verified_by, created_at
             FROM producer_registry WHERE business_user_id = $1",
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(row.as_ref().map(row_to_producer))
    }

    async fn list_by_verification(&self, status: VerificationStatus) -> Result<Vec<Producer>> {
        let rows = sqlx::query(
            "SELECT producer_id, legal_name, ministry_trade_id, business_user_id, tier,
                    verification_status, governorate, employment_count, annual_revenue_iqd,
                    verified_at, verified_by, created_at
             FROM producer_registry WHERE verification_status = $1
             ORDER BY created_at DESC LIMIT 500",
        )
        .bind(verif_to_str(status))
        .fetch_all(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(rows.iter().map(row_to_producer).collect())
    }

    async fn upsert(&self, p: &Producer) -> Result<()> {
        sqlx::query(
            "INSERT INTO producer_registry
               (producer_id, legal_name, ministry_trade_id, business_user_id, tier,
                verification_status, governorate, employment_count, annual_revenue_iqd,
                verified_at, verified_by, created_at)
             VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12)
             ON CONFLICT (producer_id) DO UPDATE SET
               legal_name = EXCLUDED.legal_name,
               ministry_trade_id = EXCLUDED.ministry_trade_id,
               business_user_id = EXCLUDED.business_user_id,
               tier = EXCLUDED.tier,
               verification_status = EXCLUDED.verification_status,
               governorate = EXCLUDED.governorate,
               employment_count = EXCLUDED.employment_count,
               annual_revenue_iqd = EXCLUDED.annual_revenue_iqd,
               verified_at = EXCLUDED.verified_at,
               verified_by = EXCLUDED.verified_by,
               updated_at = NOW()",
        )
        .bind(p.producer_id)
        .bind(&p.legal_name)
        .bind(&p.ministry_trade_id)
        .bind(p.business_user_id)
        .bind(producer_tier_to_str(p.tier))
        .bind(verif_to_str(p.verification_status))
        .bind(&p.governorate)
        .bind(p.employment_count)
        .bind(p.annual_revenue_iqd)
        .bind(p.verified_at)
        .bind(p.verified_by)
        .bind(p.created_at)
        .execute(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(())
    }

    async fn set_verification(
        &self,
        id: Uuid,
        status: VerificationStatus,
        verified_by: Option<Uuid>,
    ) -> Result<()> {
        let now = Utc::now();
        sqlx::query(
            "UPDATE producer_registry
             SET verification_status = $2, verified_at = $3, verified_by = $4, updated_at = NOW()
             WHERE producer_id = $1",
        )
        .bind(id)
        .bind(verif_to_str(status))
        .bind(if matches!(status, VerificationStatus::Verified) {
            Some(now)
        } else {
            None
        })
        .bind(verified_by)
        .execute(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Domestic Origin Certificate
// ---------------------------------------------------------------------------

#[async_trait]
pub trait DocRepository: Send + Sync {
    async fn get(&self, id: Uuid) -> Result<Option<DomesticOriginCertificate>>;
    async fn get_by_sku(
        &self,
        producer_id: Uuid,
        sku: &str,
    ) -> Result<Option<DomesticOriginCertificate>>;
    async fn list_for_producer(&self, producer_id: Uuid) -> Result<Vec<DomesticOriginCertificate>>;
    async fn upsert(&self, doc: &DomesticOriginCertificate) -> Result<()>;
    async fn revoke(&self, id: Uuid, reason: &str) -> Result<()>;
}

pub struct PgDocRepository {
    pool: PgPool,
}
impl PgDocRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

fn row_to_doc(row: &sqlx::postgres::PgRow) -> DomesticOriginCertificate {
    DomesticOriginCertificate {
        doc_id: row.get("doc_id"),
        producer_id: row.get("producer_id"),
        sku: row.get("sku"),
        product_name: row.get("product_name"),
        iraqi_content_pct: row.get::<i16, _>("iraqi_content_pct") as u8,
        bill_of_materials: row
            .try_get::<JsonValue, _>("bill_of_materials")
            .unwrap_or(JsonValue::Null),
        issued_at: row.get("issued_at"),
        expires_at: row.get("expires_at"),
        issued_by: row.get("issued_by"),
        status: doc_status_from_str(row.get::<String, _>("status").as_str()),
        revocation_reason: row.get("revocation_reason"),
    }
}

#[async_trait]
impl DocRepository for PgDocRepository {
    async fn get(&self, id: Uuid) -> Result<Option<DomesticOriginCertificate>> {
        let row = sqlx::query(
            "SELECT doc_id, producer_id, sku, product_name, iraqi_content_pct,
                    bill_of_materials, issued_at, expires_at, issued_by, status, revocation_reason
             FROM domestic_origin_certificates WHERE doc_id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(row.as_ref().map(row_to_doc))
    }

    async fn get_by_sku(
        &self,
        producer_id: Uuid,
        sku: &str,
    ) -> Result<Option<DomesticOriginCertificate>> {
        let row = sqlx::query(
            "SELECT doc_id, producer_id, sku, product_name, iraqi_content_pct,
                    bill_of_materials, issued_at, expires_at, issued_by, status, revocation_reason
             FROM domestic_origin_certificates
             WHERE producer_id = $1 AND sku = $2",
        )
        .bind(producer_id)
        .bind(sku)
        .fetch_optional(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(row.as_ref().map(row_to_doc))
    }

    async fn list_for_producer(
        &self,
        producer_id: Uuid,
    ) -> Result<Vec<DomesticOriginCertificate>> {
        let rows = sqlx::query(
            "SELECT doc_id, producer_id, sku, product_name, iraqi_content_pct,
                    bill_of_materials, issued_at, expires_at, issued_by, status, revocation_reason
             FROM domestic_origin_certificates
             WHERE producer_id = $1 ORDER BY issued_at DESC",
        )
        .bind(producer_id)
        .fetch_all(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(rows.iter().map(row_to_doc).collect())
    }

    async fn upsert(&self, doc: &DomesticOriginCertificate) -> Result<()> {
        sqlx::query(
            "INSERT INTO domestic_origin_certificates
               (doc_id, producer_id, sku, product_name, iraqi_content_pct,
                bill_of_materials, issued_at, expires_at, issued_by, status, revocation_reason)
             VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11)
             ON CONFLICT (doc_id) DO UPDATE SET
               product_name = EXCLUDED.product_name,
               iraqi_content_pct = EXCLUDED.iraqi_content_pct,
               bill_of_materials = EXCLUDED.bill_of_materials,
               expires_at = EXCLUDED.expires_at,
               status = EXCLUDED.status,
               revocation_reason = EXCLUDED.revocation_reason,
               updated_at = NOW()",
        )
        .bind(doc.doc_id)
        .bind(doc.producer_id)
        .bind(&doc.sku)
        .bind(&doc.product_name)
        .bind(doc.iraqi_content_pct as i16)
        .bind(&doc.bill_of_materials)
        .bind(doc.issued_at)
        .bind(doc.expires_at)
        .bind(doc.issued_by)
        .bind(doc_status_to_str(doc.status))
        .bind(&doc.revocation_reason)
        .execute(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(())
    }

    async fn revoke(&self, id: Uuid, reason: &str) -> Result<()> {
        sqlx::query(
            "UPDATE domestic_origin_certificates
             SET status = 'revoked', revocation_reason = $2, updated_at = NOW()
             WHERE doc_id = $1",
        )
        .bind(id)
        .bind(reason)
        .execute(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Individual Producer (IP) track
// ---------------------------------------------------------------------------

#[async_trait]
pub trait IndividualProducerRepository: Send + Sync {
    async fn register(&self, ip: &IndividualProducer) -> Result<()>;
    async fn get(&self, ip_id: Uuid) -> Result<Option<IndividualProducer>>;
    async fn get_by_user(&self, user_id: Uuid) -> Result<Option<IndividualProducer>>;
    async fn list_recent(&self, limit: i64) -> Result<Vec<IndividualProducer>>;
    async fn list_by_category(&self, cat: IpCategory) -> Result<Vec<IndividualProducer>>;
    async fn set_status(&self, ip_id: Uuid, status: IpStatus) -> Result<()>;
    async fn graduate(&self, ip_id: Uuid, producer_id: Uuid) -> Result<()>;

    async fn upsert_monthly_rollup(&self, r: &IpMonthlyRollup) -> Result<()>;
    async fn get_rollup(&self, ip_id: Uuid, period: &str) -> Result<Option<IpMonthlyRollup>>;
    async fn list_rollups(&self, ip_id: Uuid) -> Result<Vec<IpMonthlyRollup>>;

    async fn raise_flag(&self, f: &IpFlag) -> Result<()>;
    async fn list_flags(&self, ip_id: Uuid) -> Result<Vec<IpFlag>>;
}

pub struct PgIndividualProducerRepository {
    pool: PgPool,
}
impl PgIndividualProducerRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

fn row_to_ip(row: &sqlx::postgres::PgRow) -> IndividualProducer {
    IndividualProducer {
        ip_id: row.get("ip_id"),
        user_id: row.get("user_id"),
        category: ip_category_from_str(row.get::<String, _>("category").as_str()),
        governorate: row.get("governorate"),
        district: row.get("district"),
        display_name: row.get("display_name"),
        attestation_text: row.get("attestation_text"),
        registered_at: row.get("registered_at"),
        monthly_cap_iqd: row.get("monthly_cap_iqd"),
        status: ip_status_from_str(row.get::<String, _>("status").as_str()),
        graduated_to_producer_id: row.get("graduated_to_producer_id"),
        graduated_at: row.get("graduated_at"),
    }
}

fn row_to_rollup(row: &sqlx::postgres::PgRow) -> IpMonthlyRollup {
    IpMonthlyRollup {
        ip_id: row.get("ip_id"),
        period: row.get("period"),
        gross_iqd: row.get("gross_iqd"),
        tx_count: row.get("tx_count"),
        micro_tax_withheld_owc: row.get("micro_tax_withheld_owc"),
        social_security_accrual_owc: row.get("social_security_accrual_owc"),
        over_cap_volume_iqd: row.get("over_cap_volume_iqd"),
        updated_at: row.get("updated_at"),
    }
}

fn row_to_flag(row: &sqlx::postgres::PgRow) -> IpFlag {
    IpFlag {
        flag_id: row.get("flag_id"),
        ip_id: row.get("ip_id"),
        source: ip_flag_source_from_str(row.get::<String, _>("source").as_str()),
        severity: ip_flag_severity_from_str(row.get::<String, _>("severity").as_str()),
        reason: row.get("reason"),
        raised_at: row.get("raised_at"),
        resolved_at: row.get("resolved_at"),
        resolution_note: row.get("resolution_note"),
    }
}

#[async_trait]
impl IndividualProducerRepository for PgIndividualProducerRepository {
    async fn register(&self, ip: &IndividualProducer) -> Result<()> {
        sqlx::query(
            "INSERT INTO individual_producers
               (ip_id, user_id, category, governorate, district, display_name,
                attestation_text, registered_at, monthly_cap_iqd, status,
                graduated_to_producer_id, graduated_at)
             VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12)",
        )
        .bind(ip.ip_id)
        .bind(ip.user_id)
        .bind(ip.category.as_str())
        .bind(&ip.governorate)
        .bind(&ip.district)
        .bind(&ip.display_name)
        .bind(&ip.attestation_text)
        .bind(ip.registered_at)
        .bind(ip.monthly_cap_iqd)
        .bind(ip_status_to_str(ip.status))
        .bind(ip.graduated_to_producer_id)
        .bind(ip.graduated_at)
        .execute(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(())
    }

    async fn get(&self, ip_id: Uuid) -> Result<Option<IndividualProducer>> {
        let row = sqlx::query(
            "SELECT ip_id, user_id, category, governorate, district, display_name,
                    attestation_text, registered_at, monthly_cap_iqd, status,
                    graduated_to_producer_id, graduated_at
             FROM individual_producers WHERE ip_id = $1",
        )
        .bind(ip_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(row.as_ref().map(row_to_ip))
    }

    async fn get_by_user(&self, user_id: Uuid) -> Result<Option<IndividualProducer>> {
        let row = sqlx::query(
            "SELECT ip_id, user_id, category, governorate, district, display_name,
                    attestation_text, registered_at, monthly_cap_iqd, status,
                    graduated_to_producer_id, graduated_at
             FROM individual_producers WHERE user_id = $1",
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(row.as_ref().map(row_to_ip))
    }

    async fn list_recent(&self, limit: i64) -> Result<Vec<IndividualProducer>> {
        let rows = sqlx::query(
            "SELECT ip_id, user_id, category, governorate, district, display_name,
                    attestation_text, registered_at, monthly_cap_iqd, status,
                    graduated_to_producer_id, graduated_at
             FROM individual_producers ORDER BY registered_at DESC LIMIT $1",
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(rows.iter().map(row_to_ip).collect())
    }

    async fn list_by_category(&self, cat: IpCategory) -> Result<Vec<IndividualProducer>> {
        let rows = sqlx::query(
            "SELECT ip_id, user_id, category, governorate, district, display_name,
                    attestation_text, registered_at, monthly_cap_iqd, status,
                    graduated_to_producer_id, graduated_at
             FROM individual_producers
             WHERE category = $1 ORDER BY registered_at DESC LIMIT 500",
        )
        .bind(cat.as_str())
        .fetch_all(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(rows.iter().map(row_to_ip).collect())
    }

    async fn set_status(&self, ip_id: Uuid, status: IpStatus) -> Result<()> {
        sqlx::query("UPDATE individual_producers SET status = $2 WHERE ip_id = $1")
            .bind(ip_id)
            .bind(ip_status_to_str(status))
            .execute(&self.pool)
            .await
            .map_err(db_err)?;
        Ok(())
    }

    async fn graduate(&self, ip_id: Uuid, producer_id: Uuid) -> Result<()> {
        sqlx::query(
            "UPDATE individual_producers
             SET status = 'graduated', graduated_to_producer_id = $2, graduated_at = NOW()
             WHERE ip_id = $1",
        )
        .bind(ip_id)
        .bind(producer_id)
        .execute(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(())
    }

    async fn upsert_monthly_rollup(&self, r: &IpMonthlyRollup) -> Result<()> {
        sqlx::query(
            "INSERT INTO ip_monthly_rollup
               (ip_id, period, gross_iqd, tx_count, micro_tax_withheld_owc,
                social_security_accrual_owc, over_cap_volume_iqd, updated_at)
             VALUES ($1,$2,$3,$4,$5,$6,$7, NOW())
             ON CONFLICT (ip_id, period) DO UPDATE SET
               gross_iqd = EXCLUDED.gross_iqd,
               tx_count = EXCLUDED.tx_count,
               micro_tax_withheld_owc = EXCLUDED.micro_tax_withheld_owc,
               social_security_accrual_owc = EXCLUDED.social_security_accrual_owc,
               over_cap_volume_iqd = EXCLUDED.over_cap_volume_iqd,
               updated_at = NOW()",
        )
        .bind(r.ip_id)
        .bind(&r.period)
        .bind(r.gross_iqd)
        .bind(r.tx_count)
        .bind(r.micro_tax_withheld_owc)
        .bind(r.social_security_accrual_owc)
        .bind(r.over_cap_volume_iqd)
        .execute(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(())
    }

    async fn get_rollup(&self, ip_id: Uuid, period: &str) -> Result<Option<IpMonthlyRollup>> {
        let row = sqlx::query(
            "SELECT ip_id, period, gross_iqd, tx_count, micro_tax_withheld_owc,
                    social_security_accrual_owc, over_cap_volume_iqd, updated_at
             FROM ip_monthly_rollup WHERE ip_id = $1 AND period = $2",
        )
        .bind(ip_id)
        .bind(period)
        .fetch_optional(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(row.as_ref().map(row_to_rollup))
    }

    async fn list_rollups(&self, ip_id: Uuid) -> Result<Vec<IpMonthlyRollup>> {
        let rows = sqlx::query(
            "SELECT ip_id, period, gross_iqd, tx_count, micro_tax_withheld_owc,
                    social_security_accrual_owc, over_cap_volume_iqd, updated_at
             FROM ip_monthly_rollup WHERE ip_id = $1 ORDER BY period DESC",
        )
        .bind(ip_id)
        .fetch_all(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(rows.iter().map(row_to_rollup).collect())
    }

    async fn raise_flag(&self, f: &IpFlag) -> Result<()> {
        sqlx::query(
            "INSERT INTO ip_flags
               (flag_id, ip_id, source, severity, reason, raised_at, resolved_at, resolution_note)
             VALUES ($1,$2,$3,$4,$5,$6,$7,$8)",
        )
        .bind(f.flag_id)
        .bind(f.ip_id)
        .bind(ip_flag_source_to_str(f.source))
        .bind(ip_flag_severity_to_str(f.severity))
        .bind(&f.reason)
        .bind(f.raised_at)
        .bind(f.resolved_at)
        .bind(&f.resolution_note)
        .execute(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(())
    }

    async fn list_flags(&self, ip_id: Uuid) -> Result<Vec<IpFlag>> {
        let rows = sqlx::query(
            "SELECT flag_id, ip_id, source, severity, reason, raised_at, resolved_at,
                    resolution_note
             FROM ip_flags WHERE ip_id = $1 ORDER BY raised_at DESC",
        )
        .bind(ip_id)
        .fetch_all(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(rows.iter().map(row_to_flag).collect())
    }
}

// ---------------------------------------------------------------------------
// Tier transaction log
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct TierTxLogEntry {
    pub log_id: Uuid,
    pub transaction_id: Uuid,
    pub merchant_id: Option<Uuid>,
    pub producer_id: Option<Uuid>,
    pub doc_id: Option<Uuid>,
    pub ip_id: Option<Uuid>,
    pub effective_tier: u8,
    pub iraqi_content_pct: Option<u8>,
    pub fee_applied_bps: i32,
    pub funds_origin: String,
    pub product_category: Option<String>,
    pub hard_restriction_applied: bool,
    pub restriction_reason: Option<String>,
    pub amount_iqd: Option<i64>,
    pub amount_micro_owc: i64,
    pub micro_tax_withheld_owc: i64,
    pub logged_at: DateTime<Utc>,
}

#[async_trait]
pub trait TierTxLogRepository: Send + Sync {
    async fn record(&self, e: &TierTxLogEntry) -> Result<()>;
    async fn list_for_user(&self, user_id: Uuid, limit: i64) -> Result<Vec<TierTxLogEntry>>;
}

pub struct PgTierTxLogRepository {
    pool: PgPool,
}
impl PgTierTxLogRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TierTxLogRepository for PgTierTxLogRepository {
    async fn record(&self, e: &TierTxLogEntry) -> Result<()> {
        sqlx::query(
            "INSERT INTO tier_transaction_log
               (log_id, transaction_id, merchant_id, producer_id, doc_id, ip_id,
                effective_tier, iraqi_content_pct, fee_applied_bps, funds_origin,
                product_category, hard_restriction_applied, restriction_reason,
                amount_iqd, amount_micro_owc, micro_tax_withheld_owc, logged_at)
             VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16,$17)",
        )
        .bind(e.log_id)
        .bind(e.transaction_id)
        .bind(e.merchant_id)
        .bind(e.producer_id)
        .bind(e.doc_id)
        .bind(e.ip_id)
        .bind(e.effective_tier as i16)
        .bind(e.iraqi_content_pct.map(|p| p as i16))
        .bind(e.fee_applied_bps)
        .bind(&e.funds_origin)
        .bind(&e.product_category)
        .bind(e.hard_restriction_applied)
        .bind(&e.restriction_reason)
        .bind(e.amount_iqd)
        .bind(e.amount_micro_owc)
        .bind(e.micro_tax_withheld_owc)
        .bind(e.logged_at)
        .execute(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(())
    }

    async fn list_for_user(&self, user_id: Uuid, limit: i64) -> Result<Vec<TierTxLogEntry>> {
        let rows = sqlx::query(
            "SELECT l.log_id, l.transaction_id, l.merchant_id, l.producer_id, l.doc_id, l.ip_id,
                    l.effective_tier, l.iraqi_content_pct, l.fee_applied_bps, l.funds_origin,
                    l.product_category, l.hard_restriction_applied, l.restriction_reason,
                    l.amount_iqd, l.amount_micro_owc, l.micro_tax_withheld_owc, l.logged_at
             FROM tier_transaction_log l
             JOIN ledger_entries le ON le.transaction_id = l.transaction_id
             WHERE le.user_id = $1
             ORDER BY l.logged_at DESC
             LIMIT $2",
        )
        .bind(user_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(db_err)?;

        Ok(rows
            .iter()
            .map(|row| TierTxLogEntry {
                log_id: row.get("log_id"),
                transaction_id: row.get("transaction_id"),
                merchant_id: row.get("merchant_id"),
                producer_id: row.get("producer_id"),
                doc_id: row.get("doc_id"),
                ip_id: row.get("ip_id"),
                effective_tier: row.get::<i16, _>("effective_tier") as u8,
                iraqi_content_pct: row
                    .get::<Option<i16>, _>("iraqi_content_pct")
                    .map(|v| v as u8),
                fee_applied_bps: row.get("fee_applied_bps"),
                funds_origin: row.get("funds_origin"),
                product_category: row.get("product_category"),
                hard_restriction_applied: row.get("hard_restriction_applied"),
                restriction_reason: row.get("restriction_reason"),
                amount_iqd: row.get("amount_iqd"),
                amount_micro_owc: row.get("amount_micro_owc"),
                micro_tax_withheld_owc: row.get("micro_tax_withheld_owc"),
                logged_at: row.get("logged_at"),
            })
            .collect())
    }
}
