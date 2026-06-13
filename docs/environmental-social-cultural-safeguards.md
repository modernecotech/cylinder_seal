# Environmental, Social, Water, And Cultural Safeguards

Status: safeguard control model. This is not an environmental approval,
heritage authorization, water-rights decision, resettlement plan, safety
certification, MDB safeguard review, or claim that any Iraqi regulator has
approved any project.

Cylinder Seal cannot be a coherent economic, environmental, social, and
cultural model if growth is purchased by exporting costs to water basins,
marshlands, heritage sites, displaced households, workers, municipalities, or
future maintenance budgets.

## Core Rule

```text
No project, concession, industrial privilege, tourism route, rail segment,
water scheme, facility reuse, civic-work program, or dividend claim should scale
unless environmental, social, water, heritage, safety, maintenance, remediation,
monitoring, audit, and grievance gates pass.
```

The system should not count a project as productive if it only looks profitable
because pollution, water depletion, heritage loss, resettlement, disability
access, or maintenance liabilities are hidden.

## What This Layer Controls

| Risk area | Control requirement |
| --- | --- |
| Environmental assessment | Project-specific environmental and social assessment before capital release. |
| Water budget | Basin, withdrawal, reuse, tariff/social-protection, and drought-stress logic before water-intensive projects proceed. |
| Pollution control | Emissions, effluent, hazardous waste, industrial discharge, dust, noise, and enforcement plan. |
| Climate resilience | Heat, drought, flood, grid stress, dust, and lifecycle resilience built into design. |
| Marshland and biodiversity | No sensitive marshland, wetland, river, or habitat impact without approval, mitigation, monitoring, and conservation funding. |
| Cultural heritage | Heritage authority clearance before tourism, rail, civil works, real estate, or visitor-service commercialization near sensitive assets. |
| Resettlement and livelihoods | No land access or construction before resettlement and livelihood restoration are approved and funded. |
| Community consultation | Local consultation, response-to-comments, affected-person registry, and public summary. |
| Grievance mechanism | Community, worker, SME, landholder, municipal, and disability-access complaint path. |
| Worker and community safety | Safety plan, training, incident reporting, emergency response, and contractor accountability. |
| Maintenance funding | Lifecycle maintenance and monitoring funded before dividend or expansion claims. |
| Remediation escrow | Facility reuse and industrial projects fund known environmental liabilities before restart. |
| Waste and circularity | Waste handling, recycling, materials reuse, and end-of-life responsibilities. |
| Disability access | Public infrastructure, tourism, civic work, and service platforms include access requirements. |
| Monitoring publication | Privacy-safe environmental and social monitoring data is published. |
| Independent audit | External safeguard review exists before scale-up or external financing. |

## Decision States

| Decision | Meaning | Required response |
| --- | --- | --- |
| Blocked | Critical heritage, biodiversity, marshland, resettlement, or livelihood clearance is missing. | Freeze capital release and obtain legal/regulatory clearance. |
| Redesign required | Water, pollution, or ecosystem/heritage risk is too high. | Redesign project scope, technology, site, water use, or mitigation package. |
| Mitigation required | Core design may work, but maintenance, monitoring, or remediation funding is missing. | Fund escrow, O&M, and monitoring before scale. |
| Evidence only | Assessment, public monitoring, or independent audit is missing. | Continue data gathering only; no scale claim. |
| Pilot only | Social risk or readiness score is weak. | Limit scope and prove consultation, safety, accessibility, and grievance performance. |
| Monitoring required | Project can proceed only with enhanced public monitoring and annual review. | Keep scale conditional on monitoring results. |
| Eligible | Safeguard gates pass and risks are controlled. | Proceed with normal monitoring, audit, and maintenance obligations. |

## Sector Application

| Sector | Main safeguard issue |
| --- | --- |
| Petrochemicals, fertilizers, cement, glass, and raw-material processing | Pollution, hazardous waste, water use, feedstock handling, worker safety, and remediation liabilities. |
| Water, desalination, wastewater, and irrigation | Basin effects, brine/discharge, tariff protection, farmer impact, drought resilience, and maintenance funding. |
| Rail, metro, roads, ports, and logistics | Land acquisition, resettlement, disability access, noise, dust, station-area displacement, and maintenance. |
| Tourism, heritage, marshlands, pilgrimage, and cultural routes | Carrying capacity, conservation authority, heritage clearance, local community benefit, visitor safety, and waste/water pressure. |
| Facility recycling | Environmental liability, asbestos/legacy contamination, worker transition, title, cleanup escrow, and safe restart. |
| Green power, grid, district cooling, and waste-to-energy | Land, grid stability, battery/waste handling, emissions assumptions, and lifecycle O&M. |
| Food, agriculture, cold chain, and irrigation equipment | Water efficiency, soil salinity, farmer debt, food safety, spoilage, and cold-chain energy use. |
| Civic work | Safety, child protection, disability access, sensitive heritage tasks, care-work safeguards, and verifier accountability. |

## Use In The Unified Model

The safeguard layer should sit before:

- project-bankability claims;
- external loans, MDB finance, ECA finance, PPPs, green bonds, or green sukuk;
- public procurement awards;
- INDHC equity release;
- facility-reuse restart;
- tourism route commercialization;
- water, irrigation, and desalination expansion;
- rail and metro construction;
- dividend claims based on project surplus.

If a project cannot fund its own maintenance, monitoring, remediation, or
community obligations, its apparent surplus should not enter the dividend
waterfall.

## Software And Data Surface

The executable implementation is:

- `crates/cs-analytics/src/environmental_social_safeguards.rs`
- `migrations/20260718000001_environmental_social_safeguards.sql`

Core objects:

| Object | Purpose |
| --- | --- |
| `EnvironmentalSocialSafeguardInput` | Captures project, domain, governorate/region, assessment status, water budget, pollution risk, climate resilience, biodiversity/marshland sensitivity, heritage status, resettlement, livelihood restoration, consultation, grievance, safety, maintenance, remediation escrow, waste/circularity, disability access, monitoring, and independent audit. |
| `EnvironmentalSocialSafeguardAssessment` | Scores water risk, pollution risk, ecosystem/heritage risk, social risk, readiness, decision, and required actions. |
| `SafeguardGateResult` | Records pass/warn/fail state for environmental, water, pollution, climate, biodiversity, heritage, resettlement, consultation, grievance, safety, maintenance, remediation, waste, accessibility, monitoring, and audit gates. |

## Dashboard Requirements

The safeguard dashboard should show:

- project reference, governorate/region, and domain;
- assessment status;
- water withdrawal, water reuse, water stress, and water-budget result;
- emissions/pollution risk and control status;
- climate resilience score;
- biodiversity, marshland, and heritage flags;
- resettlement and livelihood-restoration status;
- consultation score and grievance status;
- worker/community safety status;
- maintenance and monitoring funding;
- remediation escrow versus estimated liability;
- waste/circularity and disability-access scores;
- monitoring publication and independent audit status;
- decision and required actions.

## Governance Boundary

This layer should be conservative:

```text
If heritage clearance is missing, block.
If marshland or biodiversity approval is missing, block.
If resettlement or livelihood restoration is unfunded, block.
If water or pollution risk is extreme, redesign.
If monitoring or audit is missing, stay evidence-only.
If maintenance or remediation is unfunded, withhold scale-up.
If social readiness is weak, keep to pilot.
```

## Bottom Line

The unified economic model must create durable national wealth, not disguised
environmental debt. A project that cannot protect water, heritage, communities,
workers, accessibility, and future maintenance is not cheap. It is simply
borrowing from Iraq's people, places, and future budgets without recording the
liability.
