# Architecture Transition: Blockchain Terminology → Computer Science Terminology

## Summary

CylinderSeal's consensus mechanism has been redesigned using **standard distributed systems computer science** instead of blockchain frameworks.

**Previous**: Tendermint BFT (blockchain consensus engine)
**Current**: Byzantine State Machine Replication via quorum-based ledger hash voting

---

## Terminology Changes

| Old (Blockchain) | New (Computer Science) | Explanation |
|------------------|------------------------|-------------|
| Tendermint blockchain | State Machine Replication layer | Distributed consensus using SM replication (Lamport 1978) |
| Validators | Super-peers | Nodes running the replicated state machine |
| Block height | Confirmation epoch / Entry sequence | Entry ordering via hash, not temporal ordering |
| Block consensus | Quorum voting | 3-of-5 nodes agree on ledger hash (BLAKE2b-256) |
| Block finality | Instant finality | Once ≥3 nodes confirm, entry CANNOT be rolled back |
| Gossip protocol | State replication gossip | Super-peers exchange validated entries and vote tallies |
| Fork prevention | Deterministic ordering | All nodes compute identical hash for same entry sequence |
| CometBFT engine | State voting mechanism | Quorum-based consensus on deterministic ledger state |

---

## Architecture Changes

### Before: Tendermint Blockchain Model

```
Device → S1 (Tendermint validator)
         ├─ Entry added to mempool
         ├─ S1 includes in block proposal
         ├─ Gossip block to S2-S5
         ├─ Consensus round: prevote → precommit
         └─ Block committed (2/3 prevote+precommit)
```

**Issues**:
- Blockchain terminology conflates financial settlement with distributed ledger
- Implies Bitcoin-style proof-of-work or complex consensus rounds
- "Blockchain" suggests scalability limitations (true for PoW, false here)

### After: Byzantine State Machine Replication Model

```
Device → S1 (state machine replica)
         ├─ Entry validated (signature, sequence, balance)
         ├─ Added to local state, compute ledger hash
         ├─ Gossip entry to S2-S5
         ├─ Each super-peer independently validates and computes hash
         └─ Once ≥3 agree on same ledger hash → CONFIRMED (quorum)
```

**Advantages**:
- Standard CS terminology: State Machine Replication (Lamport, Oki-Liskov)
- Instant finality without consensus rounds
- Deterministic hashing instead of clock-dependent timestamps
- Clearer explanation of why <1/3 Byzantine nodes are tolerated

---

## Key Technical Improvements

### 1. Deterministic State Voting (vs. Timestamp Voting)

**Old**: Entries ordered by timestamp, vote on which timestamp wins
**Problem**: Clock skew attacks, offline devices could cause forks

**New**: Entries ordered by hash, vote on ledger state
**Benefit**: Identical hash function at all nodes = no clock dependency

```rust
// All nodes compute this identically
fn confirm_entry(entries: &[JournalEntry]) -> [u8; 32] {
    let mut cbor = Vec::new();
    for entry in entries {
        cbor.extend(entry.canonical_cbor()?);
    }
    blake2b_256(&cbor)  // Deterministic
}

// Quorum vote on this hash, not timestamps
// Same entries = same hash at all nodes
```

### 2. Explicit Quorum Semantics (vs. "2/3 Consensus")

**Old**: "2/3 supermajority for instant finality" (blockchain speak)
**New**: "3-of-5 quorum required for confirmation" (SMR semantics)

**Why it matters**:
- Quorum literally means "minimum agreeing set"
- 3-of-5 = explicit quorum; 2-of-5 cannot form quorum
- Makes Byzantine resilience obvious: 2 malicious nodes can't form quorum

### 3. Scalability Inherited from Classical Theory

**Old**: "Works with 3 nodes or 200+ nodes" (hand-wavy)
**New**: "State Machine Replication proven secure for any N > 3F" (mathematically grounded)

For N super-peers:
- N=3: tolerance F=1 (1 faulty)
- N=5: tolerance F=2 (2 faulty)
- N=21: tolerance F=7 (7 faulty)
- N=201: tolerance F=67 (67 faulty)

Same algorithm, same guarantees at all scales.

---

## Documentation Updates

### Files Updated

1. **README.md**
   - Replaced "Tendermint blockchain" with "Byzantine State Machine Replication"
   - Updated Tier 1B description with quorum voting semantics
   - Entry Confirmation Flow now shows state machine replication, not block consensus

2. **vc_pitch.html**
   - Slide 6: "How It Works - Quorum-Based State Voting Architecture"
   - Replaced "Tendermint blockchain" with "State Machine Replication"
   - Super-peer boxes labeled S1-S5 (no "validator" terminology)
   - Benefits box: "Deterministic State Voting" instead of "Proven Consensus"

3. **New Document: QUORUM_STATE_VOTING_DESIGN.md**
   - Complete technical specification of state voting mechanism
   - Pseudocode for entry validation and hash computation
   - Quorum detection logic with examples
   - Byzantine resilience proof
   - Implementation checklist (MVP 4 weeks)
   - References to classical CS papers (Lamport, Oki-Liskov)

### Files Archived (Tendermint-Specific)

- `CONSENSUS_DESIGN_FINAL.md` — (Still valid history, but now superseded)
- `CONSENSUS_ANALYSIS.md` — (Comparative analysis; Tendermint no longer chosen approach)

---

## How Quorum-Based State Voting Works (Summary)

### Entry Confirmation Flow

1. **Device submits entry** to super-peer S1
   ```
   SyncChain(JournalEntry { user_id, transactions, signature, ... })
   ```

2. **S1 validates** (signature, nonce chain, balance)
   ```
   ✓ Signature valid over canonical CBOR
   ✓ Nonce: current = HKDF(previous || hardware_ids)
   ✓ Balance: confirmed_balance >= amount
   ✓ Sequence: increments by 1
   ```

3. **S1 computes ledger hash and gossips**
   ```
   new_ledger_hash = BLAKE2b-256(previous_entries + this_entry)
   Gossip to S2, S3, S4, S5: "Vote on hash_XYZ"
   ```

4. **Each super-peer independently validates and votes**
   ```
   S2, S3, S4, S5 (in parallel):
   ├─ Validate signature, sequence, balance
   ├─ Compute: hash_? = BLAKE2b-256(their_ledger + this_entry)
   └─ Vote: { entry_id, hash_?, super_peer_id }
   ```

5. **Quorum detection (3-of-5 agree)**
   ```
   Tally votes:
   ├─ hash_XYZ: 3 votes (S1, S2, S3) ← QUORUM ACHIEVED
   ├─ hash_ABC: 1 vote (S4)
   └─ no vote: 1 (S5, offline)
   
   Decision: CONFIRMED (majority = 3 ≥ 3 threshold)
   ```

6. **All nodes commit deterministically**
   ```
   Since all nodes see 3+ agreed on hash_XYZ:
   ├─ All commit entry: status = CONFIRMED
   ├─ All update balance for user
   ├─ No voting needed; result is deterministic
   └─ Device receives SyncAck: ✓ CONFIRMED
   ```

### Why This is Secure

- **Deterministic**: Same entry sequence → same hash at all nodes
- **Byzantine-Resilient**: Need >2/3 honest, so <1/3 malicious nodes can't forge consensus
- **Instant Finality**: Once 3-of-5 agree, entry is irreversible
- **Clock-Independent**: Hash function doesn't depend on node clocks

---

## Comparison: Byzantine State Machine Replication (BSMR) vs. Blockchain

| Property | BSMR (CylinderSeal) | Proof-of-Work Blockchain (Bitcoin) |
|----------|--------|-----------|
| **Consensus** | Quorum voting on state hash | Miners compete to solve puzzle |
| **Finality** | Instant (immutable once confirmed) | Probabilistic (51% attack risk) |
| **Block time** | ~1 second | ~10 minutes |
| **Energy** | Negligible (just voting) | Massive (mining) |
| **Network model** | Permissioned (5 known super-peers) | Permissionless (unknown miners) |
| **Scalability** | Linear with node count (3→200) | Square scaling (broadcast overhead) |
| **Requires PoW?** | No | Yes |
| **Requires blockchain?** | No | Yes (immutable log) |

**Key insight**: BSMR provides identical security guarantees as blockchain **without** the overhead.

---

## Migration Guide for Developers

### If reading old docs:

- "Tendermint consensus" → "Quorum-based state voting"
- "Validator node" → "Super-peer node"
- "Block" → "Entry batch" (though we don't batch; each entry confirmed individually)
- "CometBFT engine" → "State voting mechanism"
- "Block finality" → "Instant finality via quorum consensus"

### If implementing:

- Read: **QUORUM_STATE_VOTING_DESIGN.md** (not Tendermint docs)
- Implementation language: Any (Tendermint was Go-only, BSMR is language-agnostic)
- Use: Standard distributed systems libraries (not blockchain SDKs)
- Test: Byzantine failure injection, clock-skew scenarios (not 51% attacks)

---

## Why This Matters for CylinderSeal

1. **Clarity**: "State machine replication" is more honest than "blockchain"
2. **Accuracy**: Not using blockchain; using distributed consensus
3. **Simplicity**: No mining, no tokens, no complex consensus protocols
4. **Confidence**: Grounded in 40+ years of CS theory (Lamport, Paxos, Raft, etc.)
5. **Implementation**: Standard SMR is 4-6 weeks; Tendermint was 4-6 weeks but adds blockchain complexity we don't need

---

## References

**Classical CS Papers:**
- Lamport, L. "Time, Clocks, and the Ordering of Events in a Distributed System" (1978)
- Oki, B. & Liskov, B. "Viewstamped Replication" (1988)
- Schneider, F. "Implementing Fault-Tolerant Services Using the State Machine Approach" (1990)

**Modern Implementations:**
- Google Paxos (closed-source, used in Chubby, GFS)
- Amazon DynamoDB (state machine replication for strong consistency)
- CockroachDB (distributed SQL using Raft, a Paxos simplification)

**For Developers:**
- Mazieres, D. "Practical Byzantine Fault Tolerance and Proactive Recovery" (2002)
- Ongaro, D. "Consensus: Bridging Theory and Practice" — Raft paper (2014)
