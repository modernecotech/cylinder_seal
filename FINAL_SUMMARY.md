# Cylinder Seal Summary

## Status

Cylinder Seal is an economic-system proposal for Iraq with a pilot-grade Digital
IQD evidence and analytics appendix. It is not production-ready CBDC/payment
infrastructure, not an official Central Bank of Iraq project, and not a
validated investment program.

Current posture:

- **Economic model:** documented as a unified operating architecture for oil
  income, productive capital, domestic industry, tourism, services, civic work,
  ministry funding, credit, public benefits, and citizen dividends.
- **Cashflow discipline:** modeled through capital, productive asset, booked
  cash, public benefit, distribution, and risk ledgers so public benefits are
  not confused with distributable cash.
- **Political-economy discipline:** modeled through explicit capture,
  resistance, coalition, service-continuity, staff-transition, federalism,
  emergency-power, and citizen-appeal gates before reforms can scale.
- **Federalism discipline:** modeled through authority mapping, governorate or
  regional compacts, allocation-gap checks, local revenue/jobs/suppliers,
  grievance resolution, audit, appeals, and land/water/heritage dispute gates.
- **Stress discipline:** modeled through oil-equity caps, stressed DSCR, FX
  mismatch, maintenance gaps, guarantees, availability payments, collections,
  capex overruns, and dividend-affordability gates.
- **Sequencing discipline:** modeled through not-ready, evidence-only, pilot,
  build, controlled-scale, and hold/rollback phases for each domain.
- **Procurement discipline:** modeled through ownership, competition, price,
  contract-variation, evidence, delivery, payment, quality, and SME gates before
  awards or privileges proceed.
- **Outcome discipline:** modeled through benefit-realization reports that
  classify claims as verified, track-only, in-progress, underperforming,
  overstated, failed, or unsupported.
- **Affordability:** USD 65B fiscal-safe and USD 115B constrained-base scenarios
  are the credible starting envelopes; USD 190B is a strategic upper envelope
  only after fiscal, debt, cashflow, governance, and delivery gates pass.
- **Software appendix:** Rust crates, dashboard routes, migrations, and tests
  demonstrate how evidence, settlement, analytics, and policy controls might be
  measured. They are not production infrastructure.
- **Security:** threat model and production controls are documented as
  requirements in `SECURITY.md`; they are not complete.

## What Exists

- Economic-system documentation connecting Digital IQD evidence, INDHC capital
  allocation, oil-income lockbox rules, ministry feedback, credit expansion,
  domestic production, strategic resilience manufacturing, tourism/exports,
  green/rail cost reduction, civic work, reinvestment, and citizen dividends.
- Business value-chain diagrams covering sector chains, capital and repayment
  lanes, and society/economy feedback loops.
- Quantified affordability and cashflow models separating fiscal-safe,
  constrained-base, and strategic-upper capital envelopes.
- Growth and long-horizon benefit scenario models for infrastructure, industry,
  Open Source Rail, green power, food/water systems, tourism, Digital IQD
  formalization, civic work, environmental repair, and cultural production.
- Policy documentation for a proposed National Dividend Holding Company:
  oil-income lockbox, citizen non-saleable beneficial shares, ten-year
  industrial/infrastructure plan, gross-profit levy for Treasury and ministries,
  and equal Digital IQD dividend distribution from audited surplus.
- Ministry transition roadmap for deprecating, merging, regulating,
  corporatizing, or sunsetting low-feedback ministry functions only after legal,
  service-continuity, staff-transition, and audit gates.
- National Civic Work System policy architecture for turning productivity
  displacement into verified Digital IQD civic wages, training records, care,
  environmental restoration, sport, culture, municipal repair, and
  disaster-resilience work.
- Facility-recycling and capital-market logic for screening underutilized Iraqi
  assets before greenfield capex and attracting international credit, PPP/JV
  capital, local bank finance, domestic sukuk/bonds/equity, and diaspora
  co-investment only through audited project vehicles.
- Political-economy transition and anti-capture logic that can force a reform
  into visibility-only, pilot, pause, rollback, or redesign even when the
  financial case is attractive.
- Federalism, governorate-equity, and local-compact logic that blocks national
  scale-up when authority, allocation, local benefit capture, grievances,
  appeals, audit, or land/water/heritage issues are unresolved.
- Fiscal stress and contingent-liability logic that can force defensive or
  stop-scale-up mode when shocks make dividends, guarantees, FX debt, or new
  capex unsafe.
- Program sequencing logic that blocks domains from jumping to scale before
  legal, baseline, audit, procurement, delivery, political, fiscal, service, and
  cashflow dependencies pass.
- Procurement integrity logic that can restrict, suspend, cancel, or retender
  awards and industrial privileges when market-discipline gates fail.
- Benefit-realization logic that keeps public benefits out of dividend
  waterfalls and removes overstated or failed claims from front-door summaries.
- Rust workspace with core models, storage, API, policy, AML, credit, consensus,
  sync, POS, mobile-core, analytics, and dashboard crates.
- PostgreSQL-backed dashboard routes, Redis-backed session storage, Argon2id
  password hash verification, and admin action audit recording for current
  sensitive handlers.
- Specification tests covering crypto primitives, signing, nonce chains, Raft
  behavior, AML, credit scoring, wire formats, conflict resolution,
  programmability primitives, and tier policy behavior.

## What Is Not Proven Yet

- Legal authority for Digital IQD, INDHC, citizen share entitlements,
  oil-income allocation, ministry funding transition, securities issuance,
  privacy boundaries, emergency powers, procurement controls, and appeals.
- Independent macroeconomic validation, audited baselines, calibrated equations,
  project-level feasibility studies, debt-capacity analysis, procurement
  sequencing, and sensitivity testing.
- Real endpoint/database integration coverage across all dashboard routes.
- Production-grade role enforcement on every sensitive route.
- Full browser-session hardening beyond the current cookie/CSRF foundation.
- HSM or secure-element key custody.
- Audited offline double-spend prevention.
- National identity/KYC integration.
- Real multi-peer consensus deployment and recovery testing.
- Immutable audit-log storage and regulator evidence-pack workflows.
- Independent constitutional, administrative-law, labor, federalism,
  service-continuity, AML/CFT, competition, privacy, disability-access, and
  anti-corruption review for the economic model.
- Independent political-economy validation that maps real actors, incentives,
  veto points, coercive risks, governorate authority, citizen trust, and
  implementation capacity.
- Independent local-compact validation using official authority maps,
  governorate/KRG/municipal input where applicable, real needs data, grievance
  records, and land/water/environment/heritage status.
- Independent fiscal stress validation of oil-equity caps, project debt,
  contingent liabilities, guarantees, availability payments, FX exposure,
  collection efficiency, maintenance reserves, and dividend rules.
- Independent sequencing validation that turns the model into a realistic
  rollout with owners, pilot limits, milestones, public consultation, operator
  readiness tests, and rollback authority.
- Independent procurement validation using real bid data, ownership registries,
  price benchmarks, supplier-market data, sanctions/PEP feeds, protest records,
  and payment history.
- Independent monitoring, evaluation, and audit validation for any claim that is
  presented as delivered rather than modelled.

## Development Stack

```bash
cp .env.example .env
docker compose up -d
export DATABASE_URL="postgresql://postgres:${DB_PASSWORD:-change-me-dev-only}@localhost:5432/cylinder_seal"
cargo run --package cbi-dashboard
```

Use local demo operators only for development. Replace all seeded operator
hashes and all placeholder secrets before sharing, staging, or deploying.

## Credible Positioning

Use this language externally:

> Cylinder Seal is a source-disciplined proposal for an Iraqi economic operating
> model, with a pilot-grade Digital IQD evidence and analytics prototype used to
> test how cashflows, public benefits, policy controls, and citizen dividends
> could be measured.

Avoid this language:

> Production-ready national digital dinar infrastructure.

Also avoid:

> A guaranteed national growth plan, official CBI project, or validated
> investment program.

## Next Readiness Work

1. Add a full legal and institutional validation package.
2. Convert the sector plan into a project-level pipeline with capex, opex,
   revenue, DSCR, FX exposure, facility-reuse status, environmental gates, and
   responsible public authority.
3. Validate policy/economic scenarios with cited sources and independent review.
4. Add real dashboard route integration tests against PostgreSQL and Redis.
5. Enforce role-based authorization consistently.
6. Add CSRF/session hardening for browser flows.
7. Implement and test immutable audit logging.
