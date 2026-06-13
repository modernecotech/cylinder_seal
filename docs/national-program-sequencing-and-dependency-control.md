# National Program Sequencing And Dependency Control

Status: program-control model. This is not a government implementation plan,
legal instruction, procurement timetable, financing commitment, or claim that
any Iraqi institution has approved the sequence.

Cylinder Seal now has economic logic, legal gates, political-economy controls,
and fiscal stress controls. This document adds the missing management layer:
which parts can move first, which can only be measured, which can pilot, which
can build, and which must stop.

## Core Sequencing Rule

```text
No legal authority -> no deployment.
No baseline data -> no policy claim.
No audit capacity -> no money movement.
No procurement capacity -> no capital allocation.
No service continuity -> no ministry transition.
No cashflow evidence -> no debt or dividend.
No political readiness -> no scale.
No fiscal stress pass -> no expansion.
```

The purpose is to stop the model from becoming a giant simultaneous reform
where every component depends on every other component working immediately.

## Program Phases

| Phase | Meaning | Allowed work |
| --- | --- | --- |
| Not ready | Legal authority or core dependency is missing. | Research, legal review, source discipline, risk register. |
| Evidence only | The topic can be measured but not operationally moved. | Baselines, dashboards, facility registry, procurement mapping, public-benefit source tags. |
| Pilot | Limited lawful test with explicit scope. | One governorate, one project family, one sector, one payment rail, or one service function. |
| Build | Gates pass for a controlled program. | Capital allocation, contracts, service delivery, audited milestones, limited debt. |
| Controlled scale | Legal, political, fiscal, audit, service, and cashflow gates pass. | Expansion with annual review and rollback powers. |
| Hold or rollback | Political or fiscal conditions have deteriorated. | Freeze new commitments, protect services, restructure, cancel, or reverse transfer. |

## Dependency Ladder

| Dependency | Must precede | Why |
| --- | --- | --- |
| Legal authority | Digital IQD pilots, oil lockbox, INDHC, citizen entitlements, debt, securities, ministry transition. | Without authority, the system is only a research model. |
| Baseline data | Any quantified claim. | Without baselines, benefits and cashflows cannot be falsified. |
| Audit capacity | Money movement, procurement, dividends, ministry service contracts. | Without audit, the model can recreate old opacity. |
| Procurement capacity | INDHC projects, industrial champions, facility reuse, PPP/JV. | Without procurement discipline, capex becomes rent allocation. |
| Political-economy readiness | Ministry transition, industrial privileges, lockbox scaling, citizen dividend rollout. | Without coalition, continuity, and appeals, reform can create backlash or capture. |
| Fiscal stress pass | Scale-up, new debt, guarantees, availability payments, dividends. | Without stress discipline, hidden liabilities migrate back to Treasury. |
| Service continuity | Ministry transition, public utilities, PDS/vouchers, health/education-adjacent services. | Citizens must not lose essential services while the model changes institutions. |
| Cashflow evidence | Debt, dividends, domestic securities, project-company floats. | Invoices, benefits, and narratives are not cash. |
| Staff transition | Ministry transition and civic-work substitution. | Reform should not become arbitrary dismissal or sabotage incentive. |
| Citizen trust and appeal path | Entitlements, dividends, civic work, data sharing, payment suspensions. | Digital governance fails if citizens cannot challenge errors. |

## Domain Sequence

| Domain | Minimum first phase | May build only after | May scale only after |
| --- | --- | --- | --- |
| Legal framework | Not ready / evidence only | Independent legal review and public authority map. | Enacted authority, dispute forum, appeal process, audit mandate. |
| Digital evidence rail | Evidence only | CBI/payment scope, privacy boundary, operator controls. | Security review, data minimization, tested settlement and recovery. |
| Oil Income Lockbox | Evidence only | Constitutional/fiscal review, reconciliation path, audit publication. | 24 months of audited reconciliation and federal/governorate compact. |
| INDHC capital allocation | Evidence only | Charter, board, procurement rules, capital cap, project pipeline. | Clean audits, cashflow evidence, fiscal stress pass, anti-capture controls. |
| Project pipeline | Evidence only | Project registry, facility screen, legal owner, DSCR, FX, safeguards. | Bankability package, procurement evidence, stress and political gates. |
| Industrial champions | Pilot | Sector registry, scorecard, demand contracts, competition authority. | Export/FX path, price discipline, SME inclusion, no hidden bailout. |
| Ministry transition | Evidence only | Service continuity pilot, staff transition fund, appeal desk. | 12-month continuity, parliamentary/audit report, citizen complaint metrics. |
| Civic work | Pilot | Labor-law/privacy review, municipal authority, task verification. | Verified public value, bridge-to-work outcomes, grievance and disability access. |
| Citizen dividend | Evidence only | Entitlement law, identity correction, cashflow waterfall, audit. | Audited distributable surplus after maintenance, debt, levy, reserves, and stress gates. |
| Domestic capital markets | Evidence only | Securities approval, trustee/depository, disclosure, investor protection. | Audited project cashflow, no forced purchase, suitability and AML/CFT controls. |
| Tourism services | Pilot | Safety, conservation, lodging/transport, guides, payment rails. | Carrying capacity, quality certification, local supplier benefit, collection evidence. |
| Facility recycling | Evidence only | Asset registry, title, engineering, environmental, labor transition. | Revenue contract, DSCR, investor protection, finance-lane readiness. |

## Sequencing Inputs

The program controller uses these inputs:

| Input | Meaning |
| --- | --- |
| Legal authority confirmed | Statute, regulation, public mandate, or pilot authority exists. |
| Data baseline quality | Sources, dates, coverage, and confidence are good enough for decisions. |
| Audit capacity | Independent audit can inspect funds, procurement, contracts, and dashboards. |
| Procurement capacity | Tender, award, amendment, delivery, invoice, and payment data can be controlled. |
| Delivery capacity | Project or service operator can actually deliver milestones. |
| Operator readiness | Wallets, registries, dashboards, service desks, or institutions are staffed and tested. |
| Staff transition readiness | Funding, placement, retraining, compensation, and appeals are ready. |
| Citizen trust | Complaints, appeals, privacy, uptime, access, and visible benefits support legitimacy. |
| Service continuity | Critical services have operated under replacement model for enough time. |
| Cashflow evidence | Settled revenue, not just invoices or estimated benefits. |
| Predecessor dependency completion | Required earlier steps are complete enough. |
| Political mode | Output from the political-economy engine. |
| Fiscal mode | Output from the fiscal stress engine. |

## Software And Data Surface

The executable implementation is:

- `crates/cs-analytics/src/program_sequencing.rs`
- `migrations/20260714000001_program_sequencing.sql`

Core objects:

| Object | Purpose |
| --- | --- |
| `ProgramSequencingInput` | Captures legal, data, audit, procurement, delivery, operator, staff, trust, service, cashflow, predecessor, political, and fiscal readiness. |
| `ProgramSequencingDecision` | Computes readiness, dependency, operating-capacity, legitimacy scores, recommended phase, blocked dependencies, and required next actions. |
| `ProgramSequencingGateResult` | Records pass/warn/fail state for sequencing gates. |

## Management Rule

The program controller should run monthly for operational domains and quarterly
for policy domains.

| Output | Decision |
| --- | --- |
| Not ready | Do not spend capex; close legal or data gaps. |
| Evidence only | Publish baselines and risk dashboards; no operational transfer. |
| Pilot | Keep scope small, reversible, and audited. |
| Build | Release money by milestones; keep stress and political gates active. |
| Controlled scale | Expand only with annual review and rollback authority. |
| Hold or rollback | Freeze new commitments and publish recovery or reversal plan. |

## Bottom Line

This layer makes the whole project less theatrical and more governable.

```text
Sequence before scale.
Pilot before build.
Audit before money.
Cash before dividends.
Continuity before ministry transition.
Stress pass before expansion.
Rollback before denial.
```
