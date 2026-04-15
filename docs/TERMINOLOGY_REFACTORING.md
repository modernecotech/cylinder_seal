# Terminology Refactoring: Away from "Blockchain"

## Why We Changed

CylinderSeal was using terminology ("chainblock", "LedgerBlock", etc.) that invited false associations with blockchain systems. The actual architecture is fundamentally different:

| Property | Blockchain | CylinderSeal |
|----------|-----------|---|
| **Model** | Distributed consensus ledger (P2P) | Device-local journal + centralized super-peer validation |
| **Authority** | All nodes are peers | Super-peers are the source of truth |
| **Immutability** | Cryptographic (50K+ node agreement) | Database-enforced (append-only) |
| **Transaction finality** | Hours (settlement) | Seconds (super-peer confirms) |
| **Purpose** | Replace banks | Supplement banks for offline-first payments |
| **Decentralization** | Core feature | Not required (5 super-peers is enough) |

Using "blockchain" language was misleading and damaged credibility with financial institutions. We now use clearer, more accurate terminology.

---

## Mapping: Old → New

### Core Types

| Old Name | New Name | Context |
|----------|----------|---------|
| `LedgerBlock` | `JournalEntry` | A batch of transactions in a user's personal journal |
| `BlockConfirmationGossip` | `EntryConfirmationGossip` | Gossip message between super-peers |
| `SuperPeerSignature` | `SuperPeerConfirmation` | Confirmation signature from super-peer |

### Terminology

| Old | New | Why |
|-----|-----|-----|
| "chainblock" | "personal journal" or "device ledger" | Emphasizes single-user ownership, not distributed consensus |
| "block hash" | "entry hash" | Clearer what's being hashed |
| "block creation" | "entry creation" | Emphasizes batching transactions, not chain extension |
| "replicate ledger" | "replicate journal" | Journal is personal; ledger is shared |
| "ledger confirmation" | "entry confirmation" | Super-peer confirms one entry, not the whole ledger |
| "blockchain" | "append-only log" or "transaction journal" | No chain-of-custody or distributed consensus |

### Code Fields

| Old Field | New Field | Updated In |
|-----------|-----------|-----------|
| `block_id` | `entry_id` | JournalEntry struct, proto |
| `prev_block_hash` | `prev_entry_hash` | JournalEntry struct, proto |
| `block_hash` | `entry_hash` | JournalEntry struct, proto |
| `super_peer_confirmations` | `super_peer_confirmations` | (unchanged - already clear) |

### Method Names

| Old | New | Class |
|-----|-----|-------|
| `compute_block_hash()` | `compute_entry_hash()` | JournalEntry |
| `sign_with_device_key(self, key)` | (unchanged) | JournalEntry |
| `canonical_cbor_for_hashing()` | (unchanged) | JournalEntry |

---

## Files Updated

### Code (✅ Complete)
- ✅ `crates/cs-core/src/models.rs` — Type definitions, methods
- ✅ `crates/cs-core/src/error.rs` — Error message wording
- ✅ `proto/chain_sync.proto` — Proto message definitions and RPC signatures

### Documentation (✅ Complete)
- ✅ `WEEK1_STATUS.md` — Architecture overview
- ✅ `ANDROID_WEEK2_BRIDGE.md` — Android integration guide

### Documentation (🔄 To Be Updated)
The following large security documents contain code examples that reference the old types:
- `docs/IRON_SECURITY.md` — 12 hardening layers (contains ~10 code examples)
- `docs/SECURITY_VALIDATION.md` — 4 defense layers (contains ~8 code examples)
- `docs/TESTING_STRATEGY.md` — Test examples (contains ~20 test code blocks)

**Action**: When next updating these files for Week 2+, replace:
- `LedgerBlock` → `JournalEntry`
- `block_` → `entry_`
- Code comments referencing "blocks" → "entries"

### Documentation (✅ No Changes Needed)
- `SECURITY_SUMMARY.md` — Executive summary (uses generic language)
- `IMPLEMENTATION_ROADMAP.md` — Milestone tracking (uses generic language)
- `README.md` — High-level overview (would need review but not critical)

---

## Backward Compatibility

**Breaking Change**: Yes. Any code that imports `LedgerBlock` or uses `block_hash`, etc. will not compile.

**Migration Path for Android Code**:
```kotlin
// Old code (won't compile)
val block: LedgerBlock = ...
val hash = block.block_hash

// New code
val entry: JournalEntry = ...
val hash = entry.entry_hash
```

**Migration Path for Rust Code**:
```rust
// Old code (won't compile)
let block = LedgerBlock::genesis(pub_key);
block.compute_block_hash()?;

// New code
let entry = JournalEntry::genesis(pub_key);
entry.compute_entry_hash()?;
```

**Proto Compatibility**: The proto messages changed, so gRPC bindings must be regenerated.

---

## Impact on External Communications

### For Investors/Partners
- **Old pitch**: "Secure blockchain-based payments for the developing world"
- **New pitch**: "Secure offline-first payment system with super-peer validation for the developing world"

### For Regulators
- **Better compliance**: No "blockchain" claims means easier regulatory alignment
- **Clearer architecture**: "Personal ledgers + centralized validators" is easier to audit than "blockchain"

### For Developers
- **Clearer code**: `JournalEntry` and `entry_hash` are self-documenting
- **No baggage**: "Journal entry" doesn't carry the full history of blockchain failures/hacks

---

## Terminology Philosophy

Going forward:

✅ **Use**:
- "Personal ledger" — device-local transaction history
- "Journal entry" — one batch in the journal
- "Super-peer" — centralized validator (not a "node")
- "Append-only log" — the immutability guarantee
- "Confirmation" — super-peer validation
- "Offline-first" — the key feature

❌ **Avoid**:
- "Blockchain" — implies distributed consensus
- "Node" — implies peer-to-peer
- "Ledger" (alone) — ambiguous if device or super-peer
- "Decentralized" — we're not
- "Consensus protocol" — our 5-peer BFT is centralized quorum voting
- "Peer-to-peer" (for super-peers) — they're centralized infrastructure

---

## Questions?

**Q: Is this a permanent name change?**  
A: Yes. These names better reflect the architecture and won't be changed again.

**Q: Do we need to update all documentation today?**  
A: No. Large security documents can be updated incrementally. The code is already correct.

**Q: Will this confuse users?**  
A: No. Users don't interact with these types; they see "transaction history" and "confirmation".

**Q: Does this affect the security model?**  
A: No. Functionality is unchanged; only naming clarity improved.

---

**Refactoring Date**: 2026-04-15  
**Status**: Code updated, documentation in progress  
**Next**: Update security docs when next modifying them (Weeks 2+)
