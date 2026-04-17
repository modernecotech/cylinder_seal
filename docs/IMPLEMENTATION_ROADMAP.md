# Iron-Grade Security Implementation Roadmap

## Executive Summary

**Before**: Pragmatic MVP, vulnerable to sophisticated attacks  
**After**: Bank-grade security, seamless UX for users  
**Timeline**: 16 weeks (MVP + hardening)  
**Team**: 2 Rust engineers + 2 Android engineers

---

## Phase 1: MVP Core (Weeks 1-4)

### Week 1: Rust & Protocol Foundation
- [ ] `cs-core` compiles with all hardened models
- [ ] Vector clocks, device IDs, monotonic clocks in Transaction/JournalEntry
- [ ] Deterministic nonce derivation (RFC 6979 + HMAC)
- [ ] Hardware-bound nonce binding (device serial)

**Files:**
- ✅ `crates/cs-core/src/models.rs` (already updated)
- `crates/cs-core/src/nonce.rs` (NEW)
- `crates/cs-core/src/hardware_binding.rs` (NEW)

### Week 2: Android Keystore & Crypto
- [ ] Keystore integration (Tink + Strongbox)
- [ ] Deterministic nonce generation on device
- [ ] Hardware ID collection (serial, IMEI)
- [ ] Device attestation (SafetyNet/Play Integrity)

**Files:**
- `android/core/core-crypto/src/Keystore.kt` (NEW)
- `android/core/core-crypto/src/NonceDerivation.kt` (NEW)
- `android/core/core-crypto/src/HardwareIdentifier.kt` (NEW)

### Week 3: PostgreSQL Schema & Storage
- [ ] Device table (device_id, public_key, attestation, reputation)
- [ ] Device daily spending limits (checked at ledger ingestion)
- [ ] Conflict log (quarantined blocks)
- [ ] Audit log (immutable append-only)

**SQL:**
```sql
CREATE TABLE devices (
    device_id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users,
    device_public_key BYTEA NOT NULL,
    hardware_serial VARCHAR(255),
    hardware_imei VARCHAR(255),
    last_attestation JSONB,
    reputation_score SMALLINT DEFAULT 50,
    created_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(user_id, hardware_serial)
);

CREATE TABLE device_daily_spending (
    device_id UUID,
    user_id UUID,
    spending_date DATE,
    amount_spent_owc BIGINT DEFAULT 0,
    PRIMARY KEY(device_id, spending_date)
);

CREATE TABLE audit_log (
    entry_id BIGSERIAL PRIMARY KEY,
    sequence BIGINT UNIQUE NOT NULL,
    timestamp TIMESTAMP NOT NULL,
    monotonic_nanos BIGINT NOT NULL,
    action VARCHAR(50) NOT NULL,
    user_id UUID REFERENCES users,
    details JSONB,
    signed_by_peers TEXT[],
    signatures BYTEA[] NOT NULL,
    prev_hash BYTEA NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);
```

### Week 4: gRPC Sync Service (Single Super-Peer MVP)
- [ ] `SyncChain` bidirectional stream implemented
- [ ] Entry validation (sequence, hash, signature)
- [ ] Conflict detection (fork detection)
- [ ] Device daily limit checking
- [ ] Audit log recording

**Files:**
- `crates/cs-sync/src/sync_service.rs` (EXPAND)
- `crates/cs-sync/src/validation.rs` (NEW)
- `crates/cs-sync/src/conflict_detector.rs` (NEW)

---

## Phase 2: Hardened Operations (Weeks 5-10)

### Week 5: Key Rotation Protocol
**Android:**
- [ ] Background task: check if key rotation needed (every 28 days)
- [ ] Generate new keypair + rotation certificate
- [ ] Device attestation in certificate
- [ ] Submit rotation to super-peer

**Rust:**
- [ ] Validate rotation certificate (old key signature)
- [ ] Accept both old and new keys for 7 days
- [ ] Schedule old key expiration after 7 days
- [ ] Audit log recording

**Files:**
- `android/feature/feature-settings/src/KeyRotationManager.kt` (NEW)
- `crates/cs-sync/src/key_rotation.rs` (NEW)

### Week 6: User Key Recovery (Shamir Sharing)
**Android:**
- [ ] Onboarding: generate master key + 5 shares (3-of-5 threshold)
- [ ] Encrypt shares with different keys
- [ ] Send 3 shares to trusted contacts (via secure channel)
- [ ] Store 2 shares on super-peer (encrypted separately)
- [ ] Local backup (encrypted with different key)

**Rust:**
- [ ] Store recovery shares (encrypted, access-controlled)
- [ ] Recovery request handler (needs identity verification)
- [ ] Reconstruct private key from 3+ shares
- [ ] Auto-rotate device key post-recovery

**Files:**
- `android/core/core-crypto/src/ShamirSharing.kt` (NEW)
- `android/feature/feature-onboarding/src/RecoverySetup.kt` (NEW)
- `crates/cs-sync/src/key_recovery.rs` (NEW)

### Week 7: End-to-End Encryption
**Android:**
- [ ] Encrypt transactions with user's master key (AES-256-GCM)
- [ ] Include plaintext_hash for dedup
- [ ] Sign ciphertext (device key)

**Rust:**
- [ ] Process encrypted transactions (can't decrypt)
- [ ] Verify structure + signatures
- [ ] Detect replays by plaintext_hash
- [ ] Store encrypted transactions

**Files:**
- `android/core/core-crypto/src/TransactionEncryption.kt` (NEW)
- `crates/cs-sync/src/encryption.rs` (NEW)

### Week 8: 5-Super-Peer Byzantine Consensus
**Deployment:**
- [ ] Deploy 5 super-peer instances (different locations)
- [ ] Each with dedicated HSM (Thales Luna)
- [ ] Gossip protocol between peers
- [ ] Quorum: 3-of-5 required for confirmation

**Rust:**
- [ ] Consensus protocol (propose entry to all 5)
- [ ] Signature aggregation (need 3+)
- [ ] Gossip announcements
- [ ] Conflict resolution (deterministic)

**Files:**
- `crates/cs-sync/src/consensus.rs` (NEW)
- `crates/cs-sync/src/gossip.rs` (EXPAND)
- `crates/cs-node/src/distributed_setup.rs` (NEW)

### Week 9: Device Reputation & ML Anomaly Detection
**Rust:**
- [ ] Compute device reputation score (0-100)
- [ ] Factor: days active, tx count, geographic consistency, conflicts
- [ ] ML-based anomaly detection (external service or local model)
- [ ] Auto-freeze suspicious devices
- [ ] Audit log of reputation changes

**Files:**
- `crates/cs-credit/src/reputation.rs` (NEW)
- `crates/cs-sync/src/anomaly_detection.rs` (NEW)

### Week 10: Audit Logging & Compliance
**Rust:**
- [ ] Immutable append-only log (signed by 3+ super-peers)
- [ ] Every action recorded (blocks, conflicts, key rotations)
- [ ] Audit log query endpoint
- [ ] Export for regulators

**Android:**
- [ ] Request and display user's audit log
- [ ] Verify signatures + chain integrity
- [ ] Allow export/screenshot as proof

**Files:**
- `crates/cs-sync/src/audit.rs` (NEW)
- `android/feature/feature-settings/src/AuditLogViewer.kt` (NEW)

---

## Phase 3: User Experience & Security Tiers (Weeks 11-14)

### Week 11: Graduated Security & Biometric Auth
**Android:**
- [ ] Risk assessment on each transaction
- [ ] Biometric required for txs > 20 OWC
- [ ] 2FA (SMS/email) for txs > 100 OWC
- [ ] Super-peer approval for txs > 500 OWC

**Files:**
- `android/feature/feature-pay/src/SecurityTierEvaluation.kt` (NEW)
- `android/feature/feature-pay/src/BiometricAuth.kt` (NEW)

### Week 12: Transaction Witnesses
**Android:**
- [ ] For txs > 500 OWC: request witness approval
- [ ] Select trusted contact or super-peer
- [ ] Push notification to witness
- [ ] Witness approves with biometric
- [ ] Witness signature included in transaction

**Rust:**
- [ ] Validate witness signature for large txs
- [ ] Check witness reputation
- [ ] Audit log recording

**Files:**
- `android/feature/feature-pay/src/WitnessFlow.kt` (NEW)
- `crates/cs-sync/src/witness_validation.rs` (NEW)

### Week 13: Merkle Proofs & Balance Verification
**Rust:**
- [ ] Build Merkle tree of transactions
- [ ] Generate Merkle proofs for user requests
- [ ] Commit root hash (signed by 3+ super-peers)
- [ ] Publish commitments

**Android:**
- [ ] Request Merkle proof for user's transactions
- [ ] Verify proof against super-peer root hash
- [ ] Show "cryptographically verified" badge

**Files:**
- `crates/cs-sync/src/merkle.rs` (NEW)
- `android/feature/feature-wallet/src/BalanceVerification.kt` (NEW)

### Week 14: Encrypted E2E Communication & NFC
**Android:**
- [ ] ECDH key agreement for NFC/BLE
- [ ] AES-256-GCM encryption of transaction payload
- [ ] Bidirectional signed receipts
- [ ] NFC HCE implementation

**Files:**
- `android/feature/feature-pay/src/NFCEncryption.kt` (NEW)
- `android/feature/feature-pay/src/BLEEncryption.kt` (NEW)

---

## Phase 4: Integration & Hardening (Weeks 15-16)

### Week 15: End-to-End Testing
- [ ] Device creates transaction → encrypts → signs
- [ ] Device submits to super-peer → gets 3+ confirmations
- [ ] Second device receives encrypted transaction
- [ ] Decrypts and verifies signature
- [ ] Both devices in same blockchain (offline)
- [ ] Both devices in different blockchains (online sync reconciles)

**Test scenarios:**
- ✅ Offline double-spend attempt (caught by vector clock)
- ✅ Multi-device fraud (caught by per-device daily limit)
- ✅ Replay attack (caught by nonce chain)
- ✅ Clock skew (caught by monotonic clock)
- ✅ Super-peer compromise (caught by 3-of-5 quorum)
- ✅ Device key leak (caught by key rotation + auto-remediation)

### Week 16: Documentation & Deployment
- [ ] Security audit checklist (internal)
- [ ] Deployment guide for 5 super-peers
- [ ] User onboarding docs
- [ ] Regulatory compliance docs (for regulators)
- [ ] Runbook for incident response

---

## Implementation Details by Component

### 1. cs-core (Foundational Types)

**New files:**
```
crates/cs-core/src/
├── models.rs                 (✅ UPDATED)
├── crypto.rs                 (✅ UPDATED)
├── nonce.rs                  (NEW - Deterministic RFC 6979)
├── hardware_binding.rs        (NEW - Device serial binding)
└── merkle.rs                 (NEW - Merkle tree proofs)
```

**Key functions:**
```rust
// Deterministic nonce with hardware binding
pub fn derive_nonce_with_hardware(
    previous_nonce: [u8; 32],
    device_hw_ids: &HardwareIds,
    counter: u64,
) -> [u8; 32]

// Merkle tree
pub fn build_merkle_tree(transactions: &[Transaction]) -> MerkleRoot
pub fn generate_merkle_proof(leaf_idx: usize, tree: &MerkleTree) -> MerkleProof
pub fn verify_merkle_proof(proof: &MerkleProof, leaf: &Transaction) -> bool
```

### 2. cs-sync (Byzantine Consensus)

**New files:**
```
crates/cs-sync/src/
├── sync_service.rs           (✅ STUB - implement full)
├── validation.rs             (NEW - Validation rules)
├── conflict_detector.rs       (NEW - Deterministic resolution)
├── key_rotation.rs           (NEW - Device key rotation)
├── key_recovery.rs           (NEW - Shamir share reconstruction)
├── consensus.rs              (NEW - 5-peer BFT)
├── gossip.rs                 (EXPAND - peer-to-peer)
├── reputation.rs             (NEW - Device scoring)
├── anomaly_detection.rs       (NEW - ML-based)
├── audit.rs                  (NEW - Immutable logging)
├── encryption.rs             (NEW - E2E encryption handling)
├── witness_validation.rs      (NEW - Large tx witness sigs)
└── merkle.rs                 (NEW - Merkle proofs)
```

### 3. Android Implementation

**Core modules:**
```
android/core/core-crypto/src/
├── Keystore.kt               (NEW - Android Keystore + Strongbox)
├── NonceDerivation.kt        (NEW - RFC 6979 deterministic)
├── HardwareIdentifier.kt     (NEW - Collect device IDs)
├── ShamirSharing.kt          (NEW - 3-of-5 threshold sharing)
├── TransactionEncryption.kt  (NEW - AES-256-GCM)
└── MerkleVerification.kt     (NEW - Balance verification)

android/feature/feature-pay/src/
├── SecurityTierEvaluation.kt (NEW - Risk-based auth)
├── BiometricAuth.kt          (NEW - Fingerprint/FaceID)
├── WitnessFlow.kt            (NEW - Large tx approval)
├── NFCEncryption.kt          (NEW - ECDH + AES-256-GCM)
└── BLEEncryption.kt          (NEW - Custom GATT)

android/feature/feature-settings/src/
├── KeyRotationManager.kt     (NEW - Auto key rotation)
└── AuditLogViewer.kt         (NEW - Display audit trail)
```

---

## Security Verification Checklist

**Before deployment, verify:**

- [ ] Vector clocks prevent backward causality
- [ ] Monotonic clocks survive wallclock tampering
- [ ] Deterministic nonce chain is unbreakable
- [ ] Hardware-bound nonces catch device cloning
- [ ] Per-device daily limits prevent multi-device fraud
- [ ] Key rotation works seamlessly
- [ ] Key recovery works with 3-of-5 shares
- [ ] End-to-end encryption is transparent to users
- [ ] 5-peer BFT survives 2-node compromise
- [ ] Device attestation blocks jailbroken devices
- [ ] Audit logs are immutable
- [ ] Merkle proofs verify correctly
- [ ] Conflict resolution is fully deterministic

---

## Risk Mitigation

**If super-peer is compromised:**
- ✅ 2+ of 5 super-peers needed to form quorum
- ✅ Transactions are E2E encrypted (super-peer can't read)
- ✅ Audit logs are signed by all (attacker can't edit history)

**If device key is leaked:**
- ✅ Automatic key rotation (30 days)
- ✅ Device reputation drops (limits damage)
- ✅ Key recovery allows user to regain access
- ✅ Witness requirement for large txs (co-approval)

**If user loses device:**
- ✅ Shamir share recovery (3-of-5 threshold)
- ✅ Identity verification (biometric + SMS + ID upload)
- ✅ Super-peer holds 2 shares as backup
- ✅ Trusted contacts hold 3 shares

---

## Cost Estimate

| Component | Effort | Cost (Staff) |
|-----------|--------|---|
| MVP Core (Weeks 1-4) | 160 hours | $16K |
| Hardened Ops (Weeks 5-10) | 240 hours | $24K |
| UX & Tiers (Weeks 11-14) | 160 hours | $16K |
| Integration & Deploy (Weeks 15-16) | 80 hours | $8K |
| **Total** | **640 hours** | **$64K** (4 engineers, 16 weeks) |

**Infrastructure:**
- 5 × HSM (Thales Luna): ~$10K each = $50K
- 5 × VPS + DDoS protection: ~$5K/month
- PostgreSQL managed service: ~$1K/month
- Redis managed service: ~$500/month

---

## Success Criteria

✅ **Security:**
- Zero successful double-spends in production
- Zero unauthorized transactions
- Zero key compromises (non-emergency)

✅ **Operations:**
- Key rotation happens automatically, zero user complaints
- Device recovery succeeds 99%+ of the time
- Audit logs pass regulatory audit

✅ **UX:**
- Users don't think about security
- 90% of transactions require < 5 seconds
- Fraud detection has < 1% false positive rate

---

## Phase 2: Monetary Policy & Governance (Weeks 5-9, After MVP Stabilizes)

**See:**
- **[MONETARY_POLICY_SPECIFICATION.md](../MONETARY_POLICY_SPECIFICATION.md)**
- **[GOVERNANCE_FRAMEWORK.md](../GOVERNANCE_FRAMEWORK.md)**  
- **[RECOVERY_AND_KEY_ROTATION.md](../RECOVERY_AND_KEY_ROTATION.md)**

After MVP is stable and payment consensus is proven, formalize:

| Timeline | Deliverable | Impact |
|----------|-------------|--------|
| Weeks 5-6 | Reserve attestation system + CR dashboard | Weekly public reporting of reserves vs. circulating supply |
| Weeks 6-7 | Governance framework (parameter registry + approval tiers) | Non-technical policy changes can't bypass multi-party approval |
| Week 7-8 | Social recovery delegates (3-of-5 threshold) | Users can recover lost phones via trusted contacts |
| Week 8-9 | Key rotation + compromise response | Users can rotate keys + freeze compromised device instantly |

**Governance Committees formed:**
- Policy Committee (2 CylinderSeal + 2 MFI + 1 independent)
- Risk Committee (1 CFO + 1 auditor + 1 MFI expert)
- Federation Quorum (5 super-peer operators, advisory on technical changes)

**Key metrics**:
- [ ] CR (reserve coverage) ≥ 1.08, published weekly
- [ ] All policy changes captured in signed governance records
- [ ] Social recovery success rate >95%
- [ ] Zero compromised-account incidents due to missing key rotation

---

## Phase 3: Super-Peer Accountability & Federation (Weeks 10-16, Before Open Scaling)

**See: [SUPER_PEER_ACCOUNTABILITY.md](../SUPER_PEER_ACCOUNTABILITY.md)**

As system scales beyond initial 5 super-peers, formalize accountability:

| Timeline | Deliverable | Impact |
|----------|-------------|--------|
| Weeks 10-11 | Slashing framework (double-sign detection, penalties) | Bad validators removed from quorum, performance bonds slashed |
| Weeks 11-12 | Evidence verification pipeline | Cryptographic proof required for all violations |
| Weeks 12-13 | Appeal procedures + independent arbitration | Validators can contest violations; fairness ensured |
| Weeks 13-14 | SLA monitoring (99.5% uptime, <30s latency) | Dashboard shows super-peer performance vs. targets |
| Weeks 14-16 | Emergency procedures (crisis declaration, temporary powers) | System survives super-peer outages, coordinated attacks |

**Super-Peer Levels:**
- **Level 1 Penalty**: Warning + reputation reduction (reversible)
- **Level 2 Penalty**: Voting-weight reduction for 4-12 weeks (recoverable with remediation)
- **Level 3 Penalty**: Performance bond slash + ejection (requires 90-day cooldown + governance reinstatement)

**Federation Growth:**
- Weeks 10-16: Operate with initial 5 super-peers
- Weeks 16+: Expand to 7-10 nodes via open federation licensing (NGOs, telcos, governments can operate nodes)

---

## Phase 5: Marketplace MVP (Weeks 17-24, Post-Security Deployment)

**See: [MARKETPLACE_IMPLEMENTATION.md](MARKETPLACE_IMPLEMENTATION.md)**

After security hardening is complete and the payment ledger is production-proven, deploy the P2P marketplace layer (Tier 0.5):

| Timeline | Deliverable | Business Impact |
|----------|-------------|-----------------|
| Weeks 17-18 | Proto definitions + core models | Foundation for all marketplace features |
| Weeks 19-20 | Android marketplace UI (buy/sell/review) | Users can list and discover services |
| Weeks 21-22 | Super-peer backend + PostgreSQL schema + search index | Full-text search, seller ratings, order tracking |
| Week 23 | Gossip protocol for listing discovery | Offline peer-to-peer service discovery |
| Week 24 | IPFS image upload + integration | Photo evidence for products/services |
| Weeks 25-28 | Seller reputation integration + dispute resolution | Ratings feed into credit score, economic incentive loop |

**Marketplace Impact on B2B Credit Revenue:**
All marketplace transactions are completely free. Revenue comes from B2B credit data licensing:
- Y1: Marketplace activity builds credit profiles for early MFI partnerships
- Y2: $1M+ GMV generates thousands of active credit profiles for B2B licensing
- Y3: $50M+ GMV = millions of rich credit profiles for enterprise API licensing

---

## Next Steps

**Immediately (Weeks 1-16):**
1. Assign 2 Rust engineers to cs-sync (consensus, validation)
2. Assign 2 Android engineers to crypto, NFC, recovery
3. Set up 5 HSM infrastructure (order immediately, lead time 4-6 weeks)
4. Allocate PostgreSQL and Redis managed services

**Week 1:**
- Kick off MVP core implementation
- Daily standup on cs-core compilation
- Draft deployment guide for 5 super-peers

**After Security MVP (Week 17+):**
- Start marketplace team (2 Rust backend, 2 Android UI)
- Begin proto design for listings + purchase orders
- IPFS gateway setup for image hosting

This is now a complete, bankable architecture: payment system → credit building → peer-to-peer marketplace → fintech platform. Ready to build?
