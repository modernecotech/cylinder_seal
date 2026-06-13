# Iraq Integrated Growth Impact Model

This document quantifies how the Cylinder Seal, INDHC, open-source rail,
industrial, infrastructure, tourism, green, food/water, Digital IQD, and civic
work model could affect Iraq's non-oil growth path over ten years.

Status: scenario model. It is not an official forecast, budget law, investment
prospectus, debt recommendation, or externally validated macroeconomic model.

## Bottom Line

The model can plausibly create high non-oil growth if it is governed as a
disciplined productive-capital program rather than a spending program.

Using IMF 2025 Article IV data, Iraq's baseline non-oil real GDP growth is
projected at 2.5-3.0 percent for 2027-2030, with current non-oil potential
around the 3-4 percent range. IMF staff also state that reforms in labor,
business regulation, the financial sector, and governance could double non-oil
potential growth in the medium term.

This model therefore uses three paths:

| Path | 2036 non-oil real growth | 2036 non-oil GDP index, 2026=100 | 2036 non-oil GDP, constant 2026 USD | Additional real non-oil GDP vs baseline |
| --- | ---: | ---: | ---: | ---: |
| Baseline | 3.5% | 137.0 | USD 248B | N/A |
| Low-execution case | 4.3% | 143.2 | USD 260B | USD 11B |
| Constrained-base execution | 6.2% | 158.6 | USD 287B | USD 39B |
| Strategic-upper execution | 7.9% | 175.0 | USD 317B | USD 69B |

The headline is deliberately non-oil. Total GDP can still be pulled around by
oil production, OPEC+ constraints, oil prices, and export interruptions. The
success test is whether Iraq's non-oil economy compounds faster and becomes less
dependent on raw oil allocation.

## Source Discipline

| Data point | Use in the model | Source |
| --- | --- | --- |
| IMF projections for Iraq real GDP, non-oil real GDP, GDP in USD, non-oil GDP in IQD, investment, public capex, debt, reserves, and fiscal deficits for 2025-2030. | Baseline growth path and non-oil GDP starting point. | [IMF Iraq 2025 Article IV, Table 1](https://www.imf.org/-/media/files/publications/cr/2025/english/1irqea2025001-source-pdf.pdf) |
| IMF statement that structural reforms in labor, business regulation, financial sector, and governance could double non-oil potential GDP growth in the medium term. | Upper-bound plausibility check for high non-oil growth. | [IMF Iraq 2025 Article IV, paragraph 43](https://www.imf.org/-/media/files/publications/cr/2025/english/1irqea2025001-source-pdf.pdf) |
| IMF warning that fiscal risks require adjustment and non-essential spending limits, while crucial non-oil capital spending should be protected. | Keeps the growth scenario tied to affordability gates. | [IMF Iraq 2025 Article IV, paragraphs 39-40](https://www.imf.org/-/media/files/publications/cr/2025/english/1irqea2025001-source-pdf.pdf) |
| World Bank description of Iraq's oil dependence, 2025 non-oil slowdown, water/electricity/liquidity constraints, and youth employment challenge. | Identifies the binding constraints that the program targets. | [World Bank Iraq country page](https://www.worldbank.org/ext/en/country/iraq) |
| World Bank research finding that a public-investment increase of 1 percent of GDP can raise output by 1.1 percent after five years on average in EMDEs, with effects up to 1.6 percent under stronger efficiency and fiscal-space conditions, and can crowd in private investment and raise productivity. | Sanity check for infrastructure and public-investment transmission. | [World Bank, Revisiting Public Investment Multipliers](https://openknowledge.worldbank.org/server/api/core/bitstreams/bbb8be60-fdf8-4353-abb1-945f72b65448/content) |
| Iraq National Development Plan 2024-2028 focus on infrastructure, public services, sustainability, and economic sectors including agriculture, industry, and tourism. | Confirms that the sector focus is aligned with Iraq's planning frame. | [UNDP Iraq National Development Plan 2024-2028](https://www.undp.org/iraq/publications/iraq-national-development-plan-2024-2028) |
| WTTC Iraq tourism impact data separating direct and wider tourism contribution. | Tourism and second-order benefits discipline. | [WTTC Iraq Economic Impact Report 2024](https://assets-global.website-files.com/6329bc97af73223b575983ac/6643856bc693733a9f435ca5_EIR2024-Iraq.pdf) |

These sources support the baseline and the direction of transmission. They do
not validate the exact sector contributions below.

## Method

Machine-readable table:
[docs/data/iraq-integrated-growth-impact-timeline.csv](data/iraq-integrated-growth-impact-timeline.csv).

The model starts from IMF's 2026 non-oil GDP level of IQD 235.6T. Using a simple
IQD 1,300 per USD conversion, this equals about USD 181B in 2026-price terms.

The model then applies:

```text
Scenario non-oil growth
  = baseline non-oil growth
  + industrial/import-substitution contribution
  + open-source rail/logistics contribution
  + green power/grid contribution
  + food/water/irrigation contribution
  + tourism/services contribution
  + Digital IQD formalization/credit contribution
  + civic-work/workforce contribution
```

Additional real non-oil GDP is calculated as:

```text
(scenario non-oil GDP index - baseline non-oil GDP index)
  * 2026 non-oil GDP in USD
```

This is a real-output index model, not a nominal revenue forecast. It does not
count oil-price windfalls as growth success.

## Growth Contribution Timeline

Percentage points added to baseline non-oil real growth.

| Year | Phase | Baseline non-oil growth | Constrained add-on | Constrained growth | Strategic add-on | Strategic growth | Constrained extra non-oil GDP | Strategic extra non-oil GDP |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 2027 | Foundation | 2.5% | 0.2pp | 2.7% | 0.4pp | 2.9% | USD 0.4B | USD 0.7B |
| 2028 | Foundation | 2.5% | 0.4pp | 2.9% | 0.8pp | 3.3% | USD 1.1B | USD 2.2B |
| 2029 | Build | 3.0% | 0.7pp | 3.7% | 1.3pp | 4.3% | USD 2.5B | USD 4.8B |
| 2030 | Build | 3.0% | 1.1pp | 4.1% | 1.9pp | 4.9% | USD 4.7B | USD 8.7B |
| 2031 | Build | 3.5% | 1.5pp | 5.0% | 2.5pp | 6.0% | USD 7.9B | USD 14.2B |
| 2032 | Scale | 3.5% | 1.8pp | 5.3% | 3.0pp | 6.5% | USD 12.1B | USD 21.4B |
| 2033 | Scale | 3.5% | 2.1pp | 5.6% | 3.5pp | 7.0% | USD 17.3B | USD 30.4B |
| 2034 | Scale | 3.5% | 2.3pp | 5.8% | 3.8pp | 7.3% | USD 23.5B | USD 41.2B |
| 2035 | Compound | 3.5% | 2.5pp | 6.0% | 4.2pp | 7.7% | USD 30.8B | USD 54.0B |
| 2036 | Compound | 3.5% | 2.7pp | 6.2% | 4.4pp | 7.9% | USD 39.0B | USD 68.8B |

Interpretation:

- The first two years are mostly institutional and construction effects.
- The growth acceleration becomes material only when assets operate, revenue is
  collected, imports are replaced credibly, credit expands, and rail/power/water
  constraints ease.
- The strategic-upper path is plausible only if the operating logic prevents
  corruption, debt stress, import leakage, and prestige construction.

## Sector Contribution Logic

By Year 10, the constrained-base case adds 2.65 percentage points to non-oil
real growth. The strategic-upper case adds 4.37 percentage points.

| Channel | Year-10 constrained add-on | Year-10 strategic add-on | Growth mechanism |
| --- | ---: | ---: | --- |
| Industrial production and import substitution | 0.60pp | 0.99pp | Higher domestic value added, supplier networks, maintenance services, raw-material processing, public procurement substitution, selective exports. |
| Infrastructure, open-source rail, and logistics | 0.55pp | 0.91pp | Lower transport costs, higher labor-market access, denser cities, better visitor corridors, cheaper freight, domestic rail supply chains. |
| Green power, grid, and HVAC efficiency | 0.45pp | 0.74pp | Fewer outages, lower fuel and import pressure, reliable industrial power, efficient cooling, lower production risk. |
| Food, water, desalination, irrigation, and cold chain | 0.35pp | 0.58pp | Less spoilage, higher farm and processor productivity, better water security, import substitution in staples and food services. |
| Tourism and tradable services | 0.30pp | 0.50pp | Visitor spending, hospitality, cultural routes, healthcare/education/business services, diaspora demand, non-oil FX. |
| Digital IQD formalization, SME credit, and tax visibility | 0.25pp | 0.41pp | More bankable merchants, better receivables finance, lower leakage, faster settlement, wider tax base without crushing microbusinesses. |
| Civic work, workforce, and public-value maintenance | 0.15pp | 0.25pp | Paid maintenance, environmental work, municipal repair, training records, bridge-to-work pathways, social legitimacy during productivity transition. |
| **Total** | **2.65pp** | **4.37pp** | Non-oil growth acceleration by Year 10. |

These are contribution estimates. The system must publish them as model
assumptions with confidence levels, not as independently proven outcomes.

## Phase Logic

### 2027-2028: Foundation

Growth impact is small because the system is mostly building institutions and
project pipelines.

Primary work:

- Oil Income Lockbox and INDHC legal design.
- Economic operating logic, ledgers, and hard gates.
- Project registry, procurement baselines, and source tags.
- Quick rehabilitation of productive assets.
- Digital IQD merchant, procurement, and civic-work evidence rails.
- Open-source rail reference architecture and city-corridor selection.
- Tourism payment rails and city-service baselines.

Expected result: non-oil growth rises only modestly above baseline, but the
state creates the measurement and governance system required for later growth.

### 2029-2031: Build

Growth becomes visible as early assets operate.

Primary work:

- Industrial processing belts, construction inputs, HVAC/electronics assembly,
  water equipment, and defense-controlled sustainment.
- First open-source rail corridors, logistics hubs, and station-area services.
- Cold chain, food processing, irrigation equipment, and farmer/processor
  credit.
- Green power, grid efficiency, and industrial power zones.
- Tourism corridors, hotel/JV finance, guide registry, and visitor platforms.
- Civic work shifts toward maintenance, municipal repair, climate resilience,
  sanitation, parks, sport, culture, and training.

Expected result: constrained-base non-oil growth reaches about 5.0 percent by
2031; the strategic-upper case reaches about 6.0 percent if capital delivery and
private crowd-in are strong.

### 2032-2034: Scale

This is where the model should start looking like high growth rather than
project spending.

Primary work:

- Supplier networks deepen around industrial champions.
- Rail and logistics reduce the delivered cost of goods and labor movement.
- Power reliability improves industrial utilization.
- Food/water systems reduce losses and import pressure.
- Tourism second-order benefits appear in merchant revenues, food supply chains,
  city services, foreign-currency capture, and SME credit histories.
- Digital IQD evidence lets banks expand credit against actual receivables.

Expected result: constrained-base non-oil growth reaches 5.3-5.8 percent;
strategic-upper growth reaches 6.5-7.3 percent.

### 2035-2036: Compound

The system either compounds or exposes failure.

If the operating logic works:

- Infrastructure lowers the cost base.
- Industrial suppliers localize components.
- Tourism and tradable services become repeatable FX channels.
- Civic work becomes a bridge into skills and municipal productivity.
- Ministries receive more funding from visible productive surplus.
- Dividends grow only from audited surplus.

Expected result: constrained-base non-oil growth reaches about 6.2 percent by
2036; strategic-upper growth reaches about 7.9 percent. In 2026-dollar terms,
that is about USD 39B and USD 69B of additional real non-oil GDP respectively
versus baseline.

## Open-Source Rail As A Growth Platform

Open-source rail is not only a transport project. It is a productivity platform.

Growth channels:

- Faster worker access to jobs and training.
- Lower logistics cost for food, materials, tourism, and industrial parks.
- More predictable urban travel for women, students, service workers, and SMEs.
- Station-area retail, leases, advertising, and land-value capture.
- Domestic fabrication of components, depots, maintenance systems, signaling
  interfaces, software, and station equipment.
- Better visitor corridors for Najaf-Karbala, Baghdad/Kadhimiya, Babylon/Ur,
  marshland routes, Basra, Mosul, and other city systems.

The "open-source" principle matters because it reduces vendor lock-in, allows
Iraqi maintenance capability to grow, and makes metro/light-rail delivery a
repeatable national program rather than a one-off prestige procurement.

## Civic Work As Growth Infrastructure

Civic work is normally discussed as social policy. In this model it is also
economic infrastructure.

Growth channels:

- Keeps people attached to paid, verified activity during automation and
  ministry transition.
- Produces visible public value: street repair, drainage, parks, tree cover,
  sports facilities, tourism sanitation, cultural events, care work, disaster
  readiness, and environmental restoration.
- Creates work histories, training records, and municipal output data.
- Improves the physical environment for tourism, retail, transport, and local
  services.
- Reduces resistance to productivity reforms by giving citizens a participation
  pathway that is not old-style ministry payroll.

Civic work should be counted as growth only when it creates verified outputs or
improves employability, service quality, tourism readiness, public health, or
municipal productivity.

## What Makes Growth High

The high-growth path does not come from one magic sector. It comes from loops
reinforcing each other:

```text
Industrial capacity lowers imports.
Rail and logistics lower delivered costs.
Green power raises utilization.
Water and food systems lower losses.
Tourism brings outside demand.
Digital IQD makes cashflow bankable.
Civic work improves public space and employability.
Ministry feedback improves services.
Profits fund reinvestment and dividends.
Citizen and SME spending recycles demand domestically.
```

That combination is why the strategic-upper path can approach 8 percent non-oil
growth in later years. Without governance and execution, the same spending can
become debt, imports, and unfinished assets.

## Failure Cases

| Failure | Growth consequence |
| --- | --- |
| Governance failure | Capex leaks into imports, patronage, and overruns; low-execution path becomes more likely. |
| Debt stress | Projects freeze, dividends stop, and fiscal consolidation crowds out productive investment. |
| Rail lock-in | Open-source rail becomes vendor-dependent prestige construction; logistics gains shrink. |
| Industrial protectionism | Domestic prices rise without productivity; import substitution becomes a tax on citizens. |
| Power and water bottlenecks persist | Factories and food systems underutilize capacity. |
| Tourism is not formalized | Visitor spend remains informal; second-order benefits do not become tax, credit, or investment. |
| Civic work becomes fake jobs | Public value disappears and the model reverts to payroll politics. |
| Digital IQD loses trust | Formalization, credit, dashboard visibility, and tax-base expansion stall. |

## Dashboard Metrics

| Metric | Purpose |
| --- | --- |
| `NonOilRealGrowthScenario` | Shows baseline, constrained-base, strategic-upper, and actual non-oil growth. |
| `SectorGrowthContribution` | Publishes assumed and observed contribution by sector. |
| `NonOilGDPIndex` | Tracks real non-oil compounding with 2026 as base year. |
| `AdditionalNonOilGDPVsBaseline` | Shows real GDP level gain versus baseline. |
| `PrivateInvestmentCrowdIn` | Tests whether public and INDHC investment attract private capital. |
| `OpenRailCostReduction` | Measures travel time, logistics cost, ridership, and station-area revenue. |
| `IndustrialCapacityUtilization` | Shows whether factories are operating, not just built. |
| `PowerReliabilityForIndustry` | Tracks outages, industrial power availability, and production impact. |
| `WaterFoodProductivityGain` | Tracks water efficiency, spoilage, yields, processing utilization, and food-import substitution. |
| `TourismSecondOrderBenefit` | Tracks merchant spend, city services, supply chains, FX capture, and repeat visits without counting it as INDHC cash. |
| `CivicWorkPublicValueIndex` | Tracks verified civic outputs, training, employability, and municipal productivity. |
| `GrowthClaimConfidence` | Labels every growth claim as observed, modeled, estimated, or aspirational. |

## Bottom Line

The integrated program can justify a high-growth ambition, but only if the
growth claim is made in the right way:

```text
Baseline Iraq: low-to-moderate non-oil growth under fiscal and infrastructure constraints.
Constrained-base Cylinder Seal / INDHC execution: non-oil growth can rise toward 6% by Year 10.
Strategic-upper execution with strong governance and crowd-in: non-oil growth can approach 8% by Year 10.
```

The best headline is not "guaranteed GDP boom." It is:

```text
The model creates a testable path for Iraq to move from oil-financed spending
to high non-oil growth, if capital is governed through hard gates, booked cash,
public-benefit ledgers, reinvestment discipline, open infrastructure, and
citizen-facing accountability.
```
