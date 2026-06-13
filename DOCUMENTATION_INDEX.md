# Documentation Index

## Start Here

- `README.md` - concise project overview, current status, quick start, and production-readiness boundary.
- `DEVELOPMENT_GUIDE.md` - PostgreSQL/Redis development runbook.
- `SECURITY.md` - security model, threat areas, and production requirements.
- `docs/technical-primitives.md` - implementation evidence and gaps for offline payments, consensus, AML, privacy, and disaster recovery.
- `docs/system-and-financial-flow-diagrams.md` - rendered SVG architecture diagrams, transaction lifecycles, and financial-flow combinations.
- `docs/business-value-chain-charts.md` - rendered SVG business charts for all sector value chains, capital and repayment lanes, and society/economy feedback loops.
- `docs/economic-assumptions.md` - source discipline and current public facts for Iraq-specific claims.
- `docs/unified-economic-model.md` - integrated economic model linking Digital IQD, INDHC, ministries, banks, producers, strategic resilience manufacturing, infrastructure, tourism, credit, taxes, civic work, reinvestment, and citizen dividends.
- `docs/national-economic-operating-logic.md` - management spine for the model: ledgers, hard gates, portfolio modes, scoring, cash/benefit conversion, capital allocation, dashboards, and escalation rules.
- `docs/iraq-integrated-growth-impact-model.md` - quantified non-oil growth-impact timeline for infrastructure, industry, open-source rail, green power, food/water systems, tourism, Digital IQD formalization, and civic work.
- `docs/iraq-comprehensive-benefits-model.md` - long-horizon 2036, 2040, and 2050 benefits model for economic, infrastructure, environmental, social, and cultural outcomes.
- `docs/policy-paper.md` - deprecated policy-paper boundary note; the earlier long-form narrative is no longer the front-door claim source.
- `docs/national-dividend-holding-company.md` - proposed oil-income lockbox, citizen beneficial-share, ministry-funding, and Digital IQD dividend architecture.
- `docs/indhc-10-year-plan.md` - ten-year INDHC planning envelope, timelines, cashflow model, and sector plan for import substitution, profitable subsidiaries, defense-controlled supply chains, electronics, HVAC, water/desalination, irrigation, imported-food substitution, tourism/services, green capital, open rail, raw-material processing, and Iraqi-only permanent staffing.
- `docs/iraq-quantified-affordability-model.md` - IMF-baseline affordability model with fiscal-safe, constrained-base, and strategic-upper capital envelopes, project loans, PPP/JV capital, recurring revenue channels, tourism second-order benefits, DSCR gates, stress tests, and dividend math.
- `docs/import-services-diaspora-expansion.md` - missing import screens, attraction-based service production, and diaspora income, expertise, capital, marketing, and distribution channels.
- `docs/facility-recycling-and-capital-markets.md` - brownfield-first facility-reuse screening plus international credit, PPP, domestic bond/sukuk/equity, local bank, and diaspora financing lanes.
- `docs/digitally-governed-industrial-champions.md` - anti-capture model for sectoral Iraqi industrial champion groups with conditional demand, conditional credit, export discipline, competition gates, and public audit.
- `docs/national-civic-work-system.md` - digitally verified civic labor, wage, training, care, environmental, sport, culture, municipal, and climate-resilience transition layer.
- `docs/ministry-transition-roadmap.md` - staged roadmap for deprecating, merging, regulating, corporatizing, or sunsetting ministry functions once service continuity and audit gates are met.

## API And Implementation

- `API_REFERENCE.md` - dashboard endpoint reference and local demo operator notes.
- `IMPLEMENTATION_STATUS.md` - honest prototype status and gaps.
- `SPECIFICATION_AND_FIXTURE_RESULTS.md` - specification checks, fixture checks, route-level evidence, and missing live integration/security coverage.
- `FINAL_SUMMARY.md` - concise current prototype summary.
- `COMPLETION_CHECKLIST.md` - production-readiness checklist.

## Runtime Note

`cbi-dashboard` is PostgreSQL-only today. `docker-compose.yml` reads
`DB_PASSWORD` from `.env` and uses `change-me-dev-only` only as a local demo
fallback. Historical root SQLite dashboard fixtures have been removed; POS-local
SQLite remains only for the device-side terminal store.

## Repository Hygiene

Generated artifacts such as `cylinder_seal.db`, Redis `dump.rdb`, virtualenvs, local env files, logs, and build outputs are ignored and should not be committed.
