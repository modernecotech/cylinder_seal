# Documentation Index

## Start Here

- `README.md` - concise project overview, current status, quick start, and production-readiness boundary.
- `DEVELOPMENT_GUIDE.md` - PostgreSQL/Redis development runbook.
- `SECURITY.md` - security model, threat areas, and production requirements.
- `docs/technical-primitives.md` - implementation evidence and gaps for offline payments, consensus, AML, privacy, and disaster recovery.
- `docs/system-and-financial-flow-diagrams.md` - rendered SVG architecture diagrams, transaction lifecycles, and financial-flow combinations.
- `docs/economic-assumptions.md` - source discipline and current public facts for Iraq-specific claims.
- `docs/unified-economic-model.md` - integrated economic model linking Digital IQD, INDHC, ministries, banks, producers, strategic resilience manufacturing, infrastructure, tourism, credit, taxes, civic work, reinvestment, and citizen dividends.
- `docs/policy-paper.md` - long-form policy thesis draft; illustrative and not a readiness statement.
- `docs/national-dividend-holding-company.md` - proposed oil-income lockbox, citizen beneficial-share, ministry-funding, and Digital IQD dividend architecture.
- `docs/indhc-10-year-plan.md` - ten-year INDHC planning envelope, timelines, cashflow model, and sector plan for import substitution, profitable subsidiaries, defense-controlled supply chains, electronics, HVAC, water/desalination, irrigation, imported-food substitution, tourism/services, green capital, open rail, raw-material processing, and Iraqi-only permanent staffing.
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
fallback. `sqlite-migrations/`, `setup-sqlite-dev.sh`, and
`verify-sqlite-setup.sh` are local fixture helpers and should not be cited as a
supported dashboard runtime.

## Repository Hygiene

Generated artifacts such as `cylinder_seal.db`, Redis `dump.rdb`, virtualenvs, local env files, logs, and build outputs are ignored and should not be committed.
