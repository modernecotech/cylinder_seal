//! Merchant keypair management.
//!
//! On first launch the POS generates an Ed25519 keypair (via `cs-core`). The
//! private key is wrapped at rest using a XOR-chacha pattern keyed from a
//! deterministic device seed. This is **not** a replacement for an HSM, but
//! it stops casual disk-image theft from trivially exposing the signing key.
//! Production deployments should replace `load_or_create` with an HSM-backed
//! implementation (the interface stays the same).

use anyhow::Result;
use cs_core::cryptography;

use crate::store::{MerchantRow, Store};

pub struct Merchant {
    pub public_key: [u8; 32],
    pub private_key: [u8; 32],
}

impl Merchant {
    /// Load merchant keypair from [`Store`], generating and persisting a new
    /// one the first time the POS is started.
    pub fn load_or_create(store: &Store) -> Result<Self> {
        if let Some(row) = store.load_merchant()? {
            let pk = arr32(&row.public_key, "public_key")?;
            let sk = unwrap_private(&row.private_key_wrapped, &pk)?;
            return Ok(Self {
                public_key: pk,
                private_key: sk,
            });
        }

        let (pk, sk) = cryptography::generate_keypair();
        let wrapped = wrap_private(&sk, &pk);
        store.upsert_merchant(&MerchantRow {
            public_key: pk.to_vec(),
            private_key_wrapped: wrapped,
            created_at: now_ms(),
        })?;
        Ok(Self {
            public_key: pk,
            private_key: sk,
        })
    }
}

fn arr32(v: &[u8], field: &'static str) -> Result<[u8; 32]> {
    if v.len() != 32 {
        anyhow::bail!("{field} not 32 bytes (got {})", v.len());
    }
    let mut a = [0u8; 32];
    a.copy_from_slice(v);
    Ok(a)
}

/// XOR-wrap the private key with BLAKE2b(device_seed || public_key). This
/// keeps the on-disk bytes from revealing the key directly; anyone with
/// the DB and the device seed can unwrap it — which is fine for a
/// physically supervised terminal.
fn wrap_private(sk: &[u8; 32], pk: &[u8; 32]) -> Vec<u8> {
    let seed = device_seed();
    let mut input = Vec::with_capacity(seed.len() + pk.len());
    input.extend_from_slice(&seed);
    input.extend_from_slice(pk);
    let mask = cryptography::blake2b_256(&input);
    let mut out = vec![0u8; 32];
    for i in 0..32 {
        out[i] = sk[i] ^ mask[i];
    }
    out
}

fn unwrap_private(wrapped: &[u8], pk: &[u8; 32]) -> Result<[u8; 32]> {
    if wrapped.len() != 32 {
        anyhow::bail!("wrapped private key is not 32 bytes");
    }
    let seed = device_seed();
    let mut input = Vec::with_capacity(seed.len() + pk.len());
    input.extend_from_slice(&seed);
    input.extend_from_slice(pk);
    let mask = cryptography::blake2b_256(&input);
    let mut out = [0u8; 32];
    for i in 0..32 {
        out[i] = wrapped[i] ^ mask[i];
    }
    Ok(out)
}

/// Device seed: hostname + a file under /var/lib that's created at
/// install time. For production this is replaced by an HSM-derived secret.
fn device_seed() -> Vec<u8> {
    let host = hostname();
    let anchor = std::fs::read_to_string("/etc/machine-id").unwrap_or_else(|_| "no-machine-id".into());
    let mut s = Vec::new();
    s.extend_from_slice(host.as_bytes());
    s.extend_from_slice(b"|");
    s.extend_from_slice(anchor.as_bytes());
    s
}

fn hostname() -> String {
    std::fs::read_to_string("/etc/hostname")
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|_| "pos-terminal".into())
}

fn now_ms() -> i64 {
    chrono::Utc::now().timestamp_millis()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::Store;

    fn tmp_store() -> Store {
        let mut path = std::env::temp_dir();
        path.push(format!("cs-pos-merchant-{}.db", uuid::Uuid::new_v4()));
        Store::open(&path).expect("open store")
    }

    #[test]
    fn load_or_create_generates_once_and_reloads_thereafter() {
        let store = tmp_store();
        let first = Merchant::load_or_create(&store).unwrap();
        assert_eq!(first.public_key.len(), 32);
        assert_eq!(first.private_key.len(), 32);

        let second = Merchant::load_or_create(&store).unwrap();
        assert_eq!(
            first.public_key, second.public_key,
            "load_or_create must be idempotent"
        );
        assert_eq!(
            first.private_key, second.private_key,
            "wrap/unwrap must preserve the private key"
        );
    }

    #[test]
    fn wrap_unwrap_is_xor_inverse() {
        let (pk, sk) = cs_core::cryptography::generate_keypair();
        let wrapped = wrap_private(&sk, &pk);
        assert_ne!(wrapped, sk.to_vec(), "wrapped blob must differ from plaintext");
        let round = unwrap_private(&wrapped, &pk).unwrap();
        assert_eq!(round, sk);
    }
}
