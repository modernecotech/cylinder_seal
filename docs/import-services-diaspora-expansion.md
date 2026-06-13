# Import, Services, And Diaspora Expansion

Status: strategy and implementation-extension note. This is not a forecast, a
trade policy order, a tourism concession, a migration policy, or an investment
solicitation. It defines what Cylinder Seal should measure before Iraq treats
import substitution, attraction-based services, or diaspora channels as real
economic operating levers.

## Source Discipline

| Source | How it is used |
| --- | --- |
| [OEC Iraq profile](https://oec.world/en/profile/country/irq) and [World Bank WITS/Comtrade](https://wits.worldbank.org/) | Screens large import categories such as vehicles, refined petroleum, broadcasting/telecom equipment, air conditioners, medicaments, rice, jewellery/gold, machinery, and other finished goods. |
| [International Trade Portal Iraq trade profile](https://www.lloydsbanktrade.com/en/market-potential/iraq/trade-profile) | Cross-checks Iraq's broad merchandise and services trade structure, including goods imports, services imports, oil-export dependence, and primary import categories. |
| [Trade.gov Iraq market overview](https://www.trade.gov/country-commercial-guides/iraq-market-overview) | Supports the reform rationale: Iraq needs private-sector productivity, banking reform, and less oil-price dependence. |
| [UNESCO Ahwar of Southern Iraq page](https://whc.unesco.org/en/list/1481/) and [UN Iraq marshlands note](https://iraq.un.org/en/116172-water-conservation-and-human-rights-are-inseparable-iraq%E2%80%99s-marshlands-call-action-world) | Grounds the attraction-based services model in real natural, cultural, and ecological assets while requiring conservation gates. |
| [World Bank remittances work](https://www.worldbank.org/en/topic/migration/brief/remittances-knomad) and [World Bank remittance data](https://data.worldbank.org/indicator/BX.TRF.PWKR.CD.DT?locations=IQ) | Frames diaspora money as a formalization, consumer-protection, AML, and payment-cost problem, not merely a capital slogan. |

The data sources justify screening sectors. They do not prove project
profitability. Cylinder Seal should still require delivered-cost comparisons,
quality evidence, demand contracts, compliance checks, and source-tagged cash
settlement before any claim enters the dividend waterfall.

Related layer: [Facility Recycling And Capital Markets](facility-recycling-and-capital-markets.md)
adds the brownfield-first screen. For every import, service, tourism, diaspora,
or export-distribution opportunity below, Cylinder Seal should first test
whether an underutilized Iraqi facility, depot, workshop, hotel, warehouse, or
market infrastructure asset can be rehabilitated and financed before greenfield
capex is approved.

## Import Areas That Were Under-Specified

The existing plan already covered food, water, irrigation, pharma, electronics,
HVAC, construction materials, petrochemicals, fertilizers, textiles, raw
materials, and regulated defense sustainment. The remaining large import areas
that needed explicit treatment were:

| Import area | What to domesticate first | Why this is the right first move | Main gate |
| --- | --- | --- | --- |
| Vehicles and auto parts | Maintenance depots, tires, batteries, filters, body repair, fleet refurbishment, municipal buses, light assembly. | Full passenger-car manufacturing is too capital-intensive at first, but Iraq has immediate fleet, logistics, public transport, and maintenance demand. | Delivered cost, warranty performance, parts availability, safety certification. |
| Industrial machinery | Repair, rebuild, spare parts, pumps, motors, valves, control cabinets, fabrication, tool rooms. | Machinery localization should begin with uptime and maintenance rather than pretending Iraq can replace every imported machine immediately. | Local technician certification, spares inventory, downtime reduction. |
| Refined fuel and LPG | Storage, blending, distribution metering, refinery maintenance, LPG cylinder safety, petrochemical integration. | This is not ordinary import substitution; it is energy-system reliability, leakage control, and downstream value capture. | Safety, metering, environmental compliance, transparent pricing. |
| Telecom and broadcast equipment | Towers, cabinets, fiber accessories, routers/CPE assembly, repair centers, public-service connectivity kits. | Iraq can capture installation, maintenance, assembly, and network-service value before advanced chip production. | Cybersecurity review, vendor openness, standards compliance. |
| Jewellery, gold, and precious metals | Assay, hallmarking, refinery services, e-receipts, formal retail settlement, AML-supervised gold markets. | The goal is formalization and leakage control, not subsidized gold manufacturing. | AML/CFT, beneficial ownership, source-of-funds, consumer protection. |
| Plastics and packaging | Food packaging, irrigation pipe, construction films, containers, industrial packaging, recycling. | Domestic food, agriculture, water, and construction plans all need packaging and plastic inputs. | Environmental compliance, quality standards, feedstock discipline. |
| Furniture and prefabricated buildings | School furniture, hospital furniture, modular public buildings, hotel fit-out, office systems. | Public works, tourism, schools, hospitals, and housing create repeatable anchor demand. | Fire/safety standards, local wood/metal inputs, procurement competition. |
| Paper and paperboard | Packaging board, labels, cartons, public forms, sanitary paper, recycling. | Domestic food processing and e-commerce need reliable packaging; public administration needs secure forms. | Recycling rate, cost parity, quality certification. |
| Rubber and tires | Retreading, tire recycling, fleet tires, seals, hoses, industrial rubber parts. | Public fleets, logistics, rail, agriculture, and machinery need repeat maintenance inputs. | Safety, warranty, environmental handling. |
| Apparel and footwear | Uniforms, workwear, shoes, PPE, hotel linens, school textiles, cultural retail. | Public procurement and tourism hospitality create quick local labor absorption. | Quality, labor standards, SME participation. |
| Medical devices and technical apparatus | Hospital consumables, diagnostic accessories, repair/calibration, oxygen systems, sterilization equipment. | Health resilience improves before high-complexity device manufacturing is attempted. | Certification, traceability, safety, calibration evidence. |

Implementation already began in code through
`ProductionSector::major_import_gap_sectors()` in
`crates/cs-analytics/src/production_capacity.rs` and the updated production
capacity migration. Each sector must still pass anti-protectionism gates:
quality certification, import-parity discipline, local-content evidence,
maintenance funding, and public-procurement dependence limits.

## Domesticating Services From Iraq's Attractions

Iraq's natural and cultural assets should be treated as production inputs for
services, not only as heritage slogans. The service model covers:

| Attraction/service cluster | Domestic services to produce | Advantages |
| --- | --- | --- |
| Pilgrimage shrines and religious corridors | Lodging, transport, sanitation, crowd services, multilingual guides, booking, insurance, food, retail, medical support. | High recurring demand, strong diaspora links, formal FX capture. |
| Archaeology and heritage sites | Ticketing, conservation work, certified guides, museums, site transport, crafts, events, education packages. | Converts heritage into jobs while funding protection. |
| Marshlands and wetlands | Boat tours, eco-lodging, local food, reed crafts, guide cooperatives, conservation services, biodiversity education. | Creates income only if water and ecosystem protection are funded. |
| Mountains and eco-tourism | Trails, guesthouses, outdoor guides, safety services, equipment rental, local food, seasonal festivals. | Diversifies tourism beyond pilgrimage and distributes income regionally. |
| Rivers and waterfronts | River transport, promenades, restaurants, events, safety, cleaning, water sports where feasible. | Links urban services with hospitality and municipal revenue. |
| Desert and caravan routes | Heritage routes, stargazing, cultural camps, logistics, safety, conservation. | Lower capex than urban megaprojects, but requires strict safety and environmental controls. |
| Urban culture and food | Food districts, markets, festivals, creative retail, film/media services, city tours. | Turns domestic SMEs into visitor-facing export firms. |
| Wellness, medical, education, and business services | Clinics, rehab, conferences, language/culture programs, university programs, professional services. | Higher-value tradable services tied to diaspora and regional demand. |

Cylinder Seal should quantify each cluster with the following identities:

```text
Visitor spend potential
  = annual visitors * average spend per visitor

Booked service revenue
  = visitor spend potential * formal payment capture rate

Non-oil FX capture
  = booked service revenue * foreign visitor share

Local supplier demand
  = booked service revenue * local procurement rate

Second-order benefit
  = local supplier demand * attraction-specific multiplier
```

Only booked service revenue and collected fees can enter INDHC cashflow. Local
supplier demand, brand effects, repeat visits, merchant bankability, and
conservation value are public-benefit ledger entries until they settle as
taxes, leases, platform fees, service contracts, or merchant revenue.

Operational rollout:

| Phase | Years | Work | Metrics |
| --- | --- | --- | --- |
| Baseline | 1 | Visitor counts, carrying capacity, safety, sanitation, lodging, transport, guide registry, merchant acceptance. | Verified sites, guide count, hotel beds, transport seats, Digital IQD acceptance. |
| Service pilots | 1-2 | Launch booking/payment rails, certified guide marketplace, sanitation contracts, local food/craft merchant onboarding. | Formal payment capture, visitor rating, local procurement share, complaint resolution. |
| Corridor build | 3-5 | Najaf-Karbala, Baghdad/Kadhimiya, Babylon/Ur, marshlands, mountain/eco routes, medical/education packages. | Booked revenue, non-oil FX, jobs, conservation score, repeat bookings. |
| Scale | 6-8 | Integrated rail/bus links, events, diaspora packages, healthcare/education/business services, exportable booking tools. | FX capture, service exports, municipal revenue, SME credit records. |
| Mature | 9-10 | International distribution, quality marks, conservation endowment, visitor-data audit, renewal reserve. | Maintenance funding, carrying-capacity compliance, repeat visits, private reinvestment. |

The new `TourismServicesEngine` models this directly and blocks scaling when
visitor safety, conservation, carrying capacity, maintenance funding, lodging,
transport, formal payments, or guide certification are weak.

## Diaspora As Income, Expertise, Capital, Marketing, And Distribution

The diaspora should not be treated only as remittance money. The coherent role
is a five-channel operating model:

| Channel | What Cylinder Seal measures | Why it matters |
| --- | --- | --- |
| Formal remittances | Formal capture rate, payment cost, recipient wallet settlement, AML status. | Lowers leakage and creates household financial histories. |
| Iraqi goods and e-commerce | Diaspora orders, platform fees, export receipts, return rates, quality marks. | Turns diaspora demand into a distribution bridge for Iraqi food, crafts, textiles, cultural goods, and services. |
| Export distribution | Registered distributors, repeat buyers, certifications, logistics, foreign-currency settlement. | Diaspora businesses can open shelves and service channels abroad faster than state marketing offices. |
| Expertise | Verified mentor hours, technical reviews, curriculum work, board/adviser roles, value of donated/procured expertise. | Captures know-how without pretending donated hours are dividend cash. |
| Capital | Diaspora bonds, project syndicates, co-investment leads, close probability, investor suitability. | Provides optional capital, but only after suitability, disclosure, and AML gates. |
| Tourism referrals | Packages sold, referral conversion, repeat visits, cultural/religious events, medical/education referrals. | Converts identity and family ties into measurable service exports. |
| Brand marketing | Reach, conversion, order value, campaign cost, return rates. | Makes diaspora a marketing network for Iraqi products and services. |

The measurement identities are:

```text
Addressable member spend
  = verified diaspora members * conversion rate * average annual spend

Iraqi goods/services demand
  = addressable member spend * Iraqi product share

Booked income
  = platform revenue + platform-fee share of demand + export order value

Formalized remittance
  = remittance value * formal remittance capture rate

Expertise value
  = verified expertise hours * benchmark hour value

Investment pipeline
  = commitments * close probability

Marketing-attributed revenue
  = marketing reach * referral conversion rate * average order value
```

Booked income, formalized remittances, and export receipts can be cash ledger
items after settlement. Expertise value, marketing attribution, and unclosed
investment pipeline remain no-dividend public-benefit or pipeline entries until
they become paid invoices, closed investments, taxes, fees, or verified export
receipts.

Diaspora governance gates:

- KYC/AML and sanctions screening for remittances, investments, and marketplace
  sellers.
- Consumer protection, refunds, dispute resolution, and product authenticity.
- Export quality certification and traceability for Iraqi-origin goods.
- Data privacy review before using diaspora identity, family, or payment data.
- Investor suitability, risk disclosure, and use-of-proceeds registry for any
  diaspora bond, syndicate, or project vehicle.
- Distribution partner coverage before foreign-market revenue claims scale.

The new `DiasporaChannelsEngine` represents this model in code and gives the
dashboard a way to distinguish cash income, remittance formalization, export
distribution, expertise value, marketing value, and investment pipeline risk.
