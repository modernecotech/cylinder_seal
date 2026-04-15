# Security Hardening Summary

## Attack Vector Mitigation Matrix

### 🔴 CRITICAL: Offline Double-Spend

| Aspect | Original | Hardened |
|--------|----------|----------|
| **Prevention** | KYC tier limits only | Vector clocks + monotonic clocks + per-device daily limits |
| **Detection** | No conflict detection | 3-peer Byzantine consensus required |
| **Escalation** | Manual review | Automatic penalty + device reputation |
| **Attacker effort** | Low (10 devices × 50 OWC each) | High (vector clock makes time-travel impossible) |

**Key change:** Vector clocks prevent attackers from making old transactions look new. Even if device clock is set backward, monotonic clock (System.nanoTime) never goes backward.

---

### 🔴 CRITICAL: Multi-Device Fraud

| Aspect | Original | Hardened |
|--------|----------|----------|
| **Device ID** | Not tracked | UUID per phone + device public key |
| **Attestation** | None | SafetyNet/Play Integrity on txs > threshold |
| **Daily limit** | Per-user only | Per-device per-day (10-50 OWC depending on tier) |
| **Detection** | None | Geographic anomalies, frequency analysis |
| **Penalty** | None | Device reputation scoring |

**Key change:** Tracking device_id separately from user_id prevents "10 devices = 10x limit" attack. Device attestation proves it's real hardware, not a cloned APK.

---

### 🔴 CRITICAL: Super-Peer Compromise

| Aspect | Original | Hardened |
|--------|----------|----------|
| **Single point of failure** | Yes (1 super-peer) | No (3 required for consensus) |
| **Confirmation rule** | Single signature | 2+ of 3 super-peer signatures |
| **Attack: fake confirmations** | Possible | Requires compromising 2+ peers |
| **Attestation credibility** | Single issuer | Threshold signatures (2-of-3) |

**Key change:** Byzantine Fault Tolerance means attacker needs 2+ compromised peers. Threshold signatures mean KYC credentials can't be forged unless 2+ super-peers are compromised.

---

### 🟠 HIGH: Nonce Reuse / Replay

| Aspect | Original | Hardened |
|--------|----------|----------|
| **Nonce generation** | Random (16 bytes) | Deterministic HMAC-SHA256 (RFC 6979) |
| **Nonce persistence** | Redis TTL 48h | Chained in ledger (permanent) |
| **Replay prevention** | Redis dedup | Hash chain (impossible to replay same nonce) |
| **Attack: replay after Redis clear** | Possible | Impossible (nonce is cryptographically tied to prev tx) |

**Key change:** Deterministic nonces prevent replay attacks even if Redis is cleared or restarted. Nonce chain is verified at device-level before signing, at super-peer level on ingestion.

---

### 🟠 HIGH: Device Compromise

| Aspect | Original | Hardened |
|--------|----------|----------|
| **Key storage** | Android Keystore | Android Keystore + Strongbox (hardware-backed) |
| **Extractability** | Possible with root | Non-extractable (Strongbox enforced) |
| **Biometric requirement** | None | Required for txs > 5-50 OWC depending on tier |
| **User auth validity** | N/A | Time-bound (15 minutes) |
| **Attestation required** | None | Yes, for txs > 5-20 OWC |

**Key change:** Strongbox-backed keys cannot be extracted even with root access. Biometric + time-limited key access means attacker needs active user cooperation or highly specialized attack.

---

### 🟠 HIGH: Clock Skew / Time Manipulation

| Aspect | Original | Hardened |
|--------|----------|----------|
| **Timestamp type** | Wallclock (settable) | Monotonic clock (non-decreasing) |
| **Verification** | Sorted by timestamp | Verified strict monotonicity |
| **Time-travel attack** | Possible (set clock backward) | Impossible (System.nanoTime never decreases) |

**Key change:** Monotonic clocks survive wallclock tampering. Even if attacker sets device date to 2020, monotonic clock continues from where it left off.

---

### 🟠 HIGH: NFC/BLE Eavesdropping

| Aspect | Original | Hardened |
|--------|----------|----------|
| **Payload encryption** | None (just CBOR) | AES-256-GCM with ECDH key agreement |
| **Authenticity** | Ed25519 signature only | ECDH encrypted + Ed25519 signed |
| **Receipt** | None | Bidirectional signed receipt |
| **Proof of payment** | Device logs only | Both parties signed receipt in ledger |

**Key change:** Encryption protects against passive eavesdropping. Bidirectional receipt means either party can prove the transaction happened.

---

### 🟡 MEDIUM: Rate Limiting

| Aspect | Original | Hardened |
|--------|----------|----------|
| **Rate limiting** | None | Per-device limits (1000 blocks/hour, 100 MB/hour) |
| **Enforcement** | N/A | gRPC middleware + Redis counters |

---

### 🟡 MEDIUM: KYC Tier Spoofing

| Aspect | Original | Hardened |
|--------|----------|----------|
| **Credential signing** | Single super-peer | Threshold signatures (2-of-3) |
| **Forgery attack** | Possible (1 key compromised) | Requires 2+ super-peer compromise |

---

### 🟡 MEDIUM: Ledger Chain Break

| Aspect | Original | Hardened |
|--------|----------|----------|
| **Sequence validation** | Soft check | Strict enforcement (reject out-of-order) |
| **Gap detection** | Possible to miss | Impossible (sequence must increment by 1) |
| **Hash chain validation** | Soft check | Strict (prev_block_hash must match) |

---

## New Security Fields in Data Models

### Transaction
```rust
pub struct Transaction {
    pub monotonic_clock_nanos: i64,      // NEW: prevents clock skew
    pub current_nonce: [u8; 32],         // NEW: deterministic, chained
    pub previous_nonce: [u8; 32],        // NEW: forms nonce chain
    pub device_id: Uuid,                 // NEW: tracks which phone signed
    pub device_attestation: Option<String>, // NEW: SafetyNet proof
}
```

### JournalEntry
```rust
pub struct JournalEntry {
    pub device_id: Uuid,                 // NEW: which phone created entry
    pub vector_clock: HashMap<Uuid, u64>, // NEW: prevents time-travel
    pub monotonic_created_nanos: i64,    // NEW: non-decreasing timestamp
    pub device_signature: [u8; 64],      // NEW: signed by device key
    pub user_signature: Option<[u8; 64]>, // NEW: optional master sig
    pub super_peer_confirmations: Vec<SuperPeerConfirmation>, // NEW: BFT consensus
}
```

### KYCTier (Enhanced)
```rust
pub enum KYCTier {
    Anonymous,     // 20 OWC max tx, 10 OWC per device per day
    PhoneVerified, // 100 OWC max tx, 50 OWC per device per day
    FullKYC,       // 500 OWC max tx, unlimited per device
}

impl KYCTier {
    pub fn attestation_threshold(&self) -> i64  // Device attestation required above
    pub fn biometric_threshold(&self) -> i64    // Biometric auth required above
    pub fn max_daily_offline_per_device(&self) -> i64  // Per-device daily limit
}
```

---

## Validation Layers (Defense in Depth)

### Layer 1: Device (Kotlin)
- KYC tier limits
- Device daily limits
- Monotonic clock validation
- Nonce chain validation
- Device attestation (SafetyNet)
- Biometric authentication

### Layer 2: First Super-Peer (Rust)
- Block hash verification
- Signature verification (device key)
- Sequence number validation
- Conflict detection (fork detection)
- Vector clock validation (no backward steps)
- Nonce chain verification
- Device daily spending check

### Layer 3: Byzantine Consensus (3 Super-Peers)
- 2+ confirmations required
- Threshold signatures for credentials
- Gossip anomaly detection

### Layer 4: Anomaly Detection
- Device reputation scoring
- Geographic consistency checks
- Clock skew detection
- Frequency analysis

---

## Attack Complexity Before & After

| Attack | Before | After | Difficulty |
|--------|--------|-------|-----------|
| Offline double-spend (same user) | Easy (just spend offline) | Very Hard (vector clocks) | 🔴 |
| Multi-device fraud (10 devices) | Easy (each has limit) | Hard (per-device daily caps) | 🔴 |
| Replay attack | Medium (capture NFC) | Hard (deterministic nonce chain) | 🟠 |
| Clock skew attack | Easy (set clock back) | Very Hard (monotonic clock) | 🔴 |
| Super-peer compromise | Medium (forge 1 sig) | Hard (need 2+ peers) | 🟠 |
| Device key extraction | Medium (jailbreak) | Very Hard (Strongbox non-extractable) | 🔴 |
| NFC interception | Medium (capture CBOR) | Hard (AES-256-GCM encrypted) | 🟠 |
| KYC spoofing | Easy (compromise 1 signer) | Hard (threshold 2-of-3) | 🔴 |

---

## Phase Implementation

### MVP (+ Hardening)
- Vector clocks in blocks
- Monotonic clocks (System.nanoTime)
- Device identity + daily limits
- Device attestation (SafetyNet)
- Deterministic nonces (RFC 6979)
- Nonce chaining
- 3-super-peer consensus for confirmation
- Rate limiting middleware

### Phase 2
- ECDH + AES-256-GCM for NFC payload
- Bidirectional payment receipts
- Threshold signatures for KYC credentials
- Device reputation scoring
- Strongbox-only key requirement

### Phase 3
- Geographic anomaly detection (ML-based)
- Hardware security modules for super-peers
- Post-quantum crypto migration prep

---

## Remaining Low-Risk Areas

✓ **Cryptographic primitives**: Ed25519, BLAKE2b, AES-256, TLS 1.3 remain solid for 20+ years  
✓ **Message format**: Protobuf + CBOR are robust standards  
✓ **Device storage**: SQLCipher AES-256 is cryptographically sound  
✓ **Identity model**: Public key as identity is sound (can't forge without private key)

---

## Summary

**Old design**: Pragmatic MVP, but vulnerable to offline fraud and multi-device attacks.

**New design**: Defense-in-depth with:
- Vector clocks for causal ordering
- Per-device spending limits and attestation
- 3-peer Byzantine consensus
- Deterministic nonce chains
- Monotonic clocks
- Device reputation tracking

**Result**: All critical attack vectors have been significantly hardened. The system is now suitable for production use with moderate transaction values and a growing user base.
