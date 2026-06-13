# National Dividend Holding Company Architecture

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

## Rationale

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

## Source Discipline

| Public fact | Why it matters | Source |
| --- | --- | --- |
| Iraq is heavily dependent on oil income, with oil accounting for a very large share of government revenue. | Direct oil-to-budget dependence is the problem this architecture tries to rewire. | [EITI Iraq country page](https://eiti.org/countries/iraq), [EIA Iraq analysis](https://www.eia.gov/international/analysis/country/irq) |
| IMF staff have highlighted Iraq's rigid fiscal spending, subdued non-oil revenues, and vulnerability to lower oil prices. | Ministry funding should not be structurally insulated from non-oil economic performance. | [IMF Iraq 2025 Article IV](https://www.imf.org/-/media/files/publications/cr/2025/english/1irqea2025001-source-pdf.pdf), [IMF Iraq 2024 Article IV](https://www.imf.org/en/publications/cr/issues/2024/05/15/iraq-2024-article-iv-consultation-press-release-staff-report-and-statement-by-the-executive-549028) |
| World Bank analysis has examined oil-revenue management options and notes that allocating oil revenue to public capital can have the strongest non-oil GDP effect, while public-sector pay allocation can distort the traded-goods sector. | The architecture prioritizes productive investment over direct consumption or ministry payroll expansion. | [World Bank, Iraq oil revenue management for economic diversification](https://documents1.worldbank.org/curated/en/669171643036848080/pdf/Iraq-Oil-revenue-management-for-economic-diversification.pdf) |

These sources support the problem framing. They do not validate the proposed
institutional design.

## Institutional Design

Working name: **Iraq National Dividend Holding Company (INDHC)**.

Alternative local branding can use "People's Development Holding Company" or
"Citizen Development Holding Company." The industrial holding-group analogy is
useful, but the proposed institution must not reproduce opaque family control,
related-party abuse, or protected conglomerate behavior.

### Ownership

- Every eligible Iraqi citizen receives one equal base share class.
- Shares are non-saleable, non-pledgeable, and non-transferable except through
  inheritance to eligible descendants.
- A citizen's base share is a beneficial entitlement, not a speculative token.
- New citizens and births require a statutory issuance rule.
- Deceased citizens without eligible heirs revert their entitlement to a social
  reserve pool.
- No ministry, party, militia, bank, or private holding company can acquire
  citizen base shares.

### Cash Formalization Window

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

### Revenue Waterfall

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

## Ministry Feedback Mechanism

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

## Cylinder Seal System Role

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

## Flow Combination Matrix

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

## Required Transaction Primitives

| Primitive | Description | Status |
| --- | --- | --- |
| `ShareEntitlement` | Non-transferable citizen beneficial share record. Inheritance transfer only. | New design primitive. |
| `OilReceipt` | Signed oil-income receipt entering the Oil Income Lockbox. | New design primitive. |
| `CapitalAllocation` | INDHC allocation to a subsidiary, infrastructure project, industrial project, or service platform. | Can reuse earmarked spend and conditional release. |
| `GrossProfitLevy` | Treasury claim on productive operating surplus. | New accounting primitive. |
| `DividendDistribution` | Monthly equal dividend to all eligible citizen wallets. | New distribution primitive. |
| `CashConversionReceipt` | Time-limited cash deposit record with KYC, risk score, cap, hold, and conversion status. | New design primitive. |
| `InheritanceTransfer` | Legally approved movement of share entitlement to eligible offspring or heirs. | New design primitive. |

## Financial Flows

### Oil Income To Dividend

1. SOMO/export receipt or equivalent oil-income record is signed.
2. Receipt enters the Oil Income Lockbox.
3. Stabilization reserve allocation is calculated.
4. INDHC investment capital is allocated to projects and subsidiaries.
5. Subsidiary profits are measured.
6. Gross-profit tax or levy funds the Treasury.
7. Retained earnings fund reinvestment and reserves.
8. Dividend pool is distributed monthly to citizen wallets.

### Cash Formalization

1. Citizen brings physical cash during the 12-month window.
2. Operator verifies identity and records cash amount.
3. Cylinder Seal runs risk checks and applies caps, holds, or EDD.
4. Accepted amount becomes locked supplemental entitlement or transition balance.
5. Suspicious amount is held or referred.
6. After the window, physical cash is no longer accepted for conversion.

### Ministry Funding

1. Ministry proposes or receives a service mandate.
2. Budget is tied to tax/levy revenue, service contract, or performance milestone.
3. Cylinder Seal records disbursement constraints.
4. Delivery evidence triggers payment.
5. Poor performance becomes visible in the public dashboard and affects future
   allocations.

## Governance Guardrails

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

## Risks

| Risk | Mitigation |
| --- | --- |
| INDHC becomes a politicized monopoly. | Independent board, public audit, procurement transparency, competition rules, project-level performance dashboards. |
| Cash window becomes corruption laundering. | KYC, caps, EDD, holds, haircuts, sanctions screening, PEP restrictions, and law-enforcement referral. |
| Ministries resist losing direct oil allocations. | Statutory transition, service contracts, performance budgets, public dashboards. |
| Dividend becomes fiscally pro-cyclical. | Stabilization reserve and dividend formula based on audited distributable surplus, not raw oil price. |
| Citizens treat entitlement as speculative property. | Non-saleable, non-pledgeable base shares; inheritance-only transfers. |
| Automation gains concentrate in subsidiaries. | Equal monthly dividend plus open procurement and SME participation requirements. |

## What To Build First

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
