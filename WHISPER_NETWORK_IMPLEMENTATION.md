# Whisper Network Implementation Guide

## Overview

The whisper network enables **offline-first devices to propagate their transaction data through the peer mesh network** without requiring direct internet connectivity. Devices that are offline can have their pending entries relayed by online peers to super-peers for confirmation.

## Protocol Flow

### 1. Online Peer Detects Offline Peer

```
Device A (offline)  ←NFC/BLE→  Device B (online with data)
├─ No WiFi/cellular            ├─ Connected to super-peer S1
├─ Has pending entries         └─ Can relay entries
│  (in SQLite PENDING state)
```

### 2. Peer Discovery & Relay Request

```kotlin
// On Device B (online peer):
val nearbyDevices = nfcInterface.discoverDevices()  // Device A found

// Device B asks Device A: "Any pending entries?"
val pendingEntries = nfcInterface.requestPending(deviceA)

// Device A responds with up to 100 pending JournalEntry objects
```

### 3. Relay to Super-Peer

```
Device B → Super-Peer S1:
┌─────────────────────────────────────────────────┐
│ EntryRelay {                                    │
│   originating_device_pk: A's public key         │
│   originating_nonce: A's current nonce          │
│   relay_device_pk: B's public key               │
│   entries: [A's 5 pending JournalEntry objects] │
│   relay_timestamp: now()                         │
│   relay_signature: signed by B over entries     │
│   relay_device_id: B's UUIDv7                   │
│ }                                                │
└─────────────────────────────────────────────────┘
```

### 4. Super-Peer Validation

```
SuperPeer.RelayEntries(relay: EntryRelay):
  ├─ Verify relay_signature (B's Ed25519)
  ├─ Check relay_device_pk rate limit (max 10/min)
  ├─ Verify originating_nonce is monotonic
  ├─ Deduplicate against already-received entries
  ├─ Validate each JournalEntry independently
  │  ├─ Check signature (A's Ed25519)
  │  ├─ Verify sequence numbers
  │  └─ Detect double-spends
  │
  └─ Queue to Byzantine quorum for confirmation
     ├─ 3-of-5 super-peers must agree
     └─ Update relay_device_reputation based on conflicts
```

### 5. Confirmation Returns to Original Device

```
Device A comes online later:
├─ Syncs to S1
├─ S1 responds: "Entries 1-5 already CONFIRMED via relay"
└─ Device A sees: ✓ CONFIRMED status on all entries
```

## Rust Implementation

### Super-Peer: RelayEntries RPC Handler

```rust
// crates/cs-sync/src/handlers/relay.rs

pub struct RelayHandler {
    db: PostgresPool,
    redis: RedisClient,
    reputation: ReputationScorer,
}

impl RelayHandler {
    pub async fn handle_relay_entries(&self, relay: EntryRelay) -> Result<RelayAck> {
        // 1. Verify relay device signature
        let relay_pk = Ed25519PublicKey::from_bytes(&relay.relay_device_pk)?;
        let relay_sig = Ed25519Signature::from_bytes(&relay.relay_signature)?;
        
        let message = relay.entries_canonical_cbor();
        relay_pk.verify(&relay_sig, &message)
            .map_err(|_| RelayError::SignatureInvalid)?;

        // 2. Check rate limits on relay device
        let key = format!("relay-rate:{}", hex::encode(&relay.relay_device_pk));
        let relays_this_minute = redis.incr(&key, expire: 60).await?;
        if relays_this_minute > 10 {
            return Ok(RelayAck {
                accepted: false,
                status: RelayStatus::RateLimited,
                ..Default::default()
            });
        }

        // 3. Verify originating nonce is not stale/replayed
        let originating_key = format!(
            "nonce:{}", 
            hex::encode(&relay.originating_device_pk)
        );
        let last_nonce = redis.get::<Vec<u8>>(&originating_key).await?;
        
        if let Some(last) = last_nonce {
            if relay.originating_nonce <= last {
                return Ok(RelayAck {
                    accepted: false,
                    status: RelayStatus::NonceStale,
                    ..Default::default()
                });
            }
        }
        
        // Update latest nonce
        redis.set(&originating_key, &relay.originating_nonce, expire: 86400).await?;

        // 4. Process each entry
        let mut accepted = Vec::new();
        let mut rejected = Vec::new();

        for entry in relay.entries {
            match self.process_entry(&entry, &relay.relay_device_pk).await {
                Ok(entry_id) => accepted.push(entry_id),
                Err(_) => rejected.push(entry.entry_id.clone()),
            }
        }

        // 5. Deduct from relay device's reputation if conflicts detected
        let conflict_count = rejected.len();
        if conflict_count > 0 {
            self.reputation.penalize_relay_device(
                &relay.relay_device_pk,
                conflict_count as u32
            ).await?;
        }

        let reputation_score = self.reputation.score(&relay.relay_device_pk).await?;

        Ok(RelayAck {
            accepted: true,
            accepted_entry_ids: accepted,
            rejected_entry_ids: rejected,
            status: RelayStatus::Queued,
            relay_device_reputation: reputation_score,
            ..Default::default()
        })
    }

    async fn process_entry(
        &self, 
        entry: &JournalEntry, 
        relay_device_pk: &[u8]
    ) -> Result<Vec<u8>> {
        // Validate entry signature
        let user_pk = Ed25519PublicKey::from_bytes(&entry.user_public_key)?;
        user_pk.verify(
            &Ed25519Signature::from_bytes(&entry.signature)?,
            &entry.entry_hash
        )?;

        // Check for duplicates
        if self.db.entry_exists(&entry.entry_id).await? {
            return Err(RelayError::Duplicate);
        }

        // Verify sequence numbers and prev_entry_hash
        let last_seq = self.db.get_last_sequence(&entry.user_public_key).await?;
        if entry.sequence_number <= last_seq {
            return Err(RelayError::SequenceInvalid);
        }

        // Queue entry for Byzantine quorum consensus
        self.db.queue_for_quorum(&entry, relay_device_pk).await?;

        Ok(entry.entry_id.clone())
    }
}
```

### Android: Relay Collection & Sending

```kotlin
// android/feature/feature-sync/src/main/kotlin/WhisperNetworkSync.kt

class WhisperNetworkSync(
    private val nfcManager: NFCManager,
    private val grpcClient: ChainSyncServiceGrpc.ChainSyncServiceStub,
    private val db: CylinderSealDatabase
) {
    suspend fun relayPendingEntries() {
        // Step 1: Check if device has pending entries
        val pendingEntries = db.journalEntryDao()
            .getPendingEntries()
            .take(100)  // Max 100 per relay
        
        if (pendingEntries.isEmpty()) return

        // Step 2: Prepare EntryRelay message
        val relayMessage = EntryRelay.newBuilder()
            .setOriginatingDevicePk(ByteString.copyFrom(/* originating device pk */))
            .setOriginatingNonce(ByteString.copyFrom(/* originating nonce */))
            .setRelayDevicePk(ByteString.copyFrom(deviceKeystore.publicKey))
            .addAllEntries(pendingEntries.map { it.toProto() })
            .setRelayTimestamp(System.currentTimeMillis() * 1000)  // microseconds
            .setRelayDeviceId(ByteString.copyFrom(deviceId))
            .setRelaySignature(
                ByteString.copyFrom(
                    signRelay(pendingEntries, originatingDevicePk)
                )
            )
            .build()

        // Step 3: Send to super-peer via gRPC
        try {
            val ack = grpcClient.relayEntries(relayMessage)
            
            if (ack.accepted) {
                // Mark successfully relayed entries as QUEUED
                ack.acceptedEntryIdsList.forEach { entryId ->
                    db.journalEntryDao().updateStatus(entryId, SyncStatus.QUEUED)
                }
            }
            
            // Penalize relay device reputation if conflicts
            if (ack.rejectedEntryIdsCount > 0) {
                Log.w("Relay", "Relay rejected ${ack.rejectedEntryIdsCount} entries")
            }
        } catch (e: Exception) {
            Log.e("Relay", "Failed to relay entries: ${e.message}")
        }
    }

    private fun signRelay(entries: List<JournalEntry>, originatingPk: ByteArray): ByteArray {
        // Sign: BLAKE2b(originating_device_pk || entries_cbor)
        val message = hashRelay(originatingPk, entries)
        return Ed25519.sign(message, deviceKeystore.privateKey)
    }
}
```

## Rate Limiting & Spam Prevention

### Relay Device Rate Limits

```rust
// Max 10 relays per device per minute
const MAX_RELAYS_PER_MINUTE: u32 = 10;

// Checked via Redis sliding window:
redis_key = format!("relay-rate:{}", hex::encode(relay_device_pk))
relays_this_minute = redis.incr(relay_key, expire: 60)

if relays_this_minute > MAX_RELAYS_PER_MINUTE {
    return RelayStatus::RateLimited
}
```

### Nonce Sequence Validation

```rust
// Detect stale/replayed relays
redis_key = format!("nonce:{}", hex::encode(originating_device_pk))
last_nonce = redis.get(redis_key)

if relay.originating_nonce <= last_nonce {
    return RelayStatus::NonceStale  // Prevent replay attacks
}

// Update for future checks
redis.set(redis_key, relay.originating_nonce, expire: 24hours)
```

## Reputation Scoring for Relay Devices

Relay devices that forward many conflicted entries have their reputation penalized:

```rust
pub struct ReputationScorer;

impl ReputationScorer {
    pub async fn penalize_relay_device(
        &self,
        relay_device_pk: &[u8],
        conflict_count: u32,
    ) -> Result<()> {
        // Deduct reputation points
        // Base: 0-100
        // Penalty: -1 per conflicted entry
        
        let key = format!("reputation:{}", hex::encode(relay_device_pk));
        let current = redis.get::<i32>(&key).await?.unwrap_or(80);
        let new_score = (current - conflict_count as i32).max(0);
        
        redis.set(&key, new_score).await?;
        
        // Log for credit scoring system
        self.db.insert_relay_event(relay_device_pk, conflict_count).await?;
        
        Ok(())
    }
}
```

## Data Flow Diagram

```
Offline Device (A)    Online Device (B)    Super-Peer (S1-S5)
─────────────────      ──────────────      ──────────────────

Pending entries
  in SQLite
      │
      │ ← NFC discovery
      │      
      ├─ RelayRequest (device A pubkey)
      │
      ├─ PendingEntries response
      │
                        ├─ EntryRelay
                        │  (A's entries + B's signature)
                        │
                        │────→ RelayEntries RPC
                        │
                        │      1. Verify B's relay signature
                        │      2. Check rate limits
                        │      3. Validate A's nonce sequence
                        │      4. Process each entry
                        │      5. Queue to 3-of-5 quorum
                        │
                        │←────  RelayAck
                        │       (accepted_entry_ids)
                        │
                        └─ Store relay success
                           (Update reputation)
      
      Later, when A
      comes online:
      │
      │────→ SyncChain RPC
      │      
      │←─── "Entries 1-5 already CONFIRMED"
      │
      └─ Update SQLite to CONFIRMED
```

## Security Considerations

1. **Relay Signature**: Proves relay device intentionally forwarded entries, not that they created them
2. **Nonce Sequence**: Prevents replay attacks where offline device's stale nonce is submitted multiple times
3. **Rate Limiting**: Prevents relay spam/amplification attacks
4. **Reputation Tracking**: Dishonest relay devices (forwarding conflicts) lose reputation
5. **Deduplication**: Already-received entries are dropped

## Testing Strategy

### Unit Tests

```rust
#[tokio::test]
async fn test_relay_signature_invalid() {
    let relay = create_test_relay();
    let mut corrupted = relay.clone();
    corrupted.relay_signature = vec![0u8; 64];  // Invalid signature
    
    let result = handler.handle_relay_entries(corrupted).await;
    assert_eq!(result.status, RelayStatus::SignatureInvalid);
}

#[tokio::test]
async fn test_relay_nonce_stale() {
    let relay = create_test_relay();
    redis.set(&format!("nonce:{}", hex::encode(&relay.originating_device_pk)), 
              &relay.originating_nonce);  // Set nonce first
    
    let mut stale = relay.clone();
    stale.originating_nonce = vec![0u8; 32];  // Lower nonce
    
    let result = handler.handle_relay_entries(stale).await;
    assert_eq!(result.status, RelayStatus::NonceStale);
}

#[tokio::test]
async fn test_relay_rate_limit() {
    for i in 0..11 {
        let mut relay = create_test_relay();
        relay.relay_timestamp = i * 1000;
        
        let result = handler.handle_relay_entries(relay).await;
        
        if i < 10 {
            assert!(result.accepted);
        } else {
            assert_eq!(result.status, RelayStatus::RateLimited);
        }
    }
}
```

### Integration Tests

```rust
#[tokio::test]
async fn test_offline_device_sync_via_relay() {
    // Scenario: Device A (offline) → Device B (online) → Super-Peer → Device A
    
    let device_a = create_test_device();
    let device_b = create_test_device();
    let super_peer = create_test_super_peer().await;

    // Device A: Create 3 pending entries
    let entries = (1..=3)
        .map(|i| create_test_entry(device_a.id(), i))
        .collect::<Vec<_>>();
    
    // Device B: Relay entries to super-peer
    let relay = EntryRelay {
        originating_device_pk: device_a.public_key().to_vec(),
        relay_device_pk: device_b.public_key().to_vec(),
        entries: entries.clone(),
        ..Default::default()
    };
    
    let ack = super_peer.relay_entries(relay).await.unwrap();
    assert!(ack.accepted);
    assert_eq!(ack.accepted_entry_ids.len(), 3);
    
    // Device A: Verify entries show CONFIRMED after coming online
    let synced_entries = device_a.sync_chain(super_peer).await.unwrap();
    assert!(synced_entries.iter().all(|e| e.sync_status == SyncStatus::Confirmed));
}
```

## Future Enhancements

1. **Mesh Routing**: Multi-hop relays (A → B → C → Super-Peer)
2. **Compression**: CBOR compression for large relay payloads
3. **Adaptive Rate Limits**: Dynamic limits based on network conditions
4. **Geographic Awareness**: Prefer nearby relays to optimize latency
5. **Relay Incentives**: Reward devices that relay many entries (gamification)
