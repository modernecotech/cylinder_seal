# Benefit Realization And Claim Audit

Status: accountability model. This is not an official monitoring framework,
audit opinion, macroeconomic validation, or project-feasibility result.

Cylinder Seal now has architecture for ambition, affordability, stress,
sequencing, and political-economy readiness. This document adds the operating
question that matters after implementation starts:

```text
Did the claimed benefit actually happen?
Can it be attributed?
Is it cash or only public benefit?
Can it enter the dividend waterfall?
What happens if it underperforms?
```

## Core Rule

```text
No benefit claim becomes verified without:
  baseline,
  target,
  observed value,
  source confidence,
  attribution confidence,
  evidence quality,
  audit status,
  and cash/public-benefit classification.
```

This prevents scenario numbers from becoming permanent public claims.

## Claim Types

| Claim type | Example | Can pay dividends? |
| --- | --- | --- |
| Settled cash | Paid fare, lease, PPA, service contract, platform fee, export receipt, JV distribution. | Yes, after senior waterfall claims. |
| Avoided cost | Reduced import bill, lower grid losses, reduced fuel burn, avoided water loss. | No, unless converted through a settled savings contract, tariff, fee, or budget transfer. |
| Second-order benefit | Tourism multiplier, SME bankability, local supplier demand, land-value uplift. | No. |
| Capacity metric | Rail-km, MW, cold-chain capacity, water-treatment capacity, facility utilization. | No. |
| Service outcome | Better uptime, fewer stockouts, faster licensing, cleaner streets, higher collection efficiency. | No, except through priced service contracts. |
| Distribution | Citizen dividend, civic-work wages, public transfers. | No; distribution is an outflow, not revenue. |

## Claim Dispositions

| Disposition | Meaning | Required action |
| --- | --- | --- |
| Unsupported | Missing baseline, target, or evidence. | Remove from summaries; collect baseline. |
| Track only | Measured public benefit but not cash or not fully verified. | Keep in dashboard with no-dividend flag. |
| In progress | Cash claim has not settled or audit is incomplete. | Do not count as revenue yet. |
| Verified | Target is mostly met, evidence is strong, and classification is correct. | Publish with source and audit metadata. |
| Underperforming | Claim is partly delivered but below target. | Publish variance and recovery plan. |
| Overstated | Claim is materially below target but not zero. | Remove from front-door claims until revalidated. |
| Failed | No meaningful observed benefit. | Retire, restructure, or cancel the claim. |

## Benefit Domains

| Domain | Typical evidence |
| --- | --- |
| Booked cash | Settlement records, invoices, bank receipts, wallet batches, audited accounts. |
| Import substitution | Domestic sales, import baseline, delivered-cost comparison, supplier evidence. |
| Tourism services | Bookings, platform fees, hotel/JV receipts, guide payments, merchant settlement. |
| Infrastructure | Availability, uptime, farebox, PPA, service contract, station lease, maintenance records. |
| Environmental resilience | Metered savings, water use, pollution data, restoration evidence, verified tasks. |
| Social capability | Training completions, civic-work proofs, employment transition, household access metrics. |
| Ministry productivity | Service outputs, cost per output, complaint resolution, budget-release conditions. |
| Citizen dividend | Eligibility snapshot, exception queue, settled batch, waterfall statement. |
| Diaspora channel | Formal remittance, export order, referral, investment lead, expertise delivery. |
| Strategic resilience | Domestic capacity, critical spares, supplier diversification, audit boundary. |

## Gate Logic

| Gate | Pass condition |
| --- | --- |
| Baseline and target | Both are present before the claim is published. |
| Evidence quality | Evidence is strong enough for the claim's materiality. |
| Source confidence | Source is dated, traceable, and appropriate. |
| Attribution confidence | Method credibly links the observed change to the program. |
| Audit complete | Independent audit exists for cash or material claims. |
| Cash settlement | Cash claims have actually settled. |
| Dividend boundary | Non-cash benefits carry a no-dividend flag. |
| Material variance | Observed value is close enough to target, or variance is disclosed. |

## Software And Data Surface

The executable implementation is:

- `crates/cs-analytics/src/benefit_realization.rs`
- `migrations/20260715000001_benefit_realization_claim_audit.sql`

Core objects:

| Object | Purpose |
| --- | --- |
| `BenefitRealizationInput` | Captures baseline, target, observed value, claim type, cash/public benefit amounts, confidence scores, audit status, settlement status, and dividend boundary. |
| `BenefitRealizationReport` | Computes achievement, variance, evidence score, realization score, cash eligibility, public-benefit-only value, disposition, and corrective actions. |
| `BenefitRealizationGateResult` | Records pass/warn/fail state for evidence and classification gates. |

## Dashboard Requirements

Every public benefit dashboard should show:

- claim reference;
- baseline;
- target;
- observed value;
- unit;
- source;
- attribution method;
- confidence;
- audit state;
- cash-waterfall eligibility;
- no-dividend flag;
- disposition;
- corrective actions.

## Front-Door Claim Rule

Only verified claims may appear in the README, executive summary, or public
presentation as delivered outcomes.

Claims that are track-only, in-progress, underperforming, overstated, failed, or
unsupported must stay in internal dashboards or caveated scenario documents.

## Bottom Line

This layer is the antidote to optimism drift.

```text
If it is not measured, it is not delivered.
If it is not settled, it is not cash.
If it is not audited, it is not verified.
If it is not attributable, lower confidence.
If it is public benefit only, keep it out of dividends.
If it underperforms, publish the variance.
```
