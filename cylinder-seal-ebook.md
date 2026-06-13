# Cylinder Seal

Digital IQD, Industrial Dividends, Civic Work, and Iraq's Unified Economic Model

**Status:** Prototype and policy-design ebook. Not production CBDC infrastructure.

**Generated:** 2026-06-13

This ebook is generated from the repository documentation. It preserves the
prototype boundary: Cylinder Seal is suitable for technical review, policy
exploration, and demo workflows, but it is not production-ready payment
infrastructure and is not an official Central Bank of Iraq project.

# Contents

1. Project Overview
2. Current Implementation Status
3. Final Summary
4. Security Model
5. Economic Assumptions And Source Discipline
6. System And Financial Flow Diagrams
7. Unified Economic Model
8. National Dividend Holding Company
9. INDHC Ten-Year Plan
10. Digitally Governed Industrial Champions
11. National Civic Work System
12. Ministry Transition Roadmap
13. Technical Primitives
14. Legacy Policy Paper Boundary

# Diagram Atlas

## Software System Architecture

![Software System Architecture](docs/ebook/assets/software-system-architecture.png)

## Unified Economic Model

![Unified Economic Model](docs/ebook/assets/unified-economic-model.png)

## Transaction Lifecycle

![Transaction Lifecycle](docs/ebook/assets/transaction-lifecycle.png)

## Financial Flow Combinations

![Financial Flow Combinations](docs/ebook/assets/financial-flow-combinations.png)

## Transaction Combination Matrix

![Transaction Combination Matrix](docs/ebook/assets/transaction-combination-matrix.png)

## National Dividend Holding Company

![National Dividend Holding Company](docs/ebook/assets/national-dividend-holding-company.png)

## National Civic Work System

![National Civic Work System](docs/ebook/assets/national-civic-work-system.png)

# Part 1: Project Overview

## Cylinder Seal

Sovereign digital-payment and economic-visibility prototype for Iraq.

Cylinder Seal explores how CBI-backed digital IQD payment rails could support financial inclusion, SME credit scoring, public-transfer controls, domestic-production incentives, and regulator-grade economic dashboards. It is a working Rust prototype and policy architecture, not production CBDC infrastructure and not an official Central Bank of Iraq project.

![Cylinder Seal architecture](1776870497788.png)

### What This Repo Contains

The workspace is organized as a set of focused Rust crates:

| Area | Crates and files |
| --- | --- |
| Core ledger models | `crates/cs-core`, `crates/cs-storage` |
| Sync and consensus | `crates/cs-sync`, `crates/cs-consensus`, `proto/chain_sync.proto` |
| Policy, AML, credit | `crates/cs-policy`, `crates/cs-credit`, `crates/cs-exchange`, `crates/cs-feeds` |
| APIs and node runtime | `crates/cs-api`, `crates/cs-node` |
| POS and mobile surfaces | `crates/cs-pos`, `crates/cs-mobile-core`, `android/`, `ios/` |
| CBI-style dashboard | `crates/cbi-dashboard`, `crates/cs-analytics` |
| Specification tests | `crates/cs-tests` |

The thesis is narrow on purpose: use digital payment trails to make informal activity more bankable and visible, then let policy modules experiment with merchant tiers, transaction-based credit, AML workflows, and public-transfer constraints.

### Current Status

| Status | Scope |
| --- | --- |
| Implemented | Rust domain models, canonical signing primitives, transaction/wire-format primitives, KYC tier limits, POS/mobile codecs, PostgreSQL-backed CBI dashboard routes, AML/risk/credit modules, and numbered specification tests. |
| Partially implemented | Offline payment lifecycle, double-spend reconciliation, merchant-tier policy, transaction-based credit scoring, AML reporting, dashboard UI, and Raft-backed sync. These have code and tests, but need production integration and security hardening. |
| Not production-ready | HSM or secure-element custody, national identity/KYC integration, audited offline double-spend prevention, real multi-peer Raft deployment, CBI/core-banking integration, privacy review, disaster recovery, formal threat model review, and externally validated economic impact model. |

The codebase should be read as a pilot-grade prototype. It is suitable for technical review, policy exploration, and demo workflows. It should not be represented as ready for national-scale deployment.

### Quick Start

Install Rust and Docker if you want to run the dashboard stack locally. The dashboard currently uses PostgreSQL and Redis; SQLite files in this repository are legacy/local fixture helpers, not a supported dashboard runtime.

```bash
## Start PostgreSQL and Redis.
cp .env.example .env
docker compose up -d

## Build the main dashboard package.
cargo build --package cbi-dashboard

## Run the dashboard.
export DATABASE_URL="postgresql://postgres:${DB_PASSWORD:-change-me-dev-only}@localhost:5432/cylinder_seal"
cargo run --package cbi-dashboard
```

The dashboard defaults to `http://127.0.0.1:8081` when run locally. Demo operators are seeded only for local development; see `.env.example` and `API_REFERENCE.md` before using them.

`docker-compose.yml` reads `DB_PASSWORD` from `.env` and falls back to
`change-me-dev-only` for local demos. Change all demo secrets before sharing,
deploying, or connecting real systems.

### Technical Evidence

The public-facing technical evidence has been split out of the original long README:

- [Technical primitives](docs/technical-primitives.md) maps claims such as offline payments, double-spend checks, wire-format primitives, Raft, key handling, privacy, AML, and disaster recovery to code and remaining gaps.
- [System and financial flow diagrams](docs/system-and-financial-flow-diagrams.md) provides rendered SVG architecture diagrams, transaction lifecycles, and valid financial-flow combinations.
- [Implementation status](IMPLEMENTATION_STATUS.md) summarizes dashboard implementation state.
- [Specification and fixture results](SPECIFICATION_AND_FIXTURE_RESULTS.md) and [cs-tests README](crates/cs-tests/README.md) describe current test evidence and the missing live PostgreSQL/Redis coverage.
- [API reference](API_REFERENCE.md) documents the dashboard API.

### Economic And Policy Framing

The Iraq-specific policy narrative is intentionally separate from the implementation README:

- [Economic assumptions](docs/economic-assumptions.md) lists current public facts, source discipline, and claims that must remain illustrative until independently modeled.
- [Unified economic model](docs/unified-economic-model.md) connects Digital IQD, INDHC, ministries, banks, producers, strategic resilience manufacturing, tourism, green capital, rail, taxes, civic work, reinvestment, and citizen dividends into one accounting and feedback structure.
- [Policy paper draft](docs/policy-paper.md) preserves the full sovereign-economic thesis from the previous README. It is a working draft, not an externally validated forecast.
- [National dividend holding company](docs/national-dividend-holding-company.md) describes a proposed oil-income-to-productive-capital architecture where citizens hold non-saleable beneficial shares and receive audited Digital IQD dividends from distributable surplus.
- [INDHC ten-year plan](docs/indhc-10-year-plan.md) turns that architecture into a planning envelope and cashflow model for import substitution, profitable national businesses, defense-controlled supply chains, electronics, HVAC, water/desalination, irrigation, imported-food substitution, tourism and services, green capital, open rail, raw-material processing, Iraqi-only permanent staffing, and domestic reinvestment.
- [Digitally governed industrial champions](docs/digitally-governed-industrial-champions.md) reframes the Korean large-industrial-group analogy into sectoral Iraqi production groups with conditional demand, conditional credit, export discipline, debt caps, competition gates, and anti-capture controls.
- [National civic work system](docs/national-civic-work-system.md) adds a dignity-preserving transition layer where citizens are paid in Digital IQD for verified environmental, care, sport, culture, education, municipal, food-security, and disaster-resilience work.
- [Ministry transition roadmap](docs/ministry-transition-roadmap.md) lists candidate ministry functions to deprecate, merge, regulate, corporatize, or sunset as services move to audited operators, municipalities, regulators, INDHC, and Digital IQD service contracts.

Within the policy architecture, the unified model is the front-door economic frame. Digital IQD is the auditable payment, identity, compliance, civic-work, and dividend-distribution substrate; INDHC is the productive-capital engine; ministries are funded through explicit taxes, levies, and service contracts; citizens receive wages, services, credit access, civic-work income, and equal dividends from audited surplus.

The civic-work layer is framed as a national participation economy: not
unemployment benefits, not fake jobs, and not old-style ministry payroll
expansion, but verified public value paid through Digital IQD.

The README intentionally does not present national-scale timelines, sovereign
rating outcomes, diaspora capital figures, or Year 5 benefit ranges as project
deliverables. Those figures belong only in scenario documents with explicit
caveats, source notes, and independent-validation requirements.

Current public facts that shape the framing:

- Iraq's final 2024 census count was reported at 46.1 million people, not the older approximately 43 million baseline used in earlier drafts. Source: [AP, Feb. 24, 2025](https://apnews.com/article/iraq-census-final-count-45b7753ddc82c188c79faea0d5a8c90d).
- Iraq's National Financial Inclusion Strategy 2025-2029 targets account ownership of 50% by 2030 and digital payment usage of 85%. Sources: [CBI NFIS PDF](https://cbi.iq/static/uploads/up/file-175032973296039.pdf), [Arab Monetary Fund](https://www.amf.org.ae/en/news/25-05-2025/iraq-launches-national-financial-inclusion-strategy-2025-2029).
- On June 12, 2026, S&P affirmed Iraq at `B-/B`, removed the long-term rating from CreditWatch negative, and kept a negative outlook. Source: [S&P Global Ratings](https://www.spglobal.com/ratings/en/regulatory/article/-/view/type/HTML/id/3580473).
- Public sources continue to describe Iraq as highly oil-revenue-dependent and fiscally exposed to rigid spending and weak non-oil revenues. Sources: [EIA Iraq analysis](https://www.eia.gov/international/analysis/country/irq), [EITI Iraq country page](https://eiti.org/countries/iraq), [IMF Iraq 2025 Article IV](https://www.imf.org/-/media/files/publications/cr/2025/english/1irqea2025001-source-pdf.pdf).

### Production Readiness Boundary

Before this could be evaluated as real payment infrastructure, the project would need at minimum:

- A formal threat model for wallets, POS devices, offline settlement, super-peers, operator access, and emergency controls.
- Hardware-backed key custody and recovery design.
- Offline double-spend limits backed by secure monotonic counters or equivalent attestation.
- Privacy architecture separating payment data, identity data, regulatory access, and aggregate economic analytics.
- Real multi-node consensus deployment with operational runbooks and failover tests.
- Independent security audit, compliance review, and economic model validation.

### Repository Hygiene

Local artifacts such as generated databases, Redis dumps, virtualenvs, and ad hoc logs are ignored. Do not commit generated database state.


# Part 2: Current Implementation Status

## Implementation Status

### Honest Status

Cylinder Seal is a prototype implementation with meaningful code coverage across domain modules, policy logic, dashboard routes, and tests. It is not production-ready CBDC/payment infrastructure.

### Implemented Or Partially Implemented

- Core transaction models, signing primitives, nonce/journal concepts, and KYC tier limits.
- Policy modules for AML, rule evaluation, risk scoring, merchant tiers, hard restrictions, and programmability primitives.
- Consensus and sync prototypes, including Raft abstractions and conflict-resolution logic.
- POS/mobile-core codecs for QR/NFC/BLE-oriented payment payloads.
- CBI dashboard Axum service with PostgreSQL pool, Redis sessions, and route modules.
- Testable dashboard app builder plus route-level tests for session middleware, CSRF checks, logout invalidation, current role gates, and admin action audit recording.
- PostgreSQL-backed admin audit recorder for current sensitive dashboard actions, with in-memory test recorder.
- PostgreSQL migrations for the main application stack.
- Specification tests for many protocol and policy behaviors.
- Unified economic model documented as an integrated planning layer across Digital IQD, INDHC, ministries, credit, domestic production, strategic resilience manufacturing, tourism/exports, green/rail infrastructure, reinvestment, and dividends.
- National Dividend Holding Company policy architecture documented as a proposal, including oil-income lockbox, citizen share entitlements, cash formalization controls, ten-year investment plan, ministry funding, and monthly dividend flows.
- Ministry transition roadmap documented as a governance scenario for moving selected ministry functions into regulators, service contracts, municipalities, INDHC subsidiaries, or sunset agencies.
- National Civic Work System documented as a policy scenario for verified civic wages, civic credits, training records, public-value tasks, and productivity-transition support.

### Important Gaps

- Dashboard route integration tests now cover security middleware and selected skeletal handlers, but live PostgreSQL/Redis endpoint coverage is still incomplete.
- Some route handlers remain skeletal or demo-oriented.
- Role enforcement must continue to be applied as future sensitive handlers are implemented.
- Browser security hardening has a basic cookie/CSRF foundation but still needs full browser automation and deployment review.
- Admin audit logging is not yet a complete immutable retention system or regulator evidence-pack workflow.
- The unified economic model is not implemented as calibrated projection code; it remains a documented architecture that needs data models, equations, sensitivity tests, and independent macroeconomic review.
- The ministry transition roadmap is not implemented and would require legislation, civil-service transition, federal/governorate coordination, service-continuity testing, and independent audit before any real-world use.
- SQLite is not a supported dashboard runtime despite the presence of historical fixture scripts.
- Offline double-spend handling is not yet backed by audited secure hardware/attestation.
- HSM custody, national identity/KYC integration, CBI/core-banking integration, DR, and privacy review remain future work.
- The National Dividend Holding Company proposal and ten-year investment plan are not implemented in code and need legal, fiscal, debt-capacity, AML/CFT, competition, and governance validation before data models or routes are treated as deployable.
- The National Civic Work System is not implemented in code and needs labor-law, privacy, child-protection, municipal-authority, anti-corruption, disability-access, and fiscal validation before any `cs-civic-work` module is treated as deployable.

### Running The Dashboard

```bash
cp .env.example .env
docker compose up -d
export DATABASE_URL="postgresql://postgres:${DB_PASSWORD:-change-me-dev-only}@localhost:5432/cylinder_seal"
cargo run --package cbi-dashboard
```

### Verification

```bash
cargo check --package cbi-dashboard
cargo test --workspace
```

Passing these checks should be described as prototype verification, not production certification.


# Part 3: Final Summary

## Cylinder Seal Prototype Summary

### Status

Cylinder Seal is a prototype backend and dashboard for Digital IQD policy and economic-visibility workflows. It is not production-ready payment infrastructure.

Current posture:

- **Backend:** prototype implementation with substantial domain modules and route scaffolding.
- **Dashboard:** PostgreSQL-backed Axum service with CBI-style API/page routes and Redis sessions.
- **Tests:** specification and skeleton/integration-style tests exist, but they are not enough to claim production readiness.
- **Security:** threat model and production controls are documented as requirements in `SECURITY.md`; they are not complete.

### What Exists

- Rust workspace with core models, storage, API, policy, AML, credit, consensus, sync, POS, mobile-core, analytics, and dashboard crates.
- CBI dashboard routes for overview, industrial projects, analytics, compliance, monetary policy, accounts, risk, audit, producers, and authentication.
- PostgreSQL migrations for production-like development.
- SQLite fixture scripts retained only for local schema/seed inspection; the dashboard runtime is PostgreSQL-only.
- Redis-backed session storage, Argon2id password hash verification, and admin action audit recording for current sensitive handlers.
- Specification tests covering crypto primitives, signing, nonce chains, Raft behavior, AML, credit scoring, wire formats, conflict resolution, programmability primitives, and tier policy behavior.
- Unified economic model documentation connecting Digital IQD transactions, INDHC capital allocation, ministry funding, credit expansion, domestic production, strategic resilience manufacturing, tourism/exports, green/rail cost reduction, reinvestment, and citizen dividends.
- Policy documentation for a proposed National Dividend Holding Company: oil-income lockbox, citizen non-saleable beneficial shares, ten-year industrial/infrastructure plan including defense-controlled supply chains, electronics, HVAC, water/desalination, irrigation, and imported-food substitution, gross-profit levy for ministry funding, and Digital IQD monthly dividend distribution.
- Ministry transition roadmap for deprecating, merging, regulating, corporatizing, or sunsetting low-feedback ministry functions after legal, service-continuity, staff-transition, and audit gates.
- National Civic Work System policy architecture for converting productivity displacement into verified Digital IQD civic wages, training records, care, environmental restoration, sport, culture, municipal repair, and disaster-resilience work.

### What Is Not Proven Yet

- Real endpoint/database integration coverage across all dashboard routes.
- Production-grade role enforcement on every sensitive route.
- Full browser-session hardening beyond the current cookie/CSRF foundation.
- HSM or secure-element key custody.
- Audited offline double-spend prevention.
- National identity/KYC integration.
- Real multi-peer Raft deployment and recovery testing.
- Immutable audit-log storage.
- Independent macroeconomic model validation.
- Calibration of the unified economic model with audited baselines, equations, sensitivity analysis, and independent review.
- Independent legal, fiscal, debt-capacity, AML/CFT, competition, and governance validation for the National Dividend Holding Company and ten-year investment plan.
- Independent constitutional, administrative-law, labor, federalism, and service-continuity review for the ministry transition roadmap.
- Independent labor-law, privacy, child-protection, municipal-authority, anti-corruption, disability-access, and fiscal review for the National Civic Work System.

### Development Stack

```bash
cp .env.example .env
docker compose up -d
export DATABASE_URL="postgresql://postgres:${DB_PASSWORD:-change-me-dev-only}@localhost:5432/cylinder_seal"
cargo run --package cbi-dashboard
```

Use local demo operators only for development. Replace all seeded operator hashes and all placeholder secrets before sharing, staging, or deploying.

### Credible Positioning

Use this language externally:

> Cylinder Seal is an open-source prototype for Digital IQD economic-visibility infrastructure, with a CBI dashboard, policy analytics, AML/account-management APIs, and a PostgreSQL development stack.

Avoid this language:

> Production-ready national digital dinar infrastructure.

### Next Readiness Work

1. Add real dashboard route integration tests against PostgreSQL and Redis.
2. Enforce role-based authorization consistently.
3. Add CSRF/session hardening for browser flows.
4. Implement and test immutable audit logging.
5. Complete the threat model in `SECURITY.md` with design decisions and residual risks.
6. Validate policy/economic scenarios with cited sources and independent review.
7. Convert the National Dividend Holding Company and ten-year investment plan into explicit legal assumptions, data models, and tests only after policy review.


# Part 4: Security Model

## Security Model And Threat Notes

Cylinder Seal is a prototype. This document describes the security posture expected before production use and highlights the current gaps. It is not a completed audit.

### Supported Security Claims Today

- Transaction signing primitives exist in `crates/cs-core`.
- Admin password verification uses Argon2id hashes.
- Dashboard sessions are stored as opaque Redis-backed tokens with HttpOnly cookie support for page flows.
- Current sensitive dashboard handlers enforce role checks and record `ok` or `denied` admin actions through an audit recorder.
- AML/risk and audit-log route modules exist.
- Offline conflict-resolution logic exists in prototype form.

These are implementation building blocks, not certification that the system is safe for real funds.

### Key Custody

Current state:

- Wallet/POS signing code exists at prototype level.
- Admin bootstrap can generate an initial supervisor password.
- No HSM-backed key ceremony or production custody policy is implemented.

Production requirements:

- HSM or secure enclave custody for super-peer, operator, settlement, and signing keys.
- Documented key ceremony with split control and witness logs.
- Rotation, revocation, and compromise runbooks.
- Hardware-backed attestation for payment devices that can spend offline.
- Recovery policy for citizens, merchants, and operators.

### Offline Double-Spend Model

Current state:

- Offline transactions can be signed and later synchronized.
- Conflict detection/reconciliation logic exists for sibling entries.
- KYC tiers cap offline exposure in the domain model.

Production requirements:

- Secure monotonic counters or equivalent secure-element attestation.
- Offline balance reservation or risk-bounded credit exposure model.
- Clear liability rules for conflicting offline spends.
- Reconciliation process approved by legal, supervisory, and consumer-protection stakeholders.
- Red-team testing of compromised devices, cloned keys, clock tampering, and replay attempts.

### Transaction Signing And Validation

Current state:

- Canonical signing tests exist.
- Wire-format primitives exist for expiry, spend constraints, and conditional release.

Production requirements:

- Published transaction-envelope specification and golden test vectors.
- Version negotiation and backwards-compatible migration rules.
- Independent cryptographic review of canonicalization, signature scope, nonce semantics, and replay protection.
- Validation rules documented as normative protocol behavior, not only Rust implementation details.

### Device Compromise

Production threat cases:

- Stolen or rooted mobile device.
- Compromised POS terminal.
- Malware exfiltrating private keys or queued offline payments.
- Cloned merchant terminal.
- Malicious operator attempting to override risk controls.

Required controls:

- Device binding and attestation.
- Local storage encryption with hardware-backed keys.
- Remote revocation and velocity downgrades.
- Tamper-evident device logs.
- Risk-based offline spending ceilings.

### Account Recovery

Current state:

- No complete recovery model is documented.

Production requirements:

- Citizen recovery without single-operator abuse.
- Merchant recovery for lost POS devices and staff turnover.
- Multi-party approval for privileged account recovery.
- Clear audit trail and mandatory cooling-off periods for high-risk recovery events.

### Audit Log Immutability

Current state:

- Audit-log tables and routes exist.
- Current sensitive dashboard handlers write admin action records for account freeze/unfreeze, emergency directive creation, compliance-report actions, and industrial-project mutations.
- Route-level tests assert audit records for allowed and denied actions through an in-memory audit recorder.

Production requirements:

- Append-only storage with tamper-evident hashes.
- WORM or equivalent immutable retention for privileged actions.
- Exportable regulator evidence packs.
- Separate audit-reader role with no mutation authority.
- Monitoring for audit-log gaps, rewrites, and clock anomalies.

### Operator Privilege Controls

Current state:

- Role concepts exist: auditor, analyst, officer, supervisor.
- Current sensitive route stubs enforce officer or supervisor roles and have route-level tests.

Production requirements:

- Least-privilege role matrix for every endpoint.
- Four-eyes approval for account freezes, emergency directives, rule changes, and recovery.
- MFA for all operators.
- Session timeout, step-up authentication, and device-bound operator sessions.
- Break-glass authority with automatic expiry and independent review.

### Emergency Authority Limits

Current state:

- Emergency directive models and dashboard routes exist.

Production requirements:

- Legal basis for each emergency action type.
- Time-bound directives with automatic expiry.
- Scope constraints by region, account class, merchant tier, or risk class.
- Dual approval and post-event review.
- Public transparency policy where appropriate.

### Privacy And Data Minimization

Production requirements:

- Separation of payment, identity, AML, location, and aggregate analytics data.
- Role-scoped access to personally identifiable data.
- Coarsened location by default.
- Data retention schedules.
- Privacy impact assessment before any real citizen or merchant data is used.

### National Dividend And Cash Formalization Threat Boundary

The proposed National Dividend Holding Company model adds higher-risk workflows
than ordinary retail payments: oil-income receipts, citizen share entitlements,
cash formalization, ministry funding, investment allocations, and dividend
distribution. These workflows should be treated as public-finance critical
infrastructure.

Required controls before implementation:

- Statutory authority for oil-income handling, cash demonetization, dividend
  formulas, share entitlement, and inheritance rules.
- Strict separation between citizen base shares and supplemental transition
  entitlements created during any cash formalization window.
- KYC, sanctions screening, politically exposed person screening, enhanced due
  diligence, caps, holds, referrals, and appeal paths for cash conversion.
- Four-eyes approval and immutable audit records for lockbox allocation,
  stabilization-reserve release, gross-profit levy calculation, dividend-batch
  creation, and post-facto correction.
- Public aggregate transparency for oil receipts, allocations, ministry
  transfers, investment performance, dividend pool size, and audit exceptions.
- No anonymous conversion of cash into liquid or pledgeable assets.
- No operator, ministry, political party, contractor, bank, or private entity
  ability to acquire citizen base shares.

### Reporting Vulnerabilities

Do not disclose exploitable vulnerabilities through public issues, social media,
or public pull requests.

Until a dedicated security mailbox is provisioned, report privately to the
project contact listed in `EXECUTIVE_SUMMARY.md`. Include:

- Affected component, endpoint, route, crate, or document.
- Reproduction steps or proof-of-concept details.
- Potential impact, including whether funds, identity data, audit logs, or
  privileged operator actions are affected.
- Whether the issue has been shared with any third party.

Expected handling:

- Acknowledge receipt within 3 business days when a valid contact channel is
  available.
- Triage severity as critical, high, medium, or low.
- Avoid requesting public disclosure until a mitigation or documented residual
  risk decision exists.
- Credit the reporter when appropriate and requested, subject to legal and
  safety constraints.

This process is a prototype disclosure policy. Before external deployment,
replace it with a dedicated security contact, patch SLA, coordinated-disclosure
timeline, and legal safe-harbor language.


# Part 5: Economic Assumptions And Source Discipline

## Economic Assumptions And Source Discipline

The economic narrative is the strongest part of Cylinder Seal, but it must be presented as an illustrative model until every number is independently sourced, dated, and stress-tested.

### Current Public Facts To Use

| Topic | Current framing | Source |
| --- | --- | --- |
| Population | Use the final 2024 census count of 46.1 million as the latest official census baseline. Avoid the older approximately 43 million figure in external summaries. | [AP, Feb. 24, 2025](https://apnews.com/article/iraq-census-final-count-45b7753ddc82c188c79faea0d5a8c90d) |
| Financial inclusion | Iraq's NFIS says inclusion is low, with cited ranges around 11% to 19% depending on the data source. It also notes persistent cash use and weak credit infrastructure. | [CBI NFIS PDF](https://cbi.iq/static/uploads/up/file-175032973296039.pdf) |
| NFIS targets | Iraq's 2025-2029 strategy targets 50% adult bank or digital account ownership by 2030 and 85% digital payment usage. | [Arab Monetary Fund, May 25, 2025](https://www.amf.org.ae/en/news/25-05-2025/iraq-launches-national-financial-inclusion-strategy-2025-2029), [AFI](https://afi-global.org/news/iraq-launches-national-financial-inclusion-strategy-2025-2029/) |
| Credit infrastructure | The NFIS states that the credit reporting system covers only 1.3% of adults, below the MENA average cited in the strategy. | [CBI NFIS PDF](https://cbi.iq/static/uploads/up/file-175032973296039.pdf) |
| Sovereign rating | On June 12, 2026, S&P affirmed Iraq at `B-/B`, removed the long-term rating from CreditWatch negative, and kept a negative outlook. Do not describe the rating as still on CreditWatch negative after that date. | [S&P Global Ratings, Jun. 12, 2026](https://www.spglobal.com/ratings/en/regulatory/article/-/view/type/HTML/id/3580473) |
| Oil-revenue dependence | Current framing may state that Iraq remains highly dependent on oil revenue, but exact percentages must be dated and sourced. | [EIA Iraq analysis](https://www.eia.gov/international/analysis/country/irq), [EITI Iraq country page](https://eiti.org/countries/iraq) |
| Fiscal rigidity and non-oil revenue | Current framing may state that IMF staff have highlighted rigid fiscal spending, subdued non-oil revenues, and vulnerability to lower oil prices. Do not treat any policy remedy as IMF-endorsed unless directly stated by the IMF. | [IMF Iraq 2025 Article IV](https://www.imf.org/-/media/files/publications/cr/2025/english/1irqea2025001-source-pdf.pdf), [IMF Iraq 2024 Article IV](https://www.imf.org/en/publications/cr/issues/2024/05/15/iraq-2024-article-iv-consultation-press-release-staff-report-and-statement-by-the-executive-549028) |

### How To Present Modeled Numbers

Every major economic number should carry four labels:

| Label | Meaning |
| --- | --- |
| Source | Where the baseline came from, with date and URL or document name. |
| Model step | The equation or assumption used to transform the baseline. |
| Confidence | High, medium, low, or illustrative. |
| Owner | Person or institution responsible for validating the figure before external use. |

Recommended language:

- "Illustrative model" for benefit projections, adoption curves, credit-rating pathways, GDP impacts, and import-substitution gains.
- "Pilot target" for near-term deployment goals.
- "Production dependency" for prerequisites such as national ID, CBI integration, HSM custody, and audited offline settlement.

Avoid language that implies certainty:

- "12-15 months to national scale" should become "a 12-15 month pilot-to-scale hypothesis requiring regulatory approval, security audit, procurement, and banking integration."
- "$7.5-12.5B annual benefit by Year 5" should become "illustrative Year 5 benefit range pending independent macroeconomic modeling."
- "Unbanked population drops from 70% to 5%" should become "financial inclusion improvement scenario; baseline and target must be reconciled with NFIS definitions."
- "Credit rating improves from B3 to Ba1" should become "potential sovereign-credit relevance; no rating outcome should be forecast as a project deliverable."
- "Diaspora capital repatriation $80-150B/year" should become "diaspora merchant and tourism distribution channel hypothesis unless backed by flow data."
- "Cylinder Seal is a complete economic operating system" should become "unified economic model proposal requiring legal authority, audited data, calibrated equations, policy review, and independent macroeconomic validation."
- "Cylinder Seal will abolish frivolous ministries" should become "ministry-transition scenario in which specific functions are deprecated, merged, regulated, corporatized, or sunset only after legal authority, service-continuity gates, staff-transition plans, and independent audits."
- "Oil income should fund a citizen-owned state industrial holding company" should become "proposed national dividend holding-company architecture requiring constitutional, fiscal, oil-revenue, AML/CFT, competition, and governance review."
- "The holding company will invest $190B over ten years" should become "base-case planning envelope requiring oil-revenue stress testing, debt-capacity analysis, procurement sequencing, and independent project finance review."
- "INDHC will make Iraq self-sufficient in defense, electronics, HVAC, water, irrigation, and food" should become "strategic resilience manufacturing objective requiring delivered-cost tests, quality certification, legal controls, supplier development, and security/procurement review."
- "INDHC will generate $43B in Year 10 revenue" should become "illustrative Year 10 consolidated revenue run-rate in the base-case cashflow model, requiring audited baseline demand, project-level feasibility studies, utilization assumptions, and downside sensitivity tests."
- "Iraq should create a state chaebol" should become "digitally governed sectoral industrial champions with conditional demand, conditional credit, export discipline, competition gates, debt caps, and anti-capture governance."
- "Automation and industrial champions will solve employment" should become "productivity gains may displace low-productivity work and require a verified civic-work, training, and bridge-to-employment system."

### Data Gaps To Close

- Updated Iraq population projection for 2026 from an official source, if a 2026 estimate is needed.
- Current CBI digital-payment adoption metrics and definitions.
- Government wage bill and public-transfer volumes by payment channel.
- Merchant acceptance and POS penetration by governorate.
- Informal employment and MSME counts by source and year.
- Import bill and sectoral import-substitution baselines.
- Bank credit to MSMEs, collateral requirements, and rejection rates.
- Official CBI or Ministry of Finance stance on any digital dinar or CBDC pilot.

### Recommended External Pitch

Use this positioning until the economic model is independently validated:

> Cylinder Seal is a sovereign digital-payment and economic-visibility prototype for Iraq, designed to show how CBI-backed digital IQD transactions could support financial inclusion, SME credit scoring, public-transfer controls, and domestic-production incentives.

That claim is strong, defensible, and aligned with the code that exists today.


# Part 6: System And Financial Flow Diagrams

## System And Financial Flow Diagrams

This document maps the Cylinder Seal prototype as a software system and as a set
of financial-flow patterns. It is intentionally conservative: diagrams describe
the target architecture and current prototype boundaries, not production-ready
CBDC infrastructure.

The financial-flow matrix is "complete" for the design surface used in this
repository: every modeled transaction is a combination of actor pair, channel,
programmability primitive, settlement mode, and oversight path.

### Rendered Diagram Atlas

These SVGs are the primary reviewer-facing diagrams. They are kept as standalone
files so they render cleanly in GitHub, can be reused in presentations, and can
be inspected in code review.

#### Software System Architecture

![Cylinder Seal software system architecture](docs/ebook/assets/software-system-architecture.png)

#### Unified Economic Model

![Cylinder Seal unified economic model](docs/ebook/assets/unified-economic-model.png)

#### Transaction Lifecycle

![Cylinder Seal transaction lifecycle](docs/ebook/assets/transaction-lifecycle.png)

#### Financial Flow Combinations

![Cylinder Seal financial flow combinations](docs/ebook/assets/financial-flow-combinations.png)

#### Transaction Combination Matrix

![Cylinder Seal transaction combination matrix](docs/ebook/assets/transaction-combination-matrix.png)

#### National Dividend Holding Company

![National dividend holding company financial architecture](docs/ebook/assets/national-dividend-holding-company.png)

#### National Civic Work System

![National civic work verification and payment architecture](docs/ebook/assets/national-civic-work-system.png)

### Legend

| Marker | Meaning |
| --- | --- |
| Prototype | Code or tests exist in this repository, but production hardening may be incomplete. |
| Integration requirement | External system, legal rule, HSM, secure element, national ID, bank/core-banking system, or supervisory process required before real deployment. |
| Control-plane flow | Operator, policy, audit, compliance, or emergency-control action. |
| Value flow | Movement of Digital IQD or related value claim. |
| Data flow | Derived analytics, audit trail, risk signal, or credit feature. |

### Unified Economic Model

The unified model is documented in
[Unified Economic Model](unified-economic-model.md). It connects the software
system, financial flows, INDHC, ministries, banks, producers, tourism, green
capital, rail, taxes, reinvestment, and citizen dividends into one accounting
and feedback structure.

### Software System Diagrams

#### 1. System Context



> Mermaid source omitted from the ebook body. Use the rendered SVG diagram atlas or the repository Markdown for source.



Use case: gives CBI, banks, MDB reviewers, and implementers a single map of the
software boundary.

Advantage: separates repository code from required production integrations,
which reduces readiness overclaiming.

#### 2. Transaction Processing Pipeline



> Mermaid source omitted from the ebook body. Use the rendered SVG diagram atlas or the repository Markdown for source.



Use case: shows where a payment becomes more than a balance transfer: it can
also produce risk signals, credit features, policy compliance, and aggregate
economic visibility.

Advantage: makes it clear that restrictions are meant to be enforced in the
validation path, not only in wallet UI code.

#### 3. Online Transaction Lifecycle



> Mermaid source omitted from the ebook body. Use the rendered SVG diagram atlas or the repository Markdown for source.



Use case: connected retail payment, online P2P transfer, bank disbursement,
government transfer, tax payment, or procurement payment.

Advantage: fastest finality and strongest immediate policy/risk enforcement.

#### 4. Offline Transaction Lifecycle



> Mermaid source omitted from the ebook body. Use the rendered SVG diagram atlas or the repository Markdown for source.



Use case: rural retail, market stalls, taxis, conflict-zone connectivity gaps,
and low-value citizen-to-citizen payments.

Advantage: keeps transactions documented when internet service is missing, while
constraining exposure through tier limits and sync-time conflict handling.

Production boundary: real deployment still needs secure elements or equivalent
attested monotonic counters, formal liability rules, revocation, and device
recovery.

#### 5. Control Plane And Operator Security



> Mermaid source omitted from the ebook body. Use the rendered SVG diagram atlas or the repository Markdown for source.



Use case: CBI-style operators need visibility and intervention powers without
turning every dashboard user into a superuser.

Advantage: creates a visible authorization boundary for the exact actions that
matter most to a financial-infrastructure reviewer.

#### 6. Data And Privacy Boundaries



> Mermaid source omitted from the ebook body. Use the rendered SVG diagram atlas or the repository Markdown for source.



Use case: defines the minimum split reviewers expect between payment data,
identity data, compliance access, and aggregate policy analytics.

Advantage: gives a starting point for privacy impact assessment and data
minimization work.

### Financial Flow Model

Every transaction flow is assembled from these dimensions:

| Dimension | Allowed values in the design surface |
| --- | --- |
| Actor pair | C2C, C2M, C2IP, M2C, M2M, IP2M, G2P, G2B, C2G, M2G, B2C, C2B, B2M, M2B, D2M, CBI2B, B2CBI, G2CW, CI2CW, CW2M, CW2IP, CW2G, G2CI |
| Channel | Online API, QR, NFC, BLE, bank batch, government batch, civic-work task workflow, future correspondent-bank bridge |
| Settlement mode | Immediate online finality, pending offline receipt, batch settlement, conditional release, verified civic wage release |
| Primitive | Standard transfer, expiring transfer, spend constraint, conditional release escrow, recurring debit, refund/compensating transfer, civic wage, civic credit bonus |
| Oversight path | None beyond normal validation, tier policy, AML/risk report, tax/fee withholding, credit feature extraction, civic-work verification, supervisor/emergency control |

Actor shorthand:

| Code | Actor |
| --- | --- |
| C | Citizen or household wallet |
| M | Formal merchant or business |
| IP | Individual producer / informal-worker wallet |
| G | Government ministry, salary, pension, social, tax, or procurement account |
| B | Bank, lender, or industrial-finance account |
| D | Diaspora buyer, tourist, pilgrim, or foreign customer |
| CBI | Central-bank or super-peer operating account |
| CW | Civic worker wallet |
| CI | Civic institution: municipality, school, NGO, sports club, university, environmental agency, or verifier |

#### 7. Financial Flow Combination Map



> Mermaid source omitted from the ebook body. Use the rendered SVG diagram atlas or the repository Markdown for source.



Use case: shows that the system is not a single payment path. It is a small set
of reusable rails that combine into retail, government, bank, producer, and
diaspora flows.

Advantage: reduces product sprawl. New use cases should reuse the same envelope,
validation, audit, and projection paths.

### Transaction Combination Matrix

| Flow | Actor pair | Channels | Valid primitives | Use case | Advantage | Boundary |
| --- | --- | --- | --- | --- | --- | --- |
| 1 | C2C | Online, QR, NFC, BLE | Standard, offline pending, refund | Citizen remittance, family support, informal debt repayment | Documents cash-like activity and builds payment history | Offline conflict prevention still needs secure attestation |
| 2 | C2M | Online, QR, NFC, BLE | Standard, offline pending, refund | Retail checkout at formal merchants | Low-friction acceptance, immediate credit features for merchant | Merchant onboarding and device attestation required |
| 3 | C2IP | Online, QR, NFC, BLE | Standard, offline pending | Taxi, market stall, home-food producer, small farmer | Lets informal producers receive documented income without full company registration | IP registration, caps, and tax rules need legal approval |
| 4 | IP2M | Online, QR | Standard, spend constraint where subsidized | Informal producer buys inputs from formal supplier | Creates supply-chain evidence for microcredit and audits | Offline high-value supplier flows should be capped |
| 5 | M2C | Online | Refund, rebate, wage, compensating transfer | Refunds, payroll, customer compensation | Keeps reversals auditable without deleting ledger history | Consumer-protection rules required |
| 6 | M2M | Online, batch | Standard, invoice escrow, spend constraint | Supplier invoice, distributor payment, construction materials | Turns B2B cash flow into credit evidence | Invoice authenticity and dispute workflow required |
| 7 | G2C | Government batch, online wallet | Standard, expiring, spend constraint | Salary, pension, social transfer, stimulus | Can improve inclusion and policy targeting while preserving traceability | Must be legally authorized and privacy-reviewed |
| 8 | G2M | Government batch, online | Spend constraint, conditional release, hard-restriction policy | Procurement, food/textile programs, domestic-content purchasing | Makes public demand auditable and directs funds to eligible suppliers | Procurement law and appeals process required |
| 9 | C2G | Online, batch | Standard, recurring debit | Fees, fines, utility bills, taxes | Reduces cash handling and improves receipts | Government treasury integration required |
| 10 | M2G | Online, batch, automatic withholding | Standard, tax/fee split, report trigger | VAT-like fee, presumptive IP tax, payroll withholding | Passive collection with lower filing burden | Tax authority integration and taxpayer recourse required |
| 11 | B2C | Online | Earmarked loan, conditional release, recurring repayment setup | Consumer loan, mortgage tranche, education finance | Loan proceeds can be restricted to approved purposes | Bank licensing, disclosures, and collateral law required |
| 12 | C2B | Online, recurring | Repayment, auto-debit, refinance settlement | Loan or mortgage repayment | Stable repayment history improves credit scoring | Debt-service caps and consent controls required |
| 13 | B2M | Online, batch | Invoice finance, working-capital escrow, spend constraint | SME working capital and industrial finance | Reduces collateral dependence by using transaction history | Bank risk model validation required |
| 14 | M2B | Online, batch | Repayment, invoice settlement | Merchant loan repayment or deposit sweep | Gives lenders real cash-flow visibility | Deposit and settlement rules required |
| 15 | D2M | Online, merchant QR, future correspondent bridge | Standard, FX-tagged receipt, refund | Diaspora purchase, tourism, pilgrimage services, Iraqi-origin goods | Captures foreign-customer demand through documented merchants | Cross-border and FX compliance not implemented |
| 16 | CBI2B | Bank batch | Liquidity allocation, policy instruction | Liquidity provision or program funding to banks | Clean separation between policy funding and retail disbursement | CBI/core-banking integration required |
| 17 | B2CBI | Bank batch | Settlement, reserve movement, report | Bank settlement and supervisory reporting | Supports monetary oversight and reconciliation | Production settlement rails required |
| 18 | Any valid payer to any valid payee | Online only for action; offline receipt may later sync | Freeze, cap, reject, report, reverse by compensating transfer | Emergency directive, AML hold, fraud response | Provides supervisory control without mutating history | Requires strict emergency powers, audit, and due process |
| 19 | G2CI | Government batch, civic-work workflow | Conditional release, spend constraint | Treasury, municipality, climate, or INDHC community-benefit budget funds approved civic tasks | Keeps civic-work budgets explicit and separate from citizen dividend funds | Appropriation law, municipal authority, and anti-corruption controls required |
| 20 | CI2CW or G2CW | Civic-work task workflow, online wallet | Civic wage, civic credit bonus, conditional release | Verified care, sport, environmental, municipal, culture, education, food-security, or resilience task | Turns spare labor capacity into paid public value, training records, and income history | `cs-civic-work` is design-only; evidence rules, labor law, privacy, safety, and appeal process required |
| 21 | CW2M or CW2IP | Online, QR, NFC, BLE where allowed | Standard transfer, spend constraint for civic credits | Civic worker spends wage or category-limited credit at merchants, transport, training, childcare, local goods, or housing-deposit programs | Converts civic income into local demand while preserving transparent program limits | Spend-limited credits need legal basis and appealable merchant/category rules |
| 22 | CW2G or CW2B | Online, recurring | Standard transfer, recurring debit | Fees, training co-payments, savings, loan repayment, or bank account linkage from verified civic income | Creates formal financial history for thin-file workers | Consent, debt-service caps, and privacy-bounded credit use required |

### Validity Rules For Combinations

| Rule | Applies to | Reason |
| --- | --- | --- |
| Offline is limited to low-value C2C, C2M, C2IP, and selected IP2M flows. | NFC, BLE, QR pending receipts | High-value, bank, government, procurement, and cross-border flows need online finality. |
| Conditional-release escrow can be initiated online and represented in the ledger; release should be online. | G2M, B2C, B2M, M2M | Release depends on third-party evidence, inspector approval, title event, or invoice state. |
| Spend constraints may be carried offline only when recipient eligibility and cap data are locally verifiable. | C2M, C2IP, G2C, G2M, B2C | Final validation still occurs at sync, so offline recipients carry settlement risk. |
| Civic wage release requires verified task evidence and an explicit budget source. | G2CI, CI2CW, G2CW | Civic work must not become hidden dividend leakage, ghost payroll, or patronage spending. |
| Civic credit spending follows the same spend-constraint rules as other earmarked value. | CW2M, CW2IP | Credits can support training, transport, childcare, local goods, sport, or housing deposits only where rules are lawful and appealable. |
| Expiring transfers can be spent before expiry; expired value must revert or be blocked by validator policy. | G2C stimulus, voucher-like flows | Prevents stale stimulus balances and supports velocity policy experiments. |
| Refunds and reversals are new compensating entries, not ledger deletion. | C2M, M2C, D2M, M2M | Preserves auditability and avoids tampering with committed entries. |
| Tax, fee, and tier effects are side effects of settlement, not separate UI promises. | C2M, C2IP, M2G, IP2M | Keeps enforcement at validation and projection layers. |
| AML and risk flags do not automatically prove wrongdoing. | All flows | They create review queues, reports, and evidence packs subject to legal process. |

### Detailed Financial Flow Diagrams

#### 8. Retail Merchant Payment With Tier Effects



> Mermaid source omitted from the ebook body. Use the rendered SVG diagram atlas or the repository Markdown for source.



Use case: retail checkout with domestic-content tiering.

Advantages:

- Merchant receives documented revenue usable for credit scoring.
- Tier policy can reward local content without relying only on after-the-fact audits.
- Government fee/tax effects are visible as ledger side effects rather than hidden cash leakage.

#### 9. Offline Citizen Or Retail Payment



> Mermaid source omitted from the ebook body. Use the rendered SVG diagram atlas or the repository Markdown for source.



Use case: rural market, taxi ride, family transfer, or merchant checkout without
network coverage.

Advantages:

- Keeps low-value activity documented instead of forcing a return to cash.
- Gives merchants and IPs a path into credit evidence.
- Limits exposure through offline caps and sync-time reconciliation.

#### 10. Government Transfer With Programmability



> Mermaid source omitted from the ebook body. Use the rendered SVG diagram atlas or the repository Markdown for source.



Use case: salary, pension, social benefit, voucher-like stimulus, or targeted
domestic-production program.

Advantages:

- Public money remains auditable from issuance to spend.
- Expiry can support velocity experiments for stimulus.
- Spend constraints can target eligible categories while producing evidence for review.

Boundary: real use requires law, appeals, privacy safeguards, and clear public
communications.

#### 11. SME Invoice And Working-Capital Flow



> Mermaid source omitted from the ebook body. Use the rendered SVG diagram atlas or the repository Markdown for source.



Use case: supplier financing, distributor financing, construction supply-chain
finance, or working-capital advance.

Advantages:

- Uses transaction history and invoices instead of only fixed collateral.
- Earmarking can keep loan proceeds inside eligible productive uses.
- Repayment behavior becomes future credit evidence.

#### 12. Mortgage And Real-Estate Flow



> Mermaid source omitted from the ebook body. Use the rendered SVG diagram atlas or the repository Markdown for source.



Use case: IQD mortgage, construction loan, staged homebuilding, or developer
tranche finance.

Advantages:

- Connects long-duration IQD savings/borrowing to real domestic assets.
- Staged release can reduce leakage and unfinished-project risk.
- Repayment records improve borrower and supplier credit history.

Boundary: title registry, foreclosure law, consumer protection, and bank risk
rules are external dependencies.

#### 13. Tax, Fee, And Withholding Flow



> Mermaid source omitted from the ebook body. Use the rendered SVG diagram atlas or the repository Markdown for source.



Use case: merchant tier fee, presumptive IP micro-tax, payroll withholding, or
government service fee.

Advantages:

- Reduces manual filing burden for small participants.
- Makes the rule and amount visible on the receipt.
- Preserves evidence for appeal and audit.

#### 14. Diaspora, Tourism, And FX-Tagged Merchant Flow



> Mermaid source omitted from the ebook body. Use the rendered SVG diagram atlas or the repository Markdown for source.



Use case: diaspora purchase of Iraqi-origin goods, pilgrimage/tourism package,
foreign customer paying an Iraqi service provider.

Advantages:

- Treats diaspora/tourism as distribution demand, not only remittance capital.
- FX-tagged receipts can distinguish external demand from domestic recycling.
- Domestic supplier payments become visible in the same credit and tier system.

Boundary: cross-border, AML, correspondent banking, and FX controls are not
implemented in the current prototype.

#### 15. Emergency, AML, And Dispute Overlay



> Mermaid source omitted from the ebook body. Use the rendered SVG diagram atlas or the repository Markdown for source.



Use case: suspected fraud, sanctions hit, compromised wallet, emergency program
control, or disputed offline receipt.

Advantages:

- Keeps intervention powers auditable.
- Supports case review rather than silent automated punishment.
- Allows emergency controls while preserving a committed evidence trail.

#### 16. Civic Work Task Verification And Wage Flow



> Mermaid source omitted from the ebook body. Use the rendered SVG diagram atlas or the repository Markdown for source.



Use case: tree care, canal maintenance, sport coaching, school tutoring,
elderly visits, disability support, heritage work, heatwave response, or food
and water resilience tasks.

Advantages:

- Makes civic work a measurable output system rather than a payroll label.
- Keeps the budget source, evidence bundle, wage release, audit trail, and
  public-impact metric tied together.
- Gives unemployed and underemployed workers Digital IQD income history,
  training records, and privacy-bounded employability signals.

Boundary: the module is not implemented. Real deployment requires labor-law,
child-protection, care-work, privacy, municipal-authority, anti-corruption,
budget, verifier, safety, and appeal rules.

### Flow Advantages By Policy Objective

| Objective | Best-fit flows | Why they help |
| --- | --- | --- |
| Financial inclusion | C2C, C2M, C2IP, offline pending receipts, IP registration | Converts cash-like activity into documented income and payment history. |
| SME credit | C2M, M2M, B2M, invoice escrow, recurring repayment | Creates cash-flow features and invoice evidence for thin-file firms. |
| Public-transfer control | G2C, G2M, expiring transfers, spend constraints | Gives program administrators a visible issuance-to-spend trail. |
| National dividend and ministry feedback | Oil-income lockbox, INDHC investment allocations, gross-profit levy, citizen dividend | Converts raw oil receipts into audited productive capital, tax-funded ministry budgets, and equal Digital IQD dividends. |
| Productivity transition and civic work | G2CI, CI2CW, G2CW, CW2M, verified civic wage and credit release | Converts spare labor capacity into paid public value, training records, civic reputation, and local demand. |
| Domestic-production incentives | C2M, G2M, B2M, tier policy, earmarked spend | Rewards eligible local suppliers through validation and settlement side effects. |
| Monetary visibility | All committed flows, aggregate analytics | Gives privacy-bounded velocity, sector, and geography signals. |
| AML and supervisory control | All online and synced flows, risk queue, freeze/cap overlay | Produces evidence packs and role-gated intervention paths. |
| Offline resilience | C2C, C2M, C2IP over NFC/BLE/QR | Maintains payment availability during connectivity gaps. |

### Implementation Mapping

| Diagram area | Main files and crates |
| --- | --- |
| Transaction envelope and signatures | `crates/cs-core/src/models.rs`, `crates/cs-core/src/cryptography.rs`, `crates/cs-mobile-core/src/wire.rs` |
| NFC/BLE/QR and POS tender | `crates/cs-pos/src/payment.rs`, `crates/cs-pos/src/nfc.rs`, `crates/cs-pos/src/ble.rs`, `crates/cs-pos/src/qr.rs` |
| Offline conflict handling | `crates/cs-sync/src/conflict_resolver.rs`, `crates/cs-tests/tests/spec_13_conflict_resolution.rs` |
| Programmability primitives | `crates/cs-core/src/primitives.rs`, `crates/cs-policy/src/primitives.rs`, `crates/cs-tests/tests/spec_22_programmability_primitives.rs` |
| Merchant tiers and hard restrictions | `crates/cs-policy`, `crates/cs-tests/tests/spec_23_tier_policy.rs` |
| AML and reporting | `crates/cs-policy/src/aml.rs`, `crates/cs-policy/src/reporting.rs`, `crates/cbi-dashboard/src/routes/compliance.rs`, `crates/cbi-dashboard/src/routes/risk.rs` |
| Credit features | `crates/cs-credit`, `crates/cs-policy/src/risk_scoring.rs` |
| Civic-work architecture | `docs/national-civic-work-system.md` only; proposed `cs-civic-work` models are not implemented |
| Consensus boundary | `crates/cs-consensus`, `crates/cs-sync/src/sync_service.rs`, `crates/cs-sync/src/state_machine.rs` |
| Dashboard sessions and roles | `crates/cbi-dashboard/src/auth.rs`, `crates/cbi-dashboard/src/middleware.rs`, `crates/cbi-dashboard/src/main.rs` |

### Remaining Diagram Gaps

These diagrams make the intended system legible, but they do not close the
remaining engineering gaps:

- HSM and secure-element attestation need a concrete design and tests.
- Offline double-spend prevention still needs hardware-backed monotonic counters
  or an equivalent attested mechanism.
- Real PostgreSQL/Redis endpoint integration tests are needed for dashboard
  route credibility.
- Cross-border, FX, diaspora, and correspondent-bank flows are scenario designs,
  not implemented rails.
- Civic-work task posting, evidence verification, civic wage release, civic
  credits, and impact metrics are policy/design artifacts only; no
  `cs-civic-work` crate, schemas, routes, or tests exist yet.
- Production privacy, legal authority, appeal, and emergency-power procedures
  must be specified before using real citizen or business data.
- The national dividend holding-company model is a policy architecture proposal;
  its legal authority, oil-revenue handling, share-entitlement rules, investment
  governance, and dividend formula require independent review.


# Part 7: Unified Economic Model

## Unified Economic Model

This document ties the Cylinder Seal payment rail, policy primitives, INDHC,
ministries, banks, producers, infrastructure projects, tourism, green capital,
citizen dividends, and verified civic work into one economic model.

Status: planning architecture. It is not a validated macroeconomic model, not a
budget law, not a CBDC launch plan, and not a production-readiness claim.

### One Sentence Model

Cylinder Seal makes economic activity visible and programmable; INDHC converts
oil income and project debt into productive Iraqi assets; private producers and
state subsidiaries expand domestic supply; ministries are funded from explicit
taxes, levies, and service contracts; citizens receive wages, public services,
credit access, verified civic-work income, and equal Digital IQD dividends from
audited surplus.

### System Boundary

The system has six layers:

| Layer | Function | Main documents |
| --- | --- | --- |
| Digital IQD transaction layer | Wallets, POS, offline transactions, settlement, policy primitives, AML, audit, credit features, dashboards. | `README.md`, `docs/technical-primitives.md`, `docs/system-and-financial-flow-diagrams.md` |
| Public-finance and capital layer | Oil Income Lockbox, INDHC, gross-profit levy, ministry budgets, cash formalization, citizen share entitlements, dividends. | `docs/national-dividend-holding-company.md` |
| Ten-year productive economy layer | Import substitution, strategic resilience manufacturing, defense-controlled supply chains, electronics, HVAC, water/desalination, irrigation, food substitution, tourism/services, green capital, open rail, raw-material processing, Iraqi-only permanent staffing. | `docs/indhc-10-year-plan.md` |
| Industrial champion governance layer | Sectoral production groups receive conditional demand, credit, and payment privileges only while they meet local-content, price, quality, export, tax, debt, SME-inclusion, and audit gates. | `docs/digitally-governed-industrial-champions.md` |
| Civic work and dignity layer | Productivity gains fund verified public-value work, training, care, environmental restoration, sport, culture, municipal repair, and disaster resilience. | `docs/national-civic-work-system.md` |
| Ministry transition layer | Deprecates, merges, regulates, corporatizes, or sunsets ministry functions once service-continuity, legal, audit, and staff-transition gates pass. | `docs/ministry-transition-roadmap.md` |

The layers should not be read separately. The payment layer provides the
evidence and controls; the public-finance layer changes the fiscal incentives;
the productive economy layer gives the system real goods, services, jobs, and
profit to measure.

### Core Actors

| Actor | Role in the unified model |
| --- | --- |
| Citizens | Hold non-saleable beneficial INDHC shares, receive wages/transfers/dividends, spend through Digital IQD wallets, build transaction histories, inherit share entitlements. |
| Civic workers | Earn Digital IQD civic wages and credits for verified public-value work, gain training certificates, and build employability records. |
| Individual producers | Enter the formal economy through lightweight registration, Digital IQD sales, presumptive micro-tax, and transaction-based credit. |
| Merchants and SMEs | Sell domestic goods/services, receive Digital IQD, build credit profiles, join tiered local-content incentives. |
| Commercial banks | Provide working capital and project finance using transaction evidence, receivables, collateral, and risk scores. |
| CBI / monetary authority | Operates or supervises Digital IQD issuance, settlement rules, monetary limits, risk controls, and aggregate policy visibility. |
| INDHC | Receives oil-equity allocations, raises project debt, invests in productive subsidiaries, pays taxes/levies, reinvests retained earnings, distributes dividends. |
| Ministries | Stop being direct claimants on raw oil income; receive budgets through taxes, levies, and performance-priced service contracts. |
| Treasury | Receives gross-profit levy, taxes, and other explicit revenue; funds ministries through visible appropriations. |
| Tourism and diaspora channels | Bring foreign currency and external demand into formal Iraqi goods and services. |
| International lenders and investors | Fund bankable green, rail, industrial, and service projects under public use-of-proceeds and debt-safety rules. |
| Auditors, parliament, and public dashboards | Provide legitimacy, aggregate transparency, anti-corruption evidence, and feedback discipline. |

### The Integrated Flow

The unified model is a set of mutually reinforcing loops.

#### 1. Oil-To-Capital Loop

1. Oil receipts enter the Oil Income Lockbox.
2. A stabilization reserve absorbs oil-price shocks.
3. A capped oil-equity allocation capitalizes INDHC.
4. INDHC invests in domestic subsidiaries, infrastructure, green assets,
   industrial parks, tourism services, rail, and raw-material processing.
5. Projects generate operating revenue.
6. Gross-profit levy and taxes fund Treasury and ministries.
7. Retained earnings fund maintenance, reinvestment, debt reduction, workforce,
   R&D, and dividend stabilization.
8. Remaining distributable surplus becomes equal monthly Digital IQD dividends.

Economic purpose: oil becomes productive capital before it becomes household
income or ministry budget.

#### 2. Domestic Production Loop

1. INDHC and private firms invest in domestic capacity.
2. Digital IQD spend constraints, merchant tiers, procurement rules, and
   local-content attestations steer demand toward verified Iraqi production.
3. Priority sectors include food substitution, raw-material processing,
   electronics, HVAC, desalination, irrigation, and regulated defense supply.
4. Domestic producers earn revenue and build transaction histories.
5. Banks lend against verified cash flow and receivables.
6. Producers scale output, employ Iraqi workers, and reduce import leakage.
7. Higher domestic sales increase tax/levy revenue and citizen dividend capacity.

Economic purpose: demand, credit, and industrial policy point in the same
direction instead of fighting each other.

#### 2A. Strategic Resilience Loop

1. INDHC identifies critical import dependencies in defense-controlled supplies,
   electronics, HVAC, water/desalination, irrigation, and staple foods.
2. Public procurement, merchant tiers, service contracts, and project milestones
   create lawful anchor demand for Iraqi production.
3. Foreign vendors can participate only through technology transfer, Iraqi
   counterpart training, open interfaces where possible, and audited handover.
4. Defense and dual-use production is governed by statutory authority, licensing,
   classification boundaries, end-use controls, and audit proofs.
5. Domestic capability reduces emergency import exposure and improves the cost
   base of housing, hospitals, schools, rail, agriculture, tourism, and industry.

Economic purpose: national resilience becomes a profitable industrial program,
not a hidden subsidy or opaque procurement channel.

#### 3. Citizen Income Loop

1. Citizens earn wages from private firms, SMEs, INDHC subsidiaries, tourism,
   rail, green projects, and public-service work.
2. Citizens receive public transfers and equal INDHC dividends in Digital IQD.
3. Citizens spend at merchants, individual producers, services, transport, and
   housing-related suppliers.
4. Transaction histories improve credit access for households and microbusinesses.
5. Spending data, bounded by privacy rules, informs policy and production
   planning.

Economic purpose: citizens benefit through wages, ownership income, services,
credit access, and better local supply.

#### 4. Ministry Feedback Loop

1. Ministries receive budgets from Treasury revenue, gross-profit levy, and
   priced service contracts rather than direct raw oil allocation.
2. Service contracts specify outputs: roads maintained, licenses processed,
   inspections completed, health delivered, education outcomes, utility uptime.
3. Cylinder Seal records disbursement constraints, milestone evidence, and audit
   trails.
4. Better ministry performance improves infrastructure, licensing, courts,
   training, safety, and services.
5. Better public services raise private/INDHC productivity, taxable surplus, and
   citizen dividends.

Economic purpose: ministries become part of the production function instead of
being insulated from economic outcomes.

#### 5. Credit And Formalization Loop

1. Cash transactions move into Digital IQD.
2. Wallet/POS histories create verified revenue, expenses, inventory, repayment,
   and income-stability features.
3. Banks and public credit programs lend using cash-flow evidence instead of
   only collateral.
4. SMEs and individual producers formalize gradually through lower-friction
   registration, micro-tax, and graduation thresholds.
5. Formalization expands the tax base without crushing small operators.

Economic purpose: visibility becomes bankability, not just surveillance.

#### 6. FX, Tourism, And Export Loop

1. Tourism platforms, diaspora merchants, and service exporters accept foreign
   currency into formal channels.
2. CBI and bank rails convert or settle foreign earnings into Digital IQD-linked
   domestic payments.
3. Local providers receive Digital IQD and build taxable, bankable histories.
4. Export-capable sectors scale from domestic substitution into regional and
   diaspora demand.
5. Foreign currency inflows reduce dependence on crude oil exports.

Economic purpose: Iraq earns foreign currency by selling Iraqi goods, services,
tourism, and culture, not only crude oil.

#### 7. Green And Rail Cost-Reduction Loop

1. International green capital and oil equity finance solar, storage, grid,
   waste-to-energy, efficiency, and metro/light-metro assets.
2. Rail and green power lower congestion, fuel burn, logistics friction, and
   industrial energy risk.
3. Lower cost infrastructure improves domestic production competitiveness.
4. Domestic industry supplies more rail, power, building, and maintenance inputs.
5. The system reinvests savings into further infrastructure and industry.

Economic purpose: infrastructure is not a prestige expense; it is a cost base
reduction engine for the whole economy.

#### 8. Civic Work And Productivity Transition Loop

1. Industrial champions, Digital IQD formalization, automation, and ministry
   transition raise productivity.
2. Some low-value admin, informal middleman, logistics, and patronage roles
   shrink.
3. Treasury, municipalities, climate programs, and approved INDHC community
   budgets fund verified civic work.
4. Citizens earn Digital IQD civic wages and credits for environmental,
   municipal, care, sport, culture, education, food-security, and disaster
   resilience tasks.
5. Task evidence creates civic reputation, training records, and income history.
6. Banks, SMEs, INDHC subsidiaries, municipalities, schools, and NGOs can use
   privacy-bounded records to offer apprenticeships, credit, or formal jobs.

Economic purpose: productivity gains become socially legitimate because spare
labor capacity is converted into paid public value, skills, and employability.
This is the model's participation-economy layer: it complements welfare and
dividends without disguising unemployment as permanent fake jobs.

### Accounting Spine

The model should be auditable with explicit accounts.

#### Public-Finance Identity

```text
Treasury Revenue
  = GrossProfitLevy(INDHC subsidiaries)
  + Taxes(private firms and citizens)
  + ServiceFees(where lawful)
  + Other non-oil revenue

Ministry Budget Capacity
  = Treasury Revenue
  - Debt service owed by the state
  - Statutory reserves
  - Legally protected transfers
```

Policy meaning: ministry budgets rise when the productive economy rises, not
just when oil receipts rise.

#### INDHC Operating Identity

```text
INDHC Distributable Surplus
  = Operating Revenue
  - Operating Costs
  - Maintenance Reserve
  - Project Debt Service
  - GrossProfitLevy
  - Required Reinvestment
  - Dividend Stabilization Reserve
```

Policy meaning: dividends come from audited surplus, not raw oil.

#### Citizen Income Identity

```text
Citizen Digital Income
  = Wages
  + SME / IP business income
  + Public transfers
  + Civic wage / civic credit
  + INDHC dividend
  + Credit disbursements
  - Taxes / fees / repayments
```

Policy meaning: the citizen sees the system as income, services, credit, and
ownership, not as an abstract institutional reform.

#### Import Leakage Identity

```text
Import Leakage Reduction
  = Domestic substitution in selected value chains
  + Domestic raw-material processing
  + Domestic tourism/service capture
  + Domestic infrastructure inputs
  - Imported machinery and transition inputs
```

Policy meaning: import substitution is credible only when it beats imports on
delivered cost, reliability, and quality after the transition period.

#### INDHC Cashflow Waterfall

```text
Oil equity + project debt
  -> capex by sector
  -> operating revenue
  -> operating costs and maintenance
  -> debt service
  -> gross-profit levy / tax
  -> retained earnings and reserves
  -> monthly citizen dividend
```

Policy meaning: every dinar has a job before dividends are calculated. The
ten-year plan includes the detailed sector cashflow and sensitivity model.

### Policy Controls

| Control | Economic function |
| --- | --- |
| Merchant tiers | Lower fees and higher eligibility for verified domestic-content merchants. |
| Spend constraints | Prevent public transfers or project funds from leaking into excluded categories where domestic supply exists. |
| Expiring transfers | Increase velocity for targeted stimulus and prevent idle subsidy balances. |
| Conditional release | Tie construction, rail, green, and ministry payments to evidence and milestones. |
| Transaction-based credit | Convert payment history into working-capital access. |
| Oil Income Lockbox | Stops raw oil receipts from bypassing capital, reserve, levy, and dividend rules. |
| Gross-profit levy | Funds ministries from productive surplus. |
| Retained earnings allocation | Forces reinvestment, maintenance, debt reduction, training, and dividend stabilization before distribution. |
| Strategic-sector controls | Keep defense, dual-use, water, food, and critical electronics programs legal, licensed, auditable, and protected from procurement abuse. |
| Industrial champion gates | Make demand, credit, procurement preference, and Tier 1 privileges conditional on local content, price discipline, export progress, debt safety, and competition review. |
| Civic work verification | Pays civic wages only after task evidence, verifier checks, audit rules, and appeal paths. |
| Ministry deprecation gates | Prevent service loss by requiring legal authority, staff transition, audit, service metrics, and citizen appeals before ministry form changes. |
| Public dashboards | Creates citizen, parliamentary, and audit feedback. |
| Iraqi-only permanent staffing | Converts investment into national capability instead of permanent dependency. |

### Balance-Sheet View

| Balance sheet | Assets | Liabilities / claims | Performance signal |
| --- | --- | --- | --- |
| Citizen | Wallet balance, non-saleable INDHC entitlement, transaction history, skills | Taxes, loan repayments, household obligations | Income stability, dividend receipt, access to services and credit. |
| Civic worker | Civic wallet, work history, certificates, reputation, income record | Task obligations, safety rules, tax/benefit interactions | Verified hours, task quality, bridge-to-work progress. |
| SME / individual producer | Inventory, receivables, equipment, transaction history | Supplier payables, working-capital loans, tax obligations | Sales growth, repayment history, local-content score. |
| INDHC subsidiary | Plants, rail assets, hotels, grid assets, IP, cash | Project debt, supplier payables, levy obligations | ROIC, uptime, maintenance, local employment, dividend capacity. |
| Treasury | Levy/tax revenue, service-fee revenue | Ministry budgets, public debt, statutory transfers | Non-oil revenue share, budget resilience, service outcomes. |
| CBI / payment system | Settlement ledger, aggregate analytics, policy tools | Digital IQD liabilities and supervisory duties | Monetary visibility, payment reliability, privacy compliance. |
| Banks | Loans, deposits, collateral claims | Deposits, wholesale funding, risk provisions | SME lending growth, NPLs, cash-flow underwriting quality. |

### Implementation Model In Cylinder Seal

The current codebase should evolve toward a unified projection model rather than
separate dashboard widgets.

| Projection | Inputs | Outputs |
| --- | --- | --- |
| `EconomicCycleProjection` | Oil receipts, INDHC allocations, project revenues, taxes, dividends, domestic spend | National feedback-loop view. |
| `ProductionCapacityProjection` | Merchant tiers, local-content attestations, project outputs, inventory, procurement | Import substitution, food substitution, and domestic supply view. |
| `StrategicResilienceProjection` | Defense-sector licenses, electronics/HVAC output, water/desalination equipment, irrigation systems, food-substitution capacity, end-use controls | Critical domestic capability and import-vulnerability view. |
| `IndustrialChampionProjection` | Champion registry, conditional demand contracts, tier privileges, debt caps, export discipline, related-party exposure, SME inclusion | Anti-capture and sectoral champion performance view. |
| `CivicWorkProjection` | Civic tasks, workers, verifiers, evidence bundles, payments, training, impact metrics, appeals | Productivity-transition, social cohesion, and verified public-value view. |
| `CitizenIncomeProjection` | Wages, transfers, dividends, IP income, repayments, spending | Household welfare and inclusion view. |
| `MinistryPerformanceProjection` | Budgets, service contracts, milestones, delivery evidence | Ministry feedback and productivity view. |
| `CreditExpansionProjection` | Transaction histories, risk features, loan disbursements, repayments | SME finance and formalization view. |
| `ForeignCurrencyProjection` | Tourism, diaspora merchant sales, exports, imports, FX conversion | Non-oil FX and leakage view. |
| `GreenRailCostProjection` | Rail ridership, logistics costs, grid losses, energy costs, emissions | Infrastructure cost-base view. |
| `DividendSustainabilityProjection` | Surplus, reserves, debt service, citizen eligibility, exception queue | Monthly dividend reliability view. |

### Dashboard Design

The unified dashboard should answer ten questions:

1. Where did oil income go?
2. Which investments are producing cash, jobs, and domestic supply?
3. Which imports are being replaced credibly and at what cost?
4. Which critical sectors remain import-vulnerable: defense supply, electronics,
   HVAC, water/desalination, irrigation, and food staples?
5. Which ministries are improving economic productivity?
6. Which citizens, civic workers, and producers are becoming more bankable?
7. Which green and rail assets are lowering system costs?
8. How much non-oil foreign currency is being captured?
9. Is the citizen dividend funded by real surplus?
10. Is productivity displacement being absorbed into verified civic work,
    training, and bridge-to-work pathways?
11. Are capex, debt service, retained earnings, and dividends consistent with the
    ten-year cashflow model?

### Failure Modes The Model Must Surface

| Failure mode | Early warning signal |
| --- | --- |
| INDHC becomes another ministry | Rising payroll share, weak subsidiary accounts, poor ROIC, opaque transfers. |
| Import substitution becomes protectionism | Domestic prices exceed import parity without quality improvement or learning curve. |
| Defense manufacturing becomes opaque patronage | Weak statutory authority, hidden procurement, poor end-use controls, or classified spending outside audit boundaries. |
| Food/water substitution fails | Desalination, irrigation, cold-chain, or food projects lack unit-cost discipline, maintenance plans, or farmer/SME adoption. |
| Industrial champions become protected monopolies | Tier privileges persist despite weak exports, high prices, related-party abuse, debt stress, or SME crowd-out. |
| Civic work becomes fake jobs | Payments rise while verified outputs, training, audits, and bridge-to-work outcomes remain weak. |
| Dividends become oil handouts | Dividend pool tracks oil receipts instead of audited surplus. |
| Ministries resist feedback | Service contracts lack outputs, milestone evidence, or public reporting. |
| Credit becomes political lending | Weak repayment, concentrated borrowers, override-heavy approvals. |
| Green capital becomes branding | Weak use-of-proceeds evidence, no verified output, no cost reduction. |
| Rail becomes prestige construction | Low ridership, poor maintenance, closed vendor dependency, weak city integration. |
| Iraqi-only staffing becomes patronage | Hiring not merit-based, weak training outcomes, leadership captured by factions. |

### What This Unifies

The model gives each existing Cylinder Seal component a place:

- **Digital IQD** is the transaction and settlement substrate.
- **Policy primitives** are the programmable controls.
- **AML and audit** are the legitimacy layer.
- **Credit scoring** converts visibility into finance.
- **Merchant tiers and spend constraints** steer demand toward domestic supply.
- **INDHC** turns oil and debt into productive assets.
- **The ten-year plan** defines where capital goes.
- **Industrial champion gates** prevent the INDHC model from becoming a
  protected monopoly system.
- **Strategic resilience manufacturing** covers defense-controlled supply,
  electronics, HVAC, water/desalination, irrigation, and food substitution.
- **Ministry service contracts** create state-performance feedback.
- **Civic work** converts productivity displacement into paid public value,
  training, care, climate resilience, sport, culture, and municipal repair.
- **Tourism and exports** bring in non-oil foreign currency.
- **Green and rail investments** lower the national cost base.
- **Citizen dividends** distribute capital returns in a post-automation economy.

### Build Sequence

1. Define the unified ledger projection tables and event taxonomy.
2. Add `InvestmentPlan`, `CapitalStack`, `ProjectMilestone`,
   `GrossProfitLevy`, `RetainedEarningsAllocation`, and
   `DividendDistribution` models.
3. Add `EconomicCycleProjection` and `CitizenIncomeProjection` dashboard views.
4. Add `ProductionCapacityProjection` for import substitution and local content.
5. Add `StrategicResilienceProjection` for defense-controlled supply,
   electronics, HVAC, water/desalination, irrigation, and food substitution.
6. Add `MinistryPerformanceProjection` for service contracts and budget feedback.
7. Add `CivicWorkProjection` for verified tasks, payments, training, impact,
   audits, and bridge-to-work outcomes.
8. Add `ForeignCurrencyProjection` for tourism, diaspora channels, and exports.
9. Add `GreenRailCostProjection` for rail, grid, energy, and logistics effects.
10. Gate all public claims behind source labels, model assumptions, and confidence
   levels from `docs/economic-assumptions.md`.

### Bottom Line

Cylinder Seal should be read as one economic operating system:

```text
Oil and international capital
  -> productive Iraqi assets
  -> domestic supply, strategic resilience, jobs, services, green power, rail, tourism, exports
  -> taxable surplus and lower import leakage
  -> ministry budgets, civic work, reinvestment, credit expansion, citizen dividends
  -> more visible demand and stronger domestic production
```

The point is not only to digitize money. The point is to make Iraq's economic
feedback loops visible, governable, productive, and citizen-owned.


# Part 8: National Dividend Holding Company

## National Dividend Holding Company Architecture

This document adds a policy architecture to Cylinder Seal: Iraq's oil income is
not treated as a direct ministry-funding stream. It is treated as national
productive capital held through a state development holding company whose
beneficial shareholders are Iraqi citizens.

Status: policy architecture proposal. It is not legal advice, not an investment
product, and not a production-ready public-finance design.

This document is one component of the broader
[Unified Economic Model](unified-economic-model.md), which connects the Oil
Income Lockbox, INDHC, Digital IQD, ministries, banks, producers, infrastructure,
tourism, credit, reinvestment, and citizen dividends into one accounting loop.

### Rationale

Iraq's public finance problem is not only that the state receives oil income. It
is that oil income can fund ministries directly even when the broader economy is
weak. That weakens the feedback loop between ministry performance, productive
investment, private-sector growth, tax base growth, and citizen welfare.

Cylinder Seal can model a different loop:

1. Oil income becomes citizen-owned productive capital.
2. Productive capital invests in infrastructure, industrial capacity, and
   service-sector platforms.
3. Gross profit creates a taxable base for the state.
4. Ministries are funded from explicit taxes, service contracts, and performance
   budgets rather than automatic oil allocation.
5. Remaining distributable profit is paid as an equal monthly citizen dividend
   through Digital IQD wallets.

The objective is a post-automation social contract: if AI and automation reduce
the labor share of income, citizens still participate in national capital
returns.

Capital income is only one side of that social contract. The companion
[National Civic Work System](national-civic-work-system.md) converts spare labor
capacity into paid, verified public value, training, and bridge-to-work records.
Dividends should not be used to hide unemployment, and civic wages should not be
funded by quietly raiding the dividend pool.

### Source Discipline

| Public fact | Why it matters | Source |
| --- | --- | --- |
| Iraq is heavily dependent on oil income, with oil accounting for a very large share of government revenue. | Direct oil-to-budget dependence is the problem this architecture tries to rewire. | [EITI Iraq country page](https://eiti.org/countries/iraq), [EIA Iraq analysis](https://www.eia.gov/international/analysis/country/irq) |
| IMF staff have highlighted Iraq's rigid fiscal spending, subdued non-oil revenues, and vulnerability to lower oil prices. | Ministry funding should not be structurally insulated from non-oil economic performance. | [IMF Iraq 2025 Article IV](https://www.imf.org/-/media/files/publications/cr/2025/english/1irqea2025001-source-pdf.pdf), [IMF Iraq 2024 Article IV](https://www.imf.org/en/publications/cr/issues/2024/05/15/iraq-2024-article-iv-consultation-press-release-staff-report-and-statement-by-the-executive-549028) |
| World Bank analysis has examined oil-revenue management options and notes that allocating oil revenue to public capital can have the strongest non-oil GDP effect, while public-sector pay allocation can distort the traded-goods sector. | The architecture prioritizes productive investment over direct consumption or ministry payroll expansion. | [World Bank, Iraq oil revenue management for economic diversification](https://documents1.worldbank.org/curated/en/669171643036848080/pdf/Iraq-Oil-revenue-management-for-economic-diversification.pdf) |

These sources support the problem framing. They do not validate the proposed
institutional design.

### Institutional Design

Working name: **Iraq National Dividend Holding Company (INDHC)**.

Alternative local branding can use "People's Development Holding Company" or
"Citizen Development Holding Company." The industrial holding-group analogy is
useful, but the proposed institution must not reproduce opaque family control,
related-party abuse, or protected conglomerate behavior.

For the industrial operating model, use
[Digitally Governed Industrial Champions](digitally-governed-industrial-champions.md)
rather than the shorthand "chaebol." The intended lesson is coordinated scale,
export discipline, and technology absorption, not family-controlled monopolies
or permanent protection.

#### Ownership

- Every eligible Iraqi citizen receives one equal base share class.
- Shares are non-saleable, non-pledgeable, and non-transferable except through
  inheritance to eligible descendants.
- A citizen's base share is a beneficial entitlement, not a speculative token.
- New citizens and births require a statutory issuance rule.
- Deceased citizens without eligible heirs revert their entitlement to a social
  reserve pool.
- No ministry, party, militia, bank, or private holding company can acquire
  citizen base shares.

#### Cash Formalization Window

The policy question is how to transition legacy physical cash into the formal
Digital IQD system without creating an amnesty for illicit proceeds.

For credibility and AML compliance, Cylinder Seal should model the transition
as a **time-limited cash formalization and demonetization window**, not
anonymous cash laundering:

- Window length: 12 months from legal launch.
- Cash can be deposited into supervised conversion points and recorded in
  Cylinder Seal.
- Deposits generate locked supplemental shares or a capped transition balance,
  not freely tradable assets.
- Every deposit creates a signed receipt, identity link, amount, location,
  operator, risk score, and audit trail.
- Large or suspicious deposits enter enhanced due diligence and may be held,
  rejected, or referred.
- Politically exposed persons, sanctioned parties, public officials, and
  high-risk entities receive stricter limits and manual review.
- Conversion can include haircuts, caps, tax settlement, or quarantine periods.
- After the window, physical cash is no longer eligible for share conversion and
  may be demonetized or made non-redeemable only through explicit monetary law.

The goal is to pull cash into the formal ledger while avoiding an amnesty for
theft, sanctions evasion, terrorism finance, or corruption proceeds.

#### Revenue Waterfall

1. Oil export receipts flow to a monitored Oil Income Lockbox.
2. A statutory stabilization reserve absorbs oil-price volatility.
3. A capital allocation mandate funds INDHC infrastructure, industrial, and
   service-sector subsidiaries.
4. Subsidiaries earn operating revenue.
5. A gross-profit levy or tax funds the Treasury and ministry budgets.
6. Approved reinvestment and reserves are retained inside INDHC.
7. Remaining distributable profit is paid monthly as an equal Digital IQD
   dividend to citizen wallets.

This is not the same as directly distributing oil revenue. The dividend is tied
to productive surplus after investment, taxation, reserve policy, and audit.

### Ministry Feedback Mechanism

The core governance change is that ministries stop being direct claimants on
raw oil receipts.

Ministries instead receive:

- Statutory budgets funded from taxes and levies on actual productive activity.
- Service-contract payments for outcomes delivered to INDHC projects or citizens.
- Performance-based capital budgets tied to project completion, maintenance,
  uptime, education outcomes, health delivery, or regulatory throughput.
- Public dashboards showing cost, output, time-to-delivery, and citizen impact.

If the non-oil economy weakens, ministry funding pressure becomes visible. If
ministries improve infrastructure, licensing, courts, training, logistics, and
utilities, the taxable base and citizen dividend improve.

### Cylinder Seal System Role

Cylinder Seal is the transaction, identity, audit, and dividend-distribution
layer. It does not need to become the legal owner of assets. It should provide
the ledger and policy controls that make the model auditable.

Within the policy architecture, this is a core public-finance workflow rather
than a side appendix. The Digital IQD rail is the payment substrate; the holding
company model is the fiscal feedback loop that connects oil receipts,
productive investment, ministry funding, and citizen capital income.

| Capability | Cylinder Seal role |
| --- | --- |
| Citizen shareholder registry | Non-transferable share-entitlement ledger tied to national identity and inheritance rules. |
| Oil income lockbox | Digital receipt trail from export proceeds to stabilization reserve, INDHC capital account, Treasury levy, and dividend pool. |
| Cash formalization window | Deposit receipts, KYC, risk scoring, caps, holds, conversion records, and post-window rejection. |
| Investment allocations | Project-level disbursements with earmarked-spend constraints and conditional-release escrow. |
| Ministry funding | Treasury levy receipts and performance-linked service payments. |
| Monthly dividend | Equal Digital IQD dividend distribution to citizen wallets. |
| Anti-corruption controls | Immutable audit target, public aggregate dashboards, role-gated interventions, and suspicious deposit reports. |
| Post-automation income | A recurring citizen capital dividend independent of formal wage employment. |

### Flow Combination Matrix

| Flow | Sender or source | Receiver or sink | Instrument | Controls | Use case |
| --- | --- | --- | --- | --- | --- |
| Oil receipt | Export proceeds / SOMO-equivalent record | Oil Income Lockbox | `OilReceipt` | Signature, source reconciliation, audit hash | Stops raw oil receipts from bypassing the allocation rulebook. |
| Stabilization allocation | Oil Income Lockbox | Stabilization reserve | Reserve transfer | Formula, board approval, public aggregate | Smooths oil-price shocks before budgets or dividends are calculated. |
| Productive capital allocation | Lockbox / INDHC capital account | Subsidiary or project | `CapitalAllocation` | Earmarked spend, conditional release, procurement audit | Funds infrastructure, industrial, housing, logistics, and service assets. |
| Project procurement | Subsidiary or project account | Contractor / supplier | Spend-constrained Digital IQD | Domestic-content tiering, invoice evidence, beneficial-owner checks | Directs investment into verified productive supply chains. |
| Operating revenue | Customers / users / offtakers | INDHC subsidiary | Digital IQD or bank-settled receipt | Tax and audit projection | Measures actual productive performance rather than budget consumption. |
| Gross-profit levy | INDHC subsidiary | Treasury | `GrossProfitLevy` | Audited accounts, formula, dispute window | Funds ministries from explicit taxable surplus. |
| Ministry service payment | Treasury or INDHC | Ministry or service contractor | Conditional release | Output milestone, inspector sign-off, public dashboard | Pays ministries for delivery instead of automatic oil draw. |
| Retained earnings | INDHC subsidiary | INDHC reserve / reinvestment pool | Retention entry | Board policy, capital plan, audit | Preserves capital for maintenance, expansion, and resilience. |
| Monthly dividend | Dividend pool | Citizen wallets | `DividendDistribution` | Eligibility snapshot, duplicate check, exception queue | Shares audited distributable surplus equally with citizens. |
| Cash conversion accepted | Citizen physical cash deposit | Locked supplemental entitlement or transition balance | `CashConversionReceipt` | KYC, caps, source-of-funds risk, receipt | Pulls cash into the formal system during the one-year window. |
| Cash conversion held | High-risk cash deposit | Quarantine account | Held receipt | EDD, PEP/sanctions review, referral | Prevents the cash window from laundering corruption or illicit funds. |
| Cash conversion rejected | Ineligible or post-window cash | No conversion | Rejection receipt | Appeal path, operator audit | Enforces the demonetization boundary. |
| Inheritance transfer | Deceased citizen entitlement | Eligible heir or social reserve | `InheritanceTransfer` | Civil registry proof, dispute window, court order where needed | Allows intergenerational continuity without saleable shares. |
| Correction / appeal | Citizen, auditor, or court | Corrected ledger state | Adjustment entry | Four-eyes approval, immutable reason code | Repairs identity, inheritance, dividend, or cash-window errors. |
| Public aggregate disclosure | Cylinder Seal analytics | Public dashboard / parliament / auditors | Aggregate report | Privacy thresholding, audit trail | Lets citizens see oil receipts, allocations, levy, and dividend performance. |

### Required Transaction Primitives

| Primitive | Description | Status |
| --- | --- | --- |
| `ShareEntitlement` | Non-transferable citizen beneficial share record. Inheritance transfer only. | New design primitive. |
| `OilReceipt` | Signed oil-income receipt entering the Oil Income Lockbox. | New design primitive. |
| `CapitalAllocation` | INDHC allocation to a subsidiary, infrastructure project, industrial project, or service platform. | Can reuse earmarked spend and conditional release. |
| `GrossProfitLevy` | Treasury claim on productive operating surplus. | New accounting primitive. |
| `DividendDistribution` | Monthly equal dividend to all eligible citizen wallets. | New distribution primitive. |
| `CashConversionReceipt` | Time-limited cash deposit record with KYC, risk score, cap, hold, and conversion status. | New design primitive. |
| `InheritanceTransfer` | Legally approved movement of share entitlement to eligible offspring or heirs. | New design primitive. |

### Financial Flows

#### Oil Income To Dividend

1. SOMO/export receipt or equivalent oil-income record is signed.
2. Receipt enters the Oil Income Lockbox.
3. Stabilization reserve allocation is calculated.
4. INDHC investment capital is allocated to projects and subsidiaries.
5. Subsidiary profits are measured.
6. Gross-profit tax or levy funds the Treasury.
7. Retained earnings fund reinvestment and reserves.
8. Dividend pool is distributed monthly to citizen wallets.

#### Cash Formalization

1. Citizen brings physical cash during the 12-month window.
2. Operator verifies identity and records cash amount.
3. Cylinder Seal runs risk checks and applies caps, holds, or EDD.
4. Accepted amount becomes locked supplemental entitlement or transition balance.
5. Suspicious amount is held or referred.
6. After the window, physical cash is no longer accepted for conversion.

#### Ministry Funding

1. Ministry proposes or receives a service mandate.
2. Budget is tied to tax/levy revenue, service contract, or performance milestone.
3. Cylinder Seal records disbursement constraints.
4. Delivery evidence triggers payment.
5. Poor performance becomes visible in the public dashboard and affects future
   allocations.

### Governance Guardrails

- Constitutional or statutory basis for redirecting oil receipts.
- Independent board with citizen, parliamentary, CBI, audit, and technical
  representation.
- No political-party control over investment allocation.
- Public project register and beneficial-ownership disclosure for contractors.
- External audit and parliamentary review.
- Conflict-of-interest rules for ministers, board members, banks, contractors,
  and operators.
- Clear dividend formula and reserve policy.
- Explicit ban on borrowing against citizen base shares.
- Recourse process for mistaken identity, inheritance disputes, cash holds, and
  dividend errors.

### Risks

| Risk | Mitigation |
| --- | --- |
| INDHC becomes a politicized monopoly. | Independent board, public audit, procurement transparency, competition rules, project-level performance dashboards, sectoral champion groups rather than one mega-conglomerate, and automatic loss of privileges after performance failure. |
| Cash window becomes corruption laundering. | KYC, caps, EDD, holds, haircuts, sanctions screening, PEP restrictions, and law-enforcement referral. |
| Ministries resist losing direct oil allocations. | Statutory transition, service contracts, performance budgets, public dashboards. |
| Dividend becomes fiscally pro-cyclical. | Stabilization reserve and dividend formula based on audited distributable surplus, not raw oil price. |
| Citizens treat entitlement as speculative property. | Non-saleable, non-pledgeable base shares; inheritance-only transfers. |
| Automation gains concentrate in subsidiaries. | Equal monthly dividend plus open procurement and SME participation requirements. |

### What To Build First

1. Add a `ShareEntitlement` registry model and migration.
2. Add a `DividendDistribution` batch model and route-level tests.
3. Add a `CashConversionReceipt` model with risk states: `accepted`, `held`,
   `rejected`, `referred`.
4. Add an Oil Income Lockbox projection and dashboard.
5. Add a public aggregate dividend dashboard.
6. Add governance and legal review notes before any implementation is described
   as deployable.

See [INDHC ten-year industrial and infrastructure plan](indhc-10-year-plan.md)
for the proposed investment program, capital stack, staffing model, sector
priorities, reinvestment waterfall, and implementation primitives.


# Part 9: INDHC Ten-Year Plan

## INDHC Ten-Year Industrial And Infrastructure Plan

This document translates the National Dividend Holding Company architecture into
a ten-year investment plan. It is a planning scenario, not a budget law, a
procurement package, or a validated macroeconomic forecast.

The premise is simple: oil income should stop acting as an automatic ministry
funding tap. A defined portion of oil income becomes equity capital in the Iraq
National Dividend Holding Company (INDHC). INDHC then raises additional debt only
for bankable projects, builds profitable Iraqi businesses, taxes gross profit to
fund the state, reinvests into domestic industry and infrastructure, and pays
the remaining distributable surplus as monthly Digital IQD dividends.

This plan sits inside the [Unified Economic Model](unified-economic-model.md).
The ten-year investments are the productive-asset side of the model; Digital IQD
is the evidence and distribution rail; ministry funding, credit, taxes,
reinvestment, and dividends are the feedback mechanisms.
See [National Civic Work System](national-civic-work-system.md) for the
transition layer that turns productivity gains and displaced low-productivity
labor into paid civic work, training, care, environmental restoration, sport,
culture, municipal repair, and bridge-to-work records.

See [Ministry Transition And Deprecation Roadmap](ministry-transition-roadmap.md)
for the staged plan to move duplicative ministry functions into regulators,
service contracts, municipalities, INDHC subsidiaries, or sunset agencies.
See [Digitally Governed Industrial Champions](digitally-governed-industrial-champions.md)
for the anti-capture model governing sectoral champion groups, conditional
demand, conditional credit, export discipline, competition gates, and debt caps.

### Source Discipline

| Planning fact | Use in this plan | Source |
| --- | --- | --- |
| Iraq's National Development Plan 2024-2028 emphasizes infrastructure linked to agriculture, industry, and tourism. | The plan aligns investment sectors with Iraq's official development frame rather than creating a parallel agenda. | [National Development Plan 2024-2028 PDF](https://www.undp.org/sites/g/files/zskgke326/files/2024-12/national-development-plan-2024-2028.pdf) |
| Iraq remains heavily oil-revenue-dependent. | INDHC is designed to convert part of oil revenue into productive capital and ministry-funding feedback. | [EIA Iraq analysis](https://www.eia.gov/international/analysis/country/irq), [EITI Iraq](https://eiti.org/countries/iraq) |
| Iraq's electricity ministry plan includes more than 12,000 MW of solar by 2030, plus wind and waste-to-energy ambitions. | Green technology investment and international green capital raising are core pillars. | [IRENA Energy Transition Assessment: Iraq 2025](https://www.irena.org/-/media/Files/IRENA/Agency/Publication/2025/Jul/IRENA_COU_Energy_transition_assessment_Iraq_2025.pdf) |
| The Baghdad Metro RFI describes a 148 km, 64-station system with automated trains. | The open rail program starts with Baghdad, then standardizes city metro/light-metro delivery across Iraq. | [National Investment Commission Baghdad Metro RFI, Feb. 20, 2024](https://investpromo.gov.iq/wp-content/uploads/2024/03/3.-RFI-English-%D9%86%D8%A8%D8%B0%D8%A9-%D8%B9%D9%86-%D9%85%D8%AA%D8%B1%D9%88-%D8%A8%D8%BA%D8%AF%D8%A7%D8%AF-E-20.2.2024.pdf) |
| World Bank energy work identifies gas, fertilizers, petrochemicals, steel, aluminum, cement, and bricks as downstream opportunities tied to Iraq's raw materials and domestic reconstruction needs. | Raw-material post-processing and import substitution are treated as commercial anchors. | [World Bank Iraq Energy Sector Summary](https://documents1.worldbank.org/curated/en/406941467995791680/txt/105893-WP-PUBLIC-INES-Summary-Final-Report-VF.txt) |
| Iraq's NDC links climate action to national development, energy, industry, water, agriculture, and just transition priorities. | Green investment is treated as industrial policy, not only climate compliance. | [Iraq NDC 3.0, 2025](https://unfccc.int/sites/default/files/2026-01/NDC%20Report%20EN%20-%202025.pdf) |
| Iraq's 2024 import basket includes large values for rice, air conditioners, packaged medicaments, cars, and other finished goods. | Food substitution, HVAC/electronics assembly, medical inputs, and parts localization are credible target screens. | [OEC Iraq profile](https://oec.world/en/profile/country/irq) |
| FAO GIEWS reports Iraq's wheat import requirements are still material even when domestic harvests are strong. | Food substitution should focus on storage, milling, feed, irrigation, productivity, and staples rather than pretending imports disappear immediately. | [FAO GIEWS Iraq country brief archive](https://www.fao.org/giews/countrybrief/country/IRQ/pdf_archive/IRQ_Archive.pdf) |
| World Bank analysis warns that lower water supply and crop-yield impacts could materially reduce Iraq's GDP. | Desalination, irrigation, water treatment, leakage reduction, and water-efficiency manufacturing are treated as economic infrastructure. | [World Bank Iraq Economic Monitor press release, Nov. 24, 2021](https://www.worldbank.org/en/news/press-release/2021/11/24/iraq-rising-fiscal-risks-water-scarcity-and-climate-change-threaten-gradual-recovery-from-pandemic) |
| Defense expenditure and arms-import indicators are tracked by SIPRI and World Bank datasets. | Defense manufacturing is framed as regulated domestic sustainment and resilience, with legal and audit controls, not public technical weapons design. | [SIPRI Military Expenditure Database](https://www.sipri.org/databases/milex), [World Bank military expenditure indicators for Iraq](https://data.worldbank.org/indicator/MS.MIL.XPND.GD.ZS?locations=IQ) |
| Korea's large business groups helped scale development and exports, but also created concentration, governance, and competition risks. | INDHC should use sectoral industrial champions with anti-capture controls rather than a family-controlled chaebol model. | [OECD Korea large business groups paper](https://www.oecd.org/en/publications/reforming-the-large-business-groups-to-promote-productivity-and-inclusion-in-korea_9e9052b5-en.html) |
| Korea's export support system used export targets, credit allocation, technology acquisition, and marketing institutions. | INDHC privileges should be tied to export discipline, technology transfer, and performance monitoring. | [World Bank, Korea: A Case of Government-Led Development](https://documents1.worldbank.org/curated/en/441571468753249695/pdf/multi0page.pdf) |

These sources support sector selection. They do not validate the capital
envelope, dividend formula, debt capacity, or legal design.

### Strategic Objectives

INDHC exists to create a sovereign economic feedback loop:

1. Replace direct oil-to-ministry funding with oil-to-productive-capital funding.
2. Build profitable Iraqi companies that meet internal needs first.
3. Substitute imports where Iraq has a defensible cost, logistics, security,
   raw-material, water, food, or demand advantage.
4. Process Iraqi raw materials domestically before they are consumed or exported.
5. Build strategic domestic manufacturing in defense-controlled supply chains,
   electronics, HVAC, water/desalination, irrigation, and food staples.
6. Expand tourism and tradable services as non-oil revenue sources.
7. Build green power, green manufacturing, and climate-resilient infrastructure.
8. Use open rail standards to deliver metro and light-metro networks across Iraqi
   cities with domestic operating capability.
9. Employ Iraqi citizens as all permanent staff, at every level.
10. Reinvest retained earnings into domestic infrastructure and industry before
   surplus is distributed.
11. Pay monthly Digital IQD dividends from audited distributable surplus.

### Capital Stack

The ten-year base case uses a USD 190 billion planning envelope.

This is not a recommendation to borrow blindly. It is a stress-testable planning
envelope showing how oil equity and debt could combine without turning INDHC into
a payroll vehicle.

| Source | Ten-year base case | Use | Guardrail |
| --- | ---: | --- | --- |
| Oil-income equity capital | USD 120B | First-loss equity, domestic capital projects, strategic assets, training, early works | Set by statute as a capped share of oil receipts after stabilization allocation. |
| Concessional and MDB loans | USD 22B | Water, grid, climate adaptation, public transport, skills, governance systems | Only for projects with public-good value and transparent repayment source. |
| Green bonds / green sukuk | USD 20B | Solar, wind, storage, grid, waste-to-energy, energy efficiency | Certified use-of-proceeds, external verification, public project register. |
| Export-credit and supplier finance | USD 18B | Rail equipment, factories, grid equipment, industrial machinery | Requires technology transfer, Iraqi staff training, and open-interface procurement. |
| Project finance / PPP debt | USD 10B | Revenue assets with user fees, offtake contracts, industrial parks, ports, tourism | No hidden ministry bailout; project debt service coverage ratio above 1.30. |
| Local IQD infrastructure bonds | USD 0-10B equivalent | Optional domestic savings instrument after market validation | No forced bank purchases; CBI monetary-policy compatibility required. |

Base case totals USD 190B without counting optional local bonds. A conservative
case is USD 115B over ten years. An aggressive case is USD 260B, but only if
oil receipts, debt capacity, governance, and project delivery improve.

### Capital Allocation By Sector

| Sector | Base case allocation | Oil equity | Loans and project debt | Main return path |
| --- | ---: | ---: | ---: | --- |
| Strategic manufacturing, import substitution, electronics, HVAC, defense industrial base, and raw-material post-processing | USD 62B | USD 42B | USD 20B | Domestic sales, public procurement savings, export margin, gross-profit levy. |
| Open rail, metro, logistics, and intercity connections | USD 38B | USD 20B | USD 18B | Fares, land-value capture, service contracts, freight/logistics revenue. |
| Green technology, power, grid, and waste-to-energy | USD 32B | USD 16B | USD 16B | PPAs, industrial power sales, avoided fuel cost, green finance. |
| Tourism and tradable services | USD 20B | USD 13B | USD 7B | Visitor spending, hotel/platform revenue, service exports. |
| Agriculture, food substitution, water/desalination, irrigation, and cold chain | USD 22B | USD 14B | USD 8B | Domestic food sales, reduced imports, water-service revenue, agro-export margin. |
| Housing inputs and urban services | USD 8B | USD 7B | USD 1B | Materials sales, utility service contracts, municipal availability payments. |
| Digital public infrastructure and SME platforms | USD 5B | USD 5B | USD 0B | Transaction fees, analytics services, credit enablement, public savings. |
| Workforce, R&D, and industrial standards | USD 3B | USD 3B | USD 0B | Capability building; not expected to be a standalone profit center. |
| **Total** | **USD 190B** | **USD 120B** | **USD 70B** | Base-case planning envelope. |

### Ten-Year Phasing

| Year | Oil equity | Loans / project debt | Core work |
| --- | ---: | ---: | --- |
| 1 | USD 8B | USD 2B | Pass enabling law, create Oil Income Lockbox, appoint board, open public project registry, launch Iraqi-only permanent staffing rule, define defense-industrial legal controls, begin feasibility studies, start quick-rehab projects in cement, food processing, grid loss reduction, and tourism services. |
| 2 | USD 10B | USD 4B | Create first subsidiaries, launch INDHC Academy, start cash formalization systems, issue first green sukuk pilot, complete open rail reference architecture, begin Baghdad metro co-investment, and design electronics/HVAC/water-equipment procurement localization. |
| 3 | USD 11B | USD 5B | Start Basra industrial processing belt, fertilizer/gas-feedstock projects, construction-material network, solar procurement, regulated defense sustainment/protective-equipment lines, irrigation-equipment assembly, religious tourism logistics platform, and first domestic rail fabrication work packages. |
| 4 | USD 12B | USD 6B | Build first rail corridors, expand cement/brick/glass/pipes, launch agro-processing clusters, start Najaf-Karbala visitor services corridor, launch desalination and water-treatment equipment assembly, and move first ministry service payments onto performance contracts. |
| 5 | USD 12B | USD 7B | Reach first operating-profit cycle in quick-win subsidiaries, commission first large solar/storage batches, scale domestic steel/rebar capacity, expand electronics and HVAC assembly, publish first dividend formula stress test, and begin monthly pilot dividends from audited surplus. |
| 6 | USD 13B | USD 8B | Expand metro/light-metro delivery to Basra, Mosul, Najaf-Karbala, and Erbil/Sulaymaniyah subject to local agreements; scale petrochemical/plastics, fertilizer exports, food-staple substitution, and national tourism booking/payment rails. |
| 7 | USD 13B | USD 9B | Move from import substitution to export competition in selected products, expand grid and industrial parks, launch domestic battery-pack/smart-meter assembly, mature HVAC and electronics supplier networks, and deepen rail operations training. |
| 8 | USD 13B | USD 9B | Complete second wave of city transit networks, integrate cold chain and food logistics nationally, scale medical/pharma import substitution, expand desalination/irrigation manufacturing, and increase retained earnings reinvestment from operating subsidiaries. |
| 9 | USD 14B | USD 10B | Consolidate underperforming projects, list minority non-voting project bonds where appropriate, expand regional service exports, upgrade industrial parks to low-carbon export zones, and raise dividend stability reserve. |
| 10 | USD 14B | USD 10B | Shift from construction-heavy spending to renewal, maintenance, export growth, dividends, and debt reduction; publish ten-year audit, citizen return statement, and next ten-year plan. |

### Cashflow Model Rules

The tables below are a base-case planning model, not a forecast. They are
intended to make the economic model testable: reviewers can change capex,
revenue, margins, debt cost, delivery delays, import prices, or oil receipts and
see what happens to ministry funding, reinvestment, and citizen dividends.

Core rules:

- Oil equity funds strategic assets, feasibility work, early domestic
  capability, first-loss capital, and workforce formation.
- Loans fund only bankable assets with identifiable repayment sources.
- No debt proceeds fund dividends, ordinary ministry payroll, or loss cover.
- Revenue starts with quick-win rehabilitation and procurement substitution,
  then shifts toward commercial sales, service contracts, exports, and fares.
- Debt service is senior to dividends.
- Gross-profit levy is paid before dividends.
- Retained earnings protect maintenance, debt reduction, supplier upgrading,
  workforce training, and future investment.
- Dividends begin only after audited surplus exists and a stabilization reserve
  is funded.

### Ten-Year Consolidated Cashflow

Illustrative base case, USD billions.

| Year | New capex | Oil equity draw | Loan/debt draw | Operating revenue | Operating cash after maintenance | Debt service | Gross-profit levy / tax | Retained earnings | Dividend pool | Notes |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| 1 | 10.0 | 8.0 | 2.0 | 0.4 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | Legal setup, lockbox, project registry, quick rehab. |
| 2 | 14.0 | 10.0 | 4.0 | 1.2 | 0.1 | 0.1 | 0.0 | 0.0 | 0.0 | First subsidiaries and procurement localization. |
| 3 | 16.0 | 11.0 | 5.0 | 2.8 | 0.5 | 0.3 | 0.1 | 0.1 | 0.0 | Materials, food, irrigation, defense sustainment starts. |
| 4 | 18.0 | 12.0 | 6.0 | 5.2 | 1.1 | 0.6 | 0.2 | 0.3 | 0.0 | Water equipment, rail works, agro-processing. |
| 5 | 19.0 | 12.0 | 7.0 | 8.5 | 2.0 | 1.0 | 0.5 | 0.7 | 0.1 | First audited surplus; pilot dividend only. |
| 6 | 21.0 | 13.0 | 8.0 | 13.0 | 3.5 | 1.5 | 0.9 | 1.0 | 0.3 | Food, plastics, tourism, green power scale. |
| 7 | 22.0 | 13.0 | 9.0 | 18.5 | 5.4 | 2.1 | 1.4 | 1.6 | 0.6 | Electronics/HVAC and industrial parks mature. |
| 8 | 22.0 | 13.0 | 9.0 | 25.0 | 7.7 | 2.7 | 2.0 | 2.2 | 0.8 | Rail, cold chain, pharma/food substitution scale. |
| 9 | 24.0 | 14.0 | 10.0 | 33.0 | 10.5 | 3.4 | 2.8 | 3.0 | 1.3 | Export-capable sectors and dividend reserve grow. |
| 10 | 24.0 | 14.0 | 10.0 | 43.0 | 14.0 | 4.1 | 3.8 | 3.9 | 2.2 | Shift toward renewal, debt reduction, stable dividends. |
| **Total** | **190.0** | **120.0** | **70.0** | **150.6** | **44.8** | **15.8** | **11.7** | **12.8** | **5.3** | Planning model; not an audited forecast. |

Interpretation:

- Years 1-4 are capital formation and governance years, not dividend years.
- Years 5-7 prove whether the model can produce surplus without hiding losses.
- Years 8-10 decide whether INDHC becomes a productive asset owner or another
  capital-spending bureaucracy.
- If debt service coverage falls below 1.30 at holding-company level, new debt
  stops and dividends are suspended until the ratio recovers.
- If maintenance reserves are underfunded, dividends are suspended.

### Sector Cashflow By Year 10

Illustrative annual run-rate in Year 10, USD billions.

| Sector | Ten-year capex | Year 10 revenue | Year 10 operating cash after maintenance | Main revenue channels | Source-backed rationale |
| --- | ---: | ---: | ---: | --- | --- |
| Strategic manufacturing, electronics, HVAC, defense-controlled sustainment, and raw-material processing | 62.0 | 16.0 | 3.8 | Public procurement, industrial inputs, HVAC/electronics sales, maintenance contracts, domestic materials, selective exports | OEC import basket shows large finished-goods imports; World Bank energy work identifies downstream gas/materials opportunity. |
| Open rail, metro, logistics, and intercity connections | 38.0 | 4.0 | 0.7 | Fares, availability payments, land-value capture, logistics, station retail | Baghdad Metro RFI creates a reference case; rail lowers congestion and anchors domestic materials/electrical demand. |
| Green technology, power, grid, and waste-to-energy | 32.0 | 6.0 | 2.2 | PPAs, industrial power, grid services, waste fees, avoided fuel costs | IRENA documents Iraq's 12 GW solar direction and renewable-energy plans. |
| Tourism and tradable services | 20.0 | 7.0 | 1.6 | Hotels, visitor logistics, booking/payment platforms, guide services, healthcare/education/business services | Iraq's NDP prioritizes tourism-linked infrastructure; foreign-currency capture diversifies oil dependence. |
| Agriculture, imported-food substitution, water/desalination, irrigation, and cold chain | 22.0 | 7.5 | 1.4 | Food processing, storage, irrigation equipment, desalination/water services, cold chain, feed and packaging | FAO shows continuing wheat/cereal import requirements; World Bank flags water scarcity as macro risk. |
| Housing inputs and urban services | 8.0 | 3.5 | 0.7 | Construction inputs, municipal service contracts, utilities, maintenance | Materials demand is tied to housing, rail, public works, and urban services. |
| Digital public infrastructure and SME platforms | 5.0 | 1.2 | 0.5 | Payment services, compliance services, analytics, credit enablement, registry services | Digital IQD creates transaction evidence and lowers administrative leakage. |
| Workforce, R&D, and standards | 3.0 | 0.0 | -0.9 | Not a profit center | Treated as capability capex; benefits appear through productivity and reduced foreign dependency. |
| **Total** | **190.0** | **45.2** | **10.0** | Multiple | Consolidated run-rate differs from accounting cashflow because group-level levy, debt service, and dividends are applied afterward. |

### Detailed Sector Timelines

#### Strategic Manufacturing, Electronics, HVAC, And Defense-Controlled Supply

| Phase | Years | Capex | Milestones | Cashflow logic |
| --- | --- | ---: | --- | --- |
| Foundation | 1-2 | USD 8B | Legal controls for defense/dual-use activity, supplier registry, quality labs, electronics/HVAC localization plans, first maintenance depots. | Revenue mostly from procurement substitution and maintenance contracts. |
| Build | 3-5 | USD 21B | Protective equipment, uniforms, secure communications assembly, vehicle sustainment, switchgear, meters, HVAC assembly, control cabinets, component repair. | Operating revenue rises as public procurement shifts to qualified Iraqi suppliers. |
| Scale | 6-8 | USD 22B | Local supplier tiers, circuit-board assembly, sensors, efficient HVAC components, rail/grid electronics, certified maintenance exports where lawful. | Margins improve as components localize and warranty/maintenance revenue grows. |
| Consolidate | 9-10 | USD 11B | Product certification, export-licensing review, regional parts sales, lifecycle maintenance. | Cashflow shifts from capex-heavy assembly to service, maintenance, and parts. |

Defense controls:

- Public dashboards show budgets, suppliers, audit status, local content, and
  workforce data.
- Sensitive specifications, end-use details, and classified procurement remain
  outside public dashboards.
- Exports require explicit law, licensing, end-use certification, and external
  compliance review.

#### Water, Desalination, Irrigation, And Food Substitution

| Phase | Years | Capex | Milestones | Cashflow logic |
| --- | --- | ---: | --- | --- |
| Foundation | 1-2 | USD 4B | Map food-import exposure, water-stress zones, cold-chain gaps, pump/filter/pipe demand, and irrigation equipment needs. | Limited revenue from storage, milling, and existing cold-chain upgrades. |
| Build | 3-5 | USD 8B | Drip irrigation, pumps, valves, pipes, filters, modular water-treatment units, grain storage, dairy/poultry/feed upgrades, tomato paste and vegetable processing. | Revenue comes from equipment sales, service contracts, food processing, and reduced spoilage. |
| Scale | 6-8 | USD 7B | Desalination/water-treatment assembly, industrial water reuse, cold-chain corridors, feed mills, staple processing, farmer credit tied to water-efficient equipment. | Cashflow improves as water equipment and food plants reach utilization. |
| Consolidate | 9-10 | USD 3B | Maintenance networks, spare parts, certified water equipment, food quality/export standards, drought-response reserves. | Stable service revenue and procurement savings become the main return. |

Food-substitution rules:

- Target rice, wheat/flour, dairy, poultry, eggs, legumes, vegetable oils, tomato
  paste, frozen vegetables, feed, and packaging.
- Do not force self-sufficiency where water use or delivered cost is irrational.
- Pair every food project with water-efficiency and cold-chain assumptions.
- Use Digital IQD purchase histories to finance farmers, processors, retailers,
  and logistics operators.

#### Green Power, Grid, And HVAC Efficiency

| Phase | Years | Capex | Milestones | Cashflow logic |
| --- | --- | ---: | --- | --- |
| Foundation | 1-2 | USD 4B | Green sukuk framework, project register, solar/storage sites, grid-loss baselines, efficiency standards for public HVAC procurement. | No major revenue; preparation for bankable projects. |
| Build | 3-5 | USD 11B | First utility solar/storage batches, smart meters, grid upgrades, efficient public-building cooling, waste-to-energy pilots. | PPA and industrial power revenue begins; avoided fuel costs are measured. |
| Scale | 6-8 | USD 11B | Expand solar/storage, smart-meter assembly, efficient HVAC localization, industrial power zones. | Recurring PPA, grid-service, and industrial power cashflow. |
| Consolidate | 9-10 | USD 6B | Maintenance, repowering fund, verified emissions/cost reduction, domestic component upgrades. | Debt service coverage improves as capex slows and revenue stabilizes. |

#### Rail, Metro, Logistics, And Domestic Inputs

| Phase | Years | Capex | Milestones | Cashflow logic |
| --- | --- | ---: | --- | --- |
| Foundation | 1-2 | USD 5B | Open rail standards, Baghdad reference audit, fare API, asset registry, land-value capture law, domestic fabrication work packages. | Minimal revenue; procurement discipline and design standardization matter most. |
| Build | 3-5 | USD 15B | Baghdad first corridors, Najaf-Karbala visitor corridor, Basra starter corridor, depots, fare integration, station-area services. | Fares and availability payments begin; domestic materials demand rises. |
| Scale | 6-8 | USD 13B | Mosul/Kirkuk/Nasiriyah/Hilla pilots, logistics corridors, maintenance depots, rail workforce certification. | Ridership, logistics fees, and station revenue improve. |
| Consolidate | 9-10 | USD 5B | National operations platform, spare-parts localization, renewal reserve, performance-based operator contracts. | Cashflow stabilizes through maintenance, service contracts, fares, and land-value capture. |

#### Tourism, Services, And Non-Oil FX

| Phase | Years | Capex | Milestones | Cashflow logic |
| --- | --- | ---: | --- | --- |
| Foundation | 1-2 | USD 3B | Tourism payment rails, guide registry, hotel school, visitor safety/sanitation baselines, diaspora merchant integration. | Platform fees and early service revenue. |
| Build | 3-5 | USD 7B | Najaf-Karbala corridor, Baghdad/Kadhimiya routes, Babylon/Ur/marshland packages, hotel and SME finance. | Visitor spending and hotel/logistics revenue scale. |
| Scale | 6-8 | USD 7B | Medical, education, business-process, logistics, and Arabic software services; foreign-currency settlement into Digital IQD. | Non-oil FX capture becomes measurable. |
| Consolidate | 9-10 | USD 3B | Quality certification, international marketing, repeat visitor channels, service-export standards. | Higher margins from services and platforms. |

### Cashflow Sensitivity Tests

The base case should be rejected or revised if these tests fail:

| Test | Failure threshold | Response |
| --- | --- | --- |
| Oil stress | Oil-equity allocation falls 30% for two years. | Freeze new nonessential capex; protect maintenance and debt service; suspend dividend growth. |
| Delivery delay | Major rail/green/water projects slip by more than 18 months. | Move funds to quick-turnaround food, HVAC, maintenance, and water-efficiency projects. |
| Import parity | Domestic product exceeds landed import cost by more than 15% after learning period. | Remove protection, restructure management, or close the line. |
| Debt safety | Debt service coverage below 1.30. | Stop new borrowing and dividends until coverage recovers. |
| FX exposure | Foreign-currency debt service exceeds verified foreign-currency revenue plus reserves. | Shift borrowing to concessional/local currency or delay imports. |
| Defense governance | End-use controls, legal authority, or audit boundary is incomplete. | Halt defense-controlled procurement and keep only civilian dual-use lines. |
| Water productivity | Irrigation/desalination projects do not reduce water loss or crop risk. | Reprice service contracts and redirect to leakage, metering, or cold-chain projects. |

### Pillar 1: Import Substitution And Raw-Material Post-Processing

INDHC should not attempt to make everything. It should target imports where Iraq
has durable advantages: local demand, heavy transport costs, raw materials, gas
feedstock, construction demand, religious-tourism demand, or proximity to Gulf
and regional markets.

#### Priority Value Chains

| Value chain | Domestic need | Processing target | Export potential |
| --- | --- | --- | --- |
| Associated gas to fertilizer | Agriculture, food security, import reduction | Ammonia, urea, NPK blending, packaged fertilizer | Gulf, Turkey, Levant, South Asia where competitive. |
| Petrochemicals and plastics | Pipes, packaging, construction, agriculture | Ethylene/polyethylene derivatives, irrigation pipe, packaging film, industrial containers | Regional manufactured inputs. |
| Steel and rebar | Housing, rail, bridges, utilities | Scrap processing, direct-reduced iron where gas/power allow, rebar, wire rod, rail components | Primarily domestic; export only if cost-competitive. |
| Cement, brick, tile, glass, insulation | Housing and infrastructure | Regional cement rehab, low-carbon cement additives, bricks, tiles, bottles, flat glass | Mostly domestic due to transport economics. |
| Sulfur, phosphate, and chemicals | Fertilizer, industrial inputs | Purified sulfur, phosphate fertilizers, industrial chemicals | Regional export where quality and logistics permit. |
| Regulated defense industrial base | Sovereign resilience, emergency response, border/security logistics | Uniforms, protective equipment, field medical kits, secure communications assembly, vehicle sustainment, sensors, maintenance depots | Domestic security supply first; exports only under explicit law and license. |
| Electronics and electrical equipment | Rail, grid, buildings, SMEs, public services | Switchgear, meters, control cabinets, cables, circuit boards, sensors, appliance components | Regional components after certification and scale. |
| HVAC and cooling systems | Housing, hospitals, hotels, schools, industry | Air-conditioning assembly, heat pumps, chillers, ducts, filters, efficient controls, maintenance parts | Domestic first; regional export where quality and energy efficiency compete. |
| Desalination and water-treatment equipment | Water security, industry, agriculture, cities | Pumps, membranes/modules assembly, filters, brine-handling components, control systems, mobile treatment units | Regional arid-market opportunities after certification. |
| Irrigation and water-efficiency equipment | Agriculture, food substitution, water scarcity | Drip irrigation, sprinklers, pumps, valves, pipes, meters, soil-moisture sensors | Regional agricultural equipment markets. |
| Dates and horticulture | Food security, rural incomes | Sorting, cold chain, paste, syrup, packaged dates, premium brands | Diaspora, Gulf, EU/UK niche channels. |
| Imported food substitution | Basic internal needs and price resilience | Wheat/rice milling, dairy, poultry, eggs, legumes, vegetable oils, tomato paste, frozen vegetables, animal feed, packaging | Mostly domestic resilience; export only surplus and premium products. |
| Pharmaceuticals and medical supplies | Health-sector imports | Generics packaging, IV fluids, disposables, sterile supplies | Domestic first; regional export after certification. |
| Textiles, uniforms, workwear | Public procurement, tourism, retail | Cotton/wool blending, uniforms, PPE, hotel linens | Domestic procurement and niche export. |

#### Investment Rules

- Every import-substitution project needs a delivered-cost comparison against
  imports, including logistics, FX leakage, power reliability, quality, and scale.
- Public procurement can provide anchor demand, but not permanent protection for
  poor quality or excessive prices.
- Domestic-content incentives should decline as firms mature.
- Subsidiaries must publish product quality, unit cost, energy intensity, local
  employment, and procurement data.
- Downstream processing receives priority over raw-material export whenever the
  domestic margin is positive after capital cost.

### Pillar 2: Profitable Iraqi Businesses

INDHC should behave like a disciplined national holding company, not a ministry.

Required business rules:

- Each subsidiary has its own board, audited accounts, balance sheet, business
  plan, and dividend policy.
- No subsidiary can hide losses by receiving automatic oil transfers.
- Public-service obligations must be priced as explicit service contracts.
- Debt is project-linked, not payroll-linked.
- Debt caps are enforced at subsidiary and holding-company level.
- Management compensation is tied to delivery, profitability, maintenance, local
  employment, and audit quality.
- Procurement is open-book, with beneficial ownership disclosure.
- Loss-making subsidiaries enter turnaround, merger, sale, or closure review.
- Tier advantages, procurement preferences, and credit support expire unless
  the subsidiary passes local-content, price, quality, export, tax, SME
  inclusion, and debt-safety gates.
- No family-control pyramids, circular holdings, hidden related-party control,
  or permanent protection.

Initial subsidiary groups:

1. **INDHC Materials** - cement, brick, glass, steel, rebar, pipes, insulation.
2. **INDHC Chemicals** - fertilizers, sulfur, petrochemicals, industrial gases.
3. **INDHC Strategic Manufacturing** - regulated defense supply chains,
   electronics, electrical equipment, HVAC, and maintenance systems.
4. **INDHC Water And Irrigation** - desalination equipment, water treatment,
   pumps, valves, pipes, irrigation systems, meters, and sensors.
5. **INDHC Food And Cold Chain** - grain storage, dates, dairy, poultry, feed,
   food-staple substitution, packaging, and refrigerated logistics.
6. **INDHC Rail And Urban Mobility** - metro delivery, fare systems, operations.
7. **INDHC Green Power** - solar, wind, storage, grid support, waste-to-energy.
8. **INDHC Tourism And Services** - hospitality, visitor logistics, platforms.
9. **INDHC Digital Infrastructure** - Digital IQD rails, registries, analytics.
10. **INDHC Industrial Parks** - land, utilities, logistics, tenant services.

The champion-group design should remain sectoral. Iraq needs coordinated scale,
but not one untouchable mega-conglomerate. Public demand and credit should move
toward the groups that prove performance through Cylinder Seal data, and away
from groups that become lazy, expensive, politically captured, or hostile to
SMEs.

### Pillar 2A: Strategic Resilience Manufacturing

Strategic resilience manufacturing covers sectors where dependence on imports
creates national vulnerability: defense-controlled supply chains, electronics,
HVAC, desalination equipment, irrigation equipment, and staple-food processing.

Defense manufacturing must be treated as a lawful, audited industrial base, not
as an opaque procurement channel. The plan should focus on domestic sustainment
and resilience capabilities such as uniforms, protective equipment, secure
communications assembly, vehicle maintenance, field medical supplies, sensors,
logistics equipment, and certified maintenance depots. Any lethal or controlled
defense production must sit under explicit Iraqi law, parliamentary oversight,
export-control compliance, end-use monitoring, and a separate classified
procurement process outside public dashboards. Cylinder Seal should record
budgets, approvals, suppliers, local-content attestations, workforce records, and
audit proofs without exposing sensitive technical details.

Electronics and HVAC are priority civilian industries because they sit inside
almost every other pillar: rail systems, hospitals, schools, hotels, housing,
industrial controls, cold chain, grid modernization, irrigation, and data
centers. The first target is assembly, maintenance, quality control, and
component localization; the second target is Iraqi engineering design; the third
target is regional export.

Water and irrigation manufacturing should be treated as food-security
infrastructure. Iraq should build domestic capacity for pumps, valves, pipes,
meters, filters, modular desalination/water-treatment units, drip irrigation,
sprinklers, and soil/water sensors. This reduces import dependency while making
agriculture less exposed to water scarcity.

### Pillar 3: Tourism And Tradable Services

Tourism is one of Iraq's highest-leverage non-oil service opportunities because
it converts identity, history, pilgrimage, and hospitality into recurring
domestic employment and foreign-currency earnings.

Ten-year investments:

- Najaf-Karbala visitor corridor: rail/bus integration, sanitation, shaded
  pedestrian routes, hotels, crowd management, emergency services, multilingual
  payment and booking tools.
- Baghdad, Kadhimiya, Samarra, Mosul, Babylon, Ur, and marshland cultural routes
  with security-screened mobility and regulated local guides.
- Digital tourism platform accepting foreign currency into formal channels and
  paying local providers in Digital IQD.
- Hotel school and hospitality academy with Iraqi instructors trained through
  temporary international partnerships.
- Medical, education, logistics, Arabic software, payments operations, and
  business-process services as exportable service lines.
- Tourism SME finance: working-capital loans tied to verified bookings, reviews,
  tax records, and Digital IQD transaction history.

Advantages:

- Faster job creation than heavy industry.
- Strong fit with Iraqi small businesses.
- Foreign-currency capture without relying on oil.
- Direct feedback into city services, transport, sanitation, and safety.

### Pillar 4: Green Technology And International Capital

INDHC should raise international capital for green assets because these projects
can be externally verified, revenue-linked, and aligned with Iraq's stated energy
transition needs.

Eligible green programs:

- Utility-scale solar and storage near load centers and industrial parks.
- Wind pilots where resource studies support them.
- Waste-to-energy and landfill methane capture.
- Grid modernization, smart meters, and distribution-loss reduction.
- Efficient cooling, rooftop solar finance, and municipal energy efficiency.
- Domestic assembly of solar mounting structures, cables, switchgear, smart
  meters, battery packs, and eventually inverter components.
- Low-carbon cement additives, efficient kilns, and industrial heat recovery.

Capital-raising instruments:

- Green sukuk with verified use of proceeds.
- MDB and climate-fund co-financing.
- Export-credit loans for equipment with Iraqi training and localization.
- Project finance backed by power-purchase agreements or industrial offtake.
- Diaspora green bonds only after transparent governance and repayment rules are
  proven.

Rules:

- No green debt for non-green operating deficits.
- Every green project has a public use-of-proceeds register.
- Carbon claims must be externally verified before being monetized.
- International contractors must leave behind Iraqi operations, maintenance, and
  engineering capability.

### Pillar 5: Open Rail And Metro Networks

The rail program should be open where openness improves cost, competition, and
maintainability, while still using certified safety systems.

Open-source rail does not mean uncertified signalling. It means Iraq owns the
interfaces, data, standards, reference designs, fare integration, maintenance
records, and procurement playbooks.

#### Open Rail Iraq Stack

- Open fare-clearing APIs linked to Digital IQD wallets.
- GTFS/NeTEx-style open transit data for routing, schedules, and passenger
  information.
- Open asset registry for track, stations, depots, rolling stock, elevators,
  power systems, and maintenance events.
- Standard station kits, viaduct components, depot layouts, and wayfinding
  templates that can be adapted by city.
- Open procurement specifications for non-safety-critical components.
- Certified CBTC/signalling, rolling stock, and power systems with open
  interfaces and data escrow.
- Iraqi maintenance manuals, training simulators, and spare-parts catalogues.

#### City Network Sequencing

| Phase | Cities / corridors | Work |
| --- | --- | --- |
| Years 1-2 | Baghdad reference program | Audit the 148 km / 64-station concept, define open interfaces, protect procurement transparency, and identify domestic fabrication packages. |
| Years 3-5 | Baghdad first corridors, Basra starter corridor, Najaf-Karbala visitor corridor | Build high-demand lines, integrate Digital IQD fares, and create Iraqi operations and maintenance teams. |
| Years 6-8 | Mosul reconstruction transit, Erbil/Sulaymaniyah subject to KRG agreements, Kirkuk and Nasiriyah/Hilla pilots | Use light metro, tram, bus rapid transit, and rail-bus hybrids where full metro is not yet justified. |
| Years 9-10 | National urban mobility platform | Standardize fares, maintenance, spare parts, safety audits, and passenger information across participating cities. |

Rail advantages:

- Reduces urban congestion and fuel imports.
- Creates high-skill Iraqi engineering and operations jobs.
- Supports tourism and religious visitor flows.
- Creates anchor demand for steel, concrete, glass, electrical equipment, and
  maintenance services.
- Builds a national capability instead of one-off vendor dependency.

### Pillar 6: Broad Internal Needs

INDHC should cover Iraq's internal needs through commercial or service-contract
models, not open-ended subsidy.

| Need | INDHC investment route |
| --- | --- |
| Reliable electricity | Solar, storage, grid modernization, industrial power contracts, efficient cooling. |
| Water, desalination, and wastewater | Desalination equipment production, treatment plants, filters, pumps, leakage reduction, pumping efficiency, industrial water reuse. |
| Housing inputs | Cement, brick, glass, steel, insulation, pipes, tiles, modular components. |
| Food security and imported-food substitution | Storage, cold chain, milling, dates, dairy, poultry, eggs, legumes, vegetable oils, tomato paste, feed, packaging, irrigation equipment. |
| Urban mobility | Metro, light metro, tram/BRT, integrated fares, transit-oriented development. |
| Health resilience | Essential medicines, IV fluids, disposables, cold-chain logistics. |
| Defense and emergency resilience | Audited domestic supply of uniforms, protective equipment, field medical kits, secure communications assembly, maintenance, and logistics equipment. |
| Electronics and HVAC | Switchgear, meters, sensors, controls, circuit-board assembly, air-conditioning, heat pumps, chillers, filters, and spare parts. |
| Irrigation production | Drip lines, sprinklers, pumps, valves, pipes, meters, soil-moisture sensors, and maintenance services. |
| Education and skills | INDHC Academy, technical colleges, rail simulators, industrial apprenticeships. |
| Digital economic visibility | Digital IQD payments, project registries, audit trails, SME credit features. |
| Waste and sanitation | Waste-to-energy, recycling, landfill methane capture, municipal service contracts. |
| Export logistics | Industrial parks, cold chain, rail-freight links, ports and customs data integration. |

### Iraqi-Only Permanent Staffing Policy

INDHC should employ Iraqi citizens exclusively as permanent staff at all levels:
board, executive management, engineers, operators, analysts, auditors, project
managers, customer-service teams, technicians, station staff, plant workers, and
maintenance crews.

Implementation rules:

- All INDHC employees must be Iraqi citizens.
- Foreign specialists may be hired only as temporary vendor personnel, trainers,
  certifiers, or technical advisers. They do not hold INDHC staff positions or
  operational command.
- Every foreign vendor contract must include named Iraqi counterparts, training
  hours, Arabic documentation, source or interface documentation where possible,
  certification transfer, and handover milestones.
- By contract close, operations and maintenance must be executable by Iraqi
  teams.
- Technical leadership succession must be reviewed quarterly.
- Scholarships and apprenticeships prioritize governorates where projects are
  located.
- Staff hiring, promotion, and pay are merit-based and audited to prevent party,
  militia, family, or contractor capture.

Workforce targets:

| Year | Workforce milestone |
| --- | --- |
| 1 | Establish INDHC Academy; map national engineers, accountants, operators, rail staff, power technicians, tourism workers, and industrial trades. |
| 2 | Launch 25,000 apprenticeships across rail, power, materials, food, water, irrigation, electronics, HVAC, regulated defense sustainment, tourism, audit, and software operations. |
| 3 | Require every major project to maintain an Iraqi counterpart ratio and training ledger. |
| 5 | Iraqi teams operate all quick-win subsidiaries and first-wave public service platforms. |
| 7 | Iraqi teams lead metro operations, industrial park operations, green-power O&M, and financial controls. |
| 10 | Iraqi staff run all permanent operating roles; foreign involvement is limited to new technology transfer, audits, and certification support. |

### Reinvestment And Dividend Waterfall

INDHC should not distribute all cash immediately. The dividend must be real, but
it must come after maintenance, debt service, taxes, and statutory reinvestment.

Annual operating waterfall:

1. Operating revenue enters subsidiary accounts.
2. Operating costs and maintenance reserves are funded.
3. Debt service is paid only from eligible project or holding-company cash flows.
4. Gross-profit levy is paid to Treasury for ministry budgets.
5. Statutory retained earnings are allocated:
   - 35% to expansion of profitable domestic industries.
   - 25% to infrastructure maintenance and renewal.
   - 15% to debt reduction and liquidity buffers.
   - 15% to workforce, R&D, and Iraqi supplier upgrading.
   - 10% to dividend stabilization reserve.
6. Remaining distributable surplus enters the monthly dividend pool.
7. Dividend pool is paid equally to eligible citizen wallets in Digital IQD.

This preserves the user's core principle: ministries are funded from productive
surplus, and citizens receive the remaining distributable national capital
return. It also prevents asset stripping and under-maintenance.

### Cylinder Seal Implementation Surface

Cylinder Seal should model the plan with explicit primitives and dashboards
before any production implementation:

| Primitive / model | Purpose |
| --- | --- |
| `InvestmentPlan` | Ten-year capital plan, sector allocation, source-of-funds rules. |
| `CapitalStack` | Oil equity, concessional loans, green bonds, export-credit facilities, project debt. |
| `SubsidiaryRegistry` | INDHC subsidiaries, boards, mandates, accounts, audit status. |
| `ProjectMilestone` | Budget, schedule, delivery evidence, inspector sign-off, payment release. |
| `LocalContentAttestation` | Iraqi supplier, material, labor, and technology-transfer evidence. |
| `IraqiEmploymentAttestation` | Confirms permanent staff are Iraqi citizens and tracks training transfer. |
| `StrategicSectorControl` | Legal authority, license, end-use controls, classification boundary, and audit scope for defense and dual-use manufacturing. |
| `WaterFoodSecurityProject` | Desalination, irrigation, water-treatment, cold-chain, and food-substitution project records. |
| `LoanFacility` | Lender, currency, rate, maturity, covenant, project linkage, debt-service schedule. |
| `GrossProfitLevy` | Treasury claim on audited productive surplus. |
| `RetainedEarningsAllocation` | Reinvestment, maintenance, debt reduction, training, and dividend reserve. |
| `DividendDistribution` | Monthly equal dividend batch to citizen wallets. |
| `PublicProjectDisclosure` | Aggregated public evidence for citizens, parliament, auditors, and lenders. |

Dashboard views:

- Oil Income Lockbox.
- Ten-year allocation map.
- Sector profitability.
- Debt exposure and currency risk.
- Iraqi employment and training transfer.
- Import substitution scoreboard.
- Strategic resilience manufacturing dashboard.
- Food substitution, water, desalination, and irrigation dashboard.
- Green use-of-proceeds register.
- Rail project delivery map.
- Tourism and service revenue.
- Dividend pool and monthly citizen payment.

### Governance Tests Before Launch

Do not treat this plan as deployable until these questions have written answers:

1. What constitutional or statutory authority redirects oil income into INDHC?
2. Which entity owns oil receipts before they enter the lockbox?
3. What share of oil receipts can be capitalized without destabilizing the budget?
4. Who approves borrowing, and what debt ceiling applies?
5. Are INDHC debts sovereign debts, project debts, or holding-company debts?
6. How are KRG, governorate, and federal authorities represented?
7. How are ministry service contracts priced and audited?
8. How is the citizen dividend formula protected from political manipulation?
9. What happens to loss-making subsidiaries?
10. What court or tribunal resolves shareholder entitlement and inheritance
    disputes?
11. What procurement law applies?
12. What are the sanctions, PEP, AML/CFT, and beneficial-owner rules for cash
    formalization, suppliers, and lenders?

### Success Measures

| Measure | Ten-year target type |
| --- | --- |
| Import substitution | Share of selected public and private demand met by Iraqi production, by value chain. |
| Strategic resilience | Domestic production share for regulated defense supply, electronics, HVAC, water/desalination, irrigation, and food-staple substitution. |
| Profitability | Subsidiary return on invested capital, operating margin, and cash conversion. |
| Debt safety | Debt-service coverage, FX exposure, maturity profile, covenant compliance. |
| Employment | Iraqi permanent staff share, apprenticeship completion, technical leadership transfer. |
| Green investment | MW commissioned, grid losses reduced, emissions intensity, verified use of proceeds. |
| Rail delivery | Km opened, stations operational, ridership, farebox recovery, maintenance performance. |
| Tourism services | Formal visitor spending, local SME participation, hotel occupancy, Digital IQD capture. |
| Raw-material processing | Share of Iraqi raw materials processed domestically before consumption or export. |
| Ministry feedback | Share of ministry budgets funded by explicit levy, tax, or service-contract flows. |
| Citizen dividend | Monthly dividend reliability, exception rate, equal distribution audit. |

### Bottom Line

The ten-year plan makes INDHC more than a sovereign fund. It becomes a national
productive-capital machine:

- oil income becomes equity;
- loans fund bankable green, rail, industrial, and service assets;
- Iraqi workers operate the system at every permanent level;
- raw materials are processed inside Iraq;
- ministries are funded by productive surplus rather than entitlement to oil;
- profits are reinvested into domestic capacity;
- remaining distributable surplus is paid as a Digital IQD citizen dividend.


# Part 10: Digitally Governed Industrial Champions

## Digitally Governed Industrial Champions

This document refines the INDHC industrial-conglomerate idea. The model should
not be described as copying Korean chaebols. It should be described as building
digitally governed Iraqi industrial champions: sector-focused production groups
that receive demand, credit, and payment privileges only while they prove local
content, job creation, price discipline, tax compliance, export progress, and
audit quality through Cylinder Seal data.

Status: policy-design scenario. It is not a competition-law opinion, investment
recommendation, procurement plan, or claim that any champion group should receive
permanent protection.

### Source Discipline

| Lesson | Use in Cylinder Seal | Source |
| --- | --- | --- |
| Korea's large business groups played a major role in development and exports, but concentration of economic power created governance and competition risks. | Iraq should borrow coordination and scale discipline, not family-controlled monopoly structures. | [OECD, Reforming the large business groups to promote productivity and inclusion in Korea](https://www.oecd.org/en/publications/reforming-the-large-business-groups-to-promote-productivity-and-inclusion-in-korea_9e9052b5-en.html) |
| Korea's export support system included export targets, credit allocation for export purposes, technology acquisition, and strong marketing institutions. | Iraqi support should be conditional on measured export progress, technology transfer, and independently monitored performance. | [World Bank, Korea: A Case of Government-Led Development](https://documents1.worldbank.org/curated/en/441571468753249695/pdf/multi0page.pdf) |

These sources support the analogy. They do not validate the Iraqi institutional
design, capital envelope, or legality of any demand restriction.

### Terminology

Avoid using **chaebol** as the official framing. It carries the right intuition
about scale and state coordination, but the wrong institutional signal: family
control, excessive leverage, monopoly power, political capture, and
too-big-to-fail risk.

Preferred terms:

- National Production Champions.
- Iraqi Industrial Champion Groups.
- Digitally Governed Industrial Houses.

Short framing:

```text
Cylinder Seal enables Iraq to build digitally governed industrial champions:
sector-focused production groups that receive credit, demand, and payment
privileges only while they prove local content, job creation, price discipline,
tax compliance, and export performance through auditable Digital IQD data.
```

### Operating Model

The old state-capitalism model is:

```text
cheap credit + political protection + weak measurement
```

The Cylinder Seal version should be:

```text
conditional demand + conditional credit + conditional payment privileges
+ transaction evidence + export discipline + competition gates
```

Industrial champions are useful only if they solve coordination failures:
uncertain demand, missing credit histories, fragmented suppliers, low
technology absorption, and weak export channels. They become dangerous when they
turn into protected monopolies.

### Design Principles

1. **Many sectoral groups, not one national giant.** Sector focus keeps accounts,
   management, debt, pricing, and failures visible.
2. **Privileges expire by default.** Demand guarantees, credit support, and Tier
   advantages need renewal through measured performance.
3. **Domestic demand is a launchpad, not a comfort zone.** Every tradable group
   needs an export or foreign-currency path unless the project is a pure
   domestic-public-good asset.
4. **SMEs are suppliers, competitors, and acquisition targets of last resort.**
   Champion groups should crowd in Iraqi SMEs, not crush them.
5. **Technology transfer must be contractual.** Foreign vendors can supply
   machinery, designs, training, certification, and commissioning, but the
   permanent operating staff should be Iraqi.
6. **Debt discipline comes before dividends.** If debt-service coverage or FX
   exposure breaks thresholds, new privileges stop.
7. **Public evidence, protected trade secrets.** Public dashboards show aggregate
   performance, prices, local content, audit exceptions, and privilege status;
   confidential formulas, security details, and commercial secrets stay in
   controlled audit channels.

### Sectoral Champion Groups

Do not create one mega-conglomerate. Create several focused groups with separate
boards, accounts, debt limits, and performance gates.

| Champion group | Focus | Main Cylinder Seal evidence |
| --- | --- | --- |
| Mesopotamia Cement And Materials | Cement, aggregates, precast, glass, insulation, construction inputs. | Unit cost, local content, public-works delivery, housing-input prices, procurement trails. |
| Tigris Steel And Fabrication | Rebar, light steel, structural products, rail components, maintenance parts. | Energy intensity, delivered cost against imports, domestic offtake, quality certification. |
| Rafidain Food Industries | Flour, dairy, dates, poultry, eggs, feed, tomato paste, packaged staples, cold chain. | Food security volumes, spoilage reduction, farmer payments, water-efficiency links. |
| Babylon Pharma And Medical Supplies | Generic medicines, IV fluids, consumables, medical textiles, sterile supplies. | Health procurement savings, certification, batch traceability, stockout reduction. |
| Uruk Textiles And Household Goods | Uniforms, workwear, bedding, carpets, hotel linens, PPE textiles. | Tier status, public procurement performance, SME supplier participation, unit quality. |
| Basra Petrochem And Plastics | Polymers, packaging, pipes, fertilizers, industrial containers. | Feedstock use, domestic downstream sales, export receipts, environmental compliance. |
| Iraq Tourism And Heritage Group | Hotels, tours, visitor logistics, diaspora tourism, cultural retail. | Foreign-currency capture, bookings, guide payments, service quality, local SME spend. |
| Diyala Electronics And Cooling | Switchgear, meters, control cabinets, appliance components, efficient HVAC systems. | Warranty claims, local component share, energy-efficiency performance, maintenance revenue. |
| Nineveh Water And Irrigation Systems | Pumps, valves, pipes, filters, drip irrigation, meters, mobile treatment units. | Water-loss reduction, farmer adoption, service uptime, equipment lifecycle cost. |

These names are placeholders. The governance rule matters more than the brand:
each group must be replaceable, audited, and exposed to competition.

### Ten-Year Rollout

Timelines are counted from the legal launch of the champion framework, not from
the date of this document.

| Phase | Years | Goal | Main actions | Decision gate |
| --- | --- | --- | --- | --- |
| Legal design | 0-1 | Create lawful authority and prevent capture before money moves. | Pass enabling law, create champion registry, define competition mandate, publish scorecard, appoint interim board, map import baselines, list public-demand categories. | No demand contract before registry, audit, debt-cap, and conflict rules exist. |
| Pilot champions | 1-2 | Prove the model in quick-turn sectors. | Launch 3-4 pilots in materials, food/cold chain, tourism services, and water/irrigation equipment; start supplier registry and working-capital pilots. | Continue only if first contracts produce price, quality, local-content, and delivery evidence. |
| Build-out | 3-5 | Scale production capacity and supplier networks. | Add steel/fabrication, pharma/medical supplies, textiles, petrochem/plastics, electronics/HVAC; deploy conditional demand contracts and supplier finance. | Privileges renew only above scorecard threshold and with audited accounts. |
| Competition | 6-8 | Move from import substitution to competitive production. | Decline fee advantages, open more procurement to challengers, require export or FX evidence for tradable sectors, expand SME supplier quotas. | Weak exporters or high-cost protected lines lose champion status. |
| Renewal or graduation | 9-10 | Decide which groups become normal competitive companies. | Renew strategic groups, graduate mature groups to normal finance/procurement, restructure or close persistent failures. | No group receives a second ten-year mandate without competition review. |

### Champion Group Operating Plans

Each champion starts with anchor demand, then must graduate toward competition.

| Group | Years 1-2 foundation | Years 3-5 build | Years 6-10 competition path | Main risk gate |
| --- | --- | --- | --- | --- |
| Mesopotamia Cement And Materials | Audit idle plants, map public works demand, standardize cement/aggregate/precast specs, start rehab contracts. | Expand low-carbon cement additives, precast, insulation, glass, and public-housing inputs. | Compete on delivered cost for housing, rail, schools, hospitals, and municipal works; export only where transport economics work. | Remove preference if price exceeds landed import parity by more than the approved learning margin. |
| Tigris Steel And Fabrication | Map scrap, gas/power constraints, rail and construction demand; start fabrication and maintenance shops. | Scale rebar, light steel, structural products, rail components, and repair services. | Compete for industrial parks, rail, bridges, water systems, and regional maintenance exports. | Stop expansion if energy intensity, quality failures, or debt service breaches thresholds. |
| Rafidain Food Industries | Build farmer registry, cold-chain gaps, storage/milling baselines, and first dairy/poultry/date contracts. | Scale grain storage, flour, dairy, poultry, eggs, feed, tomato paste, packaging, and cold chain. | Expand premium dates, packaged food, and diaspora retail channels; keep staples domestic-resilience focused. | No project proceeds if water use or delivered cost is irrational. |
| Babylon Pharma And Medical Supplies | Map health procurement imports, create certification plan, start consumables and medical textiles. | Add IV fluids, generics packaging, sterile supplies, batch traceability, and hospital-stock dashboards. | Export only after certification, pharmacovigilance, and quality audits; prioritize domestic stockout reduction. | Any safety or certification breach suspends procurement preference. |
| Uruk Textiles And Household Goods | Start uniforms, bedding, hotel linens, workwear, and PPE textiles with SME sewing networks. | Build fabric finishing, quality labs, public procurement catalog, and tourism/hospitality supply. | Compete on uniforms, linens, carpets, and branded cultural goods; export niche products through diaspora channels. | Preference falls if SME participation or quality targets fail. |
| Basra Petrochem And Plastics | Map feedstock, fertilizer, pipe, packaging, and irrigation demand; start downstream feasibility. | Build polymers, packaging, pipes, fertilizers, and industrial containers. | Export selected inputs regionally if environmental and cost controls hold. | Halt privileges on pollution, feedstock diversion, or related-party procurement abuse. |
| Iraq Tourism And Heritage Group | Build guide registry, hotel school, safety/sanitation baselines, booking/payment rails, and diaspora packages. | Scale Najaf-Karbala, Baghdad/Kadhimiya, Babylon/Ur, marshland, medical, education, and business travel services. | Foreign-currency capture, repeat visitors, quality certification, and SME tourism marketplaces. | Commercial revenue must stay separate from heritage protection and safety oversight. |
| Diyala Electronics And Cooling | Begin switchgear, meters, control cabinets, HVAC assembly, repair centers, and energy-efficiency standards. | Localize components, maintenance parts, warranty systems, smart meters, efficient public-building cooling. | Regional components, certified maintenance exports, and Iraqi engineering design capability. | Warranty failure, energy inefficiency, or closed vendor lock-in removes preference. |
| Nineveh Water And Irrigation Systems | Map pumps, valves, pipes, filters, drip irrigation, mobile treatment, leakage, and farmer demand. | Scale water-treatment assembly, irrigation kits, metering, soil/water sensors, and service contracts. | Regional arid-market equipment and maintenance services after certification. | Privileges stop if water-loss or crop-risk metrics do not improve. |

### Conditional Demand Contracts

Government procurement, public transfers, and INDHC project spending may provide
anchor demand only when measurable conditions are met:

| Condition | Measurement |
| --- | --- |
| Local content | Domestic-origin attestations, supplier invoices, inspection records, and tier status. |
| Delivery reliability | On-time delivery, defect rate, service uptime, warranty claims. |
| Price discipline | Delivered-cost comparison against imports and domestic competitors. |
| Employment quality | Iraqi payroll records, apprenticeship completions, safety incidents, wage compliance. |
| Export progress | Foreign-currency receipts, repeat orders, certification, distributor contracts. |
| Tax compliance | VAT/sales records, gross-profit levy, payroll tax, audit exceptions. |
| SME inclusion | Share of procurement from independent Iraqi SMEs and individual producers. |
| Debt safety | Debt-service coverage, maturity profile, FX exposure, related-party lending. |

Failure should automatically reduce privileges:

- Tier 1 status drops to Tier 2 or Tier 3.
- Public-transfer eligibility narrows.
- New subsidized credit pauses.
- Procurement preference expires.
- Management enters turnaround review.

#### Demand Contract Structure

| Contract field | Requirement |
| --- | --- |
| Buyer | Ministry, municipality, INDHC project, hospital, school, utility, tourism platform, or public procurement authority. |
| Supplier | Champion group, SME consortium, or private producer with published beneficial ownership. |
| Product or service | Standardized catalog item, service-level agreement, or project milestone. |
| Price rule | Benchmark against import parity, domestic competitors, or regulated cost-plus ceiling for public-good assets. |
| Release rule | Conditional release after delivery evidence, inspection, invoice match, and tax/audit checks. |
| Expiry | Contract support expires unless renewal gates pass. |
| Public dashboard | Aggregate value, supplier class, delivery status, local content, delay, defect rate, and audit exceptions. |
| Appeal path | Supplier and buyer can dispute inspection, payment hold, tier status, or penalty. |

Demand contracts should be modular. A large public works package can be split
into materials, fabrication, logistics, installation, maintenance, and warranty
contracts so SMEs can participate without needing to become a conglomerate.

### Finance And Credit Plan

Champion groups receive financial support through instruments that can be turned
off without destabilizing the budget.

| Instrument | Use | Guardrail |
| --- | --- | --- |
| Oil-equity capital | First-loss capital for strategic assets and early rehabilitation. | No automatic replenishment after losses. |
| Working-capital guarantee | Lets banks lend against receivables and public offtake contracts. | Guarantee falls as repayment history improves. |
| Conditional offtake | Public buyer commits to volume if price, quality, and delivery conditions are met. | No delivery evidence, no payment. |
| Supplier finance | SME suppliers borrow against verified purchase orders or receivables. | Supplier exposure cap per champion. |
| Export credit | Supports certified exporters with repeat buyers and FX receipts. | Export credit stops if returns, defects, or nonpayment rise. |
| Technology-transfer finance | Pays for machinery, certification, training, and commissioning. | Foreign vendor must train Iraqi operators and disclose handover plan. |
| Restructuring facility | Temporary turnaround finance for viable distressed lines. | Requires management change and published turnaround metrics. |

Debt rules:

- Minimum debt-service coverage ratio: 1.30 at subsidiary level.
- FX debt must be matched to FX revenue, hedged, concessional, or explicitly
  stress tested.
- Cross-guarantees between champion groups require public board approval.
- No debt proceeds fund dividends, ordinary payroll expansion, or political
  transfers.
- Related-party lending is capped, disclosed, and independently reviewed.

### Performance Scorecard

Every champion group receives a quarterly score. The score determines Tier
status, procurement preference, credit eligibility, and public dashboard color.

| Category | Weight | Evidence |
| --- | ---: | --- |
| Local content | 15 | Supplier invoices, domestic-origin certificates, inspections, tier data. |
| Price discipline | 15 | Delivered-cost comparison against imports and domestic alternatives. |
| Quality and safety | 10 | Defect rates, warranty claims, certification, safety incidents. |
| Delivery reliability | 10 | On-time delivery, uptime, milestone completion, stockout reduction. |
| Jobs and skills | 10 | Iraqi payroll, apprenticeships, technical certifications, safety compliance. |
| Export / FX progress | 15 | FX receipts, repeat orders, tourism bookings, certifications, return rates. |
| Tax and audit compliance | 10 | Gross-profit levy, payroll tax, audit exceptions, filing timeliness. |
| SME inclusion | 10 | Independent SME procurement share, payment speed, supplier concentration. |
| Debt safety | 5 | Debt-service coverage, FX exposure, maturity profile, covenant breaches. |

| Score | Status | Consequence |
| ---: | --- | --- |
| 80-100 | Green | Full eligible privileges until next review. |
| 65-79 | Watch | Privileges continue, but new credit is capped and corrective plan is required. |
| 50-64 | Probation | Procurement preference and Tier advantages narrow; board must approve recovery plan. |
| Below 50 | Suspended | New privileges stop; existing contracts enter review. |

Hard vetoes override the score:

- fraud, sanctions breach, or terrorist-finance concern;
- repeated safety or certification failure;
- hidden related-party control;
- debt-service coverage below minimum without approved rescue plan;
- refusal to provide audit evidence;
- political interference in hiring, procurement, or pricing.

### No Permanent Protection

Protection must be time-limited and performance-linked.

| Stage | Years | Support allowed | Exit rule |
| --- | --- | --- | --- |
| Launch | 1-2 | Feasibility funding, working-capital guarantees, first public offtake contracts, technology-transfer support. | No production milestone means support stops. |
| Learning | 3-5 | Tier advantages, procurement preference, quality-lab support, export-market assistance. | Unit cost must converge toward import parity after learning period. |
| Competition | 6-8 | Declining fee advantages and procurement preference; export credit only for proven lines. | Excessive prices, weak quality, or low export progress removes privileges. |
| Mature | 9-10 | Normal commercial finance, open procurement competition, targeted R&D only. | Champion status is renewed only for strategic sectors with proven public value. |

### Export Discipline

Import substitution is not enough. Every champion group needs an export or
foreign-currency pathway unless the sector is explicitly domestic-public-good
infrastructure.

Examples:

- Materials: regional cement additives, insulation, glass, construction parts
  only if transport economics work.
- Food: dates, packaged foods, premium agricultural goods, and diaspora retail
  channels.
- Pharma: regional export only after certification and pharmacovigilance
  capacity.
- Tourism: direct foreign-currency capture from religious, heritage, medical,
  education, and business travel.
- Electronics/HVAC/water: regional components and maintenance services after
  quality certification.

Cylinder Seal should show export discipline through foreign-currency receipts,
repeat buyers, certification status, return rates, and gross margins.

### Technology Transfer And Skills Plan

Foreign vendors should be paid for capability transfer, not permanent
dependency.

| Stage | Requirement |
| --- | --- |
| Procurement | Every foreign equipment or system contract includes training, documentation, spare-parts strategy, open interfaces where feasible, and Iraqi counterpart teams. |
| Commissioning | Iraqi engineers and technicians sign off alongside foreign vendors. |
| Year 2 operations | At least one Iraqi deputy for each foreign technical lead. |
| Year 3 operations | Iraqi staff own routine maintenance, procurement planning, data reporting, and quality checks. |
| Year 5 operations | Foreign experts remain only as advisors, certifiers, or specialist contractors. |

The INDHC Academy should provide:

- industrial accounting and cost-control training;
- procurement and contract-management certification;
- export documentation and standards training;
- maintenance and reliability engineering;
- water, cooling, electronics, food-safety, and pharma quality programs;
- competition, ethics, and conflict-of-interest training for boards and managers.

### SME Inclusion Plan

Champion groups should create markets for smaller Iraqi firms.

| Tool | Purpose |
| --- | --- |
| SME supplier quota | Require a rising share of qualified procurement from independent Iraqi SMEs. |
| Fast-payment rule | Pay SME invoices faster after delivery evidence to reduce working-capital stress. |
| Receivables finance | Let banks lend against verified champion purchase orders. |
| Open catalog | Publish product specs, procurement calendars, quality standards, and onboarding rules. |
| Supplier graduation | Move high-performing SMEs from subcontractors to direct public procurement eligibility. |
| Anti-exclusivity rule | Ban supplier lock-ins that prevent SMEs from selling to competitors. |

Suggested SME procurement targets:

| Year | Minimum independent Iraqi SME procurement share |
| --- | ---: |
| 1 | 5% |
| 2 | 10% |
| 3 | 15% |
| 5 | 25% |
| 8 | 35% |
| 10 | 40% where sector structure allows |

These are planning targets, not legal mandates. They should be adjusted by
sector, quality requirements, security classification, and supplier capacity.

### Anti-Capture Governance

Every champion group requires:

- Independent board with published fit-and-proper rules.
- No family-control pyramids, circular holdings, or hidden related-party control.
- Debt cap by subsidiary and holding-company level.
- No cross-subsidy without public board approval and audit note.
- Related-party transaction limits and mandatory disclosure.
- Public beneficial ownership for suppliers and contractors.
- Minority investor and bondholder protections where outside capital is used.
- Annual competition review against SMEs and independent producers.
- Automatic loss of Tier 1 or procurement preference after performance failure.
- Independent audit of local-content claims, export receipts, debt, payroll, and
  procurement.

### Competition And Renewal Plan

Champion status should be reviewed annually and renewed every three years.

| Review | Questions |
| --- | --- |
| Annual performance review | Did the group meet scorecard thresholds, contract milestones, debt rules, and SME inclusion targets? |
| Annual competition review | Are prices converging to import parity, are SMEs able to enter, and are citizens receiving better products or services? |
| Three-year mandate renewal | Does the sector still need champion coordination, or should it graduate to normal competition? |
| Failure review | Should the group be restructured, merged, privatized, opened to private challengers, or closed? |

Competition remedies:

- reduce or end Tier advantages;
- split procurement packages into SME-accessible lots;
- force open standards and interoperability;
- publish import-parity benchmarks;
- require management replacement;
- sell non-strategic subsidiaries through transparent process;
- open a challenger round for private firms or SME consortia.

### Cylinder Seal Implementation Surface

| Model | Purpose |
| --- | --- |
| `IndustrialChampionRegistry` | Champion group, sector mandate, board, ownership, debt cap, audit status. |
| `ChampionPerformanceGate` | Local content, price, quality, jobs, exports, tax, debt, and SME inclusion thresholds. |
| `ConditionalDemandContract` | Public or INDHC offtake that releases payments only after measurable delivery. |
| `ChampionPrivilegeStatus` | Tier status, procurement preference, credit eligibility, and expiry date. |
| `ExportDisciplineMetric` | FX receipts, repeat orders, certification, return rates, export gross margin. |
| `RelatedPartyExposure` | Supplier, lender, director, beneficial-owner, and affiliate exposure tracking. |
| `CompetitionReview` | Measures whether a champion is crowding out SMEs or abusing public privileges. |

Additional event types:

| Event | Meaning |
| --- | --- |
| `ChampionCreated` | A champion group receives legal mandate, board, sector, and debt-cap record. |
| `PrivilegeGranted` | Tier advantage, procurement preference, credit eligibility, or public-transfer eligibility is granted. |
| `PrivilegeReduced` | Privilege narrows after scorecard, audit, or competition failure. |
| `DemandContractSigned` | Public or INDHC buyer creates conditional offtake or service contract. |
| `DeliveryEvidenceSubmitted` | Supplier submits invoice, inspection, local-content proof, and delivery evidence. |
| `PaymentReleased` | Conditional release pays after evidence passes. |
| `ExportReceiptRecorded` | FX or export receipt is linked to champion group and product line. |
| `RelatedPartyFlagRaised` | Beneficial ownership, supplier concentration, or affiliate exposure breach is flagged. |
| `CompetitionReviewFiled` | Annual review records price, market share, SME access, and privilege decision. |

Suggested database tables:

| Table | Core fields |
| --- | --- |
| `industrial_champions` | `champion_id`, `name`, `sector`, `mandate`, `status`, `debt_cap`, `board_id`, `audit_status`. |
| `champion_scorecards` | `champion_id`, `period`, score components, hard veto flags, reviewer, publication status. |
| `champion_privileges` | `champion_id`, `privilege_type`, `start_date`, `expiry_date`, `status`, `basis`. |
| `conditional_demand_contracts` | buyer, supplier, product, quantity, price rule, release rule, local-content requirement. |
| `champion_exports` | product, destination, buyer class, FX amount, repeat-order flag, certification reference. |
| `related_party_exposures` | champion, counterparty, relationship, exposure amount, approval, mitigation. |
| `competition_reviews` | market, concentration, price benchmark, SME access, decision, appeal status. |

### Dashboard Plan

The industrial-champion dashboard should have five views:

1. **Portfolio view** - all champions, score, privilege status, debt exposure,
   public demand exposure, and audit exceptions.
2. **Sector view** - domestic output, import parity, local content, SME share,
   and export progress by sector.
3. **Contract view** - demand contracts, delivery evidence, holds, releases,
   disputes, and late payments.
4. **Competition view** - market share, price spread, SME entry, related-party
   exposure, and privilege expiry.
5. **Citizen view** - jobs, dividend contribution, domestic price effects,
   service reliability, and regional distribution.

Public dashboards should show aggregates and accountability metrics. Regulator
dashboards can show counterparty-level risk, beneficial ownership, audit
evidence, and enforcement actions.

### First 180 Days

| Month | Work |
| --- | --- |
| 1 | Draft champion framework law, define competition authority role, identify public-demand categories, and freeze any informal privilege grants. |
| 2 | Build import baseline by product line, create champion registry schema, define scorecard, and publish conflict-of-interest rules. |
| 3 | Select first pilot sectors: materials, food/cold chain, tourism services, and water/irrigation equipment. |
| 4 | Issue first conditional demand contract templates, supplier registry rules, SME onboarding process, and audit evidence taxonomy. |
| 5 | Launch first working-capital and receivables-finance pilots with commercial banks. |
| 6 | Publish first dashboard prototype with scorecard, contract, local-content, delivery, and audit views. |

No group should receive a long-term legal mandate during the first 180 days.
The first six months should prove that the measurement and enforcement machinery
works before major privileges are granted.

### Open Policy Questions

1. Which legal body grants and removes champion status?
2. Does the competition authority have power to suspend privileges directly?
3. How are governorates represented when champion activity is regionally
   concentrated?
4. What public information is published without exposing trade secrets or
   security-sensitive defense details?
5. Can private firms become champions without INDHC ownership?
6. What debt counts as sovereign, holding-company, project, or private debt?
7. How are citizen dividend interests protected when a champion needs
   restructuring?
8. How are workers protected when privileges are withdrawn and a line closes?
9. How are politically exposed persons barred from hidden supplier control?
10. Which courts or tribunals handle supplier, citizen, and competitor appeals?

### Integration With INDHC

INDHC should own or co-own champion groups only where public coordination is
needed. It should also finance independent private champions and SMEs through
transparent credit and demand contracts. The success metric is not INDHC size.
The success metric is Iraqi productive capability, export performance, domestic
cost reduction, tax base growth, and citizen dividend capacity.

The governance rule is simple:

```text
No evidence, no privilege.
No export or cost discipline, no protection.
No audit, no public demand contract.
No competition review, no champion status renewal.
```


# Part 11: National Civic Work System

## National Civic Work System

As Iraq formalizes, automates, and industrializes through Digital IQD, INDHC,
and digitally governed industrial champions, some low-productivity work will
disappear. Cylinder Seal therefore includes a National Civic Work System: a
digitally verified, locally administered, dignity-preserving labor platform that
pays citizens for measurable social, environmental, cultural, sport, care,
education, municipal, food-security, and disaster-resilience work.

Status: policy-design scenario. It is not a welfare law, labor-market forecast,
budget appropriation, or implemented software module.

The objective is not to hide unemployment. The objective is to convert spare
labor capacity into public value.

### Source Discipline

| Public fact | Use in this design | Source |
| --- | --- | --- |
| World Bank WDI reports Iraq youth unemployment near 32% in recent modeled ILO estimates, including 31.8% in 2022 and 32.0% in 2025. | Civic work is designed as a transition and participation layer for youth and underemployed workers, not as a cosmetic add-on. | [World Bank WDI youth unemployment indicator](https://data.worldbank.org/indicator/SL.UEM.1524.ZS?locations=IQ) |
| IMF staff identify informality, lack of diversification, low financial inclusion, high reliance on cash, labor-market challenges, gender gaps, and structural obstacles to revenue mobilization in Iraq. | Civic work must create formal records, training history, payment evidence, and pathways into private employment rather than trapping people in low-value public work. | [IMF Iraq Selected Issues, 2024](https://www.imf.org/en/publications/cr/issues/2024/05/15/iraq-selected-issues-549033) |
| UNDP's Climate Vulnerability Index for Iraq integrates climate, socio-economic, and spatial data across all governorates and highlights impacts on water, agriculture, health, and infrastructure. | Civic work should prioritize climate adaptation, water resilience, heat response, food security, and local restoration. | [UNDP Climate Vulnerability Index of Iraq](https://www.undp.org/iraq/publications/climate-vulnerability-index-iraq) |
| UNDP describes climate-resilient agriculture in Iraq as reducing pressure on shared water resources and preventing disputes, and frames environmental action as a pathway to social cohesion. | Civic work should treat environmental restoration as public value, peacebuilding, and employability infrastructure. | [UNDP environmental action story, June 4, 2026](https://www.undp.org/stories/environmental-action-climate-peace-and-security) |

These sources support the need for a transition system. They do not validate the
payment levels, budget envelope, or institutional design below.

### Core Principle

The system must not feel like:

```text
You are unemployed, so go clean streets.
```

It should feel like:

```text
You are part of rebuilding Iraq, and your contribution is measured, paid,
respected, and visible.
```

This is a national participation economy, not old-style ministry payroll
expansion and not punitive workfare.

### Why Not Welfare Alone

Plain cash transfers can reduce poverty, but they do not by themselves create
purpose, skills, community repair, or a visible route from informal labor into
formal economic records.

The civic-work system is therefore designed to create several outputs at once:

- income for people who would otherwise be excluded from productivity gains;
- dignity through paid, visible, socially useful contribution;
- training records and certifications that improve employability;
- measurable community improvement in parks, schools, sport, care, heritage,
  water, food security, and environmental restoration;
- lower unrest risk by giving young people a respected participation ladder;
- formal income and reliability histories for workers with thin financial files.

The system should complement welfare, dividends, and ordinary employment. It
should not replace legal social protection, and it should not trap people in
permanent low-wage public tasks.

### Module Boundary

Proposed software module:

```text
cs-civic-work
```

Purpose:

- civic labor registry;
- task marketplace;
- verification engine;
- civic wage and credit payments;
- civic reputation and training records;
- public-impact dashboard;
- fraud and ghost-worker controls;
- privacy-bounded aggregate reporting.

The module connects to Digital IQD wallets but should not expose full payment
history to local supervisors. Task verification, wage payment, identity, and
reputation need separate permissions.

### Civic Work Flow

1. Municipality, school, sports club, NGO, environmental agency, health clinic,
   heritage authority, or approved community institution posts a task.
2. The task is checked against approved categories, wage rules, safety rules,
   budget availability, and verifier requirements.
3. Citizen accepts the task through a Civic Work Wallet linked to Digital IQD.
4. Citizen completes the work.
5. Evidence is submitted: supervisor approval, GPS check-in where lawful,
   timestamped photo, sensor evidence, peer validation, institutional sign-off,
   or output count.
6. Cylinder Seal calculates wage, credit bonus, training record, and reputation
   update.
7. Payment is released in Digital IQD.
8. Public dashboards show aggregate public value by district and category.

### Work Categories

| Sector | Examples of meaningful work | Public value signal |
| --- | --- | --- |
| Environment | Tree planting, riverbank cleanup, marsh restoration, anti-desertification, recycling, canal maintenance. | Survival rate, hectares restored, waste removed, canals cleared, heat-island reduction. |
| Social care | Elderly visits, disability support, childcare support, school meals, community health outreach. | Verified visits, care hours, referrals, meals delivered, missed-visit rate. |
| Sport | Local football coaching, girls' sport programs, youth leagues, public fitness events. | Teams supported, attendance, coach certification, female participation, safe-play compliance. |
| Culture | Heritage restoration, tourism guides, museum support, traditional crafts, local festivals. | Sites maintained, tours delivered, visitor ratings, craft income, preservation tasks. |
| Education | Literacy tutoring, after-school STEM clubs, vocational mentoring, homework support. | Tutoring hours, learner attendance, assessment gains, certification completions. |
| Municipal work | Street cleaning, park maintenance, public-space repair, lighting reports, pothole reports. | Streets cleaned, assets repaired, response time, citizen complaints resolved. |
| Food security | Urban farming, greenhouse support, date-palm care, irrigation monitoring, storage support. | Crop survival, irrigation checks, spoilage reduction, farmer support visits. |
| Disaster resilience | Flood response training, heatwave support teams, emergency supply distribution, first-aid teams. | Training completions, response drills, vulnerable-household checks, supplies delivered. |

### Payment Design

There are three payment types.

| Payment | Purpose | Guardrail |
| --- | --- | --- |
| Civic wage | Direct Digital IQD payment for verified work. | Paid only after task evidence passes; capped by hours, category, and local budget. |
| Civic credit bonus | Extra credit for transport, training, sports memberships, childcare, local goods, or housing deposits. | Spend categories are transparent and appealable; credits must not become hidden patronage. |
| Progression wage | Higher pay after verified certification. | Requires training certificate, task quality record, and periodic reassessment. |

Suggested progression ladder:

| Level | Requirement | Example wage logic |
| --- | --- | --- |
| Entry | Identity verified, task safety briefing completed. | Base civic wage. |
| Reliable | 40 verified hours, low dispute rate, supervisor or peer validation. | Base wage plus small reliability bonus. |
| Certified | First aid, coaching, irrigation, restoration, care, tutoring, construction safety, or tourism-guide certificate. | Progression wage for certified task categories. |
| Team lead | 200 verified hours, certification, no fraud flags, ability to supervise small crews. | Higher wage, but capped crew size and random audits. |
| Bridge-to-work | Employer, INDHC subsidiary, SME, school, municipality, or NGO apprenticeship offer. | Temporary wage support while transitioning to normal job or enterprise. |

The system should be a bridge into better work, not a permanent low-wage holding
pen.

### Civic Service Year

Iraq could create a voluntary Civic Service Year for people aged 18-30, with
paths for older participants, women returning to work, people with disabilities,
and displaced people where appropriate.

| Track | Work |
| --- | --- |
| Green Iraq Corps | Trees, marshes, canals, recycling, heat resilience, riverbank work. |
| Sports Iraq Corps | Coaching, school leagues, women's sport, community clubs, public fitness. |
| Care Iraq Corps | Elderly care, disability support, child services, health outreach. |
| Skills Iraq Corps | Tutoring, coding clubs, vocational workshops, apprenticeship support. |
| Heritage Iraq Corps | Archaeology support, tourism, culture, old-city restoration, festivals. |
| Municipal Iraq Corps | Parks, streets, public assets, neighborhood reporting, maintenance. |
| Food And Water Corps | Irrigation, farming support, greenhouses, water-saving campaigns. |
| Disaster Resilience Corps | Heatwave teams, flood drills, first aid, emergency supply distribution. |

After 12 months, participants receive:

- Digital IQD income history;
- verified work record;
- training certificates;
- preferential access to apprenticeships;
- mortgage or rent-support points where lawful;
- SME credit score boost;
- public recognition badge;
- optional transition interview with banks, SMEs, INDHC subsidiaries, schools,
  municipalities, or NGOs.

### Funding Model

Civic work should be funded by explicit appropriations and project budgets, not
by quietly raiding the citizen dividend pool.

Potential funding sources:

| Source | Use |
| --- | --- |
| Treasury social-transition allocation | Base civic wage, platform operations, verifier training. |
| Gross-profit levy share | Productivity gains from INDHC and champions can fund social-transition work. |
| Municipal service budgets | Parks, streets, waste, lighting, local repairs, community sports. |
| Climate adaptation and MDB grants | Water, heat, agriculture, restoration, disaster resilience. |
| INDHC project community budgets | Local maintenance, training, supplier outreach, environmental mitigation. |
| NGO or school co-funding | Care, tutoring, sports, heritage, local civic programs. |

Suggested accounting identity:

```text
Civic Work Budget
  = Treasury Social Transition Allocation
  + Eligible Municipal Service Budgets
  + Climate / MDB / Donor Co-Funding
  + Approved INDHC Community Benefit Budgets
  - Platform Operations
  - Verification And Audit Reserve
```

Policy rule:

```text
Citizen dividend funds are not civic-work payroll funds.
```

Dividends distribute capital returns. Civic wages pay verified public work.

### Verification Design

Verification should scale with task risk.

| Task risk | Example | Evidence |
| --- | --- | --- |
| Low | Park cleanup, public fitness event, festival support. | Supervisor sign-off, random photo sample, peer validation. |
| Medium | Tutoring, canal cleaning, tree maintenance, sports coaching. | Attendance log, location check, supervisor sign-off, output count, random audit. |
| High | Elderly care, childcare, disability support, disaster response, water infrastructure. | Certified worker, institutional sign-off, safety checklist, two-person verification, audit sampling. |
| Sensitive | Heritage sites, domestic violence support, child protection, protected wetlands, security-adjacent areas. | Restricted verifier list, privacy controls, no public location disclosure, specialized oversight. |

Photo, GPS, and biometric evidence must be lawful, proportionate, and
privacy-bounded. The system should not normalize surveillance for low-risk work.

### Governance Safeguards

The civic-work system can become corrupt unless designed carefully.

Required controls:

- no ghost workers;
- wallet-based attendance and lawful identity checks;
- random audits;
- public dashboards by district and category;
- NGO, school, university, and civil-society verification roles;
- photo, GPS, sensor, or supervisor evidence only where appropriate;
- grievance and appeal system;
- anti-nepotism controls;
- caps on local political appointments;
- verifier rotation;
- conflict-of-interest disclosure;
- worker safety rules and incident reporting;
- disability and gender-access review;
- independent audit by civil society, universities, and the supreme audit
  authority or equivalent.

### Dashboard Metrics

| Metric | Example |
| --- | --- |
| Active workers | Civic workers active this month, by governorate, age band, gender, and track. |
| Work completed | Verified civic hours, tasks completed, dispute rate, rejection rate. |
| Environmental output | Trees planted and maintained, canals cleared, waste removed, hectares restored. |
| Sport output | Youth teams supported, girls' sport sessions, coach certificates, attendance. |
| Care output | Elderly visits, disability support hours, childcare sessions, referrals. |
| Municipal output | Parks, streets, lighting reports, repairs, citizen complaints resolved. |
| Food and water output | Irrigation checks, greenhouse support, farmer visits, storage support. |
| Training output | Certificates issued, apprenticeships started, bridge-to-work placements. |
| Integrity output | Audit flags, ghost-worker attempts, verifier sanctions, appeal outcomes. |
| Fiscal output | Civic wage paid, cost per verified output, budget remaining, co-funding. |

### Data Model

Suggested primitives:

| Model | Purpose |
| --- | --- |
| `CivicWorkWallet` | Links a participant to Digital IQD payments, civic reputation, training, and work limits. |
| `CivicTask` | Approved unit of work with category, location rules, verifier, wage, budget, safety level, and expiry. |
| `CivicTaskPosting` | Institution request to create tasks, with budget source and approval state. |
| `CivicWorkClaim` | Worker claim that a task was performed. |
| `CivicEvidenceBundle` | Supervisor sign-off, photo, sensor, GPS, attendance, peer, or institutional evidence. |
| `CivicVerificationDecision` | Approved, rejected, held, disputed, or audit-required. |
| `CivicWagePayment` | Digital IQD payment for verified work. |
| `CivicCreditBonus` | Spend-limited bonus for transport, training, childcare, housing deposit, sport, or local goods. |
| `CivicReputationScore` | Reliability, certification, dispute, safety, and task-completion history. |
| `CivicCertificate` | Training or skill credential. |
| `CivicImpactMetric` | Public output measure linked to verified work. |
| `CivicAppeal` | Worker, verifier, or institution appeal. |
| `VerifierRegistry` | Approved supervisors, NGOs, schools, clubs, universities, agencies, and auditors. |

Suggested events:

| Event | Meaning |
| --- | --- |
| `TaskPosted` | Institution proposes work. |
| `TaskApproved` | Budget, safety, category, and verifier checks pass. |
| `TaskAccepted` | Worker accepts a task. |
| `EvidenceSubmitted` | Worker or verifier submits proof. |
| `TaskVerified` | Evidence passes. |
| `TaskRejected` | Evidence fails or task is invalid. |
| `PaymentReleased` | Civic wage or credit is paid. |
| `CertificateIssued` | Training credential is recorded. |
| `ReputationUpdated` | Civic score changes. |
| `AuditFlagRaised` | Fraud, ghost-worker, nepotism, or verifier abuse is suspected. |
| `AppealResolved` | Dispute is closed with reason code. |

### Privacy And Safety Boundaries

- Public dashboards show aggregates, not individual identities.
- Care, child, disability, domestic-violence, and sensitive heritage tasks need
  strict privacy controls.
- GPS should be coarse or time-limited unless high-risk work requires precision.
- Biometric use requires explicit legal authority, minimization, and appeal.
- Workers must be able to dispute false attendance or supervisor abuse.
- Safety training is mandatory before environmental, municipal, disaster, care,
  or child-facing work.
- No participant should lose ordinary welfare, dividend, or legal rights for
  refusing civic work.

### Ten-Year Rollout

| Phase | Years | Goal | Main work |
| --- | --- | --- | --- |
| Foundation | 0-1 | Define law, task categories, wage rules, privacy rules, and verifier registry. | Pilot in 3 governorates with municipal cleanup, sport, tutoring, and tree-care tasks. |
| Pilot | 1-2 | Prove verification, payments, anti-ghost-worker controls, and grievance process. | Add care, irrigation monitoring, heritage support, and disaster drills. |
| Scale | 3-5 | Launch Civic Service Year and connect training to apprenticeships. | Expand to all participating governorates; integrate banks, SMEs, INDHC Academy, and municipalities. |
| Productivity transition | 5-8 | Absorb workers displaced from low-productivity admin, informal middlemen, and inefficient logistics. | Add bridge-to-work wage support, certification ladders, and sector-specific civic corps. |
| Mature | 8-10 | Turn civic work into a permanent participation layer, not emergency relief. | Renew only programs with audited impact; retire low-value tasks and fund higher-skill tracks. |

### First 180 Days

| Month | Work |
| --- | --- |
| 1 | Draft civic-work charter, define dignity-of-work rules, ban punitive workfare, and list eligible institutions. |
| 2 | Define task taxonomy, wage bands, evidence tiers, privacy rules, and appeal process. |
| 3 | Build `CivicTask`, `CivicWorkWallet`, `CivicEvidenceBundle`, and `VerifierRegistry` schema proposals. |
| 4 | Select pilot districts and institutions: municipality, school, sports club, NGO, environmental agency, and health clinic. |
| 5 | Launch limited pilots for park maintenance, tutoring, sport coaching, tree care, canal cleanup, and elderly visits. |
| 6 | Publish first public dashboard with aggregate hours, outputs, payments, audits, and appeals. |

### Integration With Other Pillars

| Pillar | Integration |
| --- | --- |
| Digital IQD | Pays civic wages and credits, records income history, supports category-limited bonuses. |
| INDHC | Funds community-benefit work and creates bridge-to-work apprenticeships. |
| Industrial champions | Offer progression routes from civic training into supplier jobs, maintenance, tourism, food, water, and green sectors. |
| Ministry transition | Absorbs some staff and citizens into measurable public-value work rather than hidden payroll. |
| Credit scoring | Verified civic income, certificates, and reliability improve thin-file worker profiles. |
| Tourism | Heritage, guide, festival, and public-space work improve visitor experience and local income. |
| Green and rail | Civic work supports station-area maintenance, urban cooling, tree care, recycling, and public-space safety. |
| Dividend system | Dividends provide capital income; civic work provides paid participation and skill development. |

### Risks

| Risk | Mitigation |
| --- | --- |
| Becomes fake jobs. | Output metrics, task expiry, random audit, no payment without evidence. |
| Becomes punitive workfare. | Voluntary participation, no loss of legal rights for refusal, dignity charter, grievance path. |
| Becomes patronage. | Verifier rotation, anti-nepotism rules, public dashboards, local appointment caps. |
| Ghost workers appear. | Wallet-based attendance, evidence bundles, random audits, duplicate detection. |
| Supervisors abuse workers. | Appeals, worker ratings of verifiers, civil-society audit, sanctions. |
| Low-value tasks crowd out real jobs. | Wage bands below skilled market roles, bridge-to-work design, SME/private placement targets. |
| Privacy harms vulnerable groups. | Aggregate public reporting, sensitive-task controls, minimal GPS/photo use. |
| Budget becomes open-ended. | Explicit appropriation, task caps, cost-per-output review, sunset rules. |

### Build Sequence

1. Keep the civic-work architecture in policy-design status until legal,
   privacy, labor, and fiscal review are complete.
2. Add `CivicTask`, `VerifierRegistry`, `CivicEvidenceBundle`,
   `CivicWorkClaim`, and `CivicWagePayment` design models.
3. Add route-level prototype tests for task posting, evidence submission,
   verification, payment release, and appeal.
4. Add dashboard projections: active workers, verified hours, public outputs,
   audit flags, payments, and bridge-to-work outcomes.
5. Integrate with credit scoring only after privacy review.
6. Add legal review for labor law, child protection, care work, biometric use,
   data protection, municipal authority, and anti-corruption enforcement.

### Bottom Line

Productivity gains are socially legitimate only if citizens see a pathway from
lost low-value work into income, dignity, skill, and visible contribution.

The National Civic Work System makes that pathway measurable:

```text
productivity gains
  -> fiscal space and higher surplus
  -> verified civic work, training, care, restoration, sport, culture, and municipal repair
  -> income history, skills, public value, and social cohesion
  -> better private-sector and INDHC employability
```


# Part 12: Ministry Transition Roadmap

## Ministry Transition And Deprecation Roadmap

This document describes how the unified Cylinder Seal / INDHC model could
eventually deprecate ministry structures that become duplicative, low-feedback,
or primarily commercial once Digital IQD, INDHC subsidiaries, regulators,
municipal service contracts, and public dashboards are operating.

Status: governance-transition scenario. It is not a legal recommendation, not a
constitutional plan, and not a claim that any current ministry can be abolished
without parliamentary action, staff transition, service-continuity guarantees,
and public audit.

### Source Discipline

| Public fact | Use in this roadmap | Source |
| --- | --- | --- |
| Iraq's cabinet structure is politically fluid. AP reported on May 14, 2026 that parliament approved 14 ministers in a 23-member cabinet while several posts were delayed or rejected. | Any ministry list must be verified at transition launch; this roadmap uses ministry functions, not named officeholders. | [AP News, May 14, 2026](https://apnews.com/article/iraq-cabinet-parliament-government-10d14e41dd8a5c54d982874aeece4005) |
| The Embassy of Iraq's government links list federal ministry websites, including Oil, Finance, Trade, Planning, Electricity, Agriculture, Communications, Construction and Housing, Health, Education, Higher Education, Transportation, Industry and Minerals, Water Resources, Culture, Labor and Social Affairs, Migration and Displacement, and Youth and Sports. | Provides a practical baseline list of current/recent federal ministry functions. | [Embassy of Iraq government links](https://iraqiembassy.us/government-links/) |
| Iraq's government formation has historically treated some ministries as "sovereign" and ministry allocation can follow political power-sharing logic. | The roadmap preserves sovereign functions and tries to reduce ministry-as-patronage incentives. | [Washington Institute, Minister Allocation in Iraq's New Government](https://www.washingtoninstitute.org/policy-analysis/minister-allocation-iraqs-new-government) |

These sources support the institutional baseline. They do not validate the legal
feasibility of any transition.

### Principle

Deprecating a ministry does not mean deleting the public service.

It means moving from:

```text
ministry as budget claimant
```

to:

```text
regulator + digital payments + service contracts + public dashboard + audited
operator
```

The state keeps sovereign authority. It removes automatic budget entitlement,
opaque procurement, duplicated administration, and weak feedback.

### No-Abolition Gate

No ministry can be deprecated until all gates pass:

1. Parliament or competent legal authority approves the transfer.
2. Essential service continuity is proven for at least 12 months.
3. Replacement entity has a published mandate, budget, audit process, and appeal
   path.
4. Staff are mapped into regulator, municipality, INDHC, private operator,
   school, hospital, or retraining pathways.
5. Procurement, payroll, and service payments are visible in Cylinder Seal.
6. Citizen complaint and appeal channels are live.
7. Anti-corruption, conflict-of-interest, and beneficial-ownership controls are
   in force.
8. Independent audit confirms the transition does not hide debt, service cuts, or
   politically directed layoffs.

### Permanent Sovereign Core

These functions should not be deprecated. They may be modernized, audited, and
digitized, but they remain core state functions.

| Function | Future form |
| --- | --- |
| Foreign affairs | Ministry or sovereign diplomatic service. |
| Defense | Ministry of Defense plus lawful defense procurement and oversight. INDHC may support regulated sustainment and industrial supply, not command authority. |
| Interior / policing | Ministry of Interior or equivalent public-security authority with civil-rights safeguards and audit. |
| Finance / Treasury | Treasury, debt office, tax authority, budget office, and fiscal-risk monitor. |
| Justice | Courts, corrections, legal registry, administrative justice, and appeal mechanisms. |
| Public health authority | Health ministry may shrink as provider operations become autonomous, but epidemic control, standards, licensing, and health equity remain sovereign. |
| Education standards | School operations can decentralize, but curriculum standards, child protection, teacher accreditation, and equal access remain public duties. |
| Environmental and water sovereignty | Environment, water rights, basin negotiations, pollution control, and climate adaptation require a strong regulator even if service delivery is corporatized. |

### Candidate Ministry Deprecation Register

Timelines are counted from legal launch of the unified model, not from the date
of this document.

| Current / recent ministry function | Why ministry form becomes redundant | Replacement home | Transition timeline | Formal deprecation target | Hard gate |
| --- | --- | --- | --- | --- | --- |
| Planning | Real-time project dashboards, investment ledgers, and performance budgets replace static central planning. | National Performance and Investment Office under Treasury/PMO; Cylinder Seal economic projections. | Years 1-2: publish project registry. Years 3-4: move capital-budget scoring to performance office. | Year 5 | All public investment projects have digital milestones and audit trails. |
| Industry and Minerals | Commercial industrial policy moves to INDHC subsidiaries, SME finance, standards, and competition rules. | INDHC Materials/Chemicals/Strategic Manufacturing; Industrial Standards and Competition Authority. | Years 1-2: SOE inventory. Years 3-5: transfer viable assets to subsidiaries; close or restructure loss-makers. | Year 6 | Every transferred asset has audited accounts and no hidden payroll liability. |
| Trade | Digital procurement, strategic reserves, customs data, Digital IQD merchant rails, and targeted vouchers replace broad trade-ministry administration. | Trade Facilitation and Strategic Reserves Agency; Treasury; digital PDS/voucher platform. | Years 1-3: digitize PDS and import licenses. Years 4-6: move procurement to transparent platforms. | Year 7 | Strategic food reserves and voucher payments operate for 12 months without service failure. |
| Electricity | Generation, distribution, renewable PPAs, metering, and grid services become regulated operating companies, not a classic ministry. | Energy regulator; grid operator; GENCO/DISCO companies; INDHC Green Power. | Years 2-4: regulator and metering baseline. Years 5-7: corporatize assets. | Year 8 | Grid reliability, tariff protection, and PPA obligations are independently audited. |
| Communications | Telecom and digital infrastructure should be regulated, not ministry-operated. | Digital Communications Regulator; corporatized/public-private operators; Digital IQD infrastructure office. | Years 1-2: license audit. Years 3-5: operator separation. | Year 6 | Universal-service obligations and cybersecurity regulator are live. |
| Transport | Rail, ports, airports, and logistics can operate as regulated companies and authorities. | Transport Safety and Network Regulator; INDHC Rail and Urban Mobility; port/airport/rail companies. | Years 2-4: asset registry. Years 5-8: operational transfer and performance contracts. | Year 9 | Safety regulator is independent and service contracts meet uptime/ridership targets. |
| Construction, Housing, Municipalities, and Public Works | Delivery belongs to municipalities, utilities, housing finance, contractors, and INDHC urban services. | Municipal Finance and Urban Standards Authority; local governments; INDHC Materials/Urban Services. | Years 2-5: municipal service contracts. Years 6-9: move works delivery to operators. | Year 10 | Municipal finance, maintenance, and complaint systems operate in all participating governorates. |
| Agriculture | Input subsidies, irrigation equipment, cold chain, food processing, and farmer credit can be delivered through INDHC, banks, and extension platforms. | Food and Water Productivity Agency; INDHC Food/Cold Chain; INDHC Water/Irrigation; bank credit. | Years 2-4: farmer registry and water-efficient equipment finance. Years 5-8: move input programs to voucher/service contracts. | Year 9 | Food security reserves and farmer payments are stable across two crop cycles. |
| Water Resources | Water is too critical to abolish as a sovereign function, but construction/service delivery should move to operators and water-efficiency contracts. | Water Security and Basin Authority; INDHC Water/Irrigation; municipal utilities. | Years 2-5: water accounting and basin data. Years 6-9: transfer works and equipment programs to operators. | Year 10 as service ministry; regulator remains | Basin, desalination, irrigation, and water-rights functions are legally protected. |
| Oil | Oil should stop funding ministries directly. The ministry form shrinks as lockbox, regulator, national oil companies, and INDHC handle receipts and investment. | Petroleum Resource Regulator; Oil Income Lockbox; national oil companies; Treasury. | Years 1-3: lockbox and revenue rules. Years 4-7: separate regulator from operators. | Year 8 as revenue allocator; regulator remains | Oil receipts flow through lockbox for 24 months with audited reconciliation. |
| Culture / tourism-adjacent functions | Commercial tourism and cultural services can be managed by heritage authorities, municipalities, and INDHC tourism platforms. | Heritage and Creative Economy Authority; INDHC Tourism and Services; local governments. | Years 2-4: heritage registry and tourism platform. Years 5-6: transfer commercial tourism programs. | Year 7 | Heritage protection remains independent from commercial tourism revenue. |
| Youth and Sports | Sports facilities, youth grants, and community programs can be municipal/service-contract functions. | Community Sport and Youth Grants Agency; municipalities; schools; civil society contracts. | Year 1: grant registry. Years 2-3: municipal transfer. | Year 4 | Youth access targets and anti-patronage grant controls are live. |
| Migration and Displacement | If displacement caseloads decline, a full ministry should sunset into a disaster/displacement agency. | Disaster, Return, and Social Reintegration Agency under Interior/Labor/Treasury coordination. | Years 2-5: case closure and benefit digitization. Years 6-9: transfer residual cases. | Year 10 | No unresolved protected caseload is abandoned. |
| Labor and Social Affairs | Routine cash transfers and benefits move to Digital IQD; labor standards and social insurance remain. | Labor Standards Authority; Social Insurance and Dividend Interface; Treasury transfer platform. | Years 3-5: digitize benefits. Years 6-9: integrate with dividends, pensions, and labor inspection. | Year 10 as broad welfare ministry; standards/insurance remain | Vulnerable households are protected and appeals are live. |
| Higher Education and Scientific Research | Universities and research institutes should become autonomous institutions funded by transparent formulas and competitive grants. | Higher Education Accreditation and Research Funding Council; autonomous universities; INDHC Academy/R&D. | Years 4-6: autonomy framework. Years 7-9: funding formula and grants. | Year 10 as operating ministry; accreditation remains | Equal access, accreditation, and research integrity are protected. |
| Science and Technology, where still separate | R&D should be integrated into universities, standards labs, INDHC applied research, and grant councils. | Research Funding Council; standards labs; INDHC R&D; universities. | Years 1-2: portfolio audit. Years 3-4: merge grants/labs. | Year 5 | Research assets and staff have receiving institutions. |

### Ministries To Transform, Not Deprecate In Ten Years

Some ministries should become smaller and more measurable, but not be deprecated
within the ten-year plan.

| Ministry function | Ten-year direction |
| --- | --- |
| Health | Convert hospitals and procurement into autonomous trusts/service contracts where feasible, but keep public health, licensing, emergency preparedness, epidemic response, and health-equity authority. |
| Education | Decentralize school operations to local boards and digital funding formulas, but keep standards, curriculum, child protection, teacher accreditation, and national equity funding. |
| Environment | Strengthen into a climate/environment regulator, potentially merged with water-risk oversight, but do not bury it inside INDHC or industry ministries. |
| Defense, Interior, Justice, Foreign Affairs, Finance | Modernize and audit; do not deprecate. |

### Ten-Year Transition Timeline

| Phase | Years | Main action | Ministries/functions affected |
| --- | --- | --- | --- |
| Phase 0: legal baseline | 0-1 | Verify cabinet structure, pass transition law, create service-continuity and staff-protection rules. | All ministries. |
| Phase 1: visibility | 1-2 | Publish budgets, payroll, procurement, project milestones, service metrics, and staff maps in Cylinder Seal. | Planning, Trade, Industry, Electricity, Communications, Youth/Sports, Oil. |
| Phase 2: quick deprecation | 2-4 | Deprecate ministries with mostly grant, coordination, or duplicated planning functions. | Youth/Sports by Year 4; Science/Technology by Year 5 if still separate; Planning prepared for Year 5. |
| Phase 3: commercial transfer | 3-6 | Move commercial/industrial/service delivery into INDHC subsidiaries, municipalities, operators, and regulators. | Industry and Minerals, Communications, Trade, Culture/Tourism functions. |
| Phase 4: infrastructure corporatization | 5-8 | Corporatize generation, distribution, transport, water works, housing works, and project delivery. | Electricity, Transport, Construction/Housing/Municipalities, Water Resources service functions. |
| Phase 5: fiscal conversion | 7-10 | Ministries become regulators, standard-setters, purchasers, or sunset agencies; direct oil-funded bureaucracy is mostly gone. | Oil as revenue allocator, Agriculture service programs, Labor/Social Affairs broad welfare ministry, Migration/Displacement, Higher Education operations. |

### Budget And Cashflow Transition

Illustrative share of affected ministry operating/program budgets moved from
direct ministry administration into service contracts, regulators, municipalities,
INDHC operating companies, digital transfers, or sunset funds.

| Year | Direct ministry administration | Contracted / regulated service delivery | Regulator and standards budget | Staff transition / retraining | Notes |
| --- | ---: | ---: | ---: | ---: | --- |
| 1 | 95% | 2% | 2% | 1% | Visibility first; no abrupt cuts. |
| 2 | 88% | 6% | 3% | 3% | Youth/Sports, Science/Technology, and Planning pilots. |
| 3 | 78% | 13% | 4% | 5% | Trade/PDS, Industry asset transfer, Communications operator separation. |
| 4 | 68% | 22% | 5% | 5% | First formal deprecations possible after audit gates. |
| 5 | 57% | 33% | 5% | 5% | Planning and Science/Technology functions merged or sunset. |
| 6 | 47% | 42% | 6% | 5% | Industry and Communications ministry form can end if gates pass. |
| 7 | 38% | 50% | 7% | 5% | Trade and Culture/Tourism commercial functions transfer. |
| 8 | 30% | 58% | 7% | 5% | Electricity and Oil revenue-allocation ministry forms shrink sharply. |
| 9 | 24% | 64% | 7% | 5% | Agriculture/Transport/Water service transfers mature. |
| 10 | 20% | 68% | 7% | 5% | Residual state functions are regulators, purchasers, courts, security, diplomacy, treasury, health/education standards. |

Rules:

- Budget movement is not a spending cut unless Parliament makes it one.
- Savings from administrative duplication first fund maintenance, staff
  transition, service continuity, debt reduction, and audit capacity.
- Staff are transitioned before ministry form is deprecated.
- No ministry budget is moved to dividends. Dividends come from audited INDHC
  distributable surplus.

### Staff Transition

Deprecation must not become arbitrary mass dismissal. It should create a path
from low-feedback bureaucracy into productive, regulated, or local service work.

| Staff group | Preferred destination |
| --- | --- |
| Engineers, technicians, project managers | INDHC subsidiaries, regulators, municipal utilities, rail/power/water operators. |
| Procurement staff | Public procurement authority, audit office, project-contract management teams. |
| Social-service case workers | Digital benefits offices, municipal service centers, disaster/return agency. |
| Teachers, lecturers, researchers | Schools, autonomous universities, accreditation bodies, grant councils, INDHC Academy. |
| Inspectors | Independent regulators, standards labs, environmental/water/food safety authorities. |
| Administrative staff | Retraining into digital service centers, audit support, records management, citizen appeal desks. |

### Cylinder Seal Implementation Surface

| Model | Purpose |
| --- | --- |
| `MinistryFunctionRegistry` | Lists each ministry function, legal basis, budget, staff, service metrics, and replacement home. |
| `DeprecationGate` | Tracks legal approval, service continuity, audit, staff transition, and citizen appeal readiness. |
| `ServiceContractBudget` | Moves budget from ministry payroll/procurement into priced outputs and milestones. |
| `RegulatorMandate` | Defines what remains as sovereign regulation or standards authority. |
| `StaffTransitionLedger` | Tracks staff transfer, retraining, compensation, and receiving institution. |
| `CitizenServiceContinuityMetric` | Measures whether citizens still receive the service after transition. |
| `MinistrySunsetAudit` | Final audit before formal deprecation or merger. |

### Bottom Line

The unified model should make many ministry forms unnecessary over time, but not
because public services disappear. They become unnecessary because services are
delivered through audited operators, regulators, digital transfers, municipal
contracts, INDHC subsidiaries, and public dashboards.

The end-state is a smaller cabinet with stronger sovereign functions:

- Treasury and tax;
- justice and courts;
- defense, interior, and foreign affairs;
- public health and education standards;
- environmental/water sovereignty;
- independent regulators;
- audited service contracts;
- productive national capital through INDHC.



# Part 13: Technical Primitives

## Technical Primitives And Readiness Notes

This document maps the main technical claims to code that exists today and to the production gaps that still need to be closed. It is intentionally conservative: if something has a prototype implementation but lacks deployment hardening, it is marked as partial.

For visual maps of the software architecture, transaction lifecycle, and financial-flow combinations, see [System And Financial Flow Diagrams](system-and-financial-flow-diagrams.md).

### Summary

| Primitive | Current evidence | Readiness |
| --- | --- | --- |
| Offline NFC/BLE/QR payments | `crates/cs-mobile-core/src/wire.rs`, `crates/cs-pos/src/payment.rs`, `crates/cs-pos/src/nfc.rs`, `crates/cs-pos/src/ble.rs`, `crates/cs-tests/tests/e2e_offline_payment.rs`, `crates/cs-tests/tests/spec_12_wire_formats.rs` | Partial |
| Double-spend and conflict resolution | `crates/cs-sync/src/conflict_resolver.rs`, `crates/cs-tests/tests/spec_13_conflict_resolution.rs`, KYC offline limits in `crates/cs-core/src/models.rs` | Partial |
| Transaction envelope and wire format | `crates/cs-core/src/models.rs`, `crates/cs-core/src/primitives.rs`, `crates/cs-mobile-core/src/wire.rs`, `crates/cs-tests/tests/spec_02_canonical_signing.rs`, `crates/cs-tests/tests/spec_12_wire_formats.rs` | Implemented for prototype |
| Programmable transfer validation | `crates/cs-policy/src/primitives.rs`, `crates/cs-sync/src/sync_service.rs`, `crates/cs-sync/src/state_machine.rs`, `crates/cs-tests/tests/spec_22_programmability_primitives.rs` | Partial |
| Consensus boundary | `crates/cs-consensus`, `crates/cs-sync/src/sync_service.rs`, `crates/cs-sync/src/raft_transport.rs`, `crates/cs-tests/tests/spec_05_raft_consensus.rs` | Partial |
| AML and risk workflow | `crates/cs-policy/src/aml.rs`, `crates/cs-policy/src/rule_engine.rs`, `crates/cs-policy/src/reporting.rs`, `crates/cs-policy/src/risk_scoring.rs`, `crates/cbi-dashboard/src/routes/risk.rs`, `crates/cbi-dashboard/src/routes/compliance.rs` | Partial |
| Key management | `crates/cs-core/src/cryptography.rs`, `crates/cs-core/src/hardware_binding.rs`, `crates/cs-pos/src/store.rs`, `crates/cs-node/src/admin_bootstrap.rs` | Prototype only |
| Privacy controls | `crates/cs-core/src/location.rs`, dashboard role/session modules, aggregate analytics modules | Early |
| Disaster recovery | Raft abstractions and append-only persistence concepts exist, but no production runbooks or recovery tests are present | Not production-ready |

### Offline Payment Lifecycle

Prototype flow:

1. A wallet or POS builds a transaction using the shared core model.
2. The sender signs the canonical payload with Ed25519.
3. The payload is encoded for QR, NFC APDU, or BLE transport.
4. The receiving device verifies and queues the transaction locally.
5. On sync, `ChainSyncService` validates signatures, nonce continuity, policy primitives, and conflict status before proposing to the consensus layer.
6. The Raft state machine applies committed entries to storage.

Evidence:

- `crates/cs-tests/tests/e2e_offline_payment.rs` covers sign, encode, decode, and verify for an offline NFC-style flow.
- `crates/cs-tests/tests/spec_12_wire_formats.rs` asserts wire invariants for QR/NFC/BLE fallback behavior.
- `crates/cs-pos/src/payment.rs` and `crates/cs-pos/ui/main.slint` implement the POS-facing tender flow.

Remaining work:

- Hardware secure-element integration for offline counters.
- Device recovery and stolen-device revocation.
- Real mobile/POS interoperability testing across Android, iOS, and physical POS hardware.
- Formal offline value and velocity limits by user tier, region, and risk state.

### Double-Spend Detection And Reconciliation

Prototype behavior:

- `crates/cs-sync/src/conflict_resolver.rs` detects sibling entries in a nonce/hash chain.
- Earlier timestamps win as a soft heuristic.
- If timestamps tie, NFC evidence ranks above BLE, and BLE ranks above online.
- KYC tiers cap offline transaction and per-device daily offline exposure in `crates/cs-core/src/models.rs`.

Remaining work:

- The current design detects and reconciles conflicts at sync time; it does not yet prove that a compromised device cannot create conflicting offline spends before reconnection.
- Production needs secure monotonic counters, certified hardware binding, tamper evidence, and clear consumer-liability rules.
- The reconciliation policy needs legal and supervisory approval because it decides which offline recipient is made whole.

### Transaction Envelope And Wire Format

Prototype behavior:

- Core transaction structures live in `crates/cs-core/src/models.rs`.
- Canonical signing and hashing are covered by `crates/cs-tests/tests/spec_02_canonical_signing.rs`.
- Mobile/POS transports share codecs through `crates/cs-mobile-core/src/wire.rs`.
- Programmability fields are optional so ordinary retail payments can retain a stable wire shape.

Remaining work:

- Version negotiation for future transaction schema upgrades.
- Golden test vectors published outside Rust.
- Compatibility tests for generated mobile bindings.
- Formal schema documentation for external integrators.

### Programmable Transfer Validation

Prototype behavior:

- Expiry, spend constraint, and release condition primitives are modeled in `crates/cs-core/src/primitives.rs`.
- Policy evaluation lives in `crates/cs-policy/src/primitives.rs`.
- `ChainSyncService::validate_primitives` checks primitives before entries are proposed to Raft.
- `LedgerApplier::persist` records sidecar primitive rows for committed transactions.
- `crates/cs-tests/tests/spec_22_programmability_primitives.rs` covers expiry tamper, impostor counter-signer, replay-to-different-transaction rejection, and composed primitives.

Remaining work:

- Rule-governance workflow for CBI approval of new primitive semantics.
- User-facing disclosure and recourse for restricted transfers.
- Integration tests against actual dashboard rule-management screens.

### Consensus Boundary

Prototype behavior:

- `cs-consensus` implements Raft protocol types, leader election, log replication, and commit-index tracking.
- `cs-sync` treats Raft as the finality boundary for ledger persistence.
- Spec tests cover quorum math and basic Raft behavior.

Important limitation:

- Raft is crash-fault tolerant, not Byzantine-fault tolerant.
- The repository currently has a gRPC transport bridge, but production-grade inter-super-peer deployment and operational testing remain incomplete.

Remaining work:

- Real five-node regional deployment tests.
- Persistent Raft log storage and recovery testing.
- Network partition, clock skew, and rolling upgrade drills.
- Clear language in external materials: "3-of-5 Raft CFT", not "Byzantine consensus."

### Key Management

Prototype behavior:

- Wallet transaction signing uses Ed25519 primitives.
- POS merchant key storage has local persistence.
- Admin bootstrap generates one-time supervisor credentials using Argon2id hashes.

Remaining work:

- HSM or secure enclave custody policies for super-peer and operator signing keys.
- Device attestation for mobile/POS wallet keys.
- Key recovery and inheritance flows for citizens and merchants.
- Rotation, revocation, and audit evidence for all privileged keys.

### Privacy Model

Prototype behavior:

- Location coarsening exists in `crates/cs-core/src/location.rs`.
- Dashboard routes separate operator sessions and roles.
- Analytics modules aggregate sector and import-substitution data.

Remaining work:

- A written privacy model separating identity, payment content, location, AML access, and aggregate economic analytics.
- Data minimization by endpoint and role.
- Retention schedules and legal hold policy.
- External privacy impact assessment before any real citizen data is used.

### AML And Risk Workflow

Prototype behavior:

- AML screeners, configurable rule engine, user risk scoring, and regulatory reporting models exist in `cs-policy`.
- Dashboard modules expose risk queue, compliance reports, account freeze/unfreeze, audit logs, and emergency directives.
- Spec tests cover AML flagging, rule evaluation, risk scoring, and reporting.

Remaining work:

- Live sanctions feed operational runbooks.
- Four-eyes approval for sensitive compliance actions.
- Case-management UX beyond the JSON/API prototype.
- Supervisor audit review and exportable regulator evidence packs.

### Disaster Recovery

Current state:

- The code has consensus abstractions, append-only ledger concepts, and storage migrations.
- There is no complete disaster-recovery plan in the repo.

Required before production:

- Recovery point objective and recovery time objective by service.
- Backup encryption and restore verification.
- Regional failover exercises.
- Key ceremony and break-glass runbooks.
- Immutable audit log retention plan.


# Legacy Policy Paper Boundary

The long-form `docs/policy-paper.md` is intentionally not included as a main
ebook chapter. It is a legacy scenario workbook and narrative archive, not the
implementation status of the repository and not an externally validated
forecast.

Do not quote its dollar ranges, adoption rates, sovereign-rating paths,
timelines, employment figures, or GDP paths as validated project claims. Use the
README, implementation status, economic assumptions, unified economic model,
security model, and newer institutional-design documents as the front-door
position.
