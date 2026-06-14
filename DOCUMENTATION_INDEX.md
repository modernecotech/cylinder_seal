# Documentation Index

## Start Here

- `README.md` - economic-system front door, source discipline, production-readiness boundary, and software appendix.
- `INSTITUTIONAL_BRIEF_AR.md` - Arabic institutional brief for CBI, Ministry of Finance, MDB/development-finance, Iraqi investor, and diaspora readers.
- `INSTITUTIONAL_BRIEF.md` - two-page institutional brief for CBI, Ministry of Finance, MDB/development-finance, Iraqi investor, and diaspora readers.
- `PILOT_DESIGN.md` - Samawah / Al-Muthanna minimum viable jurisdiction pilot with Open Source Rail enabling works, 90-day, 180-day, 12-month, and executable stop/go gates.
- `https://github.com/modernecotech/OpenSourceRail` - companion rail design, simulator, operations, manufacturing, and safety-case project referenced by the Samawah pilot.
- `EXECUTIVE_SUMMARY.md` - economic-system summary for expert review.
- `FINAL_SUMMARY.md` - current repository summary with economic model first and software appendix second.
- `docs/business-value-chain-charts.md` - rendered SVG business charts for all sector value chains, capital and repayment lanes, and society/economy feedback loops.
- `docs/economic-assumptions.md` - source discipline and current public facts for Iraq-specific claims.
- `docs/unified-economic-model.md` - integrated economic model linking Digital IQD, INDHC, ministries, banks, producers, strategic resilience manufacturing, infrastructure, tourism, credit, taxes, civic work, reinvestment, and citizen dividends.
- `docs/national-economic-operating-logic.md` - management spine for the model: ledgers, hard gates, portfolio modes, scoring, cash/benefit conversion, capital allocation, dashboards, and escalation rules.
- `docs/national-data-centre-cloud-ai-sovereignty.md` - domestic data-centre, open-source cloud, social media, and AI infrastructure plan with cost bands, power demand, financing, revenue lanes, social safeguards, and growth benefits.
- `docs/national-legal-institutional-roadmap.md` - authority path for the oil lockbox, INDHC, citizen entitlements, Digital IQD, project debt, domestic securities, privacy, emergency powers, federalism, and appeals.
- `docs/project-pipeline-and-investment-gates.md` - project-family pipeline with capex, revenue source, DSCR, FX exposure, facility reuse, environmental, legal, and evidence gates.
- `docs/political-economy-transition-and-anti-capture.md` - resistance, capture-risk, coalition, service-continuity, staff-transition, procurement-transparency, and pause/rollback logic.
- `docs/citizen-entitlement-privacy-and-appeals.md` - citizen-share, dividend, identity, inheritance, minors, deceased records, diaspora/displaced claims, privacy, data minimization, payment exceptions, suspension, accessibility, dashboard, audit, and appeal gates.
- `docs/cash-formalization-and-demonetization-window.md` - one-year physical-cash transition controls for legal authority, supervised conversion points, KYC, caps, source-of-funds scoring, EDD, quarantine, tax settlement, signed receipts, audit hashes, appeals, dashboards, and post-window rejection.
- `docs/federalism-governorate-equity-and-local-compacts.md` - governorate, municipal, regional, and disputed-authority compact gates for allocation fairness, local benefit capture, grievances, audit, appeals, and land/water/heritage disputes.
- `docs/environmental-social-cultural-safeguards.md` - environmental, social, water, marshland, biodiversity, heritage, resettlement, safety, maintenance, remediation, monitoring, audit, and accessibility gates.
- `docs/macro-monetary-fx-stability.md` - macro, monetary, inflation, food-price, FX, reserve-cover, liquidity, credit, import-leakage, distribution-phasing, CBI-independence, and FX-transparency gates.
- `docs/fiscal-stress-and-contingent-liability-model.md` - downside stress controls for oil-equity caps, DSCR, FX mismatch, maintenance gaps, guarantees, collection weakness, capex overruns, and dividend suspension.
- `docs/national-program-sequencing-and-dependency-control.md` - sequencing controller that decides not-ready, evidence-only, pilot, build, controlled-scale, or hold/rollback phase for each domain.
- `docs/procurement-integrity-and-market-discipline.md` - procurement and market-discipline gates for ownership, competition, price benchmarks, contract changes, payment discipline, quality, and SME participation.
- `docs/benefit-realization-and-claim-audit.md` - claim-audit model for proving whether cashflows, public benefits, avoided costs, service outcomes, and dividends actually materialized.
- `docs/iraq-integrated-growth-impact-model.md` - quantified non-oil growth-impact timeline for infrastructure, industry, open-source rail, green power, food/water systems, tourism, Digital IQD formalization, and civic work.
- `docs/iraq-comprehensive-benefits-model.md` - long-horizon 2036, 2040, and 2050 benefits model for economic, infrastructure, environmental, social, and cultural outcomes.
- `docs/policy-paper.md` - deprecated policy-paper boundary note; the earlier long-form narrative is no longer the front-door claim source.
- `docs/national-dividend-holding-company.md` - proposed oil-income lockbox, citizen beneficial-share, ministry-funding, and Digital IQD dividend architecture.
- `docs/indhc-10-year-plan.md` - ten-year INDHC planning envelope, timelines, cashflow model, and sector plan for import substitution, profitable subsidiaries, defense-controlled supply chains, electronics, HVAC, water/desalination, irrigation, imported-food substitution, tourism/services, green capital, open rail, raw-material processing, and Iraqi-only permanent staffing.
- `docs/iraq-quantified-affordability-model.md` - IMF-baseline affordability model with fiscal-safe, constrained-base, and strategic-upper capital envelopes, project loans, PPP/JV capital, recurring revenue channels, tourism second-order benefits, DSCR gates, stress tests, and dividend math.
- `docs/import-services-diaspora-expansion.md` - missing import screens, attraction-based service production, and diaspora income, expertise, capital, marketing, and distribution channels.
- `docs/facility-recycling-and-capital-markets.md` - brownfield-first facility-reuse screening plus international credit, PPP, domestic bond/sukuk/equity, local bank, and diaspora financing lanes.
- `docs/digitally-governed-industrial-champions.md` - anti-capture model for sectoral Iraqi industrial champion groups with conditional demand, conditional credit, export discipline, competition gates, and public audit.
- `docs/national-civic-work-system.md` - digitally verified civic labor, wage, training, care, environmental, sport, culture, municipal, climate-resilience, anti-ghost-worker, dignity, public-value, and bridge-to-work transition layer.
- `docs/ministry-transition-roadmap.md` - staged roadmap and scenario-control primitive for retaining sovereign functions or deprecating, merging, regulating, corporatizing, or sunsetting ministry functions once legal, service-continuity, staff, payroll, audit, appeal, anti-capture, and local-compact gates are met.
- `docs/system-and-financial-flow-diagrams.md` - rendered SVG architecture diagrams, transaction lifecycles, and financial-flow combinations, with software treated as implementation appendix.

## API And Implementation

- `DEVELOPMENT_GUIDE.md` - PostgreSQL/Redis development runbook.
- `SECURITY.md` - security model, threat areas, and production requirements.
- `docs/technical-primitives.md` - implementation evidence and gaps for offline payments, consensus, AML, privacy, and disaster recovery.
- `API_REFERENCE.md` - dashboard endpoint reference and local demo operator notes.
- `IMPLEMENTATION_STATUS.md` - honest prototype status and gaps.
- `SPECIFICATION_AND_FIXTURE_RESULTS.md` - specification checks, fixture checks, route-level evidence, and remaining integration/security coverage.
- `COMPLETION_CHECKLIST.md` - production-readiness checklist.

## Runtime Note

`cbi-dashboard` is PostgreSQL-only today. `docker-compose.yml` reads
`DB_PASSWORD` from `.env` and uses `change-me-dev-only` only as a local demo
fallback. Historical root SQLite dashboard fixtures have been removed; POS-local
SQLite remains only for the device-side terminal store.

## Repository Hygiene

Generated artifacts such as `cylinder_seal.db`, Redis `dump.rdb`, virtualenvs, local env files, logs, and build outputs are ignored and should not be committed.
