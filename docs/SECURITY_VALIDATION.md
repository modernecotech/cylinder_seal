# CylinderSeal Security Validation Rules

## Device-Level Validation (Android App)

### Before Creating an Offline Transaction

```kotlin
fun validateOfflineTransaction(
    tx: Transaction,
    user: User,
    device: Device,
    lastConfirmedBlock: JournalEntry?
): ValidationResult {
    // 1. Check KYC tier limits
    if (tx.amount_owc > user.kyc_tier.max_offline_transaction()) {
        return ValidationError("Amount exceeds KYC tier limit")
    }

    // 2. Check device daily limit
    val todaySpent = getDeviceTodaySpent(device.id)
    if (todaySpent + tx.amount_owc > user.kyc_tier.max_daily_offline_per_device()) {
        return ValidationError("Device daily limit exceeded")
    }

    // 3. Check pending balance
    val pending = calculatePendingBalance(user, device)
    if (pending < tx.amount_owc) {
        return ValidationError("Insufficient pending balance")
    }

    // 4. Check monotonic clock (no time travel)
    if (lastConfirmedBlock != null && 
        tx.monotonic_clock_nanos < lastConfirmedBlock.monotonic_created_nanos) {
        return ValidationError("Monotonic clock went backward")
    }

    // 5. If amount > attestation threshold: require device attestation
    if (tx.amount_owc > user.kyc_tier.attestation_threshold()) {
        val attestation = getDeviceAttestation()
        if (!attestation.isValid()) {
            return ValidationError("Device attestation failed")
        }
        tx.device_attestation = attestation.toJson()
    }

    // 6. If amount > biometric threshold: require biometric
    if (tx.amount_owc > user.kyc_tier.biometric_threshold()) {
        if (!getBiometricAuth()) {
            return ValidationError("Biometric authentication failed")
        }
    }

    return ValidationSuccess
}
```

### Before Creating a JournalEntry

```kotlin
fun validateEntryBeforeLocalStorage(
    entry: JournalEntry,
    user: User,
    device: Device
): ValidationResult {
    // 1. Sequence number must increment
    val lastEntry = getLastConfirmedEntry(user.id)
    if (lastEntry != null && entry.sequence_number != lastEntry.sequence_number + 1) {
        return ValidationError("Sequence number gap or backwards")
    }

    // 2. prev_entry_hash must match
    if (lastEntry == null) {
        // Genesis entry
        val expectedHash = blake2b256(user.public_key)
        if (entry.prev_entry_hash != expectedHash) {
            return ValidationError("Invalid genesis entry hash")
        }
    } else {
        if (entry.prev_entry_hash != lastEntry.entry_hash) {
            return ValidationError("prev_entry_hash mismatch")
        }
    }

    // 3. Vector clock must be monotonically increasing
    val lastVectorClock = lastEntry?.vector_clock ?: emptyMap()
    for ((userId, clock) in entry.vector_clock) {
        if (clock < (lastVectorClock[userId] ?: 0)) {
            return ValidationError("Vector clock went backward for user $userId")
        }
    }

    // 4. Monotonic clock must never go backward
    if (lastEntry != null && 
        entry.monotonic_created_nanos < lastEntry.monotonic_created_nanos) {
        return ValidationError("Entry monotonic clock went backward")
    }

    // 5. Verify transaction signatures
    for (tx in entry.transactions) {
        if (!tx.verify_signature()) {
            return ValidationError("Transaction signature invalid")
        }
    }

    return ValidationSuccess
}
```

### Nonce Chain Validation

```kotlin
fun validateNonceChain(transactions: List<Transaction>): ValidationResult {
    if (transactions.isEmpty()) return ValidationSuccess

    // First tx must have previous_nonce == genesis_nonce
    val genesisNonce = blake2b256(user.public_key)
    if (transactions[0].previous_nonce != genesisNonce) {
        return ValidationError("First transaction nonce chain broken")
    }

    // Each subsequent tx must chain
    for (i in 1 until transactions.size) {
        if (transactions[i].previous_nonce != transactions[i-1].current_nonce) {
            return ValidationError("Nonce chain broken at index $i")
        }
    }

    return ValidationSuccess
}
```

---

## Super-Peer Validation (Rust)

### On Entry Submission

```rust
pub async fn validate_incoming_entry(
    entry: &JournalEntry,
    user_id: Uuid,
) -> Result<()> {
    // 1. Verify entry hash (recompute and compare)
    let canonical = entry.canonical_cbor_for_hashing()?;
    let expected_hash = blake2b_256(&canonical);
    if expected_hash != entry.entry_hash {
        return Err(InvalidHash);
    }

    // 2. Verify device signature
    let device = self.storage.get_device(entry.device_id).await?;
    crypto::verify_signature(&entry.entry_hash, &entry.device_signature, &device.public_key)?;

    // 3. Check sequence number (must be next expected)
    let last_seq = self.storage.get_user_last_sequence(user_id).await?;
    if entry.sequence_number != last_seq + 1 {
        return Err(OutOfSequence {
            expected: last_seq + 1,
            got: entry.sequence_number,
        });
    }

    // 4. Check prev_entry_hash
    let last_entry = self.storage.get_last_confirmed_entry(user_id).await?;
    if let Some(last) = last_entry {
        if entry.prev_entry_hash != last.entry_hash {
            return Err(Conflict("prev_entry_hash mismatch".into()));
        }
    }

    // 5. Detect double-spend (same user submitting 2 entries with same prev_hash)
    let pending = self.storage.get_pending_entries(user_id).await?;
    for pending_entry in pending {
        if pending_entry.prev_entry_hash == entry.prev_entry_hash {
            // Fork detected: two competing chains
            return self.handle_double_spend(user_id, &pending_entry, entry).await;
        }
    }

    // 6. Validate vector clock (no backward steps)
    let last_clock = last_entry.map(|b| b.vector_clock.clone()).unwrap_or_default();
    for (user, clock) in &entry.vector_clock {
        if let Some(last_val) = last_clock.get(user) {
            if clock < last_val {
                return Err(Conflict("Vector clock went backward".into()));
            }
        }
    }

    // 7. Check device daily spending (prevent multi-device fraud)
    let device_spent_today = self.storage.get_device_daily_spending(entry.device_id).await?;
    let mut total_spent = device_spent_today;
    for tx in &entry.transactions {
        if tx.from_public_key == entry.user_public_key {
            total_spent += tx.amount_owc;
        }
    }
    
    let user = self.storage.get_user(user_id).await?;
    if total_spent > user.kyc_tier.max_daily_offline_per_device() {
        // Check device reputation
        if is_device_suspicious(entry.device_id).await? {
            return Err(KYCTierLimitExceeded);
        }
    }

    // 8. Verify nonce chain
    let mut expected_prev_nonce = if last_entry.is_none() {
        blake2b_256(&entry.user_public_key)
    } else {
        // Last tx's current_nonce from previous entry
        last_entry.unwrap()
            .transactions.last().unwrap()
            .current_nonce
    };

    for tx in &entry.transactions {
        if tx.previous_nonce != expected_prev_nonce {
            return Err(Conflict("Nonce chain broken".into()));
        }
        expected_prev_nonce = tx.current_nonce;
    }

    // 9. Validate transaction signatures
    for tx in &entry.transactions {
        tx.verify_signature()?;
    }

    Ok(())
}
```

### Conflict Detection & Resolution

```rust
pub async fn handle_double_spend(
    &self,
    user_id: Uuid,
    entry_a: &JournalEntry,
    entry_b: &JournalEntry,
) -> Result<()> {
    // Both entries have same prev_hash (fork detected)

    // 1. Compare timestamps (earlier timestamp wins, soft heuristic)
    let winner = if entry_a.created_at < entry_b.created_at {
        entry_a
    } else if entry_b.created_at < entry_a.created_at {
        entry_b
    } else {
        // Timestamps equal: request NFC receipts as evidence
        // For now, reject both and escalate to human review
        self.escalate_conflict(user_id, vec![entry_a.clone(), entry_b.clone()]).await?;
        return Ok(());
    };

    // Clock skew check: if timestamps within 60 seconds, it's ambiguous
    let time_diff = (entry_a.created_at - entry_b.created_at).abs();
    if time_diff < 60_000_000 {  // 60 seconds in microseconds
        // Request both devices submit NFC receipts with signed evidence
        self.request_nfc_evidence(user_id, entry_a, entry_b).await?;
        return Ok(());
    }

    // Winner is determined; loser is quarantined
    let loser = if winner == entry_a { entry_b } else { entry_a };
    self.storage.mark_conflicted(loser.entry_hash, "double_spend").await?;

    // Notify both devices
    self.notify_conflict(user_id, winner.entry_hash, loser.entry_hash).await?;

    // Penalize losing device (lower credit score)
    let device = self.storage.get_device(loser.device_id).await?;
    self.storage.record_conflict_penalty(device.user_id, -50).await?;

    Ok(())
}
```

### Super-Peer Confirmation (3-of-3 Byzantine Consensus)

```rust
pub async fn confirm_entry_with_consensus(
    &self,
    entry: &JournalEntry,
    user_id: Uuid,
) -> Result<()> {
    // Get 5 super-peers (hardcoded or discovered)
    let peers = vec![
        "super-peer-africa.cylinderseal.io",
        "super-peer-asia.cylinderseal.io",
        "super-peer-americas.cylinderseal.io",
        "super-peer-europe.cylinderseal.io",
        "super-peer-oceania.cylinderseal.io",
    ];

    let mut confirmations = vec![];

    for peer in peers {
        // Send entry to peer for validation
        match self.gossip_client.propose_entry(peer, entry).await {
            Ok(sig) => confirmations.push(sig),
            Err(e) => {
                tracing::warn!("Peer {} rejected entry: {}", peer, e);
            }
        }
    }

    // Need 3+ confirmations (3-of-5 quorum)
    if confirmations.len() < 3 {
        return Err(Conflict("Insufficient confirmations".into()));
    }

    // Store confirmations with entry
    let mut confirmed_entry = entry.clone();
    confirmed_entry.super_peer_confirmations = confirmations;

    self.storage.confirm_entry(&confirmed_entry).await?;

    Ok(())
}
```

---

## Anomaly Detection

### Device Behavior Scoring

```rust
pub struct DeviceReputation {
    pub device_id: Uuid,
    pub score: i32,  // 0-100
}

pub async fn compute_device_reputation(device_id: Uuid) -> Result<DeviceReputation> {
    let blocks = self.storage.get_device_blocks(device_id).await?;
    let mut score = 100;

    // Geographic inconsistency (if user is in Kenya but device syncs from Japan)
    let locations = extract_locations(blocks)?;
    if locations.len() > 2 && are_too_far_apart(&locations) {
        score -= 30;
    }

    // Clock skew attempts
    let clock_anomalies = detect_clock_skew_attempts(blocks)?;
    score -= clock_anomalies.len() as i32 * 5;

    // Frequent conflicts
    let conflicts = count_device_conflicts(device_id).await?;
    score -= (conflicts as i32) * 10;

    // Large offline spending
    let large_txs = blocks.iter()
        .flat_map(|b| &b.transactions)
        .filter(|tx| tx.amount_owc > 50_000_000)  // > 50 OWC
        .count();
    if large_txs > 5 {
        score -= 20;
    }

    Ok(DeviceReputation {
        device_id,
        score: score.max(0).min(100),
    })
}
```

---

## Summary: Validation Layers

1. **Device-level** (Kotlin): Offline validation before signing
   - KYC limits, device limits, monotonic clocks, nonce chains, attestation, biometric

2. **First super-peer** (Rust): Entry ingestion
   - Hash verification, signature verification, sequence validation, conflict detection, nonce chain

3. **Consensus layer** (5 super-peers): Byzantine tolerance
   - 3+ confirmations required for finality (3-of-5 quorum)

4. **Gossip layer** (Peer-to-peer): Anomaly detection
   - Device reputation, geographic consistency, clock skew detection

This defense-in-depth approach makes double-spend and other attacks exponentially harder.
