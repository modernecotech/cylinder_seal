//! Admin endpoints for Iraq-specific user attributes (region, status).
//!
//! Currently scoped to the region tag; account-status freeze/block is wired
//! through a sibling endpoint set once the public surface is finalised.

use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Extension;
use axum::Json;
use cs_storage::iraq_phase2::{
    DeviceBindingRepository, Region, UserRegionRepository,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::middleware::AdminPrincipal;

#[derive(Clone)]
pub struct IraqAdminState {
    pub region_repo: Arc<dyn UserRegionRepository>,
    pub device_binding_repo: Arc<dyn DeviceBindingRepository>,
}

#[derive(Deserialize)]
pub struct SetRegionRequest {
    pub region: String, // "federal" | "krg"
}

#[derive(Serialize)]
pub struct SetRegionResponse {
    pub user_id: Uuid,
    pub previous_region: String,
    pub region: String,
}

pub async fn set_user_region(
    State(state): State<IraqAdminState>,
    Extension(actor): Extension<AdminPrincipal>,
    Path(user_id): Path<Uuid>,
    Json(req): Json<SetRegionRequest>,
) -> Result<Json<SetRegionResponse>, (StatusCode, String)> {
    if !actor.has_role("officer") {
        return Err((
            StatusCode::FORBIDDEN,
            "officer role required to retag region".into(),
        ));
    }
    let region = Region::from_str(&req.region)
        .ok_or((StatusCode::BAD_REQUEST, "region must be 'federal' or 'krg'".into()))?;
    let prev = state
        .region_repo
        .set_region(user_id, region)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(SetRegionResponse {
        user_id,
        previous_region: prev.as_str().to_string(),
        region: region.as_str().to_string(),
    }))
}

#[derive(Serialize)]
pub struct GetRegionResponse {
    pub user_id: Uuid,
    pub region: String,
}

pub async fn get_user_region(
    State(state): State<IraqAdminState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<GetRegionResponse>, (StatusCode, String)> {
    let region = state
        .region_repo
        .current(user_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(GetRegionResponse {
        user_id,
        region: region.as_str().to_string(),
    }))
}

#[derive(Deserialize)]
pub struct SetDeviceBindingRequest {
    /// Hex-encoded 32-byte hash of (SIM-serial || IMEI || keystore attestation).
    /// The mobile client recomputes this on every cold start; it should
    /// remain stable across launches unless the SIM, device, or keystore
    /// has been rotated.
    pub signature_hex: String,
}

#[derive(Serialize)]
pub struct DeviceBindingResponse {
    pub user_id: Uuid,
    pub bound: bool,
    pub cooldown_remaining_hours: Option<i64>,
}

pub async fn set_device_binding(
    State(state): State<IraqAdminState>,
    Path(user_id): Path<Uuid>,
    Json(req): Json<SetDeviceBindingRequest>,
) -> Result<Json<DeviceBindingResponse>, (StatusCode, String)> {
    let signature = hex::decode(req.signature_hex.trim())
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("invalid signature_hex: {e}")))?;
    if signature.len() != 32 {
        return Err((
            StatusCode::BAD_REQUEST,
            "device signature must be 32 bytes".into(),
        ));
    }
    state
        .device_binding_repo
        .set_signature(user_id, &signature)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let status = state
        .device_binding_repo
        .status(user_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(DeviceBindingResponse {
        user_id,
        bound: status.device_signature.is_some(),
        cooldown_remaining_hours: status.cooldown_remaining_hours,
    }))
}

pub async fn get_device_binding(
    State(state): State<IraqAdminState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<DeviceBindingResponse>, (StatusCode, String)> {
    let status = state
        .device_binding_repo
        .status(user_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(DeviceBindingResponse {
        user_id,
        bound: status.device_signature.is_some(),
        cooldown_remaining_hours: status.cooldown_remaining_hours,
    }))
}
