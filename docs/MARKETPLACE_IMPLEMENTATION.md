# Peer-to-Peer Marketplace Implementation Guide

## Overview

The CylinderSeal Marketplace transforms the payment system into a complete economic platform where peers can:
1. **Create listings** (products/services) with photos, prices, variants, delivery methods
2. **Discover services** via peer-to-peer gossip network (offline browsing possible)
3. **Purchase securely** (transactions confirmed via quorum voting)
4. **Build seller reputation** (ratings feed into credit scoring)
5. **Resolve disputes** (escrow + reputation-based arbitration)

---

## Phase 5: Marketplace MVP (Weeks 17-24, Post-Security Deployment)

### Week 17-18: Proto Definitions & Core Models

**Proto Definition** (`proto/marketplace.proto`):
```protobuf
// Marketplace Listing
message MarketplaceListing {
    string listing_id = 1;                    // UUIDv7
    string seller_user_id = 2;
    string seller_public_key = 3;
    
    string title = 4;
    string description = 5;
    string category = 6;                      // FastFood, Taxi, Cleaning, etc
    string subcategory = 7;
    
    int64 base_price_owc = 8;
    string currency_context = 9;              // KES, NGN, etc (display only)
    
    repeated ListingVariant variants = 10;
    repeated string image_ipfs_hashes = 11;   // IPFS hashes for images
    
    string operating_hours_tz = 12;
    string operating_hours = 13;              // "09:00-17:00 Mon-Fri"
    double seller_latitude = 14;
    double seller_longitude = 15;
    
    repeated DeliveryMethod delivery_methods = 16;
    
    int64 created_at_utc = 17;
    int64 last_updated_utc = 18;
    bool is_active = 19;
    
    // Computed on super-peer
    int32 view_count = 20;
    float average_rating = 21;
    int32 completed_sales = 22;
}

message ListingVariant {
    string variant_id = 1;
    string name = 2;
    int64 price_delta_owc = 3;
    string description = 4;
}

enum DeliveryMethod {
    PICKUP = 0;
    DELIVERY_LOCAL = 1;
    DELIVERY_SHIPPING = 2;
    DIGITAL = 3;
}

// Purchase Order
message MarketplacePurchase {
    string order_id = 1;                      // UUIDv7
    string listing_id = 2;
    string buyer_user_id = 3;
    string seller_user_id = 4;
    
    int64 quantity = 5;
    repeated string selected_variants = 6;
    int64 total_price_owc = 7;
    string delivery_method = 8;
    string delivery_address = 9;
    string buyer_notes = 10;
    
    int64 ordered_at_utc = 11;
    int64 expected_delivery_utc = 12;
    string order_status = 13;                 // PENDING, CONFIRMED, SHIPPED, DELIVERED, DISPUTED
}

// Seller Review
message SellerReview {
    string review_id = 1;
    string listing_id = 2;
    string buyer_user_id = 3;
    string seller_user_id = 4;
    
    float rating = 5;                         // 1.0-5.0
    string review_text = 6;
    repeated string photos_ipfs = 7;
    
    int64 created_at_utc = 8;
}
```

**Rust Data Structures** (`crates/cs-marketplace/src/models.rs`):
- Listing struct with all fields
- Purchase enum for order lifecycle
- Review struct
- Category enum (extensible)

**Files:**
- ✅ `proto/marketplace.proto` (NEW)
- `crates/cs-marketplace/` (NEW crate)
- `crates/cs-marketplace/src/models.rs` (NEW)

---

### Week 19-20: Android Marketplace UI

**Create Listing Flow:**
- `feature-marketplace/feature-marketplace-sell/CreateListingActivity.kt`
  - Title, description input
  - Category + subcategory picker
  - Price input (in OWC + display currency)
  - Image upload (local camera or gallery)
  - Add variants dialog (size, color, delivery type)
  - Delivery method checkboxes
  - Operating hours (timezone-aware)
  - "Publish Listing" button

**Search & Browse:**
- `feature-marketplace/feature-marketplace-buy/SearchListingsFragment.kt`
  - Search bar (keyword)
  - Category filter dropdown
  - Price range slider
  - Distance filter (5km, 10km, 25km, 50km, any)
  - Sort by (relevance, price, distance, rating, sales)
  - Listing cards (photo, title, price, seller rating, distance)
  - Tap card → open detail page

**Listing Detail Page:**
- `feature-marketplace/feature-marketplace-buy/ListingDetailActivity.kt`
  - Full photos gallery (IPFS)
  - Title, description, full price breakdown
  - Seller card (name, rating, # reviews, credit score)
  - Variants selector (size → price delta)
  - Delivery method selector
  - Quantity spinner
  - "Order Now" button

**Place Order Flow:**
- Order confirmation dialog
- Select delivery address (auto-fill from user profile or manual)
- Add notes (optional)
- Review order summary
- "Confirm Purchase" → creates Purchase entry

**Order Tracking:**
- List of user's orders (active + completed)
- Per-order status (PENDING, CONFIRMED, SHIPPED, DELIVERED)
- Seller can update status (SHIPPED, DELIVERED)
- Buyer can mark as received + submit review

**Files:**
- `android/feature/feature-marketplace/` (NEW directory)
- `android/feature/feature-marketplace/feature-marketplace-sell/` (NEW)
  - `CreateListingActivity.kt`
  - `CreateListingViewModel.kt`
  - `ImageUploadManager.kt`
  - `VariantBuilder.kt`
- `android/feature/feature-marketplace/feature-marketplace-buy/` (NEW)
  - `SearchListingsFragment.kt`
  - `SearchViewModel.kt`
  - `ListingDetailActivity.kt`
  - `PlaceOrderFlow.kt`
  - `OrderTrackingFragment.kt`

---

### Week 21-22: Super-Peer Marketplace Backend

**Rust Backend** (`crates/cs-marketplace/`):

```rust
// Listing management
pub async fn store_listing(listing: &MarketplaceListing) -> Result<()>
pub async fn get_listing(listing_id: &str) -> Result<MarketplaceListing>
pub async fn update_listing(listing_id: &str, updates: ListingUpdate) -> Result<()>
pub async fn deactivate_listing(listing_id: &str) -> Result<()>

// Search & Discovery
pub async fn search_listings(
    query: &str,
    category: Option<&str>,
    latitude: f64,
    longitude: f64,
    max_distance_km: f64,
    max_price: Option<i64>,
    min_rating: Option<f32>,
) -> Result<Vec<ListingSearchResult>>

// Purchase Order Handling
pub async fn create_purchase_order(purchase: &MarketplacePurchase) -> Result<OrderId>
pub async fn get_order(order_id: &str) -> Result<MarketplacePurchase>
pub async fn update_order_status(order_id: &str, new_status: &str) -> Result<()>

// Review Management
pub async fn submit_review(review: &SellerReview) -> Result<ReviewId>
pub async fn get_seller_reviews(seller_user_id: &str) -> Result<Vec<SellerReview>>
pub async fn calculate_seller_rating(seller_user_id: &str) -> Result<f32>

// Dispute Resolution
pub async fn file_dispute(order_id: &str, reason: &str, evidence_ipfs: &str) -> Result<()>
pub async fn resolve_dispute(order_id: &str, decision: DisputeDecision) -> Result<()>
```

**PostgreSQL Schema:**

```sql
-- Marketplace listings (separate from ledger)
CREATE TABLE marketplace_listings (
    listing_id UUID PRIMARY KEY,
    seller_user_id UUID NOT NULL REFERENCES users(user_id),
    seller_public_key BYTEA NOT NULL,
    
    title VARCHAR(255) NOT NULL,
    description TEXT,
    category VARCHAR(50) NOT NULL,
    subcategory VARCHAR(50),
    
    base_price_owc BIGINT NOT NULL,
    currency_context VARCHAR(3),
    
    image_ipfs_hashes TEXT[],
    
    seller_latitude FLOAT,
    seller_longitude FLOAT,
    
    operating_hours_tz VARCHAR(50),
    operating_hours VARCHAR(100),
    
    delivery_methods INT[],
    
    created_at TIMESTAMP NOT NULL,
    last_updated_utc TIMESTAMP,
    is_active BOOLEAN DEFAULT true,
    
    -- Computed fields
    view_count INT DEFAULT 0,
    average_rating FLOAT,
    completed_sales INT DEFAULT 0,
    
    -- Full-text search
    search_vector tsvector GENERATED ALWAYS AS (
        to_tsvector('english', title || ' ' || description || ' ' || category)
    ) STORED
);

CREATE INDEX idx_listings_seller ON marketplace_listings(seller_user_id);
CREATE INDEX idx_listings_active ON marketplace_listings(is_active);
CREATE INDEX idx_listings_fts ON marketplace_listings USING GIN(search_vector);
CREATE INDEX idx_listings_geo ON marketplace_listings USING GIST(
    earth_distance(ll_to_earth(seller_latitude, seller_longitude), 
                   ll_to_earth(CAST(0 AS FLOAT), CAST(0 AS FLOAT)))
);

-- Purchase orders
CREATE TABLE marketplace_orders (
    order_id UUID PRIMARY KEY,
    listing_id UUID REFERENCES marketplace_listings(listing_id),
    buyer_user_id UUID REFERENCES users(user_id),
    seller_user_id UUID REFERENCES users(user_id),
    
    quantity BIGINT,
    selected_variants TEXT[],
    total_price_owc BIGINT,
    delivery_method INT,
    delivery_address TEXT,
    buyer_notes TEXT,
    
    ordered_at_utc TIMESTAMP,
    expected_delivery_utc TIMESTAMP,
    
    order_status VARCHAR(50),
    
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

CREATE INDEX idx_orders_buyer ON marketplace_orders(buyer_user_id);
CREATE INDEX idx_orders_seller ON marketplace_orders(seller_user_id);
CREATE INDEX idx_orders_status ON marketplace_orders(order_status);

-- Reviews
CREATE TABLE seller_reviews (
    review_id UUID PRIMARY KEY,
    order_id UUID REFERENCES marketplace_orders(order_id),
    listing_id UUID REFERENCES marketplace_listings(listing_id),
    buyer_user_id UUID REFERENCES users(user_id),
    seller_user_id UUID REFERENCES users(user_id),
    
    rating FLOAT NOT NULL,
    review_text TEXT,
    photos_ipfs TEXT[],
    
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE INDEX idx_reviews_seller ON seller_reviews(seller_user_id);
CREATE INDEX idx_reviews_order ON seller_reviews(order_id);

-- Disputes
CREATE TABLE marketplace_disputes (
    dispute_id UUID PRIMARY KEY,
    order_id UUID UNIQUE REFERENCES marketplace_orders(order_id),
    buyer_user_id UUID REFERENCES users(user_id),
    seller_user_id UUID REFERENCES users(user_id),
    
    dispute_reason TEXT NOT NULL,
    dispute_evidence_ipfs TEXT,
    
    dispute_status VARCHAR(50),
    resolution_decision TEXT,
    
    filed_at TIMESTAMP,
    resolved_at TIMESTAMP
);

CREATE INDEX idx_disputes_buyer ON marketplace_disputes(buyer_user_id);
CREATE INDEX idx_disputes_seller ON marketplace_disputes(seller_user_id);
```

**Files:**
- `crates/cs-marketplace/src/` (NEW crate)
  - `models.rs` (listing, purchase, review structs)
  - `storage.rs` (PostgreSQL queries)
  - `search.rs` (FTS + geo queries)
  - `purchase_handler.rs` (order lifecycle)
  - `reputation.rs` (seller rating calculation)
  - `dispute.rs` (conflict resolution)
  - `gossip.rs` (broadcast listings to peers)

---

### Week 23: Gossip Protocol for Listing Discovery

**How Listings Propagate:**

1. **Device creates listing** → signs → stores locally in Room DB
2. **Device gossips listing** via whisper network (similar to transaction gossip)
3. **Other devices receive listing** → store in local cache
4. **Search works offline** on cached listings
5. **Super-peer indexes all gossiped listings** → enables full-text search

**Implementation:**

```rust
// crates/cs-marketplace/src/gossip.rs

pub async fn broadcast_listing(
    listing: &MarketplaceListing,
    peer_addresses: &[PeerAddress]
) -> Result<()> {
    // Sign listing with seller's key
    let signature = sign_listing(listing)?;
    
    // Serialize
    let payload = serialize_listing(listing, &signature)?;
    
    // Gossip to nearby peers (similar to whisper protocol)
    for peer in peer_addresses {
        send_listing(peer, &payload).await?;
    }
}

pub async fn receive_listing(
    listing: &MarketplaceListing,
    signature: &[u8]
) -> Result<()> {
    // Verify signature
    verify_listing_signature(listing, signature)?;
    
    // Store locally
    db.insert_listing(listing).await?;
    
    // Re-gossip to other peers (with TTL to prevent loops)
    re_gossip_listing(listing).await?;
}
```

**Files:**
- `crates/cs-marketplace/src/gossip.rs` (NEW)
- Update `WHISPER_NETWORK_IMPLEMENTATION.md` with listing gossip

---

### Week 24: IPFS Integration for Images

**Image Upload Flow:**

1. User picks image from gallery → compress to ~500KB
2. Upload to IPFS via super-peer gateway (`/api/ipfs/upload`)
3. Receive IPFS hash (Qm...)
4. Include hash in listing

**Implementation:**

```rust
// Super-peer endpoint
#[post("/api/ipfs/upload")]
async fn upload_image(
    user_id: Uuid,
    file: UploadedFile
) -> Result<IpfsUploadResponse> {
    // Validate file size + format
    if file.size() > 5 * 1024 * 1024 {
        return Err("Image too large");
    }
    
    // Compress image
    let compressed = compress_image(&file)?;
    
    // Upload to IPFS
    let ipfs_hash = ipfs_client.add(compressed).await?;
    
    // Log for user (not included in listing; just returned)
    audit_log::record_image_upload(user_id, &ipfs_hash);
    
    Ok(IpfsUploadResponse {
        ipfs_hash,
        url: format!("https://ipfs.io/ipfs/{}", ipfs_hash)
    })
}
```

**Android Integration:**

```kotlin
// android/feature/feature-marketplace/feature-marketplace-sell/ImageUploadManager.kt

suspend fun uploadImage(uri: Uri): String {
    val file = File(context.cacheDir, "temp_image.jpg")
    
    // Copy and compress
    context.contentResolver.openInputStream(uri)?.use { input ->
        file.outputStream().use { output ->
            input.copyTo(output)
        }
    }
    
    // Upload to super-peer IPFS gateway
    val response = apiClient.postForm(
        "/api/ipfs/upload",
        mapOf("file" to file)
    )
    
    return response.ipfsHash  // Qm...
}
```

**Files:**
- `crates/cs-api/src/ipfs.rs` (NEW endpoint)
- `android/feature/feature-marketplace/feature-marketplace-sell/ImageUploadManager.kt` (NEW)

---

## Phase 6: Seller Reputation Integration (Weeks 25-28)

### Week 25-26: Integrate Reviews into Credit Scoring

**Credit Formula Update:**

```rust
// crates/cs-credit/src/scoring.rs

pub fn calculate_credit_score(user_id: &str) -> f32 {
    let days_active = (now() - user.created_at).days();
    let tx_count_30d = count_transactions_30d(user_id);
    let conflicts = count_conflicts_30d(user_id);
    let velocity_score = calculate_velocity(user_id);
    let geographic_score = calculate_geographic_stability(user_id);
    let device_reputation = get_device_reputation(user_id);
    let seller_rating = get_average_seller_rating(user_id);  // NEW
    
    let score = (
        (MIN(days_active / 90.0, 1.0)) * 20.0
        + (MIN(tx_count_30d / 20.0, 1.0)) * 20.0
        + (MAX(100.0 - (conflicts as f32 * 5.0), 0.0))
        + velocity_score * 15.0
        + geographic_score * 15.0
        + device_reputation * 10.0
        + seller_rating * 10.0  // 0-10 points from avg rating
    ) / 1.7;  // Normalize to 0-100
    
    MIN(score, 100.0)
}

fn get_average_seller_rating(user_id: &str) -> f32 {
    // Query all reviews for this seller
    let reviews = database.query_seller_reviews(user_id);
    
    if reviews.is_empty() {
        return 0.0;  // New sellers start at 0 bonus
    }
    
    let avg_rating = reviews.iter().map(|r| r.rating).sum::<f32>() / reviews.len() as f32;
    
    // Convert 1.0-5.0 rating to 0.0-10.0 points
    (avg_rating - 1.0) * 2.5  // Linear map: 1.0→0, 5.0→10
}
```

**Testing:**
- Seller with no reviews: score unaffected
- Seller with 5 reviews averaging 4.5 stars: +8.75 points to credit score
- Seller with 1-star review: -1.25 points

**Files:**
- Update `crates/cs-credit/src/scoring.rs`
- Update `NETWORK_AND_CREDIT_ARCHITECTURE.md` formula

### Week 27-28: Dispute Resolution System

**Dispute Workflow:**

```
Buyer receives poor product → Files dispute
    ↓
Super-peer freezes payment (escrow)
    ↓
Request seller response (48-hour window)
    ↓
Evaluate evidence from both sides
    ↓
Decision logic:
  - High buyer credit + low seller rating + photo evidence → REFUND BUYER
  - Low buyer credit + high seller history → DENY REFUND
  - Unclear → SPLIT PAYMENT or request arbiter
    ↓
Update both parties' reputation
```

**Implementation:**

```rust
// crates/cs-marketplace/src/dispute.rs

pub async fn resolve_marketplace_dispute(
    order_id: &str,
    evidence: &DisputeEvidence
) -> Result<DisputeDecision> {
    let order = db.get_order(order_id)?;
    let buyer = db.get_user(&order.buyer_user_id)?;
    let seller = db.get_user(&order.seller_user_id)?;
    
    // Scoring logic
    let buyer_credibility = buyer.credit_score;
    let seller_reliability = seller.credit_score;
    let seller_reviews = db.get_seller_reviews(&seller.user_id)?;
    let avg_seller_rating = calculate_average_rating(&seller_reviews);
    
    // Decision logic
    let decision = if buyer_credibility > 70.0 && avg_seller_rating < 3.5 {
        // Buyer has good history, seller has poor reviews
        DisputeDecision::RefundBuyer
    } else if buyer_credibility < 50.0 && seller_reliability > 75.0 {
        // Buyer has poor history, seller has excellent record
        DisputeDecision::DenyRefund
    } else if avg_seller_rating < 2.0 {
        // Seller has terrible reviews overall
        DisputeDecision::RefundBuyer
    } else {
        // Unclear → escalate
        DisputeDecision::RequiresArbiter
    };
    
    // Execute decision
    match decision {
        DisputeDecision::RefundBuyer => {
            // Refund: buyer -0 OWC, seller -X OWC (refund), buyer rep +5, seller rep -15
            transfer_funds(&seller.public_key, &buyer.public_key, order.total_price_owc)?;
            update_reputation(&buyer.user_id, +5)?;
            update_reputation(&seller.user_id, -15)?;
        },
        DisputeDecision::DenyRefund => {
            // Deny: buyer rep -10 (for frivolous dispute), seller rep +5
            update_reputation(&buyer.user_id, -10)?;
            update_reputation(&seller.user_id, +5)?;
        },
        DisputeDecision::RequiresArbiter => {
            // Escalate to human review (future)
        }
    }
    
    Ok(decision)
}
```

**Files:**
- `crates/cs-marketplace/src/dispute.rs` (NEW)

---

## Marketplace Revenue Model

| Revenue Stream | Year 1 | Year 2 | Year 3 |
|---|---|---|---|
| **Transaction Fees** | 1-2% per marketplace purchase | Scales with GMV | ~$50M+ estimated |
| **Seller Success Fees** | 2% on P2P lending (sellers as lenders) | Premium seller badge ($1/month) | Premium features |
| **Data Insights** | Anonymized commerce trends (CPG) | Real-time regional demand signals | $100K+/month |
| **Total Marketplace Revenue** | $150K-500K Y1 | $3M-5M Y2 | $50M+ Y3 |

---

## Integration with Existing Systems

### Transaction Flow (Purchase = Normal Entry)

```
User places marketplace order
    ↓
Creates MarketplacePurchase proto
    ↓
Wraps in JournalEntry (standard ledger entry)
    ↓
Signs with buyer's key
    ↓
Gossips to peers + submits to super-peer
    ↓
Super-peer validates + adds to mempool
    ↓
3-of-5 quorum votes → CONFIRMED
    ↓
Seller notified via push
    ↓
Marketplace backend updates order status
```

### Credit Score Impact

- Transaction count ↑ (if selling items)
- Seller rating ↑ (positive reviews)
- Velocity metrics ↑ (consistent selling activity)
- Geographic stability (consistent pickup location)
- Device reputation (used for marketplace transactions)

---

## Testing & QA

### Unit Tests
- [ ] Listing creation + signing
- [ ] Search query parsing (FTS)
- [ ] Geolocation distance calculation
- [ ] Review rating aggregation
- [ ] Dispute resolution logic

### Integration Tests
- [ ] End-to-end: create listing → buyer searches → purchases → leaves review → seller rating updates → credit score increases
- [ ] Offline: create listing while offline → gossip when online → appear in peers' searches
- [ ] Conflict: two buyers purchase same item simultaneously → first one confirmed, second marked CONFLICTED

### Manual Testing
- [ ] Create listing with 3+ photos (IPFS upload)
- [ ] Search with various filters (category, distance, price, rating)
- [ ] Place order with variants
- [ ] Track order status (SHIPPED, DELIVERED)
- [ ] Submit review + see rating update
- [ ] File dispute + see resolution

---

## Success Metrics

✅ **Adoption:**
- 10K active sellers by end of Year 1
- $1M monthly GMV by end of Year 2
- 50%+ of transactions are peer-to-peer marketplace (not just pure transfers)

✅ **Quality:**
- Average seller rating > 4.2/5
- <2% disputed orders
- <0.5% fraud rate

✅ **Economics:**
- $0.50-2.00 transaction fee per order
- Year 1 marketplace revenue: $500K-2M
- Year 3 marketplace revenue: $50M+

---

## Next Steps

1. **Week 17**: Finalize proto definitions with team
2. **Week 18**: Design PostgreSQL schema with DBA
3. **Week 19**: Kick off Android UI development (parallel with backend)
4. **Week 23**: Complete first search query for testing
5. **Week 24**: IPFS integration complete, test image upload
6. **Week 25**: Run end-to-end test: list item → buy → review → credit score updates

This marketplace is the final piece that transforms CylinderSeal from **payment system** into a **complete economic platform**.
