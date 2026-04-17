# cs-tests — workspace-wide validation

Two things live here:

1. **Spec checks** — one integration-test file per README promise. Each
   test asserts a specific claim in the project specification. Failure
   messages are phrased as `"Spec violation: …"` so mismatches between
   implementation and documentation are easy to find.
2. **End-to-end harnesses** — hermetic, in-process simulations of the
   full payment and invoice flows. No Docker required.

## Run the full suite

```bash
cargo test -p cs-tests
```

Individual tests:

```bash
cargo test -p cs-tests --test spec_01_crypto_primitives
cargo test -p cs-tests --test e2e_offline_payment
```

## File index

| File                               | README section validated                           |
|------------------------------------|----------------------------------------------------|
| `spec_01_crypto_primitives.rs`     | Technical Implementation — crypto primitives       |
| `spec_02_canonical_signing.rs`     | Security Model — Ed25519 over canonical CBOR       |
| `spec_03_nonce_chain.rs`           | Security Model — RFC 6979 hardware-bound nonces    |
| `spec_04_journal_chain.rs`         | Architecture — per-user chained journals           |
| `spec_05_raft_consensus.rs`        | Architecture Decisions — 3-of-5 Raft (CFT)         |
| `spec_06_merchant_tiers.rs`        | Key Features #3 — trade policy without tariffs    |
| `spec_07_aml_flagging.rs`          | Monetary Policy Framework — AML/CFT monitoring     |
| `spec_08_credit_scoring.rs`        | Key Features #4 — supply chain financing           |
| `spec_09_account_types.rs`         | Account Types — Individual / POS / Electronic      |
| `spec_10_api_key_auth.rs`          | Account Types — hash-only API-key storage          |
| `spec_11_invoice_lifecycle.rs`     | Account Types — invoice + CS1:INV: URI             |
| `spec_12_wire_formats.rs`          | Architecture Decisions — NFC → BLE → QR fallback   |
| `spec_13_conflict_resolution.rs`   | Security Model — timestamp + NFC receipt tiebreaker|
| `e2e_offline_payment.rs`           | Full flow: sign → CBOR → wire → decode → verify    |
| `e2e_invoice_flow.rs`              | Full flow: register → issue key → pay → webhook    |

## Shared fixtures

`src/fixtures.rs` exposes:

- `seeded_keypair(seed)` — generate an Ed25519 keypair
- `signed_tx(kp, to_pk, amount)` — pre-signed Transaction fixture
- `signed_entry(kp, seq, prev_hash, txs)` — pre-signed JournalEntry
- `individual_user(pk, name)` / `business_user(pk, name, kind)`
- `tagged_nonce(tag)` — 32-byte nonce whose first byte is `tag`

## Unit tests in the other crates

The spec checks cover end-to-end guarantees. Per-module unit tests live
inside each crate's `#[cfg(test)]` block:

| Crate              | Notable unit-test files                       |
|--------------------|-----------------------------------------------|
| `cs-core`          | `cryptography.rs`, `models.rs`, `nonce.rs`, `hardware_binding.rs` |
| `cs-consensus`     | `log.rs`, `node.rs`                          |
| `cs-policy`        | `aml.rs`, `merchant_tier.rs`                 |
| `cs-credit`        | `scorer.rs`                                  |
| `cs-mobile-core`   | `lib.rs`, `wire.rs`                          |
| `cs-api`           | `middleware.rs` (token parsing)              |

Run the whole lot:

```bash
cargo test --workspace
```

## Mobile harnesses

- **Android**: `./gradlew test` (unit) and `./gradlew connectedAndroidTest`
  (instrumented, requires device/emulator).
- **iOS**: Open the XcodeGen-produced project and run the `CylinderSealTests`
  scheme, or `xcodebuild test -scheme CylinderSealTests`.

## Adding a spec check

When you change the README's technical claims, add (or update) the
matching `spec_*.rs` file. The naming convention is
`spec_NN_<topic>.rs` where NN is incremented monotonically. Each test
should:

- reference the exact README section in its module doc comment,
- assert the promise as explicitly as possible,
- phrase failures as `"Spec violation: …"` so they trace back to the doc.
