# Cylinder Seal

Sovereign digital-payment and economic-visibility prototype for Iraq.

Cylinder Seal explores how CBI-backed digital IQD payment rails could support financial inclusion, SME credit scoring, public-transfer controls, domestic-production incentives, and regulator-grade economic dashboards. It is a working Rust prototype and policy architecture, not production CBDC infrastructure and not an official Central Bank of Iraq project.

![Cylinder Seal architecture](1776870497788.png)

## What This Repo Contains

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

## Current Status

| Status | Scope |
| --- | --- |
| Implemented | Rust domain models, canonical signing primitives, transaction/wire-format primitives, KYC tier limits, POS/mobile codecs, PostgreSQL-backed CBI dashboard routes, AML/risk/credit modules, and numbered specification tests. |
| Partially implemented | Offline payment lifecycle, double-spend reconciliation, merchant-tier policy, transaction-based credit scoring, AML reporting, dashboard UI, and Raft-backed sync. These have code and tests, but need production integration and security hardening. |
| Not production-ready | HSM or secure-element custody, national identity/KYC integration, audited offline double-spend prevention, real multi-peer Raft deployment, CBI/core-banking integration, privacy review, disaster recovery, formal threat model review, and externally validated economic impact model. |

The codebase should be read as a pilot-grade prototype. It is suitable for technical review, policy exploration, and demo workflows. It should not be represented as ready for national-scale deployment.

## Quick Start

Install Rust and Docker if you want to run the dashboard stack locally. The dashboard currently uses PostgreSQL and Redis; SQLite files in this repository are legacy/local fixture helpers, not a supported dashboard runtime.

```bash
# Start PostgreSQL and Redis.
cp .env.example .env
docker compose up -d

# Build the main dashboard package.
cargo build --package cbi-dashboard

# Run the dashboard.
export DATABASE_URL="postgresql://postgres:${DB_PASSWORD:-change-me-dev-only}@localhost:5432/cylinder_seal"
cargo run --package cbi-dashboard
```

The dashboard defaults to `http://127.0.0.1:8081` when run locally. Demo operators are seeded only for local development; see `.env.example` and `API_REFERENCE.md` before using them.

Change all demo secrets before sharing, deploying, or connecting real systems.

## Technical Evidence

The public-facing technical evidence has been split out of the original long README:

- [Technical primitives](docs/technical-primitives.md) maps claims such as offline payments, double-spend checks, wire-format primitives, Raft, key handling, privacy, AML, and disaster recovery to code and remaining gaps.
- [System and financial flow diagrams](docs/system-and-financial-flow-diagrams.md) provides rendered SVG architecture diagrams, transaction lifecycles, and valid financial-flow combinations.
- [Implementation status](IMPLEMENTATION_STATUS.md) summarizes dashboard implementation state.
- [Specification and fixture results](SPECIFICATION_AND_FIXTURE_RESULTS.md) and [cs-tests README](crates/cs-tests/README.md) describe current test evidence and the missing live PostgreSQL/Redis coverage.
- [API reference](API_REFERENCE.md) documents the dashboard API.

## Economic And Policy Framing

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

## Production Readiness Boundary

Before this could be evaluated as real payment infrastructure, the project would need at minimum:

- A formal threat model for wallets, POS devices, offline settlement, super-peers, operator access, and emergency controls.
- Hardware-backed key custody and recovery design.
- Offline double-spend limits backed by secure monotonic counters or equivalent attestation.
- Privacy architecture separating payment data, identity data, regulatory access, and aggregate economic analytics.
- Real multi-node consensus deployment with operational runbooks and failover tests.
- Independent security audit, compliance review, and economic model validation.

## Repository Hygiene

Local artifacts such as generated databases, Redis dumps, virtualenvs, and ad hoc logs are ignored. Do not commit generated database state.
