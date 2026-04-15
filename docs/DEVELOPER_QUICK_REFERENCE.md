# CylinderSeal: Developer Quick Reference

## The Three Golden Rules

### Rule 1: Amounts Are Always i64 Micro-OWC
```rust
// ✅ CORRECT
let amount: i64 = 1_000_000;  // 1 OWC

// ❌ WRONG
let amount: f64 = 1.0;  // NEVER use float for financial amounts
```

### Rule 2: All Transactions Are Immutable
```rust
// ✅ Correct: create new, sign, append
let mut tx = Transaction::new(...);
tx.sign(&private_key)?;
ledger.append(tx);

// ❌ Wrong: modifying existing transaction
tx.amount_owc = 500;  // NEVER modify after signing
```

### Rule 3: Verify Signatures on Every Transaction
```rust
// ✅ Always verify before trusting
tx.verify_signature()?;  // Fails if tampered

// ❌ Never trust unverified
// process(tx)  // NEVER without verification
```

---

## Common Patterns

### Creating a Transaction

**Kotlin (Android):**
```kotlin
// Get user's previous nonce from ledger
val previousNonce = ledger.getLastTransaction()?.current_nonce 
    ?: blake2b256(userPublicKey)

// Create and sign
var tx = Transaction(
    from_public_key = userPublicKey,
    to_public_key = recipientPublicKey,
    amount_owc = 50_000_000,  // 50 OWC
    currency_context = "KES",
    fx_rate_snapshot = Decimal("0.987654"),
    channel = PaymentChannel.NFC,
    memo = "Payment for goods",
    device_id = deviceId,
    previous_nonce = previousNonce,
    monotonic_clock_nanos = System.nanoTime(),
)

tx.sign(devicePrivateKey)

// Verify locally before sending
tx.verifySignature()
```

**Rust (Super-Peer):**
```rust
// Validate incoming transaction
pub async fn validate_transaction(tx: &Transaction) -> Result<()> {
    // 1. Verify signature
    tx.verify_signature()?;

    // 2. Verify device binding (nonce includes device IMEI)
    let device = storage.get_device(tx.device_id).await?;
    verify_nonce_with_device(&tx.current_nonce, &device)?;

    // 3. Check device daily limit
    let spent_today = storage.get_device_daily_spending(tx.device_id).await?;
    if spent_today + tx.amount_owc > device.daily_limit {
        return Err(DailyLimitExceeded);
    }

    // 4. Update pending
    storage.record_device_spending(tx.device_id, tx.amount_owc).await?;

    Ok(())
}
```

### Creating a LedgerBlock

**Kotlin (Android):**
```kotlin
// Collect transactions
val transactions = listOf(tx1, tx2, tx3)  // All signed

// Create block
var block = LedgerBlock(
    user_public_key = userPublicKey,
    device_id = deviceId,
    sequence_number = lastBlock.sequence_number + 1,
    prev_block_hash = lastBlock.block_hash,
    transactions = transactions,
    vector_clock = updateVectorClock(lastBlock.vector_clock),  // Add our sequence
)

// Compute hash
block.computeBlockHash()

// Sign with device key
block.signWithDeviceKey(devicePrivateKey)

// Store locally
ledger.append(block)
```

### Processing Encrypted Transactions

**Kotlin (Android - Encrypt):**
```kotlin
fun encryptTransaction(
    tx: Transaction,
    userMasterKey: Key,
): EncryptedTransaction {
    val plaintext = tx.canonicalCbor()
    val plaintextHash = blake2b256(plaintext)

    val cipher = Cipher.getInstance("AES/GCM/NoPadding")
    val gcmSpec = GCMParameterSpec(128, random(12))
    cipher.init(Cipher.ENCRYPT_MODE, userMasterKey, gcmSpec)

    val ciphertext = cipher.doFinal(plaintext)
    val authTag = cipher.authenticationTag

    return EncryptedTransaction(
        plaintext_hash = plaintextHash,
        ciphertext = ciphertext,
        nonce = gcmSpec.iv,
        auth_tag = authTag,
    )
}
```

**Rust (Super-Peer - Detect Replays):**
```rust
pub async fn check_duplicate_by_hash(
    plaintext_hash: &[u8; 32],
) -> Result<()> {
    let already_seen = storage
        .transaction_by_plaintext_hash(plaintext_hash)
        .await?;

    if already_seen.is_some() {
        return Err(DuplicateTransaction);
    }

    Ok(())
}
```

---

## Security Checklist

### Before Signing a Transaction
- [ ] Amount is i64, > 0
- [ ] From/to keys are different (not sending to self)
- [ ] Nonce chain is valid (previous_nonce matches ledger)
- [ ] Monotonic clock is not going backward
- [ ] Device has sufficient pending balance
- [ ] Device daily limit not exceeded

### Before Syncing a Block to Super-Peer
- [ ] Block hash is correct (recompute and verify)
- [ ] All transaction signatures are valid
- [ ] Sequence number is exactly last + 1
- [ ] prev_block_hash matches last block
- [ ] Vector clock doesn't go backward
- [ ] Monotonic clock is monotonically increasing

### Before Confirming a Block at Super-Peer
- [ ] Device attestation is valid
- [ ] Device reputation score is acceptable (or escalate)
- [ ] Geographic anomalies checked
- [ ] Device daily limit checked
- [ ] 3+ of 5 super-peers agree
- [ ] Quorum signatures collected

---

## Debugging Guide

### "Sequence Number Mismatch"
```
Expected: 5, Got: 7
→ Device submitted blocks out of order
→ Device must submit all blocks 5, 6 in order first
→ Check device sync queue
```

### "Vector Clock Went Backward"
```
Block A: vector_clock = {alice: 5, bob: 3}
Block B: vector_clock = {alice: 5, bob: 2}  ← ERROR
→ Monotonic causality violation
→ Possible attack: clock manipulation
→ Check device monotonic_clock_nanos
```

### "Nonce Chain Broken"
```
Expected previous_nonce: A
Got previous_nonce: B
→ Nonce chain is broken
→ Device state is corrupted OR attack
→ Force full resync
```

### "Super-Peer Returned Insufficient Confirmations"
```
Got: 2 signatures, Need: 3
→ 1 super-peer is down or rejecting block
→ Check which peer rejected it
→ Retry after peer recovery
```

### "Device Reputation Score Too Low"
```
Score: 35 (needs >= 40)
→ Device exhibits suspicious behavior
→ Offline txs are frozen
→ Force user to sync and fix reputation
→ Check: geographic jumps, unusual times, frequency
```

---

## Performance Targets

| Operation | Target | Reality |
|-----------|--------|---------|
| Transaction signing | < 50ms | ~10ms (Ed25519) |
| Block hashing | < 50ms | ~5ms (BLAKE2b) |
| NFC exchange | < 500ms | ~300ms (APDU handshake) |
| BLE exchange | < 2s | ~1-1.5s (GATT write) |
| Device attestation | < 500ms | ~200-400ms (SafetyNet) |
| Super-peer validation | < 200ms | ~50-150ms (3+ checks) |
| Consensus quorum | < 5s | ~2-3s (5 nodes, network latency) |
| Key rotation | < 1 min | < 30 seconds (background) |
| Key recovery | < 10 min | ~5-10 min (contact requests) |

---

## Testing Strategies

### Unit Tests

**Kotlin:**
```kotlin
@Test
fun testNonceChainValidation() {
    val genesis = blake2b256(userPublicKey)
    val tx1 = createTx(previousNonce = genesis)
    val tx2 = createTx(previousNonce = tx1.current_nonce)
    
    assertTrue(tx2.previous_nonce == tx1.current_nonce)
}

@Test
fun testVectorClockMonotonicity() {
    val clock1 = mapOf(alice to 5)
    val clock2 = mapOf(alice to 6)
    
    assertTrue(clock2[alice]!! >= clock1[alice]!!)
}
```

**Rust:**
```rust
#[test]
fn test_deterministic_nonce() {
    let prev_nonce = [42u8; 32];
    let device_id = Uuid::nil();
    
    let nonce1 = derive_nonce_with_hardware(&prev_nonce, &device_id, 1);
    let nonce2 = derive_nonce_with_hardware(&prev_nonce, &device_id, 1);
    
    assert_eq!(nonce1, nonce2);  // Deterministic
}
```

### Integration Tests

**Scenario: Offline Double-Spend Detection**
```
Setup:
  Device A: balance 100 OWC
  Device B: balance 100 OWC (same user)
  Both offline

Attack:
  Device A: creates Block 5, spends 100 OWC
  Device B: creates Block 5, spends 100 OWC
  
Result:
  Both blocks have same sequence, same prev_hash
  Super-peer detects fork
  Loser block is quarantined
  Credit score penalized
```

### Security Tests

**Certificate Pinning:**
```kotlin
@Test
fun testCertificatePinning() {
    // Try to connect with wrong cert
    assertThrows<SSLPeerUnverifiedException> {
        syncWithWrongCert()
    }
}
```

**Signature Verification:**
```rust
#[test]
fn test_tampered_block_rejected() {
    let mut block = create_valid_block();
    block.amount_owc += 1;  // Tamper
    
    assert!(block.verify().is_err());
}
```

---

## Common Mistakes

### ❌ Mistake 1: Using Random Nonces
```rust
// WRONG
let nonce = random(32);  // Not deterministic, can replay

// CORRECT
let nonce = derive_nonce_with_hardware(prev_nonce, device_id, counter);
```

### ❌ Mistake 2: Trusting Timestamps
```rust
// WRONG
if block_a.created_at < block_b.created_at {
    winner = block_a;  // Can be attacked with NTP
}

// CORRECT
if block_a.sequence_number > block_b.sequence_number {
    winner = block_a;  // Deterministic, attack-proof
}
```

### ❌ Mistake 3: Skipping Device Attestation
```kotlin
// WRONG
if (amount < 100) {
    skip_attestation = true  // Jailbroken device gets in!
}

// CORRECT
verify_attestation()  // Always, even for small amounts
```

### ❌ Mistake 4: Modifying Signed Data
```rust
// WRONG
tx.memo = "updated memo";  // Signature is now invalid!

// CORRECT
// Create new transaction if memo needs to change
```

### ❌ Mistake 5: Accepting Out-of-Order Blocks
```rust
// WRONG
for block in submitted_blocks {
    process(block)  // What if sequences are [1, 3, 2]?
}

// CORRECT
let mut expected = last_sequence + 1;
for block in submitted_blocks {
    if block.sequence != expected {
        return Err(OutOfSequence);
    }
    expected += 1;
    process(block);
}
```

---

## Regulatory Compliance Checklist

**For deploying in regulated jurisdictions:**

- [ ] Immutable audit log (signed by 3+ super-peers)
- [ ] User can export transaction history
- [ ] Transaction reversal capability (for disputes)
- [ ] AML transaction monitoring
- [ ] KYC verification at tiers
- [ ] Transaction limits per KYC tier
- [ ] Geographic blocking (if required)
- [ ] Data retention policy (typically 5+ years)
- [ ] Regulatory API for inspectors
- [ ] Annual security audit (Big 4 firm)
- [ ] Incident notification protocol
- [ ] Insurance (cyber + errors & omissions)

---

## Deployment Checklist

**Before going live:**

- [ ] All 5 super-peers operational
- [ ] HSMs initialized with threshold keys
- [ ] PostgreSQL replicated and backed up
- [ ] Redis persisted and backed up
- [ ] Load balancer + DDoS protection
- [ ] Monitoring (CloudWatch / DataDog / New Relic)
- [ ] Incident response playbook
- [ ] 24/7 on-call rotation
- [ ] Legal review (terms of service, privacy policy)
- [ ] Security audit by external firm
- [ ] Penetration testing (at least annually)
- [ ] Compliance audit (for regulators)

---

## Getting Help

**When in doubt:**

1. **Is it about amounts?** → Check: always i64 micro-OWC, never float
2. **Is it about signatures?** → Check: verify before trusting
3. **Is it about ordering?** → Check: use sequence numbers or vector clocks, never timestamps
4. **Is it about keys?** → Check: Keystore/HSM, never plaintext
5. **Is it about privacy?** → Check: E2E encrypt with user's master key

**Read in order:**
1. This quick reference
2. `/docs/IRON_SECURITY.md` (architecture)
3. `/docs/SECURITY_VALIDATION.md` (validation rules)
4. `/plan.md` (system design)

Good luck building iron-grade security! 🔐
