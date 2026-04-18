//! gRPC `BusinessApi` service.
//!
//! Counterpart to the REST endpoints in `cs-api`. Mobile business apps
//! may prefer this gRPC-native interface — it streams natively and
//! shares TLS with the ChainSync stream. Bearer-token auth is expected
//! to come through a gRPC interceptor that sets a `BusinessPrincipal` on
//! the request extensions; this scaffold uses the metadata key
//! `x-cs-api-key`.

use std::sync::Arc;

use chrono::Utc;
use cs_core::cryptography;
use cs_storage::models::{BusinessProfileRecord, InvoiceRecord};
use cs_storage::repository::{
    ApiKeyRepository, BusinessProfileRepository, InvoiceRepository, UserRepository,
};
use tonic::{Request, Response, Status};
use uuid::Uuid;

use crate::proto::chain_sync as pb;

pub struct BusinessApiService {
    users: Arc<dyn UserRepository>,
    profiles: Arc<dyn BusinessProfileRepository>,
    api_keys: Arc<dyn ApiKeyRepository>,
    invoices: Arc<dyn InvoiceRepository>,
}

impl BusinessApiService {
    pub fn new(
        users: Arc<dyn UserRepository>,
        profiles: Arc<dyn BusinessProfileRepository>,
        api_keys: Arc<dyn ApiKeyRepository>,
        invoices: Arc<dyn InvoiceRepository>,
    ) -> Self {
        Self {
            users,
            profiles,
            api_keys,
            invoices,
        }
    }

    /// Pull the `x-cs-api-key` metadata header, BLAKE2b-256 it, look up
    /// the API key record, and return `(user_id, scopes)` if valid.
    async fn authenticate(&self, req_meta: &tonic::metadata::MetadataMap) -> Result<Uuid, Status> {
        let value = req_meta
            .get("x-cs-api-key")
            .ok_or_else(|| Status::unauthenticated("missing x-cs-api-key"))?;
        let token = value
            .to_str()
            .map_err(|_| Status::unauthenticated("non-ASCII x-cs-api-key"))?;
        let secret_hex = token
            .strip_prefix("cs_sk_")
            .ok_or_else(|| Status::unauthenticated("expected cs_sk_<hex>"))?;
        let secret = hex::decode(secret_hex)
            .map_err(|_| Status::unauthenticated("malformed token hex"))?;
        if secret.len() != 32 {
            return Err(Status::unauthenticated("token must be 32 bytes"));
        }
        let hash = cryptography::blake2b_256(&secret);
        let record = self
            .api_keys
            .find_by_hash(&hash)
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .ok_or_else(|| Status::permission_denied("api key revoked or unknown"))?;
        let _ = self.api_keys.touch(record.id).await;
        Ok(record.user_id)
    }
}

#[tonic::async_trait]
impl pb::business_api_server::BusinessApi for BusinessApiService {
    async fn register_business(
        &self,
        request: Request<pb::BusinessRegistrationRequest>,
    ) -> Result<Response<pb::BusinessRegistrationResponse>, Status> {
        let req = request.into_inner();

        // Translate proto account_type.
        let type_str = match pb::AccountType::try_from(req.account_type)
            .unwrap_or(pb::AccountType::Unspecified)
        {
            pb::AccountType::BusinessPos => "business_pos",
            pb::AccountType::BusinessElectronic => "business_electronic",
            _ => {
                return Err(Status::invalid_argument(
                    "account_type must be BusinessPos or BusinessElectronic",
                ));
            }
        };

        // The account's primary key must already exist as a user.
        if req.account_public_key.len() != 32 {
            return Err(Status::invalid_argument("account_public_key must be 32 bytes"));
        }
        let mut pk = [0u8; 32];
        pk.copy_from_slice(&req.account_public_key);
        let user_id = cs_core::models::User::derive_user_id_from_public_key(&pk);

        let Some(mut user) = self
            .users
            .get_user(user_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?
        else {
            return Err(Status::not_found("user not found"));
        };
        user.account_type = type_str.to_string();
        user.updated_at = Utc::now();
        self.users
            .upsert_user(&user)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let signer_hexes: Vec<String> = req
            .authorized_signer_public_keys
            .iter()
            .filter(|k| k.len() == 32)
            .map(hex::encode)
            .collect();

        let profile = BusinessProfileRecord {
            user_id,
            legal_name: req.legal_name,
            commercial_registration_id: req.commercial_registration_id,
            tax_id: req.tax_id,
            industry_code: req.industry_code,
            registered_address: req.registered_address,
            contact_email: req.contact_email,
            authorized_signer_public_keys: serde_json::json!(signer_hexes),
            signature_threshold: 1,
            multisig_threshold_owc: None,
            daily_volume_limit_owc: 5_000_000_000_000,
            per_transaction_limit_owc: None,
            edd_cleared: false,
            approved_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        self.profiles
            .upsert(&profile)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(pb::BusinessRegistrationResponse {
            status: pb::business_registration_response::Status::PendingReview as i32,
            user_id: user_id.as_bytes().to_vec(),
            reason: String::new(),
        }))
    }

    async fn get_business_profile(
        &self,
        request: Request<pb::GetBusinessProfileRequest>,
    ) -> Result<Response<pb::BusinessProfile>, Status> {
        let meta = request.metadata().clone();
        let req = request.into_inner();
        let caller = self.authenticate(&meta).await?;

        if req.user_id.len() != 16 {
            return Err(Status::invalid_argument("user_id must be 16 bytes"));
        }
        let mut arr = [0u8; 16];
        arr.copy_from_slice(&req.user_id);
        let user_id = Uuid::from_bytes(arr);
        if user_id != caller {
            return Err(Status::permission_denied("user_id mismatch"));
        }

        let p = self
            .profiles
            .get(user_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .ok_or_else(|| Status::not_found("business profile not found"))?;

        Ok(Response::new(record_to_pb(&p)))
    }

    async fn update_business_profile(
        &self,
        request: Request<pb::BusinessProfile>,
    ) -> Result<Response<pb::BusinessProfile>, Status> {
        let meta = request.metadata().clone();
        let incoming = request.into_inner();
        let caller = self.authenticate(&meta).await?;

        if incoming.user_id.len() != 16 {
            return Err(Status::invalid_argument("user_id must be 16 bytes"));
        }
        let mut arr = [0u8; 16];
        arr.copy_from_slice(&incoming.user_id);
        let user_id = Uuid::from_bytes(arr);
        if user_id != caller {
            return Err(Status::permission_denied("user_id mismatch"));
        }

        // Load existing then overlay the caller-provided fields.
        let mut existing = self
            .profiles
            .get(user_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .ok_or_else(|| Status::not_found("business profile not found"))?;

        if !incoming.legal_name.is_empty() {
            existing.legal_name = incoming.legal_name;
        }
        if !incoming.contact_email.is_empty() {
            existing.contact_email = incoming.contact_email;
        }
        if !incoming.registered_address.is_empty() {
            existing.registered_address = incoming.registered_address;
        }
        let signer_hexes: Vec<String> = incoming
            .authorized_signer_public_keys
            .iter()
            .filter(|k| k.len() == 32)
            .map(hex::encode)
            .collect();
        if !signer_hexes.is_empty() {
            existing.authorized_signer_public_keys = serde_json::json!(signer_hexes);
        }
        if incoming.signature_threshold > 0 {
            existing.signature_threshold = incoming.signature_threshold as i16;
        }
        if incoming.multisig_threshold_owc > 0 {
            existing.multisig_threshold_owc = Some(incoming.multisig_threshold_owc);
        }
        existing.updated_at = Utc::now();
        self.profiles
            .upsert(&existing)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(record_to_pb(&existing)))
    }

    async fn create_invoice(
        &self,
        request: Request<pb::CreateInvoiceRequest>,
    ) -> Result<Response<pb::CreateInvoiceResponse>, Status> {
        let meta = request.metadata().clone();
        let req = request.into_inner();
        let caller = self.authenticate(&meta).await?;

        let inv_req = req
            .invoice
            .ok_or_else(|| Status::invalid_argument("missing invoice"))?;
        if inv_req.amount_owc <= 0 {
            return Err(Status::invalid_argument("amount_owc must be > 0"));
        }

        let invoice_id = if inv_req.invoice_id.len() == 16 {
            let mut a = [0u8; 16];
            a.copy_from_slice(&inv_req.invoice_id);
            Uuid::from_bytes(a)
        } else {
            Uuid::now_v7()
        };

        let now = Utc::now();
        let expires_at = if inv_req.expires_at > 0 {
            chrono::DateTime::<Utc>::from_timestamp_micros(inv_req.expires_at)
                .unwrap_or_else(|| now + chrono::Duration::hours(1))
        } else {
            now + chrono::Duration::hours(1)
        };

        let record = InvoiceRecord {
            invoice_id,
            user_id: caller,
            amount_owc: inv_req.amount_owc,
            currency: inv_req.currency.clone(),
            description: Some(inv_req.description).filter(|s| !s.is_empty()),
            external_reference: Some(inv_req.external_reference).filter(|s| !s.is_empty()),
            status: "open".to_string(),
            paid_by_user_id: None,
            paid_by_transaction_id: None,
            webhook_url: Some(inv_req.webhook_url).filter(|s| !s.is_empty()),
            webhook_delivered_at: None,
            created_at: now,
            expires_at,
            paid_at: None,
            merchant_tax_id: None,
            withholding_pct: rust_decimal::Decimal::ZERO,
            fiscal_receipt_ref: None,
        };
        self.invoices
            .create(&record)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(pb::CreateInvoiceResponse {
            invoice_id: invoice_id.as_bytes().to_vec(),
            payment_uri: format!("CS1:INV:{}", hex::encode_upper(invoice_id.as_bytes())),
            expires_at: expires_at.timestamp_micros(),
        }))
    }

    async fn get_invoice_status(
        &self,
        request: Request<pb::InvoiceStatusRequest>,
    ) -> Result<Response<pb::InvoiceStatusResponse>, Status> {
        let meta = request.metadata().clone();
        let req = request.into_inner();
        let caller = self.authenticate(&meta).await?;

        if req.invoice_id.len() != 16 {
            return Err(Status::invalid_argument("invoice_id must be 16 bytes"));
        }
        let mut arr = [0u8; 16];
        arr.copy_from_slice(&req.invoice_id);
        let invoice_id = Uuid::from_bytes(arr);

        let inv = self
            .invoices
            .get(invoice_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .ok_or_else(|| Status::not_found("invoice not found"))?;
        if inv.user_id != caller {
            return Err(Status::not_found("invoice not found"));
        }

        let status = match inv.status.as_str() {
            "open" => pb::invoice_status_response::Status::Open,
            "paid" => pb::invoice_status_response::Status::Paid,
            "expired" => pb::invoice_status_response::Status::Expired,
            "cancelled" => pb::invoice_status_response::Status::Cancelled,
            _ => pb::invoice_status_response::Status::Unspecified,
        };

        Ok(Response::new(pb::InvoiceStatusResponse {
            status: status as i32,
            paying_user_id: inv
                .paid_by_user_id
                .map(|u| u.as_bytes().to_vec())
                .unwrap_or_default(),
            paying_transaction_id: inv
                .paid_by_transaction_id
                .map(|u| u.as_bytes().to_vec())
                .unwrap_or_default(),
            paid_at: inv.paid_at.map(|t| t.timestamp_micros()).unwrap_or_default(),
        }))
    }
}

fn record_to_pb(p: &BusinessProfileRecord) -> pb::BusinessProfile {
    let signers: Vec<Vec<u8>> = p
        .authorized_signer_public_keys
        .as_array()
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .filter_map(|v| v.as_str().and_then(|s| hex::decode(s).ok()))
        .filter(|b| b.len() == 32)
        .collect();

    pb::BusinessProfile {
        user_id: p.user_id.as_bytes().to_vec(),
        legal_name: p.legal_name.clone(),
        commercial_registration_id: p.commercial_registration_id.clone(),
        tax_id: p.tax_id.clone(),
        industry_code: p.industry_code.clone(),
        registered_address: p.registered_address.clone(),
        contact_email: p.contact_email.clone(),
        authorized_signer_public_keys: signers,
        signature_threshold: p.signature_threshold.max(0) as u32,
        multisig_threshold_owc: p.multisig_threshold_owc.unwrap_or(0),
        daily_volume_limit_owc: p.daily_volume_limit_owc,
        per_transaction_limit_owc: p.per_transaction_limit_owc.unwrap_or(0),
        edd_cleared: p.edd_cleared,
        approved_at: p.approved_at.map(|t| t.timestamp_micros()).unwrap_or_default(),
        created_at: p.created_at.timestamp_micros(),
    }
}
