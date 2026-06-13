# Federalism, Governorate Equity, And Local Compacts

Status: governance control model. This is not a constitutional opinion,
revenue-sharing law, KRG settlement, land-title determination, governorate
allocation formula, or claim that any Iraqi authority has accepted this model.

Cylinder Seal cannot be credible if oil income is merely centralized under a
new name. A national oil lockbox, INDHC, Digital IQD dividend, rail program,
water program, tourism program, facility-reuse program, or ministry-service
transition has to pass a federalism and governorate-equity layer before it
scales.

## Core Rule

```text
No national flow is scalable unless:
  the local authority is mapped,
  the compact is negotiated or signed,
  allocation variance is explained,
  local revenue, jobs, suppliers, and benefits are visible,
  grievances and appeals work,
  audit data is published,
  and land, water, environmental, heritage, municipal, or regional disputes are resolved.
```

The model is national, but it cannot behave as if every project belongs only to
the center. Producing governorates, municipalities, damaged regions, disputed
areas, and regional authorities need enforceable rules for participation,
benefit, grievance, and audit.

## Why This Layer Exists

The oil-lockbox proposal is meant to stop ministries from passively consuming
oil income. It should not replace ministry capture with central-company capture.

This layer exists to prevent five failures:

- Baghdad-centered allocation without governorate consent or evidence.
- Producing regions bearing environmental and infrastructure burdens while
  benefits are booked elsewhere.
- KRG, disputed-territory, municipal, or governorate authority issues being
  hidden until projects are already financed.
- Local citizens seeing construction, logistics, water, rail, tourism, or
  facility-reuse projects but not receiving jobs, supplier access, services, or
  grievance rights.
- National dashboards counting a benefit that the affected locality cannot see
  or challenge.

## Compact Surface

Each project family or program should have a governorate or regional compact
record before scale-up.

| Compact issue | Required control |
| --- | --- |
| Authority map | Identify whether the competent authority is federal, governorate, municipal, regional, joint, producing-governorate, or disputed. |
| Compact status | Missing, draft, negotiated, signed, disputed, or suspended. |
| Allocation fairness | Compare planned allocation share with population, damage, production burden, poverty, infrastructure deficit, and balanced-development need. |
| Local revenue share | Identify what share of cash revenue, fees, leases, service contracts, taxes, or municipal income is locally visible. |
| Local employment | Track Iraqi staffing by governorate, training seats, apprenticeships, worker safety, and anti-patronage rules. |
| Local suppliers | Track local SME participation, payment discipline, certification, and supplier upgrading. |
| Local benefit capture | Measure whether roads, water, power, transport, parks, services, tourism facilities, or environmental restoration are delivered locally. |
| Grievances | Record open grievances, resolution rate, escalation path, and independent review. |
| Land and water | Block scale-up when title, water rights, land-use permits, resettlement, or authority are disputed. |
| Municipality approval | Require local approval where city assets, streets, utilities, permits, or service contracts are affected. |
| Regional/KRG coordination | Require a separate compact where regional authority, revenue sharing, payments, data, border flows, or disputed authority is involved. |
| Data publication | Publish governorate-level allocation and benefit data at aggregate/privacy-safe level. |
| Local audit | Enable local audit access and public summary reporting. |
| Citizen appeals | Give citizens, SMEs, workers, landholders, and municipalities a usable appeal path. |
| Environmental and heritage consent | Require marshland, water, pollution, resettlement, carrying-capacity, and heritage gates before projects scale. |

## Decision States

| Decision | Meaning | Required response |
| --- | --- | --- |
| Blocked | Authority risk, land-title dispute, water/land authority dispute, or unresolved local legal conflict prevents scale-up. | Freeze capital release and publish dispute-resolution path. |
| Evidence only | Compact may be acceptable but publication, audit, or appeals are not live. | Continue analysis or limited evidence gathering, but do not scale. |
| Compact required | Local, regional, municipal, or disputed-authority compact is missing or too immature. | Negotiate compact and publish minimum terms. |
| Pilot only | Compact exists, but allocation variance or equity score is weak. | Limit scope, test delivery, and fix allocation or benefit capture before expansion. |
| Eligible | Authority, compact, allocation, local capture, grievance, audit, and appeal gates pass. | Proceed with routine annual compact review. |
| Pause or renegotiate | Compact is disputed or suspended. | Stop new commitments and renegotiate terms. |

## Allocation Logic

The model should not use population alone. A fair-share estimate should combine:

- population;
- infrastructure deficit;
- poverty and unemployment;
- historical underinvestment;
- oil and gas production burden;
- environmental damage or water stress;
- displacement and reconstruction needs;
- regional balance;
- project-specific land, water, tourism, logistics, or industrial footprint.

The executable engine compares the planned allocation share with a
needs-adjusted fair share. A large unexplained variance does not automatically
prove unfairness, but it forces disclosure and can cap the program at pilot
scale.

## Use In The Unified Model

This layer should be applied to:

- oil-equity allocations from the lockbox;
- INDHC industrial subsidiaries and industrial parks;
- rail, metro, freight, port, and logistics projects;
- water, desalination, irrigation, marshland, and wastewater projects;
- tourism and cultural-service clusters;
- facility recycling and brownfield concessions;
- ministry service contracts affecting local services;
- civic-work programs;
- strategic resilience and controlled-sector production where lawful;
- Digital IQD dividend, appeal, and service-channel deployment.

## Software And Data Surface

The executable implementation is:

- `crates/cs-analytics/src/federalism_equity.rs`
- `migrations/20260717000001_federalism_equity_compact.sql`

Core objects:

| Object | Purpose |
| --- | --- |
| `FederalismEquityInput` | Captures period, program, governorate/region, authority type, compact status, population and fair-share assumptions, planned allocation, local revenue/jobs/suppliers/benefits, grievances, disputes, publication, audit, appeals, and consent status. |
| `FederalismEquityAssessment` | Scores allocation gap, compact readiness, local capture, grievance quality, authority risk, equity score, decision, and required actions. |
| `FederalismEquityGateResult` | Records pass/warn/fail state for authority, compact, allocation, revenue, employment, supplier, benefit, grievance, land/water, municipal, data, audit, appeal, and consent gates. |

## Dashboard Requirements

The federalism dashboard should show:

- program reference and period;
- governorate or region;
- authority type;
- compact status;
- population share, needs-adjusted fair share, planned allocation share, and
  allocation gap;
- local revenue, employment, supplier, and benefit-capture shares;
- grievance resolution rate and backlog;
- land-title, water, municipal, regional, environmental, and heritage flags;
- data-publication, local-audit, and citizen-appeal status;
- decision and required actions.

## Governance Boundary

This layer does not solve Iraq's federalism disputes. It makes the model honest
about them.

The correct behavior is conservative:

```text
If authority is disputed, block.
If compact is missing, require compact.
If compact is disputed, pause and renegotiate.
If local data, audit, or appeals are missing, stay evidence-only.
If allocation variance is large, limit to pilot.
If local jobs, suppliers, and benefits are weak, fix the compact before scaling.
```

## Bottom Line

The national economic model has to be national in benefit, not merely national
in control. The federalism-equity layer turns governorates, municipalities,
producing regions, regional authorities, and affected communities into explicit
counterparties with measurable rights, rather than treating them as passive
locations where capital happens to land.
