# Citizen Entitlement, Privacy, And Appeals

Status: citizen-rights control model. This is not an identity law, social
protection law, inheritance law, sanctions decision, privacy authorization,
dividend entitlement determination, or official citizen registry design.

The national dividend idea is politically powerful because every citizen is
treated as a beneficiary of national productive capital. That makes the
citizen-rights layer one of the highest-risk parts of the model. If identity,
inheritance, privacy, appeals, payment exceptions, accessibility, or suspension
rules are weak, the system can become arbitrary even if the economic model is
financially sound.

## Core Rule

```text
No citizen-share registry, dividend batch, civic entitlement, wallet suspension,
or Digital IQD benefit rollout should scale unless legal authority, identity
integrity, non-saleability, inheritance, minors, deceased records, diaspora and
displacement rules, privacy separation, appeal paths, accessibility, public
dashboards, and independent rights audit pass.
```

The citizen share must be a public entitlement, not a tradable asset, a coerced
collateral object, a patronage channel, or a surveillance bargain.

## What This Layer Controls

| Risk area | Control requirement |
| --- | --- |
| Legal authority | Citizen entitlement, dividend, correction, privacy, and appeal authority exists before rollout. |
| Identity coverage | Registry coverage is high enough that exclusion risk is controlled. |
| Duplicate identities | Duplicate, forged, or conflicting identities remain below tolerance. |
| Identity exceptions | Unresolved records are tracked before dividend batches. |
| Non-saleability | Citizen base shares cannot be sold, pledged, seized, collateralized, or coercively transferred. |
| Inheritance | Share transfer to eligible heirs is rules-based, auditable, and appealable. |
| Minors and guardians | Minor dividend flows have guardian controls and misuse-prevention rules. |
| Deceased records | Death, dormant wallet, fraud, and estate records are reconciled before distributions. |
| Diaspora eligibility | Diaspora, residency, and documentation rules are published before claims. |
| Displaced-person claims | Displaced citizens have a usable claims and correction path. |
| Privacy separation | Identity, entitlement, wallet, regulatory, and analytics data are separated. |
| Data minimization | Benefits and analytics use only the minimum data needed for lawful purpose. |
| Payment exceptions | Failed, duplicate, blocked, or disputed payments stay below tolerance. |
| Appeals | Citizens can challenge exclusion, payment error, privacy breach, suspension, inheritance, or identity disputes. |
| Sanctions due process | Suspension for sanctions, AML, fraud, court order, or investigation has a lawful review path. |
| Accessibility | Offline, disabled, elderly, rural, displaced, and low-literacy users can use service channels. |
| Public dashboard | Aggregate rights, exceptions, appeals, privacy, and payment-health metrics are published. |
| Independent audit | External rights audit exists before scale. |

## Decision States

| Decision | Meaning | Required response |
| --- | --- | --- |
| Blocked | Legal authority, non-saleability, or pledge/collateral protections are missing. | Do not launch entitlement or dividend rollout. |
| Evidence only | Public dashboard or independent rights audit is missing. | Continue registry testing only; do not present as citizen-ready. |
| Remediation required | Privacy, appeals, due process, or identity integrity is too weak. | Fix controls before expanding rollout. |
| Pilot only | Core controls work, but inclusion or operational risk is not yet good enough for national scale. | Limit scope and publish pilot exception metrics. |
| Suspend batch | Payment exceptions, duplicate identities, or unresolved identity exceptions exceed tolerance. | Suspend affected batch until the exception queue is remediated. |
| Eligible | Legal, identity, share, privacy, appeal, inclusion, dashboard, and audit gates pass. | Proceed with monitored batch and public rights reporting. |

## Dividend Batch Firewall

The dividend batch should be blocked or suspended when:

- legal authority is missing;
- citizen shares can be sold, pledged, seized, or collateralized;
- identity duplicates or unresolved exceptions exceed tolerance;
- deceased records are poorly reconciled;
- appeal mechanisms are not live;
- sanctions or AML suspensions have no due process;
- privacy separation is weak;
- payment exception rates are high;
- accessibility channels exclude vulnerable users;
- public dashboard or independent rights audit is missing.

This firewall protects citizens and protects the legitimacy of the model. A
wrongful exclusion can be more politically damaging than a delayed batch.

## Software And Data Surface

The executable implementation is:

- `crates/cs-analytics/src/citizen_rights.rs`
- `migrations/20260720000001_citizen_rights.sql`

Core objects:

| Object | Purpose |
| --- | --- |
| `CitizenRightsInput` | Captures registry reference, legal authority, identity coverage, duplicate and exception rates, non-saleability, pledge/collateral protections, inheritance, minors, deceased reconciliation, diaspora and displaced-person rules, privacy, data minimization, payment exceptions, appeals, sanctions due process, accessibility, dashboard, and audit status. |
| `CitizenRightsAssessment` | Scores identity integrity, rights readiness, privacy, appeals, inclusion, operational risk, decision, and required actions. |
| `CitizenRightsGateResult` | Records pass/warn/fail state for legal, identity, share, inheritance, minor, deceased, diaspora, displaced, privacy, payment, appeal, sanctions, accessibility, dashboard, and audit gates. |

## Dashboard Requirements

The citizen-rights dashboard should show:

- registry snapshot reference;
- legal-authority status;
- identity coverage;
- duplicate identity rate;
- unresolved identity exception rate;
- non-saleability and pledge/collateral status;
- inheritance and minor/guardian controls;
- deceased-record reconciliation;
- diaspora and displaced-person claims status;
- privacy separation and data-minimization scores;
- payment exception rate;
- appeal mechanism, SLA, backlog, and resolution rate;
- suspension due-process status;
- accessibility channel coverage;
- public dashboard and independent audit status;
- decision and required actions.

## Governance Boundary

This layer should be conservative:

```text
If shares can be sold or pledged, block.
If legal authority is missing, block.
If identity or payment exceptions are high, suspend affected batch.
If citizens cannot appeal, remediate before scale.
If privacy separation is weak, remediate before scale.
If audit or public dashboard is missing, stay evidence-only.
```

## Bottom Line

The citizen dividend is not only a cashflow. It is a public-rights system. If
citizens cannot understand, challenge, inherit, protect, and privately receive
their entitlement, the model loses legitimacy at the exact point where it is
supposed to become most universal.
