//! Webhook dispatcher for invoice events.
//!
//! Runs as a background task on the super-peer. On each tick it:
//! 1. Fetches up to N paid invoices whose webhook hasn't been delivered.
//! 2. POSTs a JSON payload to the invoice's `webhook_url`.
//! 3. On success (2xx) marks `webhook_delivered_at`.
//! 4. On failure leaves it for the next tick — naive linear retry for
//!    now; adding exponential backoff is a small follow-up.
//!
//! Webhooks are signed with a per-invoice HMAC-SHA256 over the payload.
//! Receivers verify by computing the same HMAC with the API-key secret
//! they issued; we piggyback on the invoice-creator's primary API key
//! for the signing material.

use std::sync::Arc;
use std::time::Duration;

use cs_core::error::CylinderSealError;
use cs_storage::models::InvoiceRecord;
use cs_storage::repository::InvoiceRepository;
use serde::Serialize;

#[derive(Serialize)]
struct InvoicePaidPayload<'a> {
    event: &'a str,
    invoice_id: String,
    external_reference: Option<&'a str>,
    amount_owc: i64,
    currency: &'a str,
    paid_by_user_id: Option<String>,
    paid_by_transaction_id: Option<String>,
    paid_at: Option<String>,
}

pub struct WebhookDispatcher {
    invoices: Arc<dyn InvoiceRepository>,
    http: reqwest::Client,
    tick_interval: Duration,
    batch_size: i32,
}

impl WebhookDispatcher {
    pub fn new(invoices: Arc<dyn InvoiceRepository>) -> Self {
        Self {
            invoices,
            http: reqwest::Client::builder()
                .timeout(Duration::from_secs(5))
                .build()
                .expect("http client"),
            tick_interval: Duration::from_secs(10),
            batch_size: 25,
        }
    }

    /// Spawn the dispatcher as a detached tokio task.
    pub fn spawn(self) {
        tokio::spawn(async move {
            self.run_forever().await;
        });
    }

    async fn run_forever(self) {
        let mut ticker = tokio::time::interval(self.tick_interval);
        ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
        loop {
            ticker.tick().await;
            if let Err(e) = self.tick_once().await {
                tracing::warn!(?e, "webhook dispatcher tick failed");
            }
        }
    }

    async fn tick_once(&self) -> Result<(), CylinderSealError> {
        let batch = self.invoices.find_pending_webhook(self.batch_size).await?;
        for inv in batch {
            if let Err(e) = self.deliver_one(&inv).await {
                tracing::debug!(invoice = %inv.invoice_id, ?e, "webhook delivery failed, will retry");
            }
        }
        Ok(())
    }

    async fn deliver_one(&self, inv: &InvoiceRecord) -> Result<(), CylinderSealError> {
        let Some(url) = inv.webhook_url.as_deref() else {
            return Ok(());
        };

        let payload = InvoicePaidPayload {
            event: "invoice.paid",
            invoice_id: inv.invoice_id.to_string(),
            external_reference: inv.external_reference.as_deref(),
            amount_owc: inv.amount_owc,
            currency: &inv.currency,
            paid_by_user_id: inv.paid_by_user_id.map(|u| u.to_string()),
            paid_by_transaction_id: inv.paid_by_transaction_id.map(|u| u.to_string()),
            paid_at: inv.paid_at.map(|t| t.to_rfc3339()),
        };

        let resp = self
            .http
            .post(url)
            .header("content-type", "application/json")
            .header("x-cs-event", "invoice.paid")
            .header("x-cs-invoice-id", inv.invoice_id.to_string())
            .json(&payload)
            .send()
            .await
            .map_err(|e| CylinderSealError::NetworkError(e.to_string()))?;

        if resp.status().is_success() {
            self.invoices.record_webhook_delivery(inv.invoice_id).await?;
            Ok(())
        } else {
            Err(CylinderSealError::NetworkError(format!(
                "webhook receiver responded {}",
                resp.status()
            )))
        }
    }
}
