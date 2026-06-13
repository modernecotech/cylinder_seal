# Implementation Status

## Honest Status

Cylinder Seal is a prototype implementation with meaningful code coverage across domain modules, policy logic, dashboard routes, and tests. It is not production-ready CBDC/payment infrastructure.

## Implemented Or Partially Implemented

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
- National Legal and Institutional Roadmap documented as an authority checklist for the oil lockbox, INDHC, citizen entitlements, Digital IQD, project debt, securities, privacy, federalism, emergency powers, and appeals.
- Project Pipeline and Investment Gates documented as a project-family structure for capex, revenue sources, DSCR, FX exposure, facility reuse, environmental gates, legal authority, and evidence bundles.
- Political-Economy Transition and Anti-Capture model documented as a reform-readiness layer for capture risk, resistance pressure, coalition support, service continuity, staff transition, procurement transparency, beneficial ownership, competition controls, federalism, emergency powers, and citizen appeals.
- Federalism, Governorate Equity, and Local Compacts documented as a scale-control layer for authority mapping, compact status, allocation fairness, local revenue, local employment, supplier participation, benefit capture, grievances, local audit, citizen appeals, and land/water/heritage disputes.
- Fiscal Stress and Contingent Liability model documented as a downside-control layer for stressed oil-equity capacity, DSCR, FX mismatch, maintenance gaps, guarantees, availability payments, collections, capex overruns, and dividend affordability.
- National Program Sequencing and Dependency Control documented as a phase-control layer for not-ready, evidence-only, pilot, build, controlled-scale, and hold/rollback decisions.
- Benefit Realization and Claim Audit documented as an accountability layer for baseline, target, observed value, source confidence, attribution confidence, evidence quality, audit status, cash settlement, no-dividend flags, and corrective actions.
- Procurement Integrity and Market Discipline documented as an award/privilege-control layer for beneficial ownership, competition, single-source justification, open data, independent evaluation, price benchmarks, contract changes, advances, milestone evidence, delivery, payment discipline, quality, and SME participation.
- Scenario analytics now cover the economic operating kernel, sovereign holding plan, economic cycle, growth impact, comprehensive benefits, production capacity, strategic resilience, tourism services, diaspora channels, facility recycling, political-economy transition readiness, federalism/governorate equity, fiscal stress, program sequencing, benefit realization, and procurement integrity. These are planning primitives, not calibrated national forecasts.
- Rendered SVG value-chain charts now cover sector business chains, capital and repayment lanes, and society/economy feedback loops.

## Important Gaps

- Dashboard route integration tests now cover security middleware and selected skeletal handlers, but live PostgreSQL/Redis endpoint coverage is still incomplete.
- Some route handlers remain skeletal or demo-oriented.
- Role enforcement must continue to be applied as future sensitive handlers are implemented.
- Browser security hardening has a basic cookie/CSRF foundation but still needs full browser automation and deployment review.
- Admin audit logging is not yet a complete immutable retention system or regulator evidence-pack workflow.
- The unified economic model has scenario engines and persistence tables for several layers, but it is still not calibrated, independently validated, or suitable for real budget, debt, investment, or dividend decisions.
- The ministry transition roadmap is not implemented and would require legislation, civil-service transition, federal/governorate coordination, service-continuity testing, and independent audit before any real-world use.
- SQLite is not a supported dashboard runtime despite the presence of historical fixture scripts.
- Offline double-spend handling is not yet backed by audited secure hardware/attestation.
- HSM custody, national identity/KYC integration, CBI/core-banking integration, DR, and privacy review remain future work.
- The National Dividend Holding Company proposal and ten-year investment plan are only partially represented in scenario code and need legal, fiscal, debt-capacity, AML/CFT, competition, and governance validation before data models or routes are treated as deployable.
- The National Civic Work System is not implemented in code and needs labor-law, privacy, child-protection, municipal-authority, anti-corruption, disability-access, and fiscal validation before any `cs-civic-work` module is treated as deployable.
- The legal/institutional roadmap and project pipeline are documentation layers only; they need independent legal opinions, official authority mapping, project feasibility studies, procurement review, environmental/social safeguards, and audited baseline data before any project is treated as investable.
- The political-economy engine is scenario scoring, not a real power map or legitimacy assessment. It needs independent Iraqi constitutional, federalism, anti-corruption, civil-service, labor, competition, citizen-rights, and security-sector review before it can guide any real transition.
- The federalism equity engine is a compact-readiness screen, not a legal allocation formula or constitutional settlement. It needs official authority maps, governorate/KRG/municipal review where applicable, real population and needs data, grievance records, land/water/heritage status, and independent legal validation before it can guide any allocation or project scale-up.
- The fiscal stress engine is a planning control, not a sovereign debt-sustainability analysis. It needs Ministry of Finance, debt-office, CBI, IMF/MDB-style, auditor, and project-finance validation before any real capital allocation or dividend decision.
- The program sequencer is a dependency-control model, not an official rollout plan. It needs real institutional owners, statutory milestones, operator readiness testing, citizen consultation, and independent PMO review before any public timeline is claimed.
- The benefit-realization engine is a claim-audit model, not proof that benefits exist. It needs real baselines, audited source systems, attribution methods, evaluator independence, and publication governance before any outcome is treated as delivered.
- The procurement integrity engine is a screening primitive, not a legal procurement decision or debarment system. It needs procurement-law review, official supplier data, beneficial-owner registries, sanctions/PEP feeds, price benchmarks, bid records, and independent audit before operational use.

## Running The Dashboard

```bash
cp .env.example .env
docker compose up -d
export DATABASE_URL="postgresql://postgres:${DB_PASSWORD:-change-me-dev-only}@localhost:5432/cylinder_seal"
cargo run --package cbi-dashboard
```

## Verification

```bash
cargo check --package cbi-dashboard
cargo test --workspace
```

Passing these checks should be described as prototype verification, not production certification.
