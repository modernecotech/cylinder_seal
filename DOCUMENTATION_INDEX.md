# Documentation Index

## Start Here

- `README.md` - concise project overview, current status, quick start, and production-readiness boundary.
- `DEVELOPMENT_GUIDE.md` - PostgreSQL/Redis development runbook.
- `SECURITY.md` - security model, threat areas, and production requirements.
- `docs/technical-primitives.md` - implementation evidence and gaps for offline payments, consensus, AML, privacy, and disaster recovery.
- `docs/economic-assumptions.md` - source discipline and current public facts for Iraq-specific claims.
- `docs/policy-paper.md` - long-form policy thesis draft; illustrative and not a readiness statement.

## API And Implementation

- `API_REFERENCE.md` - dashboard endpoint reference and local demo operator notes.
- `IMPLEMENTATION_STATUS.md` - honest prototype status and gaps.
- `TEST_RESULTS.md` - test evidence and missing integration/security coverage.
- `FINAL_SUMMARY.md` - concise current prototype summary.
- `COMPLETION_CHECKLIST.md` - production-readiness checklist.

## Runtime Note

`cbi-dashboard` is PostgreSQL-only today. `sqlite-migrations/`, `setup-sqlite-dev.sh`, and `verify-sqlite-setup.sh` are local fixture helpers and should not be cited as a supported dashboard runtime.

## Repository Hygiene

Generated artifacts such as `cylinder_seal.db`, Redis `dump.rdb`, virtualenvs, local env files, logs, and build outputs are ignored and should not be committed.
