# National Economic Operating Logic

This document defines the management logic that holds the Cylinder Seal,
Digital IQD, INDHC, ministry-transition, industrial-champion, civic-work,
tourism, green, rail, and dividend model together.

Status: operating architecture. It is not a budget law, investment mandate,
central-bank rulebook, sovereign-debt recommendation, or production system.

## Core Principle

The whole system should be managed as a national productive-capital operating
system:

```text
No cash claim without settled evidence.
No benefit claim without source-tagged measurement.
No capital allocation without gates.
No dividend without audited distributable surplus.
No ministry funding without visible public value.
No strategic program without anti-capture controls.
```

The purpose is to stop oil income from flowing directly into passive ministry
budgets while also avoiding a new opaque conglomerate. Oil, loans, PPP capital,
public procurement, tourism, civic work, and citizen dividends all become parts
of one managed circuit.

## One Operating Sentence

Cylinder Seal records economic events; INDHC and private producers turn capital
into productive assets; those assets generate cash, public benefits, resilience,
and citizen income; the operating logic decides which claims are cash, which are
benefits, which are risks, and which are eligible for reinvestment, ministry
funding, or dividends.

## The Six Ledgers

Every activity belongs to at least one ledger. The ledgers are separate because
mixing them is how overclaiming, double counting, and fiscal illusion begin.

| Ledger | What it records | Can fund debt or dividends? | Main controls |
| --- | --- | --- | --- |
| Capital ledger | Oil equity, concessional loans, green sukuk, ECA finance, PPP/JV equity, retained earnings, grants, local bonds. | No, not directly. | Fiscal cap, debt approval, source-of-funds tags, use-of-proceeds registry. |
| Productive asset ledger | Factories, rail assets, hotels, logistics hubs, power assets, water systems, platforms, intellectual property, service contracts. | No, not until they generate cash. | Asset registry, depreciation, maintenance reserve, ownership and custody records. |
| Booked cash ledger | Settled sales, leases, PPAs, availability payments, platform fees, service contracts, fares, exports, JV distributions, taxes, levies. | Yes, after waterfall rules. | Invoice-to-settlement matching, collection efficiency, audit trail, currency matching. |
| Public benefit ledger | Import substitution, avoided fuel cost, reduced grid losses, tourism second-order effects, jobs, SME bankability, city-service outcomes, resilience gains. | No, unless converted into booked cash. | Source tags, attribution method, confidence score, no-dividend flag. |
| Citizen and state distribution ledger | Wages, civic-work income, public transfers, ministry service payments, gross-profit levy, retained earnings, dividend batches. | It is the distribution layer. | Eligibility, equity, appeals, audit publication, privacy limits. |
| Risk, rights, and control ledger | AML/CFT flags, sanctions exposure, conflict of interest, related-party exposure, privacy tier, security status, project overruns, DSCR, FX mismatch. | No; it can veto. | Hard gates, escalation, suspension, independent audit, legal review. |

Management rule:

```text
Capital and public benefits can justify action.
Only booked cash can satisfy debt service, levies, retained earnings, and
dividends.
Risk can stop any flow.
```

## Canonical Economic Event

Cylinder Seal should treat each policy-relevant transaction or measurement as a
typed economic event.

```text
EconomicEvent
  = actor
  + counterparty
  + amount
  + currency
  + sector
  + governorate
  + contract or mandate
  + source of funds
  + source of revenue or benefit
  + evidence bundle
  + privacy tier
  + risk tags
  + audit hash
  + ledger impacts
```

Examples:

| Event | Ledger impacts |
| --- | --- |
| Oil-equity allocation to INDHC | Capital ledger increases; risk ledger checks fiscal cap. |
| Loan draw for a solar project | Capital ledger increases; risk ledger records currency, tenor, DSCR covenant. |
| Payment to a factory after milestone inspection | Capital ledger use decreases; productive asset ledger increases; risk ledger records procurement status. |
| Hotel JV distribution | Booked cash ledger increases; Treasury levy and retained earnings become possible. |
| Tourist spending at an SME restaurant | Booked merchant cash for the SME; public benefit ledger records tourism multiplier; INDHC cash only if a platform fee or contract exists. |
| Verified grid-loss reduction | Public benefit ledger increases; booked cash only if an approved savings contract settles. |
| Civic-work payment | Citizen distribution ledger increases after task evidence; public benefit ledger records municipal output. |
| Monthly citizen dividend | Distribution ledger increases only after debt service, levy, reserves, and retained earnings pass. |

## Management Cycle

The system should run on a formal operating calendar.

| Cadence | Decision work | Output |
| --- | --- | --- |
| Daily | Settlement, AML alerts, payment failures, offline reconciliation, operator exceptions. | Exception queue and settlement health. |
| Monthly | Close booked cash ledger, pay eligible service contracts, calculate levy, update debt service, test dividend gate. | Monthly operating statement and citizen-facing summary. |
| Quarterly | Review project milestones, DSCR, collection efficiency, capex overruns, local content, ministry service results, second-order benefits. | Portfolio reallocation, project stop/go decisions, public dashboard update. |
| Annual | Refresh macro assumptions, oil-equity cap, borrowing envelope, sector priorities, dividend formula, ministry transition schedule. | National economic operating plan and audited public report. |

The monthly cycle protects solvency. The quarterly cycle protects performance.
The annual cycle protects strategy.

## Portfolio Modes

The operating system should not use the same settings in every macro condition.

| Mode | Trigger | Capital behavior | Dividend behavior | Ministry behavior |
| --- | --- | --- | --- | --- |
| Defensive | Oil stress, rising debt stress, reserve pressure, weak collections, DSCR breach. | Freeze new non-critical capex; protect maintenance, water, food, power, debt service. | Suspend growth; pay only if fully funded by audited surplus. | Protect essential services; delay deprecation. |
| Build | Fiscal cap available, projects pass gates, collections improving. | Fund quick cashflow, import-substitution, water, power, food, and logistics projects. | Small or zero; prioritize proof. | Shift budgets into service contracts. |
| Scale | DSCR strong, revenue broad, delivery credible, governance clean. | Expand proven sectors; add PPP/JV and green capital. | Increase only after reserves and reinvestment. | Corporatize or merge functions with proven alternatives. |
| Dividend | Mature portfolio, high retained earnings, stable debt, strong maintenance coverage. | More renewal and reinvestment than new speculative capex. | Stable monthly payments from surplus. | Ministries funded mainly through explicit levy, taxes, and priced outputs. |

## Hard Gates

Hard gates run before scoring. If a project fails a hard gate, it does not move
forward no matter how attractive the narrative is.

| Gate | Pass condition |
| --- | --- |
| Legal authority | Statutory mandate, procurement authority, data authority, and dispute path exist. |
| Fiscal affordability | Oil-equity draw and public exposure fit the affordability rules. |
| Debt safety | Base DSCR, stress DSCR, tenor, grace period, and currency match are acceptable. |
| Maintenance coverage | Lifecycle cost and renewal reserve are funded before dividends. |
| Revenue proof | Cashflow source is identified: sale, PPA, lease, fare, service contract, platform fee, export receipt, or lawful levy. |
| Benefit discipline | Second-order benefits are source-tagged and excluded from dividend cash. |
| Local capability | Iraqi staffing, training transfer, supplier upgrading, and handover plan exist. |
| Anti-capture | Related-party exposure, monopoly risk, PEP/sanctions risk, and procurement concentration are within limits. |
| Privacy and security | Data use, access controls, auditability, offline settlement, and operator powers meet the security model. |
| Citizen fairness | Dividends, civic work, transfers, and appeals are rules-based and auditable. |

## Portfolio Scoring

After hard gates, projects can be ranked with a transparent score. The weights
below are planning defaults, not law.

| Component | Weight | What it asks |
| --- | ---: | --- |
| Cash adequacy | 25% | Does the project generate collectible revenue after maintenance? |
| Fiscal relief | 15% | Does it fund Treasury, reduce subsidy pressure, or replace inefficient spending? |
| Import and FX effect | 15% | Does it reduce critical imports, earn foreign currency, or lower FX leakage? |
| Strategic resilience | 15% | Does it strengthen food, water, power, defense-controlled supply, electronics, HVAC, logistics, or health resilience? |
| Iraqi employment and capability | 10% | Does it create skilled Iraqi jobs and technical control? |
| Public service benefit | 10% | Does it improve city services, transport, safety, sanitation, productivity, or inclusion? |
| Citizen distribution potential | 10% | Does it raise future dividend capacity without starving maintenance or debt service? |

Negative modifiers:

- Capex overrun risk.
- Foreign-currency mismatch.
- Imported-input dependency.
- Water or energy stress.
- Monopoly or patronage risk.
- Weak audit evidence.
- High platform-fee burden on SMEs.

## Cash And Benefit Conversion Rules

Second-order benefits are useful for strategy, but they must cross a conversion
line before they become fiscal or dividend capacity.

| Benefit | Not cash when | Becomes booked cash when |
| --- | --- | --- |
| Import substitution | It is only an estimated avoided import. | A domestic sale, lease, service contract, or procurement settlement occurs. |
| Tourism multiplier | Visitors spend money at private merchants. | INDHC receives platform fees, leases, JV shares, ticketing, service contracts, or taxes/levies are settled. |
| Grid-loss reduction | Engineers estimate savings. | A verified savings contract, tariff adjustment, or budget transfer settles. |
| Rail land value | Property values rise near stations. | Lease, development charge, tax increment, concession, or revenue share is collected. |
| Civic work | A task is assigned. | Evidence is verified, appeals window passes, and payment is released. |
| SME bankability | Transaction history improves. | A loan is issued and repaid, or tax/fee settlement occurs. |
| Ministry performance | A ministry claims reform. | Service outputs are verified and tied to budget release or service payment. |

This is the most important anti-overclaim rule in the model.

## Waterfall Logic

Every operating subsidiary follows the same basic waterfall.

```text
Gross operating receipts
  - refunds, reversals, fraud losses
  - operating costs
  - maintenance and renewal reserve
  - project debt service
  - statutory risk reserve
  - gross-profit levy / tax
  - retained earnings allocation
  - dividend stabilization reserve
  = distributable surplus
```

The holding company may distribute only the consolidated surplus that remains
after all subsidiary and holding-company gates pass.

## Capital Allocation Logic

Capital allocation should proceed in this order:

1. Protect existing asset maintenance, safety, cybersecurity, and debt service.
2. Fund projects that preserve water, food, power, and critical logistics.
3. Fund projects with near-term booked cash and clear collection mechanisms.
4. Fund import-substitution and strategic-resilience projects with credible unit
   economics.
5. Fund tourism, services, and export platforms that bring non-oil demand.
6. Fund civic-work and workforce transitions that preserve social legitimacy.
7. Fund dividends only from audited surplus after reserves and retained
   earnings.

This order prevents a politically attractive dividend from consuming the capital
base that must produce future dividends.

## Governance Roles

| Body | Operating responsibility |
| --- | --- |
| Parliament | Passes legal mandate, debt limits, disclosure duties, dividend rules, emergency powers, and citizen rights. |
| CBI / payment authority | Supervises Digital IQD issuance, settlement, wallet limits, privacy boundaries, and payment-system resilience. |
| Treasury | Receives levy/tax revenue, funds ministries, reports fiscal exposure, and enforces debt limits. |
| INDHC board | Allocates capital, approves subsidiaries, enforces waterfall rules, and publishes portfolio accounts. |
| Portfolio risk committee | Can suspend disbursement, dividends, or borrowing when gates fail. |
| Sector boards | Manage industrial, tourism, rail, green, water, food, and platform subsidiaries under published mandates. |
| Ministries | Become policy owners, regulators, standard setters, or service buyers instead of automatic oil claimants. |
| Municipalities | Contract city services, tourism services, transport integration, and local maintenance with measurable outputs. |
| Banks | Lend against verified cashflows, receivables, and repayment history without political allocation. |
| Auditors and anti-corruption bodies | Review procurement, related parties, project accounts, source tags, and public dashboards. |
| Citizens | Receive equal dividends, see public dashboards, appeal entitlement errors, and benefit from services and jobs. |

## Data Model Surface

Cylinder Seal has now started representing this logic with explicit objects in
`crates/cs-analytics/src/economic_operating.rs` and persistence tables in
`migrations/20260702000001_economic_operating_kernel.sql`. This is an initial
kernel, not a complete national operating system.

| Primitive | Purpose |
| --- | --- |
| `EconomicOperatingPeriod` | Monthly, quarterly, and annual ledger close period. |
| `EconomicEvent` | Canonical event with amount, source, evidence, risk, and ledger impacts. |
| `LedgerImpact` | Links events to capital, asset, booked cash, public benefit, distribution, or risk ledgers. |
| `PortfolioMode` | Defensive, build, scale, or dividend mode with rule settings. |
| `HardGateResult` | Legal, fiscal, debt, revenue, benefit, local capability, security, and fairness checks. |
| `PortfolioScorecard` | Weighted project ranking after hard gates pass. |
| `BenefitAttribution` | Source-tagged estimate for import savings, tourism second-order effects, civic output, or city-service gains. |
| `CashBenefitConversion` | Records when a benefit becomes settled revenue, tax, fee, lease, or service payment. |
| `WaterfallStatement` | Subsidiary and holding-company distribution statement. |
| `CapitalAllocationDecision` | Board-approved movement of oil equity, loans, PPP capital, or retained earnings. |
| `DividendGateDecision` | Monthly eligibility, reserve, DSCR, audit, and distribution decision. |
| `PublicDashboardSnapshot` | Published aggregate view with confidence levels and privacy protection. |
| `GrowthImpactProjection` | Baseline, constrained-base, and strategic-upper non-oil growth paths from [Iraq Integrated Growth Impact Model](iraq-integrated-growth-impact-model.md). |
| `ComprehensiveBenefitProjection` | Long-horizon economic, infrastructure, environmental, social, and cultural benefit paths from [Iraq Comprehensive Benefits Model](iraq-comprehensive-benefits-model.md). |

Initial executable coverage:

- `LedgerKind` and `LedgerImpact` enforce that public benefits do not become
  distributable cash.
- `EconomicOperatingKernel::evaluate_hard_gates` checks the first legal,
  fiscal, debt, maintenance, revenue, benefit, local-capability, anti-capture,
  privacy/security, and citizen-fairness gates.
- `EconomicOperatingKernel::compute_waterfall` pays senior claims before
  distributable surplus.
- `EconomicOperatingKernel::decide_dividend` blocks dividends unless the
  waterfall is solvent, holding-company DSCR passes, and audit is complete.
- The new migration creates operating-period, assumption-set, event, impact,
  hard-gate, waterfall, capital-allocation, and dividend-gate tables.
- `crates/cs-analytics/src/sovereign_holding.rs` adds the first capital-plan
  layer: capital stacks, milestones, revenue streams, gross-profit levies,
  retained-earnings allocation, dividend distribution math, and holding-company
  governance gates. `migrations/20260703000001_sovereign_holding_capital_plan.sql`
  adds the corresponding persistence surface.
- `crates/cs-analytics/src/economic_cycle.rs` adds the first economic-cycle
  projection layer: capital formation, oil dependence, booked revenue, treasury
  revenue, citizen income, domestic recirculation, import leakage, non-oil FX,
  dividend revenue cover, cycle-quality warnings, and citizen-income math.
  `migrations/20260704000001_economic_cycle_projection.sql` adds the
  corresponding projection and gate tables.
- `crates/cs-analytics/src/growth_impact.rs` adds the first integrated
  non-oil growth-impact layer: baseline, constrained-base, and strategic-upper
  real-growth paths, sector contribution rows, additional GDP ranges, phase
  filtering, and claim-confidence controls. `migrations/20260705000001_growth_impact_projection.sql`
  adds projection, sector-contribution, and claim-audit tables.
- `crates/cs-analytics/src/comprehensive_benefits.rs` now produces
  comprehensive benefit-ledger statements for 2036, 2040, and 2050. It
  separates booked cash from real output, infrastructure capacity,
  environmental resilience, social capability, cultural tourism, and citizen
  distributions. `migrations/20260706000001_comprehensive_benefits_projection.sql`
  adds projection, benefit-ledger, and claim-audit tables.
- `crates/cs-analytics/src/production_capacity.rs` adds the first production
  capacity and import-substitution layer: utilization, local content, quality
  certification, delivered-cost discipline, booked domestic sales, verified
  import-substitution value, modelled FX savings, public-procurement
  dependence, and anti-protectionism gates. `migrations/20260707000001_production_capacity_projection.sql`
  adds projection, local-content, import-substitution-ledger, and gate tables.

## Management Dashboards

The operating dashboard should show the economy as a controlled portfolio:

| Dashboard | Main question |
| --- | --- |
| Source and uses | Where did oil, loans, PPP capital, and retained earnings go? |
| Booked cash | Which revenue streams actually settled? |
| Collections | Which invoices, contracts, fares, PPAs, leases, or fees are unpaid? |
| Debt safety | Which projects are close to DSCR, FX, or maturity stress? |
| Public benefits | Which second-order effects are real, source-tagged, and not double counted? |
| Growth impact | Are infrastructure, industry, open-source rail, tourism, Digital IQD, and civic work raising non-oil growth versus baseline? |
| Comprehensive benefits | Are economic, environmental, social, cultural, and infrastructure benefits tracked separately from booked cash? |
| Local capability | Which sectors are becoming Iraqi-operated and less import-dependent? |
| Ministry productivity | Which public budgets now buy measured outputs? |
| Citizen welfare | What changed in wages, transfers, civic income, dividends, prices, and credit access? |
| Risk and capture | Where are procurement, related-party, concentration, AML, or security risks rising? |
| Dividend sustainability | Is the dividend funded by recurring surplus after all senior claims? |

## Escalation Rules

| Condition | Automatic response |
| --- | --- |
| DSCR breach | Freeze new debt and dividends for the affected portfolio until recovery plan is approved. |
| Maintenance reserve breach | Block distributions from the asset or subsidiary. |
| Related-party or PEP concentration breach | Escalate to audit; suspend affected procurement privileges. |
| Unpaid public invoices | Stop counting revenue as collectible; review ministry or municipal payment capacity. |
| FX mismatch | Restrict new foreign-currency borrowing unless offset by FX revenue, hedge, or approved reserve. |
| Benefit overclaim | Move claim to low-confidence public-benefit ledger and remove from policy headline. |
| Civic-work evidence failure | Hold payment, trigger appeal or re-verification, and flag verifier reliability. |
| Privacy or security breach | Suspend affected data product, access role, or operator privilege. |

## Why This Encompasses The Whole Model

This logic gives every major part of the Cylinder Seal architecture a controlled
place:

- Oil income is capital, not a ministry entitlement.
- Loans are repayment claims, not free spending.
- INDHC subsidiaries are productive assets, not payroll vehicles.
- Tourism and second-order benefits are visible but not overcounted.
- Ministries are service buyers, regulators, or policy owners.
- Citizens are owners, workers, consumers, producers, and auditors.
- Civic work is paid public value with evidence, not disguised unemployment.
- Digital IQD is the evidence and settlement layer.
- Dashboards are management tools, not publicity pages.

## Bottom Line

The coherent logic is a rules-based national portfolio:

```text
Measure every event.
Classify it into the right ledger.
Apply hard gates.
Score the portfolio.
Allocate capital.
Collect revenue.
Separate benefits from cash.
Pay obligations before dividends.
Publish evidence.
Reallocate away from failure.
```

That is how the system can include oil, industry, ministries, tourism,
second-order benefits, civic work, credit, and citizen dividends without
collapsing into either a passive sovereign fund or another opaque state
bureaucracy.
