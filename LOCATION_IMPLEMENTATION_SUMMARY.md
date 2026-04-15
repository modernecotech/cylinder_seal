# Location Field Implementation Summary

## Changes Made

### 1. Proto Definitions (Updated)

**Files**:
- `proto/chain_sync.proto`
- `android/core/core-network/src/main/proto/chain_sync.proto`

**Changes**:
- Added 5 new fields to `Transaction` message:
  - `double latitude = 16` — decimal degrees (-90 to +90)
  - `double longitude = 17` — decimal degrees (-180 to +180)
  - `int32 location_accuracy_meters = 18` — GPS accuracy in meters
  - `int64 location_timestamp_utc = 19` — when location was captured (microseconds)
  - `LocationSource location_source = 20` — enum: GPS, NETWORK, LAST_KNOWN, OFFLINE

- Added new enum `LocationSource` with 4 values:
  - `LOCATION_SOURCE_GPS` — Real-time GPS (high accuracy)
  - `LOCATION_SOURCE_NETWORK` — Network-based (WiFi/cell, lower accuracy)
  - `LOCATION_SOURCE_LAST_KNOWN` — Cached location from prior sync
  - `LOCATION_SOURCE_OFFLINE` — User-provided or unavailable

### 2. Rust Core Models (Updated)

**File**: `crates/cs-core/src/models.rs`

**Changes**:
- Added location fields to `Transaction` struct:
  ```rust
  pub latitude: f64,
  pub longitude: f64,
  pub location_accuracy_meters: i32,
  pub location_timestamp_utc: i64,
  pub location_source: LocationSource,
  ```

- Added `LocationSource` enum:
  ```rust
  pub enum LocationSource {
      Unspecified,
      GPS,
      Network,
      LastKnown,
      Offline,
  }
  ```

- Updated `Transaction::new()` to accept location parameters:
  ```rust
  pub fn new(
      // ... existing fields ...
      latitude: f64,
      longitude: f64,
      location_accuracy_meters: i32,
      location_source: LocationSource,
  ) -> Self
  ```

- Updated `canonical_cbor_for_signing()` to include location in signed data:
  - Location fields are now part of the transaction signature
  - Cannot be modified after signing

### 3. Documentation (Created)

**New Files**:
- `LOCATION_CAPTURE_GUIDE.md` (380+ lines)
  - Complete Android implementation guide
  - Rust backend validation rules
  - Privacy considerations
  - Testing strategies
  - Future enhancements

**Updated Files**:
- `ANDROID_WEEK2_BRIDGE.md`
  - Added `LocationSource` enum definition
  - Updated Transaction proto definition with location fields
  - Updated Kotlin transaction creation example to capture and include location
  - Updated validation points to include location checks

### 4. Memory (Created)

**File**: `memory/location_field.md`
- Project memory recording the location field decision
- Explains why (fraud detection, device reputation, compliance)
- Outlines how to apply (Android FusedLocationProviderClient, Rust validation, PostgreSQL storage)

**Updated**: `memory/MEMORY.md` with index entry

## Location Capture Strategy

### Android (FusedLocationProviderClient)

1. **Real-time Capture** (Online):
   - Request GPS via `LocationServices.getFusedLocationProviderClient()`
   - Wait up to 1 second for position fix
   - Include accuracy metrics
   - Source: `LocationSource.GPS`

2. **Fallback** (No GPS):
   - Use network-based location (WiFi/cell triangulation)
   - Less accurate but sufficient for anomaly detection
   - Source: `LocationSource.NETWORK`

3. **Last Known** (Offline):
   - Cache location from previous online session
   - Use if transaction occurs while offline
   - Source: `LocationSource.LAST_KNOWN`

4. **User Provided** (No Data):
   - Leave location at (0.0, 0.0) if offline and no cache
   - Source: `LocationSource.OFFLINE`
   - No auto-location, user may provide manually

### Rust Backend Validation

1. **Coordinate Sanity Check**:
   - Latitude: -90 to +90 (required)
   - Longitude: -180 to +180 (required)
   - Accuracy: 0 to 5000+ meters (warn if >5000)

2. **Geographic Anomaly Detection**:
   - Haversine distance from previous transaction
   - **Base speed**: ~900 km/h (commercial aircraft cruising speed)
   - **Applied buffer**: 2x → 1800 km in 2 hours maximum
   - **Rationale for 2x buffer**: Accounts for flight connections/layovers + clock skew + business travel patterns
   - Violation → flag as device reputation anomaly (doesn't reject, reduces credit score)

3. **Device Reputation Impact**:
   - Anomalies stored in `DeviceReputation.anomalies`
   - Example: `["geographic_jump", "unusual_time", "high_frequency"]`
   - ML-computed reputation score reflects location history

## Integration Points

### Transaction Signing
Location is part of canonical CBOR encoding:
- Cannot be changed after signing
- Both device and super-peer verify location matches signature
- Ensures location is authentic and tampering-proof

### Conflict Resolution
Location evidence used as tiebreaker in double-spend:
- NFC receipt includes signer's location
- Receipt-holder location matches transaction location
- Helps determine which chain branch is legitimate

### Audit Trail
All transaction location data stored in PostgreSQL:
- Field: `JSONB` in `ledger_entries` table
- Indexed for geographic queries (Phase 2)
- Queryable by user for privacy review

## Privacy Model

- Location is **only stored** in transaction ledger
- Location is **not profiled** separately
- Location is **not shared** unless needed for conflict resolution
- Users can **view their own** transaction locations
- Location is **signed** (tamper-proof)

## Backward Compatibility

- Proto field numbers (16-20) don't conflict with existing fields
- Rust: Zero values (0.0, 0, UNSPECIFIED) for old/missing data
- Android: Optional field in proto (filled at transaction time)
- Super-Peer: Validates location, treats 0.0/0.0 as "no location"

## Testing

### Unit Tests (Rust)
- Coordinate validation (in/out of bounds)
- Geographic anomaly detection (fast travel)
- Signature includes location (tampering detection)

### Integration Tests (Android)
- Location capture on payment
- Correct source enum based on provider
- Location survives CBOR serialization
- Signature includes location

### End-to-End
- Device A: Payment at location (lat1, lon1)
- Device B: Receipt with location
- Super-Peer: Validates locations match, checks anomalies
- Storage: Location persisted in ledger

## Rollout Plan (MVP to Phase 2)

**MVP**:
- ✅ Location captured in all transactions
- ✅ Location signed (tampering-proof)
- ✅ Super-peer validates coordinates
- ✅ Geographic anomalies flagged in device reputation
- ❌ Location not used for live fraud blocking (Phase 2)

**Phase 2**:
- Enable geographic anomaly notifications
- Merchant fixed location registration
- User location privacy controls
- Geographic clustering analysis

**Phase 3**:
- Regulatory AML/CFT location reports
- Geographic risk scoring
- Merchant regional patterns analysis
