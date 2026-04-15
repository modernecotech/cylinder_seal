# Android Week 2 Integration Guide

This document shows Android engineers how the Rust cs-core types integrate with Kotlin code.

---

## What Rust Provides (cs-core) — Imported via gRPC Stubs

### Transaction Type
```protobuf
// From proto/chain_sync.proto (Rust → Android)
message Transaction {
  bytes transaction_id = 1;           // UUID v7
  bytes from_public_key = 2;          // Ed25519, 32 bytes
  bytes to_public_key = 3;            // Ed25519, 32 bytes
  int64 amount_owc = 4;               // micro-OWC (ALWAYS i64, never float)
  string currency_context = 5;        // "KES", "NGN", etc.
  string fx_rate_snapshot = 6;        // Decimal as string
  int64 timestamp_utc = 7;            // Unix microseconds
  bytes previous_nonce = 8;           // Previous tx's nonce (RFC 6979 chain link)
  bytes current_nonce = 9;            // Current nonce derived from previous + hardware
  int64 monotonic_clock_nanos = 10;   // System.nanoTime(), detects clock-skew attacks
  bytes device_id = 11;               // Which device created this transaction
  PaymentChannel channel = 12;        // NFC, BLE, ONLINE
  string memo = 13;                   // Max 140 chars
  string device_attestation = 14;     // SafetyNet/Play Integrity JWT (optional)
  double latitude = 16;               // Transaction location (-90 to +90), 0 if unavailable
  double longitude = 17;              // Transaction location (-180 to +180), 0 if unavailable
  int32 location_accuracy_meters = 18;  // GPS accuracy in meters (0 if unknown)
  int64 location_timestamp_utc = 19;  // When location was captured (Unix microseconds)
  LocationSource location_source = 20;  // GPS, NETWORK, LAST_KNOWN, or OFFLINE
  bytes signature = 15;               // Ed25519, 64 bytes over canonical CBOR (includes location)
}

enum LocationSource {
  LOCATION_SOURCE_UNSPECIFIED = 0;
  LOCATION_SOURCE_GPS = 1;            // Real-time GPS (high accuracy)
  LOCATION_SOURCE_NETWORK = 2;        // Network-based (WiFi/cell, lower accuracy)
  LOCATION_SOURCE_LAST_KNOWN = 3;     // Last known location from prior sync
  LOCATION_SOURCE_OFFLINE = 4;        // User provided (when offline, no automated source)
}
```

**Location Note**: All transactions now include location data (lat/lon, accuracy, timestamp, source). This enables fraud detection (geographic anomalies) and device reputation scoring. For offline NFC transactions, use LAST_KNOWN or OFFLINE. See [LOCATION_CAPTURE_GUIDE.md](LOCATION_CAPTURE_GUIDE.md) for implementation details.

### JournalEntry Type
```protobuf
message JournalEntry {
  bytes entry_id = 1;                 // UUID v7, 16 bytes
  bytes user_public_key = 2;          // Ed25519, 32 bytes (identity of journal owner)
  uint64 sequence_number = 3;         // Monotonically increasing for this user
  bytes prev_entry_hash = 4;          // BLAKE2b-256 of previous entry, 32 bytes
  repeated Transaction transactions = 5;  // One or more transactions in this entry
  bytes entry_hash = 6;               // BLAKE2b-256(prev_entry_hash || seq || txs)
  bytes signature = 7;                // Ed25519 over entry_hash, 64 bytes
  int64 created_at = 8;               // Device UTC microseconds
  SyncStatus sync_status = 9;         // PENDING, CONFIRMED, CONFLICTED
  bytes device_id = 10;               // UUIDv7 of device that created this
  map<string, uint64> vector_clock = 11;  // Maps device_id → logical clock for causality
  int64 monotonic_created_nanos = 12; // System.nanoTime() for ordering
  bytes user_signature = 13;          // Optional, for high-value transactions
  repeated SuperPeerConfirmation super_peer_confirmations = 14;  // 3+ of 5 required
}

message SuperPeerConfirmation {
  string super_peer_id = 1;           // ID of confirming super-peer
  bytes signature = 2;                // 64-byte Ed25519 confirmation signature
  int64 confirmed_at = 3;             // When super-peer confirmed (UTC micros)
}
```

---

## What Android Must Do (Kotlin Implementation)

### 1. Collect Hardware IDs at App Install

**File**: `android/core/core-crypto/src/HardwareIdentifier.kt`

```kotlin
object HardwareIdentifier {
    fun captureDeviceIds(): DeviceHardwareIds {
        val serial = Build.getSerial()  // Device manufacturing serial
        val imei = TelephonyManager.from(context).imei  // SIM card IMEI
        val model = Build.DEVICE        // Device model identifier
        
        return DeviceHardwareIds(
            device_serial = serial,
            device_imei = imei,
            device_model = model,
            captured_at = System.currentTimeMillis() * 1000  // Convert ms to micros
        )
    }
    
    fun getHardwareFingerprint(hw: DeviceHardwareIds): String {
        // Uses BLAKE2b from cs-core (via JNI if needed)
        return blake2b256(hw.toBindingBytes()).toHexString()
    }
}

data class DeviceHardwareIds(
    val device_serial: String,
    val device_imei: String,
    val device_model: String,
    val captured_at: Long
)
```

### 2. Generate Ed25519 Keypair in Android Keystore

**File**: `android/core/core-crypto/src/Keystore.kt`

```kotlin
class KeystoreManager {
    fun generateDeviceKeypair(): KeyPair {
        // Use Tink for hardware-backed Ed25519
        val keysetHandle = TinkKeyGenerator.generateEd25519Keypair(
            masterKeyAlias = "CylinderSeal_Master_Key",
            requireStrongbox = Build.VERSION.SDK_INT >= 28
        )
        
        // Private key: stays in Keystore, never leaves device
        this.devicePrivateKey = keysetHandle.getPrivateKey()
        
        // Public key: exported for transactions
        this.devicePublicKey = keysetHandle.getPublicKey()  // 32 bytes
        
        return Pair(devicePublicKey, devicePrivateKey)
    }
    
    // Sign a message (private key never leaves Keystore)
    fun signMessage(data: ByteArray): ByteArray {
        // Use JNI call or Tink's Signer interface
        return tinkSigner.sign(data)  // 64-byte Ed25519 signature
    }
}
```

### 3. Derive Deterministic Nonces (Kotlin FFI to Rust)

**File**: `android/core/core-crypto/src/NonceDerivation.kt`

```kotlin
object NonceDerivation {
    // Call Rust cs-core nonce derivation via JNI
    external fun deriveNonceWithHardware(
        previousNonce: ByteArray,      // 32 bytes
        hardwareIds: DeviceHardwareIds,
        counter: Long
    ): ByteArray  // 32 bytes
    
    fun deriveNextNonce(hwIds: DeviceHardwareIds, counter: Long): ByteArray {
        val previousNonce = ledger.getLastTransaction()?.current_nonce 
            ?: blake2b256(userPublicKey)  // Genesis nonce
        
        return deriveNonceWithHardware(previousNonce, hwIds, counter)
    }
}

// JNI Binding (Rust library: crates/cs-core-android-jni)
// Links to crates/cs-core/src/nonce.rs::derive_nonce_with_hardware()
```

### 4. Create Transaction with Nonce

**File**: `android/feature/feature-pay/src/PaymentFlow.kt`

```kotlin
// Step 1: Derive nonce
val hwIds = HardwareIdentifier.captureDeviceIds()
val nextNonce = NonceDerivation.deriveNextNonce(hwIds, txCounter)

// Step 2: Capture location (if available)
val location = LocationProvider.captureLocation()  // Suspends for GPS if needed
val (latitude, longitude, accuracy, locationSource) = if (location != null) {
    Quadruple(
        location.latitude,
        location.longitude,
        location.accuracy.toInt(),
        LocationSource.GPS
    )
} else {
    // Offline: use last known or zeros
    Quadruple(0.0, 0.0, 0, LocationSource.OFFLINE)
}

// Step 3: Create transaction (matches proto/chain_sync.proto)
val tx = Transaction(
    transaction_id = uuidv7(),               // Generate new UUIDv7
    from_public_key = devicePublicKey,       // Your public key (32 bytes)
    to_public_key = recipientPublicKey,      // Recipient's public key (32 bytes)
    amount_owc = 50_000_000,                 // 50 OWC in micro-OWC
    currency_context = "KES",                // Display currency
    fx_rate_snapshot = "0.987654",           // Decimal as string
    timestamp_utc = System.currentTimeMillis() * 1000,  // Convert ms to micros
    previous_nonce = previousNonce,          // RFC 6979 chain: prior tx's nonce
    current_nonce = nextNonce,               // RFC 6979 chain: derived from previous + hardware
    monotonic_clock_nanos = System.nanoTime(),  // Detects clock-skew attacks
    device_id = deviceId,                    // UUIDv7 of device creating this
    channel = PaymentChannel.NFC,            // NFC, BLE, or ONLINE
    memo = "Payment for goods",              // Optional memo
    device_attestation = attestationToken,   // SafetyNet/Play Integrity JWT
    latitude = latitude,                     // Transaction location (0 if unavailable)
    longitude = longitude,                   // Transaction location (0 if unavailable)
    location_accuracy_meters = accuracy,     // GPS accuracy or 0
    location_source = locationSource,        // GPS, NETWORK, LAST_KNOWN, or OFFLINE
    signature = ByteArray(64),               // Will be filled below
)

// Step 4: Sign transaction (location is included in canonical CBOR)
tx.signature = keystore.signMessage(tx.canonicalCborForSigning())

// Step 5: Verify locally (double-check)
assert(tx.verifySignature() == null)  // Should not throw

// Step 6: Send to recipient (NFC/BLE/online)
// Or store in pending if offline
ledger.append(JournalEntry(transactions = listOf(tx)))
```

### 5. Get Device Attestation (SafetyNet/Play Integrity)

**File**: `android/core/core-crypto/src/Attestation.kt`

```kotlin
class PlayIntegrityManager {
    suspend fun getDeviceAttestation(): String {
        val client = IntegrityTokenProvider.getInstance(context)
        val request = StandardIntegrityTokenRequest.builder()
            .setCloudProjectNumber(PROJECT_NUMBER)
            .build()
        
        val response = client.getIntegrityToken(request)
        return response.token  // JWT token (opaque to device)
    }
    
    // For transactions > 5 OWC (Anonymous tier), include attestation
    fun attachAttestationIfNeeded(tx: Transaction) {
        if (tx.amount_owc >= attestationThreshold) {
            tx.device_attestation = getDeviceAttestation()
        }
    }
}
```

### 6. Verify Nonce Chain Locally (Optional)

**File**: `android/feature/feature-sync/src/NonceValidation.kt`

```kotlin
// On device, before submitting to super-peer:
// (Optional: just for extra safety, super-peer will verify)

fun verifyNonceChain(tx: Transaction, hwIds: DeviceHardwareIds, counter: Long) {
    // Verify: current_nonce was derived from previous_nonce + hw + counter
    val expectedNonce = NonceDerivation.deriveNonceWithHardware(
        tx.previous_nonce,
        hwIds,
        counter
    )
    
    assert(expectedNonce.contentEquals(tx.current_nonce)) {
        "Nonce chain broken! Device state corrupted."
    }
}
```

---

## Data Flow Diagram

```
Device                          Super-Peer (Rust)
-------                         ------------------

User pays             → Derive nonce (hw-bound)
                      → Create Transaction
                      → Sign with device key (Keystore)
                      → Create JournalEntry
                      → Sign entry_hash
                      
Entry.serialize()     → Send via gRPC SyncChain →  Verify signature
                      ← SyncAck (3/5 confirmed)    Verify nonce chain
                      ← Entry confirmed             Verify sequence
                                                    Add to ledger
                                                    Gossip to 4 peers
```

---

## Key Validation Points

**On Device (Before Sending)**:
- ✅ Transaction amount is i64 micro-OWC (never float)
- ✅ from_public_key ≠ to_public_key (not sending to self)
- ✅ Nonce chain is valid: current_nonce = HKDF(previous_nonce || hardware_ids || counter)
- ✅ Monotonic clock is not going backward (clock-skew attack detection)
- ✅ Location is captured (GPS if online, LAST_KNOWN if cached, OFFLINE if unavailable)
- ✅ Signature verifies over canonical CBOR of transaction (including location, excluding signature field)
- ✅ Device daily limit not exceeded (determined by KYC tier: Anonymous/PhoneVerified/FullKYC)
- ✅ Transaction timestamp is recent (within 1 minute of device time)

**On Super-Peer**:
- ✅ Ed25519 signature verifies using transaction's from_public_key
- ✅ Device attestation is valid (SafetyNet or Play Integrity JWT not expired)
- ✅ RFC 6979 nonce chain is valid (current derives from previous via hardware binding)
- ✅ Sequence number increments by 1 (no gaps, no reuses)
- ✅ Monotonic clock never goes backward for this device
- ✅ Location coordinates are valid (-90 to +90 lat, -180 to +180 lon)
- ✅ No geographic anomalies (user didn't travel >1800km in <2 hours)
- ✅ Device daily limit not exceeded (tiered by KYC verification level)
- ✅ Device reputation score is acceptable (ML-computed anomaly detection, including location)
- ✅ 3+ of 5 super-peers agree to confirm (Byzantine consensus)

---

## Error Cases to Handle

```kotlin
// Nonce chain broken (device state corrupted)
try {
    verifyNonceChain(tx, hwIds, counter)
} catch (e: AssertionError) {
    // Force full ledger resync
    ledger.forceSync()
}

// Signature verification failed (tampered in transit)
if (!tx.verifySignature()) {
    // Reject, log, alert user
    Crashlytics.recordException(SignatureMismatchException())
}

// Device daily limit exceeded
if (deviceSpentToday + amount > dailyLimit) {
    // Show error: "You've reached your daily limit"
}

// Attestation expired or failed
if (!attestation.is_valid()) {
    // Request fresh attestation via PlayIntegrityManager
}
```

---

## Rust ↔ Kotlin Interop

### JNI Binding Needed
```rust
// crates/cs-core-android-jni/src/lib.rs
#[no_mangle]
pub extern "C" fn Java_com_cylinderseal_crypto_NonceDerivation_deriveNonceWithHardware(
    _env: JNIEnv,
    _class: JClass,
    prev_nonce: jbyteArray,
    device_serial: jstring,
    device_imei: jstring,
    counter: jlong,
) -> jbyteArray {
    // Call cs-core::nonce::derive_nonce_with_hardware()
    // Convert Java types to Rust types, call function, return result
}
```

### Kotlin ↔ JNI Wrapper
```kotlin
// android/core/core-crypto/src/NativeLib.kt
object NativeLib {
    init {
        System.loadLibrary("cs_core_android")  // libcs_core_android.so
    }
    
    external fun deriveNonceWithHardware(
        previousNonce: ByteArray,
        deviceSerial: String,
        deviceImei: String,
        counter: Long
    ): ByteArray
}
```

---

## Testing (Week 2)

### Unit Tests
```kotlin
@Test
fun testNonceChainValidation() {
    val hwIds = DeviceHardwareIds("serial123", "imei456", "pixel5", now)
    val genesis = blake2b256(userPublicKey)
    
    val nonce1 = NonceDerivation.deriveNextNonce(hwIds, 1)
    val nonce2 = NonceDerivation.deriveNextNonce(hwIds, 2)
    
    // Each nonce should be different
    assertNotEquals(nonce1, nonce2)
    
    // And reproducible (call again, get same result)
    val nonce1_again = NonceDerivation.deriveNextNonce(hwIds, 1)
    assertEquals(nonce1, nonce1_again)
}

@Test
fun testDeviceCloneDetection() {
    val hwIds1 = DeviceHardwareIds("serial123", "imei456", "pixel5", now)
    val hwIds2 = DeviceHardwareIds("serial_different", "imei456", "pixel5", now)
    
    // Same nonce derivation parameters, different hardware
    val nonce1 = NonceDerivation.deriveNonceWithHardware(
        previousNonce, hwIds1, counter
    )
    val nonce2 = NonceDerivation.deriveNonceWithHardware(
        previousNonce, hwIds2, counter
    )
    
    // Must be different (hardware-bound)
    assertNotEquals(nonce1, nonce2)
}

@Test
fun testTransactionSigning() {
    val tx = createTransaction()
    tx.signature = keystore.signMessage(tx.canonicalCborForSigning())
    
    // Verify signature works
    assertEquals(null, tx.verifySignature())
}
```

### Integration Test (Full Payment Flow)
```kotlin
@Test
fun testOfflinePayment() {
    // Device A and B, both offline, exchange payment via NFC
    
    val deviceA = DeviceUnderTest("pixel5_a")
    val deviceB = DeviceUnderTest("pixel5_b")
    
    // A sends 10 OWC to B via NFC
    val tx = deviceA.createPayment(
        to = deviceB.publicKey,
        amount = 10_000_000  // 10 OWC
    )
    
    // B receives, verifies, stores
    assert(tx.verifySignature())
    assert(deviceB.ledger.append(tx))
    
    // Later, both sync with super-peer
    // (super-peer will accept both chains, no double-spend)
}
```

---

## References

**Rust Code (Already Implemented)**:
- `crates/cs-core/src/nonce.rs` — Nonce derivation with hardware binding
- `crates/cs-core/src/hardware_binding.rs` — Device identity & reputation
- `crates/cs-core/src/models.rs` — Transaction & JournalEntry types
- `crates/cs-core/src/crypto.rs` — BLAKE2b, Ed25519 primitives

**Android Code (Week 2 Implementation)**:
- `android/core/core-crypto/` — Keystore, crypto, JNI bindings
- `android/feature/feature-pay/` — Payment creation & signing
- `android/core/core-database/` — Room schema with SQLCipher

**Architecture Docs**:
- `/docs/IRON_SECURITY.md` — Full technical specifications
- `/docs/DEVELOPER_QUICK_REFERENCE.md` — Code patterns & security checklist
- `/WEEK1_STATUS.md` — Week 1 completion status

---

**Ready for Week 2 Android implementation!** 🚀
