//! External data feed ingestion.
//!
//! Compliance depends on three classes of external data:
//!
//! 1. **Sanctions lists** — OFAC SDN (US Treasury), UN Consolidated, EU
//!    CFSP, UK HMT OFSI, and CBI Iraq's domestic list.
//! 2. **Politically Exposed Persons (PEP) lists** — typically commercial
//!    feeds (Refinitiv World-Check, Dow Jones).
//! 3. **FX/policy reference data** — CBI policy rate, IQD/USD official
//!    rate, IMF SDR basket.
//!
//! ## DMZ pattern
//!
//! Outbound HTTPS to `treasury.gov`, `un.org`, etc. is a regulated egress.
//! The feed workers **do not** run in the same network namespace as the
//! customer-facing API. In production they should run in a hardened DMZ
//! with allowlisted egress targets, write their results to Postgres
//! (`feed_runs` for audit + `sanctions_list_entries` for matched data),
//! and the API process reads only from Postgres. This crate is the
//! producer side of that boundary.
//!
//! ## Determinism
//!
//! Every run records the source URL, the SHA-256 of the response body
//! (signature), and the added/removed/unchanged counts. The signature
//! lets us detect tampering and avoid double-applying an unchanged feed.

pub mod cbi;
pub mod eu;
pub mod ofac;
pub mod scheduler;
pub mod uk;
pub mod un;
pub mod worker;

pub use scheduler::{FeedScheduler, ScheduleConfig};
pub use worker::{FeedFetchResult, FeedWorker, RawFeed};
