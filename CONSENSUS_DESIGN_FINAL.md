# Production Consensus Design: Tendermint BFT

## Executive Summary

**Replace custom 5-node quorum with Tendermint BFT from day 1.**

**Why:**
- ✅ Scales 5 → 50 → 200+ validators without architecture change
- ✅ Instant finality (no forks, no unconformed entries)
- ✅ Proven in production (Cosmos Hub, $billions in value)
- ✅ Byzantine-resilient from day 1
- ✅ Designed for financial networks (not just generic consensus)
- ✅ Active ecosystem (IBC, CosmWasm for Phase 2-3 features)

**Cost:**
- MVP latency: 1-2 second finality (acceptable for P2P payments)
- Operational: Run 3-5 super-peers in different regions
- Dev time: 4-6 weeks to integrate (not 16 weeks to build custom)

---

## Architecture: Tendermint for Super-Peer Consensus

### Three-Tier Model (Updated)

```
┌─────────────────────────────────────────────────────┐
│ TIER 0: Android Devices (Offline-First)             │
│   ├─ NFC/BLE P2P transactions (PENDING locally)     │
│   ├─ Whisper network relay (online peers forward)   │
│   └─ SQLite Room DB + SQLCipher                     │
└──────────────────┬──────────────────────────────────┘
                   │ gRPC/TLS 1.3
                   ↓
┌─────────────────────────────────────────────────────┐
│ TIER 1: Tendermint Blockchain (Consensus Layer)     │
│                                                      │
│   ┌─ Validator 1 (Nigeria)   ───┐                  │
│   ├─ Validator 2 (Kenya)      ───┼─ Tendermint BFT │
│   ├─ Validator 3 (South Africa)──┤ Consensus       │
│   ├─ Validator 4 (Germany)   ────┤ (Byzantine)     │
│   └─ Validator 5 (Singapore)  ───┘                  │
│                                                      │
│   ├─ CometBFT (consensus engine)                    │
│   ├─ PostgreSQL (ledger state)                      │
│   ├─ Redis (mempool, rate limits)                   │
│   └─ IBC (inter-chain communication)                │
│                                                      │
│   Block time: ~1 second                             │
│   Finality: Instant (no forks)                      │
│   Validators: 3-200+ (expandable)                   │
└──────────────────┬──────────────────────────────────┘
                   │
                   ↓
┌─────────────────────────────────────────────────────┐
│ TIER 2: Off-Chain Services (Monetization)           │
│   ├─ Credit data API (sells to MFIs)                │
│   ├─ OWC rate feeds (forex aggregation)             │
│   ├─ Fiat on-ramps (PayPal, Wise, M-Pesa)          │
│   └─ KYC provider callbacks                         │
└─────────────────────────────────────────────────────┘
```

---

## Why Tendermint (Not Custom Consensus)

### Design Principles

| Requirement | Custom 5-Node | Tendermint | Winner |
|---|---|---|---|
| **Scalability** | 5 nodes only | 3-200+ nodes | ✅ Tendermint |
| **Byzantine resilience** | 2-of-5 collusion = fail | 1/3 Byzantine max | ✅ Tendermint |
| **Finality** | No (entries can be unconformed) | Instant (one block) | ✅ Tendermint |
| **Latency** | ~100ms (LAN) | ~1-2s (WAN) | Custom better, but acceptable |
| **Complexity** | Low | High | ✅ Custom better |
| **Proven in production** | No | Yes ($billions staked) | ✅ Tendermint |
| **Validator governance** | Hardcoded | Dynamic (add/remove validators) | ✅ Tendermint |
| **Cross-chain support** | No | IBC protocol | ✅ Tendermint |
| **Development cost** | Custom code (risk) | Proven software (fast) | ✅ Tendermint |

### Attack Scenarios

#### Scenario 1: Collusion Attack

**Custom 5-node:**
```
Validators: S1, S2, S3, S4, S5

Attack: S1, S2, S3 collude (60% of nodes)
├─ They agree to approve fraudulent transaction
├─ Even if S4, S5 object, 3-of-5 quorum = confirmed
└─ Result: Fraud wins ❌
```

**Tendermint (5 validators):**
```
Validators: V1, V2, V3, V4, V5

Attack: V1, V2 collude (40% of nodes)
├─ V1 signs one block
├─ V2 signs different block
├─ V3, V4, V5 don't see agreement
├─ No 2/3 supermajority reached
└─ Result: No consensus, cannot proceed ✅
```

#### Scenario 2: Clock Skew Attack

**Custom 5-node with timestamp voting:**
```
Entry arrives at different times:
  S1: 1:15 PM (earliest)
  S2: 1:16 PM
  S3: 1:17 PM

If S1, S2, S3 coordinate:
├─ Set their clocks forward by 1 hour
├─ All vote on their "earliest" time
├─ They can approve conflicting entries
└─ Result: Clock manipulation wins ❌
```

**Tendermint (proposer-based):**
```
Tendermint uses:
  ├─ Proposer rotation (each validator takes turns)
  ├─ Block height (not time) for ordering
  ├─ Deterministic timestamps (median of validators)
  └─ Even if proposer lies about time, consensus still works ✅

No single node can manipulate block order.
```

---

## Implementation: Cosmos SDK

### Project Structure

```
cylinder_seal/
├── crates/
│   ├── cs-core/                 (Rust shared types)
│   ├── cs-storage/              (PostgreSQL repos)
│   └── cs-tendermint/           (NEW: Tendermint integration)
│
├── chain/                        (NEW: Cosmos SDK application)
│   ├── app/
│   │   ├── app.go               (Initialize chain)
│   │   └── encoding.go          (Proto encoding)
│   │
│   ├── x/ledger/                (NEW: Custom ledger module)
│   │   ├── keeper/
│   │   │   ├── entry.go         (Entry storage logic)
│   │   │   ├── relay.go         (Whisper network relay handler)
│   │   │   └── conflict.go      (Double-spend detection)
│   │   │
│   │   ├── types/
│   │   │   ├── messages.go      (MsgSubmitEntry, MsgRelayEntries)
│   │   │   └── events.go        (EntryConfirmed, EntryConflicted)
│   │   │
│   │   └── keeper_test.go       (Unit tests)
│   │
│   ├── x/credit/                (NEW: Credit scoring module)
│   │   ├── keeper/
│   │   │   └── score.go         (Compute credit scores)
│   │   └── types/
│   │       └── score.go         (Credit profile struct)
│   │
│   └── proto/
│       └── cylinder_seal/       (Proto definitions)
│
└── docker/
    └── docker-compose.yml       (4 Tendermint nodes + PostgreSQL)
```

### Core Ledger Module

```go
// chain/x/ledger/keeper/entry.go

package keeper

import (
	"crypto/sha256"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/modproto/cylinder_seal/proto/cylinder_seal"
)

type EntryKeeper struct {
	storeKey sdk.StoreKey
	db       *sql.DB  // PostgreSQL connection
}

// SubmitEntry processes a journal entry in a Tendermint block
func (k EntryKeeper) SubmitEntry(
	ctx sdk.Context,
	entry *cylinder_seal.JournalEntry,
) error {
	// 1. Validate entry signature
	if !k.verifySignature(entry) {
		return fmt.Errorf("invalid entry signature")
	}

	// 2. Check for double-spend
	lastSeq, err := k.getLastSequence(ctx, entry.UserPublicKey)
	if err != nil {
		return err
	}
	
	if entry.SequenceNumber != lastSeq+1 {
		return fmt.Errorf("sequence gap: expected %d, got %d", 
			lastSeq+1, entry.SequenceNumber)
	}

	// 3. Verify prev_entry_hash matches ledger state
	prevHash, err := k.getEntryHash(ctx, entry.UserPublicKey, lastSeq)
	if err != nil {
		return err
	}
	
	if !bytes.Equal(entry.PrevEntryHash, prevHash) {
		return fmt.Errorf("fork detected: prev_hash mismatch")
	}

	// 4. Compute new ledger hash (deterministic)
	// This becomes the new "canonical state" for this user
	newLedgerHash := sha256.Sum256(append(
		prevHash[:],
		entry.EntryHash...,
	))

	// 5. Store in PostgreSQL (with Tendermint block height)
	err = k.db.StoreEntry(entry, ctx.BlockHeight(), newLedgerHash[:])
	if err != nil {
		return err
	}

	// 6. Emit event for off-chain indexing
	sdk.NewEvent(
		"entry_confirmed",
		sdk.NewAttribute("user_id", hex.EncodeToString(entry.UserPublicKey)),
		sdk.NewAttribute("entry_hash", hex.EncodeToString(entry.EntryHash)),
		sdk.NewAttribute("sequence", fmt.Sprintf("%d", entry.SequenceNumber)),
		sdk.NewAttribute("block_height", fmt.Sprintf("%d", ctx.BlockHeight())),
		sdk.NewAttribute("ledger_hash", hex.EncodeToString(newLedgerHash[:])),
	)

	return nil
}

// RelayEntries handles whisper network relays
func (k EntryKeeper) RelayEntries(
	ctx sdk.Context,
	relay *cylinder_seal.EntryRelay,
) error {
	// 1. Verify relay device signature
	if !k.verifyRelaySignature(relay) {
		return fmt.Errorf("invalid relay signature")
	}

	// 2. Rate-limit relay device
	relayRate := k.getRelayRate(ctx, relay.RelayDevicePk)
	if relayRate > 10 {  // 10 relays per block
		return fmt.Errorf("relay rate limit exceeded")
	}

	// 3. Validate originating nonce (detect replays)
	lastNonce := k.getLastNonce(ctx, relay.OriginatingDevicePk)
	if bytes.Compare(relay.OriginatingNonce, lastNonce) <= 0 {
		return fmt.Errorf("stale nonce")
	}

	// 4. Process each entry in relay
	for _, entry := range relay.Entries {
		// Entry signatures still verified in SubmitEntry
		if err := k.SubmitEntry(ctx, entry); err != nil {
			// If any entry fails, entire relay fails
			// (atomic: all or nothing)
			return err
		}
	}

	// 5. Update relay device reputation
	k.penalizeRelayDevice(ctx, relay.RelayDevicePk, 0)  // No conflicts

	return nil
}

// Tendermint finality guarantee:
// Once SubmitEntry is called in a block and the block is committed,
// the entry is PERMANENTLY CONFIRMED (can never be rolled back).
```

### Consensus Flow

```
Device A (offline) → Device B (online) → Tendermint Validator V1

Step 1: Device A creates transaction
├─ Offline, stored in SQLite as PENDING
└─ No internet needed

Step 2: Device B comes online, receives relay request from Device A
├─ Collects pending entries from A
├─ Creates EntryRelay message
└─ Sends to V1 via gRPC

Step 3: V1 receives relay, adds to mempool
├─ Validates signatures
├─ Checks for double-spends
├─ Broadcasts to other validators (V2, V3, V4, V5)

Step 4: Tendermint consensus (BFT rounds)
├─ Proposer V1 builds block with entries
├─ V1 broadcasts block proposal
├─ V2, V3, V4 validate and vote PREVOTE
├─ If 2/3 prevote: broadcast PRECOMMIT
├─ If 2/3 precommit: block is COMMITTED
└─ Time: ~1 second end-to-end

Step 5: Block committed = INSTANT FINALITY
├─ Entry marked CONFIRMED in PostgreSQL
├─ Event emitted: "entry_confirmed"
├─ Tendermint stores block in RocksDB
└─ Can NEVER be rolled back (no forks possible)

Step 6: Device A syncs with V1
├─ Queries "get_entry_status" RPC
├─ Response: "CONFIRMED at block_height 12345"
└─ SQLite updated: CONFIRMED status
```

---

## Migration from Custom Consensus (If Starting Over)

### Week 1-2: Setup Tendermint Testnet
```
├─ Install Cosmos SDK + CometBFT
├─ Create 4-node local testnet (Docker)
├─ Port cs-core types to Go
└─ Create ledger module scaffold
```

### Week 3-4: Implement Ledger Module
```
├─ SubmitEntry keeper logic
├─ RelayEntries keeper logic
├─ Double-spend detection
└─ Unit tests for consensus
```

### Week 5-6: Integration & Testing
```
├─ Android gRPC client → Tendermint validator
├─ E2E test: Device A → Device B → Validator → Confirmed
├─ Load testing (1000 TPS target)
└─ Byzantine failure testing (kill validators, ensure safety)
```

---

## Security Properties

### Liveness (can always confirm entries)
```
Assumption: >2/3 validators are online and honest

Guarantee: Entry submitted to mempool will be included in some block.
           Confirmed entry cannot be un-confirmed.

Why: Tendermint can fork only if <1/3 validators are Byzantine.
     If they try to fork, the fork network has <2/3 validators = halts.
     Original network (>2/3) continues.
```

### Safety (entries cannot be unconformed)
```
Assumption: <1/3 validators are Byzantine

Guarantee: Once a block is committed, it stays committed forever.
           Device can trust "CONFIRMED" status immediately.

Why: Any fork that tries to remove a committed block would need
     >1/3 Byzantine validators to create the fork, and then convince
     >2/3 of original validators to switch. Mathematically impossible.
```

### Byzantine Resilience
```
With 5 validators: Can tolerate 1 Byzantine (2 colluding = 40%, need 66%)
With 7 validators: Can tolerate 2 Byzantine
With 21 validators: Can tolerate 7 Byzantine

Compare to custom 5-node: Can tolerate 0 Byzantine (3 colluding = 60% > 50%)
```

---

## Operational Runbook

### Day 1: Launch Testnet with 3 Validators
```bash
# Genesis validators (fully controlled by CylinderSeal team)
├─ Validator 1: AWS us-east (USA)
├─ Validator 2: DigitalOcean eu-central (Europe)
└─ Validator 3: AWS ap-south (Asia)

# 2-of-3 quorum needed (can tolerate 1 validator down)
# All 3 validators must be honest (MVP: trusted operators)
```

### Phase 2: Add Community Validators
```
Validator admission process:
├─ Applicant: MFI, NGO, telco, or CylinderSeal partner
├─ Governance: CylinderSeal foundation proposes validator swap
├─ Voting: Existing validators vote via on-chain proposal
├─ New validator stakes security deposit (slashable)
└─ Added: New validator joins consensus

Growth path: 3 → 5 → 7 → 21 validators over 24 months
```

### Slashing Rules (Penalize Byzantine Behavior)
```
1. Double-signing (propose 2 different blocks at same height)
   ├─ Penalty: 5% of stake slashed
   └─ Jail period: 1 week (cannot propose blocks)

2. Downtime (offline for >100 blocks = ~3 minutes)
   ├─ Penalty: 0.1% of stake slashed
   └─ Jail period: 10 minutes

3. Submitting conflicted entry (relayed by own validator)
   ├─ Penalty: 1% of stake slashed
   └─ Jail period: 1 day
```

---

## Cost Comparison

### Custom 5-Node Implementation
```
Development:  16 weeks (high risk of bugs)
Ops cost:     $5K/month (5 servers + monitoring)
Scalability:  1-time architectural change at >50 nodes
Security:     Never proven, audits required ($50K+)
Total cost:   $200K+ for MVP + scaling later
```

### Tendermint Implementation
```
Development:  4-6 weeks (proven codebase, low risk)
Ops cost:     $3K/month (3-5 Tendermint nodes cheaper to operate)
Scalability:  Add validators on-the-fly, no architecture change
Security:     $millions staked on Cosmos chain, audited by experts
Total cost:   $50K for MVP + cheaper ongoing
```

**ROI: Tendermint pays for itself in cost savings alone.**

---

## Roadmap

### Phase 1 (Months 1-4): MVP with 3-Validator Tendermint
```
✅ CometBFT consensus (instant finality)
✅ Whisper network relay support
✅ PostgreSQL ledger (append-only)
✅ Android sync via gRPC
✅ Credit scoring (on-chain)
└─ Testnet only (trusted operators)
```

### Phase 2 (Months 5-9): Public Testnet, Add Validators
```
✅ Expand to 5 validators
✅ Governance system (add/remove validators via voting)
✅ Slashing rules (penalize Byzantine behavior)
✅ Mainnet launch with 5 validators
└─ Partners invited to run validators
```

### Phase 3 (Months 10-18): Scaling & IBC
```
✅ Expand to 7-21 validators
✅ Inter-blockchain communication (IBC)
✅ Bridge to Cosmos ecosystem
✅ Cosmwasm smart contracts (lending, derivatives)
└─ Open ecosystem
```

---

## Decision: Tendermint BFT from Day 1

**This is the right choice if:**
- ✅ You need secure + scalable from day 1
- ✅ You want institutional credibility (proven consensus)
- ✅ You plan to become a blockchain (IBC, interop)
- ✅ You want to scale beyond 5 nodes
- ✅ You want instant finality (no forks)

**Stick with custom consensus only if:**
- ❌ You can afford 16-week dev cycle
- ❌ You accept single-region only (5 nodes = not enough for geographic spread)
- ❌ You're okay with clock skew vulnerabilities
- ❌ You don't need validator governance

**Recommendation: Use Tendermint. Build CylinderSeal as a Cosmos-SDK application from day 1.**
