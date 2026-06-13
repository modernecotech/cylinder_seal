# Procurement Integrity And Market Discipline

Status: control model. This is not a procurement accusation, legal opinion,
bid evaluation, tender document, debarment decision, or claim that any specific
Iraqi contractor or institution has failed.

Cylinder Seal cannot be rational if capital allocation simply becomes a new
route for rent extraction. This document adds the procurement and market
discipline layer for projects, ministry service contracts, industrial champion
privileges, facility reuse, PPP/JV concessions, digital platforms, tourism
services, civic work, and strategic resilience production.

## Core Rule

```text
No award, payment privilege, project disbursement, or champion preference
without:
  beneficial ownership,
  competition depth or justified exception,
  price benchmark,
  open contracting data,
  independent evaluation,
  milestone evidence,
  delivery performance,
  payment discipline,
  and SME market protection.
```

## Procurement Decisions

| Decision | Meaning | Required response |
| --- | --- | --- |
| Eligible | Gates pass and risk is low. | Award or continue with routine monitoring. |
| Watch | Some risks need monitoring. | Proceed only with enhanced disclosure and review. |
| Restricted | Competition, transparency, delivery, or market risks are high. | Cap scope, withhold privilege, or require board/audit approval. |
| Suspended | Ownership, justification, related-party, or concentration gates fail. | Freeze award or payment until remediated. |
| Cancel or retender | PEP/sanctions, severe price variance, or extreme amendment risk appears. | Cancel, retender, or refer for legal/audit review. |

## Gate Logic

| Gate | Pass condition |
| --- | --- |
| Beneficial ownership | Contractors, suppliers, SPVs, and major subcontractors disclose beneficial ownership. |
| PEP/sanctions | No unresolved PEP, sanctions, or high-risk ownership hit. |
| Competition depth | At least three qualified bidders, or justified exception. |
| Single-source justification | Direct or emergency awards have published legal and technical justification. |
| Open contracting data | Tender, award, contract, amendment, delivery, invoice, and payment data are structured and visible to auditors or the public where lawful. |
| Independent evaluation | Technical and financial evaluation is separated from political instruction. |
| Price benchmark | Winning bid is within benchmark tolerance or variance is justified. |
| Contract variation | Amendments are limited and independently reviewed. |
| Advance payment | Advances are capped and secured. |
| Milestone evidence | Payments are backed by delivery evidence. |
| Delivery performance | Delays stay inside tolerance or trigger recovery plan. |
| Payment discipline | Government/INDHC pays valid invoices on time, especially for SMEs. |
| Quality | Defect, warranty, and certification failures stay inside tolerance. |
| SME participation | Domestic SMEs are not crowded out by protected champions. |

## Why Payment Discipline Is A Market Integrity Issue

Procurement corruption is not only overpaying connected suppliers. It is also
paying legitimate suppliers late until only politically protected firms can
survive.

The model therefore treats payment delay as a market-integrity gate. If valid
SME invoices are not paid on time, supplier diversity collapses and the program
creates the monopoly structure it was meant to prevent.

## Use In Industrial Champions

Industrial champion privileges should be suspended when:

- related-party exposure is high;
- domestic SME participation is thin;
- prices drift above benchmark;
- contract amendments repeatedly inflate value;
- quality defects rise;
- public procurement data is missing;
- valid SME suppliers are paid late.

Champion status is not a license to bypass procurement. It is a temporary
privilege conditional on better measurement and better market formation.

## Use In Facility Recycling

Facility reuse can become asset stripping if procurement discipline is weak.

Before a rehabilitated facility receives public capital, external debt, PPP/JV
capital, or domestic securities, the procurement layer should check:

- title and beneficial owner chain;
- rehabilitation benchmark cost;
- environmental remediation procurement;
- worker transition contracts;
- lease or concession terms;
- related-party subcontractors;
- payment evidence and delivery milestones.

## Use In Ministry Service Contracts

Ministry transition depends on service contracts replacing opaque budget claims.
Those contracts need:

- clear service level;
- price rule;
- grievance path;
- payment trigger;
- performance evidence;
- public dashboard;
- payment discipline.

If service contracts become a disguised direct-award channel, the ministry
transition should pause.

## Software And Data Surface

The executable implementation is:

- `crates/cs-analytics/src/procurement_integrity.rs`
- `migrations/20260716000001_procurement_integrity.sql`

Core objects:

| Object | Purpose |
| --- | --- |
| `ProcurementIntegrityInput` | Captures method, contract value, reference cost, bid data, SME share, related parties, concentration, variations, advances, evidence, delays, quality, ownership, sanctions, open data, evaluation, protest window, and justification. |
| `ProcurementIntegrityAssessment` | Scores competition, integrity, value for money, delivery, market development, overall risk, decision, and required actions. |
| `ProcurementIntegrityGateResult` | Records pass/warn/fail state for procurement gates. |

## Dashboard Requirements

The procurement dashboard should show:

- procurement reference;
- domain and method;
- reference cost and winning bid;
- benchmark variance;
- bidder count and qualified bidder count;
- beneficial ownership status;
- PEP/sanctions flag;
- related-party and supplier concentration;
- contract variation;
- advance payment;
- milestone evidence;
- delivery delay;
- payment delay;
- quality defect rate;
- SME participation;
- decision and required actions.

## Bottom Line

The economic model can only be a productive-capital model if procurement builds
markets instead of allocating rents.

```text
If ownership is hidden, suspend.
If sanctions or PEP risk is unresolved, cancel or refer.
If the price is irrational, retender.
If evidence is weak, withhold payment.
If SMEs are paid late, fix the buyer before blaming the market.
If champions crush suppliers, remove privilege.
```
