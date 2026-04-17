# Recovery and Key Rotation

## Overview

Users must be able to:
1. **Recover their identity** if device is lost/stolen
2. **Rotate keys** if compromise is suspected
3. **Migrate** to a new device with full account history
4. **Prevent** unauthorized access during recovery

This document specifies **social recovery** (trusted contacts) as the primary recovery mechanism, with governance-assisted recovery as fallback.

---

## 1. Identity Model

### 1.1 Key Structure

Each user has:

```
Identity Public Key (stable)
├─ Never rotates, derives user_id
├─ Posted on ledger at account creation
└─ Used to receive payments

Spending Key (can rotate)
├─ Used to sign outbound transactions
├─ Can be replaced via key rotation
└─ Hardware-backed (Android Keystore)

Transport Session Keys (ephemeral)
├─ Used for TLS and P2P communication
├─ 24-hour TTL, automatically regenerated
└─ Cannot sign transactions or recovery requests
```

**User ID derivation**:
```
user_id = BLAKE2b-256(identity_public_key)[0:128] → UUIDv7
```

### 1.2 Key Ownership Proof

To claim a user_id during recovery, prove ownership via:
- Signature with identity key (preferred, but key is lost)
- OR social recovery from delegates (if identity key is lost)
- OR governance approval (emergency fallback)

---

## 2. Social Recovery Delegates

### 2.1 Delegate Selection

At account creation, user configures 3-7 **recovery delegates**:

```
Example Configuration:
┌──────────────────────────────────────┐
│ Recovery Delegates (3-of-5 threshold)│
├──────────────────────────────────────┤
│ 1. Sister (Phone #) — delegate_1     │
│ 2. Friend (Telegram) — delegate_2    │
│ 3. Neighbor (NFC) — delegate_3       │
│ 4. Spouse (NFC) — delegate_4         │
│ 5. MFI Partner (Online) — delegate_5 │
├──────────────────────────────────────┤
│ Threshold: 3 of 5 must approve       │
│ Approval window: 7 days              │
└──────────────────────────────────────┘
```

**Delegates are**:
- Other CylinderSeal users (not external services)
- Configured by user with explicit consent
- Can be updated anytime (7-day delay before taking effect)
- Kept in encrypted local backup only (not synced to super-peers)

### 2.2 Delegate Responsibilities

Delegates receive **no special access** to user's account. They only:
1. Receive recovery request (signed by user's identity key... but wait, user lost it)
2. Verify user's identity via out-of-band channel (phone call, in-person meeting)
3. Sign approval attestation (using delegate's own key)
4. Return attestation to user

**What delegates CANNOT do**:
- Spend user's money
- View transaction history
- Initiate transactions
- Change security settings

---

## 3. Recovery Flow: Lost Device

### 3.1 User Initiates Recovery

**Step 1: New Device**
User gets new phone and opens CylinderSeal app:
```
Recovery Flow
├─ Scan recovery QR code (if backed up)
├─ OR enter recovery passphrase (if set)
├─ OR select "Recover from Delegates"
└─ Show recovery wizard
```

**Step 2: Proof of Identity**
User attempts recovery via one of:
```
Option A: Recovery QR Code (from backup)
├─ Backed up to encrypted storage (Photos, cloud)
├─ Contains: user_id, identity_pubkey, recovery_delegates config
└─ User scans with new device

Option B: Recovery Passphrase
├─ 12-word BIP39 passphrase set at account creation
├─ User enters words on new device
└─ Derives identity key from passphrase

Option C: Social Recovery (Delegates)
├─ User contacts delegates (phone, SMS, in-person)
├─ Provides identity details (name, approximate balance, recent transactions)
└─ Delegates verify and sign approvals
```

### 3.2 Recovery via Delegates (Step-by-Step)

**Step 2.1: User broadcasts recovery request**

```protobuf
message RecoveryRequest {
  string recovery_id = 1;           // UUIDv7
  string user_id = 2;               // User being recovered
  bytes identity_public_key = 3;    // Must be on-chain already
  repeated string delegate_ids = 4; // Delegates to contact
  int64 created_at_ms = 5;
  int64 approval_deadline_ms = 6;   // 7 days from now
  string recovery_reason = 7;       // "lost_device", "compromised", etc.
  bytes signature = 8;              // Signed with identity_key (user retains copy)
}
```

**Step 2.2: Delegates verify**

Delegates receive notification and:
1. Verify signature with identity_public_key (proves key ownership)
2. Contact user via out-of-band channel (phone call, meet in person)
3. Ask security questions (recent transaction, contact name, approximate balance)
4. If confident, sign approval attestation:

```protobuf
message RecoveryApproval {
  string recovery_id = 1;
  string delegate_id = 2;
  bytes identity_public_key = 3;
  string approval_timestamp = 4;
  string verification_method = 5;   // "phone_call", "in_person", "video_chat"
  bytes delegate_signature = 6;     // Delegate's signature proving they approved
}
```

**Step 2.3: User collects approvals**

User asks delegates to share approvals (via SMS, chat, QR code).

Once threshold is met (e.g., 3-of-5):
```
Approvals collected:
├─ Sister signed ✓
├─ Friend signed ✓
├─ Neighbor signed ✓
└─ Threshold 3-of-5 MET

User can now:
├─ Generate new spending key (keeping identity key)
├─ Submit recovery transaction to super-peers
└─ Regain access to account
```

**Step 2.4: Super-peers verify and finalize**

```
Recovery finalization:
├─ Identity key already on ledger (verified at original account creation)
├─ Recovery request signed with identity key (verified against ledger)
├─ 3-of-5 delegate approvals included (delegates' sigs verified)
└─ Super-peers confirm recovery and create new journal block

Spending privileges restored:
├─ New spending key can now sign transactions
├─ Old spending key is marked revoked
├─ Balance and transaction history accessible
└─ User can resume normal operation
```

### 3.3 Recovery Timeline

```
Day 0: User initiates recovery, broadcasts request
Day 1-2: Contacts delegates out-of-band (phone, in-person)
Day 2-4: Delegates provide approvals via QR/SMS
Day 4: User submits recovery transaction to super-peers
Day 4-5: Super-peers verify and confirm
Day 5+: User can spend and transact normally

Total: 5 days from device loss to full access
```

---

## 4. Compromise Response

### 4.1 Suspected Compromise

If user suspects device is compromised (stolen, malware, etc.):

**Immediate action** (if still have access):
```
Settings → Security → Report Compromise
├─ User signs compromise report with spending key
├─ Broadcast with "urgent" priority to all super-peers
├─ All pending transactions in mempool frozen
├─ New transactions require additional verification
└─ Status: "COMPROMISED" shown on ledger
```

**If no access to device**:
```
Contact delegates manually:
├─ Call sister, friend, neighbor
├─ Request emergency recovery (accelerated)
├─ Provide identity details
├─ Delegates fast-track approvals (vote within 24 hours instead of 7 days)
└─ Recovery finalized with new spending key
```

### 4.2 Freezing Operations

Once compromise reported:

```
Automatically frozen:
├─ New outbound transactions (require device+PIN verification)
├─ Large outbound transactions (require additional witness approval)
├─ Key rotation requests (locked for 24 hours)
├─ Marketplace seller account changes
└─ Loan applications

Still allowed:
├─ Receiving payments (peer can send to compromised account)
├─ Viewing balance and history
├─ Messaging super-peers for assistance
└─ Initiating recovery via delegates
```

---

## 5. Key Rotation (Planned)

### 5.1 Rotate Spending Key

User can periodically rotate spending key (recommended annually):

```protobuf
message KeyRotation {
  string rotation_id = 1;           // UUIDv7
  string user_id = 2;
  bytes old_spending_pubkey = 3;    // Current key
  bytes new_spending_pubkey = 4;    // New key
  bytes identity_pubkey = 5;        // Unchanged
  int64 created_at_ms = 6;
  int64 activation_delay_ms = 7;    // 24-hour grace period
  bytes old_key_signature = 8;      // Old key signs the new key reference
  bytes identity_key_signature = 9; // Identity key also signs (strongest proof)
}
```

**Activation delay**: 24-hour window before new key is active. If compromise suspected:
```
User calls delegates within 24h:
├─ "Cancel that key rotation I just initiated"
├─ Delegates vote to reject rotation
└─ Old key remains active
```

### 5.2 Rotate Identity Key (Emergency Only)

Rotating identity key is **extremely disruptive** because it changes user_id derivation.

If identity key is compromised:

```
Special procedure:
├─ Requires 4-of-5 super-peers approval (not routine)
├─ Requires 2-of-3 governance committee approval
├─ Old identity key marked "revoked" on ledger
├─ New identity key associated with same user account
├─ All prior transactions remain visible
└─ Balance transfers to new identity key
```

**This should be rare** (only if cryptographic breach, not if device is lost).

---

## 6. Device Migration

### 6.1 Planned Migration (New Device)

User wants to switch to new phone (upgrade, replacement):

```
Migration flow:
├─ Old device: Backup wallet (encrypted QR code or passphrase)
├─ New device: Scan backup or enter passphrase
├─ App asks: "Migrate or Fresh Recovery?"
├─ Select "Migrate" (faster than recovery)
└─ Follow device migration wizard
```

**Migration wizard**:
```
Step 1: Verify old device is online (or have backup)
  └─ Scan QR code from old device to new device
  
Step 2: Confirm identity
  └─ Enter PIN or biometric
  
Step 3: Key transfer
  └─ Generate new keys on new device
  └─ Old device approves transfer via biometric
  
Step 4: Sync history
  └─ Fetch last checkpoint from super-peers
  └─ Download any unconfirmed transactions
  
Step 5: Verify balance
  └─ New device computes balance independently
  └─ Compare against old device's balance
  
Step 6: Old device deactivation
  └─ Spending key on old device marked "revoked"
  └─ Only new device can spend
```

### 6.2 What Transfers

**Transfers to new device**:
- ✅ Full transaction history (read-only)
- ✅ Current balance (OWC amount)
- ✅ Credit score and tier
- ✅ Marketplace listings (if seller)
- ✅ Recurring payment authorizations
- ✅ Recovery delegate configuration

**Does NOT transfer** (for security):
- ❌ Private keys (regenerated on new device)
- ❌ Pending unconfirmed transactions (must reconcile)
- ❌ Local merchant till accounts (POS terminals recreate)
- ❌ Backup passphrases (user retains control)

---

## 7. Backup Standards

### 7.1 What to Backup

```
Backup Package:
├─ Identity public key (non-sensitive, can be public)
├─ Encrypted spending key material (AES-256-GCM)
├─ Recovery delegate configuration (list of delegate IDs + thresholds)
├─ Latest checkpoint header (for faster sync)
├─ Contact mapping (merchant names, frequent payees)
├─ Marketplace listings (if seller)
├─ Optional: Encrypted recent transaction metadata
└─ Expiration: Credentials valid for 12 months before rotation recommended
```

### 7.2 Backup Encryption

Backup is encrypted with:

```
BackupKey = HKDF-SHA256(
  ikm = user_passphrase OR device_pin,
  salt = BLAKE2b(user_id),
  info = "cylinderseal_backup_v1"
)

Cipher: AES-256-GCM(
  plaintext = backup_package,
  key = BackupKey,
  nonce = random_96bits,
  aad = user_id
)
```

**Storage options**:
- Local: Photos app, SD card (user's responsibility to keep secure)
- Cloud: Encrypted backup to iCloud/Google Photos (automatic, encrypted end-to-end)
- External: USB drive, written on paper (QR code)

### 7.3 Backup Formats

```
QR Code Format:
├─ Compressed JSON with base64 encoding
├─ Fits on paper printout (~5 QR codes)
├─ Can be scanned by new device camera
└─ Encrypted as above

PDF Format:
├─ Printable backup document
├─ Contains QR code + text recovery instructions
├─ Recommended for long-term storage
└─ 2-3 pages

Cloud Format:
├─ Encrypted JSON blob in iCloud/Google Photos
├─ Automatic backup during device setup
├─ 12-month availability
└─ User can trigger manual backup anytime
```

---

## 8. Emergency Governance Recovery (Fallback)

If user has:
- Lost all devices
- Lost all recovery delegates
- No backup available

**Option: Governance-Assisted Recovery**

```
Process:
├─ User submits identity proof (government ID, selfie, etc.)
├─ Risk committee requests additional proof from MFI partners
├─ If user has loan history, MFI vouches for identity
├─ 2-of-3 risk committee approves emergency recovery
├─ 4-of-5 super-peer quorum confirms
├─ New spending key issued, account recovered
└─ Takes 2-4 weeks (slower, more approval)

Not recommended:
└─ No cryptographic proof of identity
└─ Requires manual verification
└─ Only for extreme cases
```

---

## 9. Implementation Checklist (Phase 2)

- [ ] Protobuf: RecoveryRequest, RecoveryApproval, KeyRotation messages
- [ ] Android UI: Recovery wizard (QR/passphrase/delegates)
- [ ] Android UI: Compromise reporting flow
- [ ] Crypto: BIP39 passphrase support for backup
- [ ] Crypto: Key rotation signing logic
- [ ] Backend: Recovery approval validation
- [ ] Backend: Emergency governance recovery endpoint
- [ ] Dashboard: Show recovery delegates + approved thresholds
- [ ] Testing: Recovery flow for lost device, compromised key, device migration

---

## 10. User Experience Principles

1. **Default: Social Recovery** — Delegates are the first-line recovery method (not passwords)
2. **Transparency**: User knows exactly who their delegates are and what they can approve
3. **Out-of-band verification**: Delegates verify user via phone/in-person, not just cryptography
4. **No secrets in the cloud**: Recovery passphrases and backup keys stay local or encrypted-end-to-end
5. **Graceful degradation**: If delegates unavailable, governance recovery available (slower)
6. **Opt-in**: Recovery is optional; users can choose to risk account loss if they prefer simplicity

---

## References

- Decentralized Mesh Currency System Specification (Section 24)
- GOVERNANCE_FRAMEWORK.md
- ANDROID_WEEK2_BRIDGE.md (key management)
