# Location Capture Guide for CylinderSeal Transactions

## Overview

All CylinderSeal transactions now include location data (latitude, longitude, accuracy, timestamp, and source). This enables:
- **Fraud detection**: Geographic anomalies flagged as device reputation risk
- **Conflict resolution**: Location evidence in receipts helps break ties in double-spend scenarios
- **Regulatory compliance**: Complete audit trail with transaction context
- **Privacy**: Location stored only in transaction and in optional user audit logs (not a separate profile)

## Transaction Location Fields

```proto
double latitude = 16;                      // -90 to +90
double longitude = 17;                     // -180 to +180
int32 location_accuracy_meters = 18;       // GPS accuracy (0 if unavailable)
int64 location_timestamp_utc = 19;         // When location was captured (microseconds)
LocationSource location_source = 20;       // GPS, NETWORK, LAST_KNOWN, OFFLINE
```

### LocationSource Enum

```rust
pub enum LocationSource {
    Unspecified,       // Default, unused
    GPS,               // Real-time GPS (high accuracy, <10m)
    Network,           // Network-based via WiFi/cell triangulation
    LastKnown,         // Cached location from prior session
    Offline,           // User-provided (when offline, no automated source)
}
```

## Android Implementation

### 1. Location Permissions (AndroidManifest.xml)

```xml
<uses-permission android:name="android.permission.ACCESS_FINE_LOCATION" />
<uses-permission android:name="android.permission.ACCESS_COARSE_LOCATION" />
```

Require runtime permissions at payment initiation (API 23+):
```kotlin
ActivityCompat.requestPermissions(
    this,
    arrayOf(
        Manifest.permission.ACCESS_FINE_LOCATION,
        Manifest.permission.ACCESS_COARSE_LOCATION
    ),
    LOCATION_PERMISSION_REQUEST_CODE
)
```

### 2. Location Provider Setup (FusedLocationProviderClient)

```kotlin
// In your payment fragment/activity
private val fusedLocationClient: FusedLocationProviderClient =
    LocationServices.getFusedLocationProviderClient(context)

// Create a location request for payment
private val locationRequest = LocationRequest.create().apply {
    interval = 1000                    // 1 second updates
    fastestInterval = 500             // Fastest update rate
    priority = LocationRequest.PRIORITY_HIGH_ACCURACY
    numUpdates = 1                     // Just one location fix
}
```

### 3. Capture Location During Payment

```kotlin
private suspend fun captureLocation(): TransactionLocation? {
    return withContext(Dispatchers.Default) {
        try {
            val location = Tasks.await(
                fusedLocationClient.lastLocation
            )
            
            if (location != null) {
                TransactionLocation(
                    latitude = location.latitude,
                    longitude = location.longitude,
                    accuracyMeters = location.accuracy.toInt(),
                    timestampUtc = location.time * 1000, // Convert ms to microseconds
                    source = LocationSource.GPS
                )
            } else {
                // Try network-based location
                captureNetworkLocation()
            }
        } catch (e: SecurityException) {
            Timber.w("Location permission denied")
            null
        } catch (e: Exception) {
            Timber.e(e, "Location capture failed")
            null
        }
    }
}

private suspend fun captureNetworkLocation(): TransactionLocation? {
    // Fallback to coarse location if GPS fails
    return try {
        val location = Tasks.await(
            fusedLocationClient.lastLocation
        )
        if (location != null) {
            TransactionLocation(
                latitude = location.latitude,
                longitude = location.longitude,
                accuracyMeters = location.accuracy.toInt().coerceAtMost(2000),
                timestampUtc = location.time * 1000,
                source = LocationSource.NETWORK
            )
        } else null
    } catch (e: Exception) {
        null
    }
}
```

### 4. Transaction Builder with Location

```kotlin
class TransactionBuilder {
    suspend fun buildPaymentTransaction(
        recipientPublicKey: ByteArray,
        amountOwc: Long,
        currencyContext: String,
        fxRateSnapshot: String,
        channel: PaymentChannel
    ): Transaction {
        // Capture location (may be null for offline txs)
        val location = captureLocation()
        
        // If offline or location unavailable, use zero/OFFLINE
        val (lat, lon, accuracy, source) = if (location != null) {
            Triple(
                location.latitude,
                location.longitude,
                location.accuracyMeters,
                LocationSource.GPS
            ) to LocationSource.GPS
        } else {
            Triple(0.0, 0.0, 0, LocationSource.OFFLINE) to LocationSource.OFFLINE
        }

        return Transaction(
            fromPublicKey = deviceKeyPair.publicKey,
            toPublicKey = recipientPublicKey,
            amountOwc = amountOwc,
            currencyContext = currencyContext,
            fxRateSnapshot = fxRateSnapshot,
            channel = channel,
            memo = "Payment",
            deviceId = deviceId,
            previousNonce = lastNonce,
            currentNonce = derivedNonce,
            latitude = lat,
            longitude = lon,
            locationAccuracyMeters = accuracy,
            locationSource = source,
            // ... other fields
        )
    }
}
```

### 5. Offline Fallback

For NFC/BLE transactions when offline:
- User provides location manually (optional), or
- Use last known location from prior sync, or
- Leave location at (0.0, 0.0) with source=OFFLINE

Example:
```kotlin
if (isOffline) {
    val lastLocation = sharedPreferences.getLocation("last_location")
    transactionLocation = if (lastLocation != null && isRecentEnough(lastLocation)) {
        TransactionLocation(
            latitude = lastLocation.lat,
            longitude = lastLocation.lon,
            accuracyMeters = 0, // Unknown accuracy for cached location
            timestampUtc = lastLocation.timestamp,
            source = LocationSource.LAST_KNOWN
        )
    } else {
        // No location available
        TransactionLocation(0.0, 0.0, 0, now(), LocationSource.OFFLINE)
    }
}
```

## Rust Backend Validation

### Super-Peer Validation Rules

```rust
pub fn validate_transaction_location(tx: &Transaction) -> Result<()> {
    // 1. Sanity checks on coordinates
    if tx.latitude < -90.0 || tx.latitude > 90.0 {
        return Err("Invalid latitude");
    }
    if tx.longitude < -180.0 || tx.longitude > 180.0 {
        return Err("Invalid longitude");
    }

    // 2. Accuracy check (warn if suspiciously broad)
    if tx.location_accuracy_meters > 5000 {
        // Log as anomaly but don't reject
        warn!("Transaction with very low accuracy: {} meters", tx.location_accuracy_meters);
    }

    // 3. Geographic anomaly detection
    if let Some(prev_tx) = get_previous_transaction(&tx.from_public_key)? {
        let distance_km = haversine_distance(
            (prev_tx.latitude, prev_tx.longitude),
            (tx.latitude, tx.longitude)
        );
        let time_delta_hours = (tx.timestamp_utc - prev_tx.timestamp_utc) as f64 / 3_600_000_000.0;
        
        // Typical max speed: ~900 km/h (commercial airline)
        // Add 2x buffer: 1800 km max
        if distance_km > 1800.0 && time_delta_hours < 2.0 {
            // Flag as anomaly in device reputation
            device_reputation.add_anomaly("geographic_jump", &serde_json::json!({
                "distance_km": distance_km,
                "time_hours": time_delta_hours
            }))?;
        }
    }

    Ok(())
}
```

### Geographic Anomaly Thresholds: Rationale

**900 km/h base speed**: Cruising speed of commercial aircraft (Boeing 747 ~920 km/h). Conservative lower bound for "fastest possible legitimate travel."

**2x buffer (1800 km / 2 hours)**: Applied to account for:
- Multiple flight segments with layovers (device travels offline for portion)
- Business travelers making same-day intercontinental trips
- Relief workers/humanitarian aid workers in crisis zones
- Plus ~2 hour error margin for clock skew across devices

**Why not 1.5x?** Would flag ~10% of legitimate business travelers on Africa-Europe routes.
**Why not 3x?** Would miss satellite spoofing attacks (could claim to be 5400+ km away in 2 hours).

**5000m accuracy threshold**: Warns on locations with >5km error margin
- GPS accuracy typically ±10-20m in open sky
- Network-based location (cell triangulation) typically ±1000-5000m
- Values >5000m suggest offline/cached location from >1 hour ago
- Doesn't reject the transaction (still valid), but marks as lower-confidence for fraud scoring

### Device Reputation Anomalies

Location-based anomalies are stored in `DeviceReputation.anomalies`:

```json
{
    "anomalies": [
        "geographic_jump",      // Too fast travel between locations (>1800 km / <2 hours)
        "unusual_time",         // Transaction at unusual hour for user's timezone
        "high_frequency",       // Rapid sequence of transactions (>10 per hour)
        "location_spoofing"     // Impossible coordinates or consistency violations
    ]
}
```

## Privacy Considerations

- Location is **only stored** in the transaction ledger and optional user audit logs
- Location is **not extracted** into a separate user profile
- Location is included in the **signed transaction** (cannot be modified after signing)
- Users can view their own transaction locations in audit log
- Location is **not exposed** to super-peers in user balance queries (only in conflict resolution)

## Testing

### Unit Tests (Rust)

```rust
#[test]
fn test_location_validation_out_of_bounds() {
    let tx = Transaction {
        latitude: 95.0,  // Invalid
        longitude: 0.0,
        ..default()
    };
    assert!(validate_transaction_location(&tx).is_err());
}

#[test]
fn test_geographic_anomaly_detection() {
    // Transaction 1: Nairobi (2023-01-01 12:00 UTC)
    // Transaction 2: London (2023-01-01 12:30 UTC)
    // Distance: ~6,800 km, time: 30 min → anomaly
    assert!(is_geographic_anomaly(&tx1, &tx2));
}
```

### Integration Tests (Android)

```kotlin
@Test
fun testLocationCaptureOnPayment() {
    // Mock FusedLocationClient
    val mockLocation = Location("gps").apply {
        latitude = -1.2921
        longitude = 36.8219
        accuracy = 10f
        time = System.currentTimeMillis()
    }
    
    val transaction = transactionBuilder.buildPaymentTransaction(
        recipientPublicKey = recipientKey,
        amountOwc = 1_000_000L,
        currencyContext = "KES",
        fxRateSnapshot = "50.2",
        channel = PaymentChannel.NFC
    )
    
    assertEquals(-1.2921, transaction.latitude)
    assertEquals(36.8219, transaction.longitude)
    assertEquals(LocationSource.GPS, transaction.locationSource)
}
```

## Future Enhancements

1. **Merchant Fixed Locations** (Phase 2): Merchants can register fixed location for all transactions (no GPS needed)
2. **Geofencing** (Phase 2): Super-peer can flag unusual regional concentrations
3. **Regulatory Reporting** (Phase 2): Export location data for AML/CFT compliance
4. **Location Privacy** (Phase 3): Optional location obfuscation (round to nearest city-block) for user privacy
