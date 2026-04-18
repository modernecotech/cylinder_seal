//! Multi-currency wallet endpoints (IQD + USD).
//!
//! IQD is the primary unit of account; USD wallets exist because the CBI
//! still recognises USD for cross-border, oil-sector, and remittance flows.
//! All cross-currency conversions use the CBI dynamic peg
//! (`cbi_peg_rates`); we never hard-code 1300 in the serving path.

use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use chrono::NaiveDate;
use cs_storage::iraq_phase2::{CbiPegRepository, WalletBalanceRepository};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// IQD/USD peg fall-back used only when the `cbi_peg_rates` table is empty
/// (e.g. fresh installs that haven't run the seed migration). Production
/// nodes should always have at least one peg row, so this default exists
/// purely so dev environments don't 500.
pub const FALLBACK_IQD_PER_USD: i64 = 1300;

#[derive(Clone)]
pub struct WalletState {
    pub balances: Arc<dyn WalletBalanceRepository>,
    pub peg: Arc<dyn CbiPegRepository>,
}

#[derive(Serialize)]
pub struct WalletDto {
    pub currency: String,
    pub balance_micro: i64,
}

#[derive(Serialize)]
pub struct WalletsResponse {
    pub user_id: Uuid,
    pub wallets: Vec<WalletDto>,
}

pub async fn list_wallets(
    State(state): State<WalletState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<WalletsResponse>, (StatusCode, String)> {
    let rows = state
        .balances
        .list(user_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(WalletsResponse {
        user_id,
        wallets: rows
            .into_iter()
            .map(|r| WalletDto {
                currency: r.currency,
                balance_micro: r.balance_micro,
            })
            .collect(),
    }))
}

#[derive(Serialize)]
pub struct WalletBalanceDto {
    pub user_id: Uuid,
    pub currency: String,
    pub balance_micro: i64,
}

pub async fn get_wallet(
    State(state): State<WalletState>,
    Path((user_id, currency)): Path<(Uuid, String)>,
) -> Result<Json<WalletBalanceDto>, (StatusCode, String)> {
    let currency = normalise_currency(&currency)?;
    let balance_micro = state
        .balances
        .get(user_id, &currency)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(WalletBalanceDto {
        user_id,
        currency,
        balance_micro,
    }))
}

#[derive(Deserialize)]
pub struct PegQuery {
    /// Optional historical date — defaults to current peg.
    pub as_of: Option<NaiveDate>,
}

#[derive(Serialize)]
pub struct PegDto {
    pub iqd_per_usd: Decimal,
    pub effective_from: Option<NaiveDate>,
    pub cbi_circular_ref: Option<String>,
    /// `true` iff the response came from the seeded `cbi_peg_rates` table;
    /// `false` means the FALLBACK_IQD_PER_USD constant kicked in.
    pub from_cbi_table: bool,
}

pub async fn current_peg(
    State(state): State<WalletState>,
    Query(q): Query<PegQuery>,
) -> Result<Json<PegDto>, (StatusCode, String)> {
    let row = match q.as_of {
        Some(d) => state.peg.peg_on(d).await,
        None => state.peg.current().await,
    }
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(match row {
        Some(r) => PegDto {
            iqd_per_usd: r.iqd_per_usd,
            effective_from: Some(r.effective_from),
            cbi_circular_ref: r.cbi_circular_ref,
            from_cbi_table: true,
        },
        None => PegDto {
            iqd_per_usd: Decimal::from(FALLBACK_IQD_PER_USD),
            effective_from: None,
            cbi_circular_ref: None,
            from_cbi_table: false,
        },
    }))
}

#[derive(Deserialize)]
pub struct ConvertQuery {
    pub from: String,
    pub to: String,
    pub amount_micro: i64,
    pub as_of: Option<NaiveDate>,
}

#[derive(Serialize)]
pub struct ConvertResponse {
    pub from: String,
    pub to: String,
    pub source_amount_micro: i64,
    pub target_amount_micro: i64,
    pub iqd_per_usd: Decimal,
    pub effective_from: Option<NaiveDate>,
}

/// Convert between IQD and USD using the active CBI peg. Other currency
/// pairs are not supported by this endpoint — they go through cs-exchange.
pub async fn convert(
    State(state): State<WalletState>,
    Query(q): Query<ConvertQuery>,
) -> Result<Json<ConvertResponse>, (StatusCode, String)> {
    let from = normalise_currency(&q.from)?;
    let to = normalise_currency(&q.to)?;
    if q.amount_micro < 0 {
        return Err((StatusCode::BAD_REQUEST, "amount_micro must be >= 0".into()));
    }
    let row = match q.as_of {
        Some(d) => state.peg.peg_on(d).await,
        None => state.peg.current().await,
    }
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let (rate, effective_from) = match row {
        Some(r) => (r.iqd_per_usd, Some(r.effective_from)),
        None => (Decimal::from(FALLBACK_IQD_PER_USD), None),
    };

    let target_amount = match (from.as_str(), to.as_str()) {
        ("IQD", "USD") => {
            // micro-IQD → micro-USD: amount / rate (rate is IQD per USD)
            let v = Decimal::from(q.amount_micro) / rate;
            v.round().try_into().map_err(|_| {
                (StatusCode::BAD_REQUEST, "conversion overflow".into())
            })?
        }
        ("USD", "IQD") => {
            let v = Decimal::from(q.amount_micro) * rate;
            v.round().try_into().map_err(|_| {
                (StatusCode::BAD_REQUEST, "conversion overflow".into())
            })?
        }
        (a, b) if a == b => q.amount_micro,
        _ => {
            return Err((
                StatusCode::BAD_REQUEST,
                "convert endpoint only supports IQD<->USD pairs".into(),
            ))
        }
    };

    Ok(Json(ConvertResponse {
        from,
        to,
        source_amount_micro: q.amount_micro,
        target_amount_micro: target_amount,
        iqd_per_usd: rate,
        effective_from,
    }))
}

fn normalise_currency(s: &str) -> Result<String, (StatusCode, String)> {
    let upper = s.trim().to_uppercase();
    match upper.as_str() {
        "IQD" | "USD" => Ok(upper),
        _ => Err((
            StatusCode::BAD_REQUEST,
            "currency must be one of: IQD, USD".into(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fallback_peg_is_1300() {
        assert_eq!(FALLBACK_IQD_PER_USD, 1300);
    }

    #[test]
    fn currency_validation_accepts_iqd_usd() {
        assert_eq!(normalise_currency("iqd").unwrap(), "IQD");
        assert_eq!(normalise_currency(" USD ").unwrap(), "USD");
        assert!(normalise_currency("EUR").is_err());
        assert!(normalise_currency("").is_err());
    }
}
