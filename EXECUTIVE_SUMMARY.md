# Cylinder Seal: a Digital Iraqi Dinar design for expert review

**Contact:** Hayder Aziz, Modern EcoTech — hayder@modernecotech.com
**Repository:** internal / provided on request
**Status:** working Rust reference implementation with wire-format primitives and super-peer enforcement live; not production; not deployed.

---

## What it is

A sovereign Digital Iraqi Dinar (Digital IQD) design — offline-first P2P payments over NFC/BLE, 3-of-5 Raft consensus across CBI regional branches (Baghdad, Basra, Erbil, Mosul, Najaf), and a programmable wire format that lets CBI enforce monetary and trade policy at the validation layer rather than after the fact.

It is deliberately **not** a blockchain. No token, no external consensus, no cryptocurrency bridge. CBI is the sole monetary authority; every entry is co-signed by the 3-of-5 super-peer quorum and auditable end-to-end.

## The problem it is designed against

Four Iraqi economic pathologies that each have substantial empirical literature and that reinforce each other:

1. **Invisible informal economy** — ~70% unbanked, 8–12M informal workers, cash-only transaction trails. (Relevant: AFI 2025 alt-credit for informal workers; Lee/Yang/Anderson 2026 Peru retail-data credit study.)
2. **SME credit bottleneck** — collateral-based lending excludes the thin-file majority; $50–100B of unmet working capital. (Relevant: McKinsey estimate of +$3.7T EM GDP from expanded credit via alt data.)
3. **USD leakage to finished-goods imports** — oil revenues (USD) flow through CBI to FX windows that fund finished-goods imports rather than the 1,200-project domestic industrial portfolio. The current 14:1 import-to-domestic production ratio is the headline symptom. (Relevant: IMF FTN 2024/007 on CBDC-enabled FX programmability; IMF WP 2024/086 on pitfalls of pure import-substitution.)
4. **Dollarization and weak monetary transmission** — citizens hold USD for savings because IQD has no trusted long-duration domestic home; CBI's transmission channels are indirect and slow. (Relevant: IMF WP 2025/211 on CBDC for monetary-policy delivery.)

The design attacks all four simultaneously via primitives on a single architecture. Each primitive is documented, implemented, and tested. Most EMDE CBDC pilots (eNaira, JAM-DEX) failed on at least one of these four and didn't have a single-architecture answer to the others.

## Design distinctives worth expert review

- **Merchant tier system as programmable trade policy** (0–8% fee schedule tied to verified domestic-content percentage; hard restrictions on government transfers for categories where domestic capacity exists). Trade policy without tariffs.
- **Individual Producer (IP) track** for the informal economy — 60-second in-app registration, presumptive 1–1.5% micro-tax, IQD 7M/month cap before formal-registration graduation. Designed to formalize 8–12M informal workers without crushing their livelihoods.
- **Transaction-based credit scoring with cash-flow features** (income periodicity, cash-flow stability, income/expense ratio). Blended 70/30 with the aggregate five-factor score; matches the research consensus for thin-file underwriting (FICO UltraFICO 2026, Experian Credit+Cashflow 2025, AFI 2025).
- **Wire-format programmability primitives** — expiring transfers (stimulus velocity), earmarked spend (construction-loan tranches that cannot leak to imported materials), conditional-release escrow (government forward-purchase commitments assignable as bank collateral, staged construction loans with Ministry inspector counter-signature). All three enforced by the CBI super-peer at validation time, not by application code.
- **Real estate as the integrative sector** — the only domain that activates all four pathologies at once. Residential construction formalizes construction labour, unlocks the mortgage market, drives domestic material demand through the tier system, and gives Iraqi households their first IQD-denominated long-duration asset. The 2.5–3M unit deficit (Shafaq News / Al-Bayan 2025) makes this the highest-leverage pilot sector.
- **Diaspora as distribution channel, not as capital source** — the Diaspora Merchant Node / Tourism Aggregator design lets diaspora-operated businesses abroad sell Iraqi-origin goods and religious-tourism packages with foreign currency captured into the CBI industrial pool at point of sale. Grounded in Rauch & Trindade 2002 (+30–60% bilateral trade uplift in differentiated goods via diaspora networks).

## Implementation status (honest)

The Rust workspace compiles. The cryptography, 3-of-5 Raft consensus, canonical-CBOR signing, offline nonce chain, merchant tier system, IP track, hard-restrictions gate, wire-format primitives, and tier-transaction audit log are all **implemented and tested** — 252 lib-tests plus 27 integration spec binaries pass. The CBI super-peer validation pipeline cryptographically enforces the tier rules and primitives at ingest time; the Raft state machine writes the audit trail post-commit.

**Not production.** Inter-super-peer Raft transport is currently loopback; HSM integration is specified but not integrated; expiry-reversion sweeper is a query shell not a running job; no live deployment. Mobile clients (Android, iOS, Flutter) are partial.

## What is sought

Independent expert review on any of:

1. **Economic model defensibility** — are the multipliers (1.3–1.5× visibility, 1.5–2.0× financing, 1.2× tax) defensible against CBI / IMF scrutiny? Which Iraq-specific figures need better grounding?
2. **CBI regulatory fit** — is the 3-of-5 Raft-on-CBI-branches governance model compatible with how CBI actually operates? What's missing from the Parliament / oversight-board layer?
3. **Real-estate sector realism** — is the mortgage-primitive + construction-loan + title-registry path plausible given Iraqi Ministry of Justice registry state, and what sequencing would the 2025–2030 housing plan require?
4. **Gaps we haven't seen** — cryptographic, economic, political-economy, or operational.

Not seeking: press coverage, cold outreach to CBI officials, or procurement conversations. This is a design artefact and is shared for review.

---

*Dated: April 2026. Document version 1. For annotated feedback, marginal comments on the README (~2,600 lines) are welcome and preferred over high-level critique.*
