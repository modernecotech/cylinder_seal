# Iraq Comprehensive Benefits Model

This document reworks the Cylinder Seal, Digital IQD, INDHC, open-source rail,
green power, food/water, tourism, civic-work, and ministry-transition proposal
into one long-horizon national model for Iraq.

Status: scenario model. It is not an official forecast, budget law,
investment prospectus, production CBDC design, sovereign-debt recommendation,
or externally validated macroeconomic model.

## Purpose

Cylinder Seal should not be presented as only a payment application or only an
industrial-policy idea. The coherent model is broader:

```text
Digital IQD evidence
  -> bankable households, merchants, producers, and public contracts
  -> capital allocation through INDHC, banks, PPPs, MDBs, and green finance
  -> infrastructure, industry, water, food, rail, tourism, culture, and civic work
  -> booked revenue, avoided losses, public services, skills, resilience, and trust
  -> taxes, levies, reinvestment, debt service, civic wages, and citizen dividends
```

The model is "comprehensive" only if it separates five kinds of value:

| Value type | What it means | Can pay dividends? |
| --- | --- | --- |
| Booked cash | Settled sales, leases, fares, PPAs, service contracts, platform fees, taxes, levies, JV distributions, and exports. | Yes, after debt service, maintenance, reserves, tax/levy, and retained earnings. |
| Real output | Additional non-oil GDP from higher productivity, domestic production, formalization, tourism, and services. | No, not directly. It increases the taxable and investable base. |
| Avoided losses | Reduced water, power, climate, congestion, import, spoilage, and corruption losses. | No, unless a verified savings contract, tariff, fee, or budget transfer converts it into cash. |
| Social capability | Paid work, civic work, training records, youth inclusion, women's mobility, better services, and household bankability. | No, except through wages, transfers, dividends, or future income. |
| Cultural capital | Heritage protection, pilgrimage, archaeology, museums, crafts, languages, festivals, and city identity. | No, except through tourism revenue, local merchant revenue, leases, grants, and service contracts. |

This separation is the anti-overclaim rule. The national model can measure all
five, but only booked cash funds debt service and dividends.

## Source Discipline

| Baseline | Use in this model | Source |
| --- | --- | --- |
| Iraq's 2026 non-oil GDP starting point and medium-term non-oil growth path. | Base year and baseline path for non-oil real GDP. | [IMF Iraq 2025 Article IV](https://www.imf.org/-/media/files/publications/cr/2025/english/1irqea2025001-source-pdf.pdf) and [IMF press release](https://www.imf.org/en/news/articles/2025/07/08/pr-25243-iraq-imf-executive-board-concludes-2025-article-iv-consultation) |
| IMF statement that stronger labor, business, financial-sector, and governance reforms could double non-oil potential growth in the medium term. | Plausibility boundary for the strategic-upper growth path. | [IMF Iraq 2025 Article IV](https://www.imf.org/-/media/files/publications/cr/2025/english/1irqea2025001-source-pdf.pdf) |
| Iraq's oil dependence, fiscal exposure, and constrained non-oil activity. | Explains why the model targets non-oil compounding rather than oil-price windfalls. | [World Bank Iraq country page](https://www.worldbank.org/ext/en/country/iraq), [EIA Iraq analysis](https://www.eia.gov/international/analysis/country/irq) |
| Water scarcity and climate risk. | Environmental resilience and avoided-loss logic. | [World Bank Iraq CCDR](https://www.worldbank.org/en/country/iraq/publication/iraq-country-climate-and-development-report), [World Bank Iraq water-scarcity press release](https://www.worldbank.org/en/news/press-release/2021/11/24/iraq-rising-fiscal-risks-water-scarcity-and-climate-change-threaten-gradual-recovery-from-pandemic) |
| Public water-scarcity warning that a 20% water-supply fall and lower crop yields could reduce real GDP by up to 4%, or USD 6.6B. | Anchors the avoided environmental loss range; the model does not treat avoided loss as dividend cash. | [World Bank, Nov. 24, 2021](https://www.worldbank.org/en/news/press-release/2021/11/24/iraq-rising-fiscal-risks-water-scarcity-and-climate-change-threaten-gradual-recovery-from-pandemic) |
| Iraq energy transition plan including 12,000 MW solar by 2030. | Green power and grid-capacity pathway. | [IRENA Energy Transition Assessment: Iraq, 2025](https://www.irena.org/-/media/Files/IRENA/Agency/Publication/2025/Jul/IRENA_COU_Energy_transition_assessment_Iraq_2025.pdf) |
| Iraq has 6 UNESCO World Heritage properties and 15 tentative-list properties. | Cultural-tourism asset base and heritage-protection layer. | [UNESCO Iraq World Heritage page](https://whc.unesco.org/en/statesparties/iq) |
| Children and young people in Iraq face medium-high climate risk, and Iraq ranks among the world's water-stressed countries. | Social and civic-work resilience layer. | [UNICEF / UN Iraq water scarcity statement](https://iraq.un.org/en/156319-unicef-calls-urgent-action-address-water-scarcity-and-its-impact-children-and-young-people%E2%80%99s) |
| Tourism has direct and wider GDP/employment effects. | Tourism booked-revenue and second-order benefit discipline. | [WTTC Iraq Economic Impact Report 2024](https://assets-global.website-files.com/6329bc97af73223b575983ac/6643856bc693733a9f435ca5_EIR2024-Iraq.pdf) |

The sources support the baseline problem and the direction of transmission.
They do not validate the scenario ranges below.

## Long-Horizon Scenarios

Machine-readable table:
[docs/data/iraq-comprehensive-benefits-timeline.csv](data/iraq-comprehensive-benefits-timeline.csv).

Initial executable coverage now exists in
`crates/cs-analytics/src/comprehensive_benefits.rs`, with persistence tables in
`migrations/20260706000001_comprehensive_benefits_projection.sql`. The code
turns each projection into benefit-ledger entries and claim-audit records. Only
`booked_cash` entries are eligible for the cash waterfall; real output,
infrastructure capacity, environmental resilience, social capability, cultural
tourism, and citizen-distribution entries remain non-distributable unless a
separate settled-cash conversion is proven.

Delivered benefits should then pass the separate
[Benefit Realization And Claim Audit](benefit-realization-and-claim-audit.md)
layer before they appear as verified outcomes. Scenario projections can remain
in this document, but front-door claims require baseline, target, observed
value, evidence quality, attribution confidence, audit status, cash settlement,
and dividend-boundary checks.

The long-horizon GDP path extends the ten-year
[Iraq Integrated Growth Impact Model](iraq-integrated-growth-impact-model.md).

Assumptions:

| Scenario | 2027-2036 | 2037-2040 | 2041-2050 |
| --- | --- | --- | --- |
| Baseline | IMF-aligned weak reform path, reaching 3.5% non-oil real growth after 2030. | 3.5% non-oil real growth. | 3.5% non-oil real growth. |
| Constrained-base execution | Matches the ten-year constrained-base path. | 5.5% non-oil real growth as proven assets compound. | 4.5% non-oil real growth as the portfolio matures. |
| Strategic-upper execution | Matches the ten-year strategic-upper path. | 7.0% non-oil real growth with strong governance and private crowd-in. | 5.5% non-oil real growth after the high-build phase. |

Starting point: the ten-year model uses IMF's 2026 non-oil GDP level of IQD
235.6T, converted at IQD 1,300 per USD, or about USD 181B in constant 2026 USD.

## Long-Term Benefits Snapshot

All figures are scenario outputs, not forecasts. GDP figures are constant 2026
USD. Revenue and dividend figures are annual run-rate ranges unless noted.

| Horizon | Scenario | Non-oil GDP index, 2026=100 | Non-oil GDP | Additional non-oil GDP vs baseline | Booked portfolio revenue | Dividend pool | Infrastructure and service capacity |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- |
| 2036 | Baseline | 137.0 | USD 248B | N/A | N/A | N/A | Existing reform path, weak compounding. |
| 2036 | Constrained-base | 158.5 | USD 287B | USD 39B | USD 23B | USD 1.65B | Early proof of rail/logistics, food/water, green power, industrial parks, tourism, and Digital IQD revenue. |
| 2036 | Strategic-upper | 175.0 | USD 317B | USD 69B | USD 43B | USD 2-4B | Stronger project delivery, higher local content, more private crowd-in, larger export/tourism channels. |
| 2040 | Baseline | 157.2 | USD 285B | N/A | N/A | N/A | Slow non-oil expansion; oil and fiscal constraints still dominate. |
| 2040 | Constrained-base | 196.4 | USD 356B | USD 71B | USD 42B | USD 4-7B | Network effects from rail, power, industrial supply chains, bankable SMEs, and tourism corridors. |
| 2040 | Strategic-upper | 229.3 | USD 416B | USD 131B | USD 80B | USD 8-12B | Iraq becomes a regional platform for selected goods, services, logistics, pilgrimage, heritage tourism, and green projects. |
| 2050 | Baseline | 221.8 | USD 402B | N/A | N/A | N/A | Non-oil economy grows, but slowly and with continued structural fragility. |
| 2050 | Constrained-base | 305.0 | USD 553B | USD 151B | USD 86B | USD 12-20B | Mature domestic productive portfolio with continued maintenance and reinvestment burden. |
| 2050 | Strategic-upper | 391.7 | USD 710B | USD 308B | USD 160B | USD 25-40B | Highly diversified non-oil economy if governance, water, power, skills, anti-capture, and fiscal discipline hold. |

Interpretation:

- The most important number is not the dividend. It is the non-oil GDP base,
  because it determines long-term jobs, taxes, bankability, services, and
  fiscal resilience.
- Dividends are deliberately subordinate to maintenance, debt service, retained
  earnings, and reserves.
- The strategic-upper path is not a target to announce. It is a stretch case
  used for project selection, stress testing, and governance design.

## Integrated Domain Model

### 1. Economic Production

Objective: turn oil income into domestic productive assets before it becomes
ministry spending or household consumption.

Main mechanisms:

- Oil Income Lockbox and INDHC investment gate.
- Import substitution in food, building materials, HVAC, electronics,
  water/desalination, irrigation, rail components, defense-controlled
  sustainment, and raw-material processing.
- Digital IQD transaction evidence for sales, receivables, taxes, procurement,
  and credit.
- Project debt only where cashflow is measurable and DSCR gates pass.
- PPP/JV capital for tourism, logistics, hotels, industrial parks, and
  station-area development.
- Export discipline after domestic capability is proven.

Long-term benefit:

| Metric | Constrained-base by 2050 | Strategic-upper by 2050 |
| --- | ---: | ---: |
| Additional real non-oil GDP vs baseline | USD 151B | USD 308B |
| Annual booked portfolio revenue | USD 86B | USD 160B |
| Annual dividend pool after gates | USD 12-20B | USD 25-40B |
| Main risk | Mediocre delivery, weak private crowd-in, import leakage. | Overexpansion, debt stress, monopoly capture, political interference. |

### 2. Infrastructure And Urban Productivity

Objective: lower the national cost base through power, rail, logistics, water,
housing inputs, and digital public infrastructure.

Main mechanisms:

- Open-source rail standards for metro, light metro, tram, depots, fare systems,
  maintenance tooling, signaling interfaces, and station services.
- Repeatable city corridors rather than one-off prestige projects.
- Logistics hubs connecting food processing, raw materials, ports, tourism,
  industrial parks, and urban distribution.
- Grid, solar, storage, industrial power zones, efficient cooling, and
  waste-to-energy where bankable.
- Housing inputs and urban services that reduce construction and maintenance
  costs.

Scenario capacity ranges:

| Horizon | Scenario | Open rail / urban transit corridor-km | Clean or reliable added power capacity | Main benefit |
| --- | --- | ---: | ---: | --- |
| 2036 | Constrained-base | 250-400 km | 5-8 GW | First repeatable city corridors, logistics links, and industrial power zones. |
| 2036 | Strategic-upper | 600-900 km | 12-18 GW | Multiple city systems and stronger alignment with Iraq's solar target. |
| 2040 | Constrained-base | 500-800 km | 10-15 GW | Network utilization begins to lower freight, commute, and service costs. |
| 2040 | Strategic-upper | 1,000-1,500 km | 20-30 GW | Rail and power become national productivity platforms. |
| 2050 | Constrained-base | 1,000-1,600 km | 20-30 GW | Mature transport/power backbone in major cities and industrial corridors. |
| 2050 | Strategic-upper | 2,200-3,000 km | 40-60 GW | Iraq has a large repeatable urban mobility and clean-power industrial base. |

These are planning capacity ranges, not committed projects. They should be
approved corridor by corridor after ridership, land, energy, debt, procurement,
and maintenance gates pass.

### 3. Environmental Resilience

Objective: make water, heat, agriculture, marshes, sanitation, and energy
resilience part of the national production model rather than a separate aid
agenda.

Main mechanisms:

- Water-efficiency equipment, irrigation manufacturing, canal and drainage
  repair, metering, leak reduction, wastewater reuse, and desalination where
  economically appropriate.
- Climate-smart schools, clinics, public buildings, cooling, shaded streets,
  parks, tree cover, and civic-work maintenance.
- Marshland and river-basin restoration projects where they protect livelihoods,
  tourism, biodiversity, and flood/drought resilience.
- Food processing, cold chain, storage, and domestic input production to reduce
  spoilage and climate-related import dependence.
- Digital IQD civic-work payments for verified environmental tasks.

Avoided-loss ranges:

| Horizon | Constrained-base avoided annual loss | Strategic-upper avoided annual loss | Ledger treatment |
| --- | ---: | ---: | --- |
| 2036 | USD 1-3B | USD 2-5B | Public benefit ledger unless savings contracts settle. |
| 2040 | USD 3-6B | USD 6-10B | Public benefit ledger and selected booked savings contracts. |
| 2050 | USD 8-15B | USD 15-25B | Public benefit ledger, with some cash conversion through water, energy, sanitation, and insurance-like service contracts. |

The ranges are anchored by the World Bank warning that a 20% fall in water
supply and lower crop yields could reduce Iraq real GDP by up to 4%, or USD
6.6B in the cited report period. Longer-horizon values are scenario extensions,
not source-validated losses.

### 4. Social Capability And Inclusion

Objective: make productivity politically and socially legitimate by giving
citizens visible shares, usable wallets, credit histories, services, civic-work
pathways, and skills.

Main mechanisms:

- Non-saleable citizen beneficial shares in INDHC.
- Equal Digital IQD dividend batches only from audited surplus.
- Civic-work wages for verified municipal, environmental, care, culture, sport,
  education, food-security, and disaster-readiness tasks.
- Wallet, POS, and invoice histories that make merchants, individual producers,
  households, and civic workers bankable.
- Youth and women mobility through safe transit, digital payments, training,
  and local service work.
- Ministry transition only after service-continuity and staff-transition gates.

Scenario civic-work capacity:

| Horizon | Constrained-base annual job-equivalent capacity | Strategic-upper annual job-equivalent capacity | Main use |
| --- | ---: | ---: | --- |
| 2036 | 200,000-400,000 | 350,000-750,000 | Municipal repair, environmental work, care, culture, sport, tourism readiness, and training. |
| 2040 | 400,000-700,000 | 700,000-1,400,000 | Larger city-service, water, climate, heritage, and maintenance programs. |
| 2050 | 700,000-1,200,000 | 1,500,000-2,500,000 | Mature participation economy tied to skills, service quality, and resilience. |

These are annual job-equivalent capacities, not promised permanent government
jobs. They may be part-time, seasonal, task-based, municipal, cooperative, NGO,
or private-contract work, and they must be paid only after evidence is verified.

### 5. Culture, Heritage, Tourism, And Identity

Objective: treat culture as an economic and social asset without reducing it to
ticket sales.

Main mechanisms:

- UNESCO sites, tentative-list sites, museums, pilgrimage corridors, marshland
  heritage, archaeology, craft markets, cultural festivals, Iraqi food systems,
  local languages, and historic city centers.
- Digital ticketing, guide credentials, site-service contracts, visitor
  transport, sanitation, safety, payments, and merchant settlement.
- Heritage maintenance funded through a mix of grants, site revenue, municipal
  contracts, tourism levies, philanthropy, and INDHC-adjacent services.
- City cultural calendars linked to hotels, rail, food, crafts, media, sport,
  and education.

Tourism and culture revenue model:

| Horizon | Scenario | Booked tourism/culture revenue | Second-order benefit | Ledger treatment |
| --- | --- | ---: | ---: | --- |
| 2036 | Constrained-base | USD 3B | USD 6.6B | Booked cash for direct channels; second-order benefit for wider merchant and city effects. |
| 2036 | Strategic-upper | USD 7B | USD 14-18B | Requires strong safety, service quality, heritage protection, hotel capacity, and formal payments. |
| 2040 | Constrained-base | USD 6-8B | USD 12-18B | Tourism becomes a larger formal FX and SME-credit channel. |
| 2040 | Strategic-upper | USD 12-16B | USD 25-35B | Iraq becomes a regional cultural, pilgrimage, heritage, and service-tourism platform. |
| 2050 | Constrained-base | USD 14-20B | USD 30-45B | Mature route network, stronger domestic supply chains, better city services. |
| 2050 | Strategic-upper | USD 25-35B | USD 55-75B | High-service cultural economy with broad domestic supply chains and repeat visitation. |

Culture should be scored by more than revenue:

- Protected heritage assets.
- Jobs for guides, craftspeople, conservators, artists, drivers, food
  producers, hotel staff, event staff, translators, and city workers.
- Repeat visitor confidence.
- Local pride and social cohesion.
- Youth training and cultural education.
- Safer, cleaner, better maintained public spaces for residents.

## The Coherent Operating Logic

The full system should be managed by the
[National Economic Operating Logic](national-economic-operating-logic.md), with
one additional comprehensive-benefits scorecard.

### National Portfolio Scorecard

Every major project should receive six scores after hard gates pass:

| Score | Question | Example evidence |
| --- | --- | --- |
| Cash score | Can it collect revenue after maintenance and debt service? | Contracts, invoices, PPA, fare plan, lease, platform fees, export receipts. |
| Productivity score | Does it raise non-oil output or lower costs? | Delivered-cost studies, utilization, logistics time, outage reduction, domestic value added. |
| Resilience score | Does it reduce water, food, power, climate, import, or security exposure? | Water saved, storage added, local components, emergency readiness, stress tests. |
| Social score | Does it create skills, inclusion, mobility, civic value, or service quality? | Verified work records, training completion, service outputs, gender/youth access, grievance data. |
| Cultural score | Does it protect heritage or create cultural/tourism value? | Conservation plan, visitor services, local merchant records, craft income, route quality. |
| Governance score | Can it resist capture and be audited? | Procurement data, related-party checks, public dashboard, independent audit, complaints process. |

Capital should flow first to projects with acceptable cash and governance, then
to projects that also score highly on productivity, resilience, social value,
and culture.

### National Benefit Equation

The model's annual public report should not reduce national progress to GDP.
It should publish a balanced national benefit statement:

```text
National Benefit Statement
  = booked cash generated
  + additional real non-oil output
  + source-tagged avoided losses
  + verified public-service outputs
  + civic-work and skills outcomes
  + cultural and heritage outcomes
  - debt-service burden
  - maintenance backlog
  - environmental damage
  - import leakage
  - governance and capture losses
```

Only the first term is immediately distributable cash. The rest explains why
the system is worth doing.

## Timeline

### 2027-2030: Foundation And Proof

Primary work:

- Remove stale repo claims and keep the prototype boundary visible.
- Pilot Digital IQD evidence flows for merchants, public transfers, procurement,
  civic work, tourism, and INDHC contracts.
- Create Oil Income Lockbox and INDHC legal design.
- Publish the six ledgers, hard gates, portfolio scorecard, and citizen rights.
- Fund only fiscal-safe or constrained-base projects with clear evidence.
- Begin quick-return water, power, food, rail-design, tourism-service, and
  dashboard projects.

Expected benefit:

- Modest GDP acceleration.
- Better measurement.
- First bankable merchant/producer histories.
- Reduced overclaim risk.
- Visible public discipline.

### 2031-2036: Build And Compound

Primary work:

- Scale industrial champions only after local-content, price, quality, export,
  SME, and audit gates pass.
- Bring first rail/logistics corridors and industrial power zones into service.
- Expand tourism corridors and formal visitor payment channels.
- Convert ministry work into service contracts where continuity is proven.
- Launch meaningful but still small dividends from audited surplus.
- Use civic work for municipal repair, climate resilience, culture, sport,
  care, and training.

Expected benefit:

- Constrained-base additional real non-oil GDP of about USD 39B by 2036.
- Strategic-upper additional real non-oil GDP of about USD 69B by 2036.
- Direct booked portfolio revenue between USD 23B and USD 43B per year by the
  Year-10 horizon, depending on execution.

### 2037-2040: Network Effects

Primary work:

- Link city rail, logistics, food systems, water systems, industrial parks,
  power zones, tourism routes, and ports into larger production networks.
- Use Digital IQD data to expand receivables finance and SME credit.
- Increase private crowd-in only where project accounts are clean.
- Retire ministry functions only where regulated operators and municipalities
  perform better.
- Scale cultural and tourism services while protecting heritage.

Expected benefit:

- Constrained-base non-oil GDP reaches about USD 356B by 2040.
- Strategic-upper non-oil GDP reaches about USD 416B by 2040.
- Additional real non-oil output vs baseline is about USD 71B to USD 131B.

### 2041-2050: Mature Diversified Economy

Primary work:

- Maintain and renew assets before building prestige expansions.
- Keep dividend growth below the portfolio's ability to maintain, reinvest, and
  service debt.
- Turn Iraq into a regional platform in selected sectors: pilgrimage and
  heritage tourism, food processing, water systems, construction inputs,
  HVAC/cooling, logistics, green power services, rail maintenance, and
  regulated defense sustainment.
- Preserve cultural landscapes and urban heritage as national capital.
- Use civic work as a permanent participation and resilience layer, not as fake
  employment.

Expected benefit:

- Constrained-base additional real non-oil GDP of about USD 151B by 2050.
- Strategic-upper additional real non-oil GDP of about USD 308B by 2050.
- Annual booked portfolio revenue could range from USD 86B to USD 160B if the
  operating portfolio matures and remains governed.
- Avoided environmental losses, social capability, and cultural value become
  major national benefits even where they are not dividend cash.

## Implementation Surface For Cylinder Seal

The current repository already has pieces of the payment, policy, dashboard,
analytics, credit, exchange, and civic/industrial framing. To make the
comprehensive model software-native, future code should introduce explicit
domain primitives:

| Primitive | Purpose |
| --- | --- |
| `ComprehensiveBenefitProjection` | Links scenario, horizon, GDP, revenue, dividend, infrastructure, environmental, social, and cultural metrics. |
| `NationalBenefitStatement` | Annual balanced statement separating booked cash, output, avoided loss, social value, cultural value, and risk. |
| `EnvironmentalResilienceProject` | Tracks water, irrigation, sanitation, cooling, climate, marshland, and public-health resilience outcomes. |
| `CulturalEconomyAsset` | Tracks heritage sites, routes, events, museums, crafts, visitor services, and preservation obligations. |
| `InfrastructureCapacityAsset` | Tracks rail corridor-km, grid capacity, water assets, logistics hubs, utilization, maintenance, and renewal reserve. |
| `CivicCapabilityRecord` | Tracks verified civic work, training, certification, task quality, appeals, and transition-to-work outcomes. |
| `BenefitLedgerImpact` | Connects each economic event to cash, output, avoided loss, social capability, cultural capital, and risk. |
| `MinistryFunctionContract` | Converts ministry activity into priced outputs, service-level evidence, audits, and transition gates. |
| `ScenarioAssumptionSet` | Stores baseline, constrained-base, strategic-upper, and stress assumptions with source tags. |

These primitives should not be added as narrative-only labels. They should map
to database records, APIs, dashboards, tests, and public exports if the project
continues toward a true national operating model prototype.

## Failure Modes

The model fails if any of the following become normal:

- Oil equity becomes another off-budget spending channel.
- INDHC becomes an opaque monopoly without competition gates.
- Ministries are renamed but not made accountable.
- Debt is raised for projects without cashflow.
- Public benefits are counted as dividend cash.
- Imported turnkey projects replace Iraqi capability.
- Rail, power, water, or tourism assets are built without maintenance funding.
- Civic work becomes fake payroll.
- Cultural assets are commercialized without conservation.
- Digital IQD becomes surveillance rather than privacy-bounded public evidence.

The model is only credible if the operating logic can stop attractive projects
when they fail legal, fiscal, debt, maintenance, revenue, anti-capture,
privacy, or citizen-fairness gates.

## Bottom Line

Cylinder Seal's coherent national role is to become the evidence and settlement
substrate for a post-oil, post-automation Iraqi development model:

- Oil income is converted into productive assets before distribution.
- Ministries are funded through explicit taxes, levies, and service contracts.
- Citizens hold non-saleable beneficial shares and receive only audited surplus
  dividends.
- Domestic industry, tourism, rail, water, food, green power, and cultural
  services create non-oil cashflows.
- Environmental resilience, civic work, skills, and heritage are measured as
  national value, not hidden in rhetoric.
- Long-term success is judged by non-oil GDP compounding, infrastructure
  reliability, water and climate resilience, household capability, cultural
  renewal, public trust, and audited distributable surplus.

That is the unified economic, environmental, social, and cultural model.
