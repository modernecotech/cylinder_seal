//! Beneficial Ownership (FATF Recommendations 24/25) handlers.
//!
//! Each business profile must disclose its beneficial owners. The
//! threshold for disclosure is anyone holding >= 25% ownership, voting
//! rights, or control. The aggregate disclosed ownership must reach 75%
//! before a business can be approved (the residual 25% may be widely
//! held). Enforced at approval time, not at insert time, since
//! disclosures may arrive incrementally.

use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Extension;
use axum::Json;
use chrono::NaiveDate;
use cs_storage::compliance::{BeneficialOwnerRecord, BeneficialOwnerRepository};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::middleware::AdminPrincipal;

#[derive(Clone)]
pub struct BeneficialOwnerState {
    pub repo: Arc<dyn BeneficialOwnerRepository>,
}

#[derive(Deserialize)]
pub struct AddOwnerRequest {
    pub full_name: String,
    pub nationality: String,
    pub date_of_birth: NaiveDate,
    pub id_type: String,
    pub id_number: String,
    pub id_country: String,
    pub residential_address: String,
    pub ownership_pct: Decimal,
    pub control_type: String,
    pub is_pep: Option<bool>,
    pub pep_position: Option<String>,
    pub source_doc_ref: Option<String>,
}

#[derive(Serialize)]
pub struct AddOwnerResponse {
    pub owner_id: i64,
    pub total_disclosed_pct: Decimal,
    pub meets_disclosure_threshold: bool,
}

const DISCLOSURE_REQUIRED_PCT: i64 = 75;

pub async fn add_owner(
    State(state): State<BeneficialOwnerState>,
    Path(business_user_id): Path<Uuid>,
    Json(req): Json<AddOwnerRequest>,
) -> Result<Json<AddOwnerResponse>, (StatusCode, String)> {
    if !["passport", "national_id", "residence_permit", "tax_id"].contains(&req.id_type.as_str()) {
        return Err((StatusCode::BAD_REQUEST, "invalid id_type".into()));
    }
    if !["direct_ownership", "indirect_ownership", "voting_rights", "board_appointment", "other"]
        .contains(&req.control_type.as_str())
    {
        return Err((StatusCode::BAD_REQUEST, "invalid control_type".into()));
    }
    let pct_min = Decimal::new(0, 2);
    let pct_max = Decimal::new(100, 0);
    if req.ownership_pct <= pct_min || req.ownership_pct > pct_max {
        return Err((
            StatusCode::BAD_REQUEST,
            "ownership_pct must be (0, 100]".into(),
        ));
    }

    let record = BeneficialOwnerRecord {
        owner_id: 0,
        business_user_id,
        full_name: req.full_name,
        nationality: req.nationality,
        date_of_birth: req.date_of_birth,
        id_type: req.id_type,
        id_number: req.id_number,
        id_country: req.id_country,
        residential_address: req.residential_address,
        ownership_pct: req.ownership_pct,
        control_type: req.control_type,
        is_pep: req.is_pep.unwrap_or(false),
        pep_position: req.pep_position,
        source_doc_ref: req.source_doc_ref,
        verified_at: None,
        verified_by: None,
    };

    let id = state
        .repo
        .add(&record)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let total = state
        .repo
        .total_disclosed_pct(business_user_id)
        .await
        .unwrap_or_default();
    let threshold = Decimal::new(DISCLOSURE_REQUIRED_PCT, 0);
    Ok(Json(AddOwnerResponse {
        owner_id: id,
        total_disclosed_pct: total,
        meets_disclosure_threshold: total >= threshold,
    }))
}

#[derive(Serialize)]
pub struct OwnerDto {
    pub owner_id: i64,
    pub full_name: String,
    pub nationality: String,
    pub ownership_pct: Decimal,
    pub control_type: String,
    pub is_pep: bool,
    pub verified: bool,
}

pub async fn list_owners(
    State(state): State<BeneficialOwnerState>,
    Path(business_user_id): Path<Uuid>,
) -> Result<Json<Vec<OwnerDto>>, (StatusCode, String)> {
    let owners = state
        .repo
        .list_for_business(business_user_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(
        owners
            .into_iter()
            .map(|o| OwnerDto {
                owner_id: o.owner_id,
                full_name: o.full_name,
                nationality: o.nationality,
                ownership_pct: o.ownership_pct,
                control_type: o.control_type,
                is_pep: o.is_pep,
                verified: o.verified_at.is_some(),
            })
            .collect(),
    ))
}

pub async fn verify_owner(
    State(state): State<BeneficialOwnerState>,
    Extension(actor): Extension<AdminPrincipal>,
    Path((_business_user_id, owner_id)): Path<(Uuid, i64)>,
) -> Result<StatusCode, (StatusCode, String)> {
    if !actor.has_role("officer") {
        return Err((
            StatusCode::FORBIDDEN,
            "officer role required to verify".into(),
        ));
    }
    state
        .repo
        .mark_verified(owner_id, actor.operator_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(StatusCode::NO_CONTENT)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn disclosure_threshold_is_75_pct() {
        assert_eq!(DISCLOSURE_REQUIRED_PCT, 75);
    }
}
