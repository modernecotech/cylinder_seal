//! Travel Rule (FATF Recommendation 16) handlers.
//!
//! Cross-institution transfers above the FATF threshold (USD 1,000 or
//! local equivalent — Iraq's CBI threshold under Law 39/2015 may differ;
//! configurable via [`TRAVEL_RULE_THRESHOLD_MICRO_OWC`]) must carry
//! originator and beneficiary identifying information. This module
//! receives that payload at submission time and surfaces it to
//! compliance officers.

use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use chrono::NaiveDate;
use cs_storage::compliance::{TravelRulePayloadRecord, TravelRuleRepository};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 1,000 OWC = USD-1k equivalent at 1:1 fixed peg.
pub const TRAVEL_RULE_THRESHOLD_MICRO_OWC: i64 = 1_000_000_000;

#[derive(Clone)]
pub struct TravelRuleState {
    pub repo: Arc<dyn TravelRuleRepository>,
}

#[derive(Deserialize)]
pub struct TravelRuleRequest {
    pub transaction_id: Uuid,
    pub amount_micro_owc: i64,
    pub currency: Option<String>,
    pub originator_name: String,
    pub originator_account: String,
    pub originator_address: Option<String>,
    pub originator_id_type: Option<String>,
    pub originator_id_number: Option<String>,
    pub originator_dob: Option<NaiveDate>,
    pub originator_country: String,
    pub beneficiary_name: String,
    pub beneficiary_account: String,
    pub beneficiary_country: String,
    pub vasp_originator: String,
    pub vasp_beneficiary: String,
    pub purpose_code: Option<String>,
}

#[derive(Serialize)]
pub struct TravelRuleResponse {
    pub payload_id: i64,
    pub transaction_id: Uuid,
    pub required: bool,
}

pub async fn submit_payload(
    State(state): State<TravelRuleState>,
    Json(req): Json<TravelRuleRequest>,
) -> Result<Json<TravelRuleResponse>, (StatusCode, String)> {
    let required = req.amount_micro_owc >= TRAVEL_RULE_THRESHOLD_MICRO_OWC;

    let record = TravelRulePayloadRecord {
        transaction_id: req.transaction_id,
        originator_name: validate_non_empty(&req.originator_name, "originator_name")?,
        originator_account: validate_non_empty(&req.originator_account, "originator_account")?,
        originator_address: req.originator_address,
        originator_id_type: req.originator_id_type,
        originator_id_number: req.originator_id_number,
        originator_dob: req.originator_dob,
        originator_country: validate_country(&req.originator_country)?,
        beneficiary_name: validate_non_empty(&req.beneficiary_name, "beneficiary_name")?,
        beneficiary_account: validate_non_empty(&req.beneficiary_account, "beneficiary_account")?,
        beneficiary_country: validate_country(&req.beneficiary_country)?,
        vasp_originator: validate_non_empty(&req.vasp_originator, "vasp_originator")?,
        vasp_beneficiary: validate_non_empty(&req.vasp_beneficiary, "vasp_beneficiary")?,
        amount_micro_owc: req.amount_micro_owc,
        currency: req.currency.unwrap_or_else(|| "OWC".into()),
        purpose_code: req.purpose_code,
    };

    let id = state
        .repo
        .record(&record)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(TravelRuleResponse {
        payload_id: id,
        transaction_id: req.transaction_id,
        required,
    }))
}

#[derive(Serialize)]
pub struct TravelRulePayloadDto {
    pub transaction_id: Uuid,
    pub originator_name: String,
    pub originator_country: String,
    pub beneficiary_name: String,
    pub beneficiary_country: String,
    pub vasp_originator: String,
    pub vasp_beneficiary: String,
    pub amount_micro_owc: i64,
    pub currency: String,
    pub purpose_code: Option<String>,
}

pub async fn get_payload(
    State(state): State<TravelRuleState>,
    Path(tx): Path<Uuid>,
) -> Result<Json<TravelRulePayloadDto>, (StatusCode, String)> {
    let p = state
        .repo
        .get_by_transaction(tx)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "no travel rule payload".into()))?;
    Ok(Json(TravelRulePayloadDto {
        transaction_id: p.transaction_id,
        originator_name: p.originator_name,
        originator_country: p.originator_country,
        beneficiary_name: p.beneficiary_name,
        beneficiary_country: p.beneficiary_country,
        vasp_originator: p.vasp_originator,
        vasp_beneficiary: p.vasp_beneficiary,
        amount_micro_owc: p.amount_micro_owc,
        currency: p.currency,
        purpose_code: p.purpose_code,
    }))
}

fn validate_non_empty(s: &str, field: &str) -> Result<String, (StatusCode, String)> {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        return Err((StatusCode::BAD_REQUEST, format!("{field} required")));
    }
    Ok(trimmed.to_string())
}

fn validate_country(s: &str) -> Result<String, (StatusCode, String)> {
    let trimmed = s.trim().to_uppercase();
    if trimmed.len() != 2 {
        return Err((
            StatusCode::BAD_REQUEST,
            "country must be ISO 3166-1 alpha-2".into(),
        ));
    }
    if !trimmed.chars().all(|c| c.is_ascii_uppercase()) {
        return Err((StatusCode::BAD_REQUEST, "country must be A-Z".into()));
    }
    Ok(trimmed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn threshold_constant_is_1000_owc() {
        assert_eq!(TRAVEL_RULE_THRESHOLD_MICRO_OWC, 1_000_000_000);
    }

    #[test]
    fn validate_country_accepts_iso_alpha2() {
        assert_eq!(validate_country("iq").unwrap(), "IQ");
        assert_eq!(validate_country("US").unwrap(), "US");
    }

    #[test]
    fn validate_country_rejects_three_letters() {
        assert!(validate_country("IRQ").is_err());
        assert!(validate_country("").is_err());
    }
}
