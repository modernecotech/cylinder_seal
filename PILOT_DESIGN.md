# Pilot Design

Cylinder Seal should move from national narrative to a bounded operating test:
one minimum viable jurisdiction, one payment flow, one civic-work flow, one
procurement flow, and one dashboard that records the evidence trail.

The pilot objective is not to prove a national CBDC or a complete economic
reform. The objective is to prove that public economic activity can be made
measurable, auditable, and contestable at small scale before larger legal and
fiscal commitments are considered.

Rail boundary: Samawah is chosen partly because the companion
[OpenSourceRail](https://github.com/modernecotech/OpenSourceRail) project
already carries the rail design, simulator, operations, manufacturing, and
safety-case surface. Cylinder Seal should not duplicate that engineering stack.
It should test whether rail-adjacent enabling works, municipal tasks,
procurement, payments, audit records, and public-benefit claims can be governed
with evidence before any rail capital commitment.

## Minimum Viable Jurisdiction

Recommended pilot: **Samawah / Al-Muthanna municipal civic-work, Open Source
Rail enabling-works, and procurement evidence pilot**.

Rationale:

- It can be defined as one municipality or service zone rather than a national
  reform programme.
- It aligns with the Open Source Rail strategy while staying small: the pilot
  can prepare station-area, roadside, drainage, access, shade, signage, and
  municipal-service evidence before any large rail capital commitment.
- Samawah is a better test bed than Baghdad for first discipline: smaller
  institutional surface, visible municipal needs, lower coordination
  complexity, and easier public observation.
- It can use small, observable civic-work tasks rather than abstract national
  claims.
- It can test local vendors, small procurement, wage payment, supervisor
  signoff, public dashboarding, and grievance handling in one place.
- It is large enough to be meaningful but small enough to stop safely.

Pilot cell:

| Element | Concrete scope |
| --- | --- |
| Jurisdiction | One Samawah / Al-Muthanna municipal service zone. |
| Civic-work programme | One station-area, park, canal, roadside, or access-route maintenance programme with visible public outputs. |
| Supplier category | One local supplier category, such as tools, PPE, signage, drainage-cleaning materials, shade/water points, access-route materials, or small repair support. |
| Payment flow | Verified civic-work wage or stipend release through controlled settlement accounts. |
| Procurement flow | Small local procurement package paid only after delivery evidence and supervisor acceptance. |
| Dashboard | One operator/auditor/public aggregate dashboard covering tasks, payments, procurement, exceptions, grievances, and audit hashes. |
| Explicit exclusions | No CBDC issuance, no oil lockbox, no citizen dividends, no ministry restructuring, no national macro claim. |

Baghdad/Nahrawan municipal corridors and Najaf visitor corridors remain useful
non-primary alternate templates if legal authority, local compact, and
operating readiness are stronger there. The default recommendation remains
Samawah / Al-Muthanna unless legal authority, local compact, or field readiness
make another municipality demonstrably safer.

## Pilot Governance Table

The pilot should not begin until these operating roles are named in writing.
One institution can hold more than one role only if conflict-of-interest rules
and independent audit coverage remain intact.

| Role | Required owner | Core responsibility | Evidence produced |
| --- | --- | --- | --- |
| Pilot sponsor | Named national or governorate sponsor with legal pilot authority | Authorizes the bounded pilot, budget ceiling, explicit exclusions, and stop conditions. | Signed pilot authority, budget note, exclusions register, rollback trigger list. |
| Municipality | Samawah / Al-Muthanna municipal operating team | Defines task areas, confirms local service need, supplies supervisors, and protects public access. | Local compact, task map, supervisor roster, service-output baseline. |
| Verifier | Independent verifier or municipal verifier under auditor sampling | Checks worker output, supplier delivery, geotagged evidence, safety, and duplicate-risk flags. | Verification decisions, rejected-claim log, sample-audit file. |
| Payment sandbox owner | CBI-approved bank, PSP, or controlled settlement operator | Holds pilot funds, releases only verified wage/vendor instructions, and reports exceptions. | Settlement ledger, payment exception report, reversal/rollback log. |
| Procurement reviewer | Procurement unit with beneficial-ownership and price-benchmark support | Reviews one supplier category, vendor eligibility, award basis, delivery evidence, and invoice timing. | Vendor file, price benchmark, award memo, delivery acceptance, payment-timing report. |
| Grievance body | Local grievance committee with escalation route | Receives worker, supplier, resident, privacy, safety, and exclusion complaints. | Grievance register, resolution clock, appeal outcomes, unresolved-risk list. |
| Public dashboard publisher | Dashboard operator approved by sponsor and municipality | Publishes aggregate task, payment, procurement, grievance, and audit indicators without personal data. | Public aggregate dashboard, privacy threshold checks, publication log. |
| Independent evaluator | University, audit firm, civil-society evaluator, or MDB-style reviewer | Tests whether evidence supports pilot claims and whether scale gates are met. | 90-day, 180-day, and 12-month evaluation memos with stop/go recommendation. |

## First Pilot Dataset

The first dataset should be narrow enough to inspect manually. It should cover
one civic-work package, one supplier category, one payment lane, and one public
dashboard view before any larger rail or municipal claim is made.

| Dataset | Minimum fields | Purpose |
| --- | --- | --- |
| Pilot authority register | authority reference, sponsor, municipality, budget ceiling, start/end dates, exclusions, stop triggers | Proves the pilot is legally bounded and not a national rollout. |
| Task registry | task ID, location, category, safety class, evidence rules, supervisor, status, planned worker slots | Connects public work to a visible place and defined output. |
| Worker enrollment file | worker ID, eligibility basis, consent marker, assignment, payout account/wallet, appeal channel | Prevents ghost-worker claims and supports wage/payment review. |
| Evidence bundle | photo/document URI, geotag bucket, timestamp, supervisor note, verifier decision, rejected reason if any | Proves work was submitted and reviewed before payment. |
| Wage instruction file | assignment ID, hours, rate, gross amount, withholding if any, payment status, exception reason | Connects verified work to controlled settlement. |
| Supplier file | vendor ID, beneficial-owner screen, local-content marker, price benchmark, delivery acceptance, invoice status | Keeps the procurement package auditable and locally legible. |
| Grievance register | complainant class, issue type, opened date, responsible body, resolution, appeal status | Tests whether workers, vendors, and residents can contest outcomes. |
| Public dashboard extract | aggregate tasks, paid wages, supplier payments, exceptions, grievances, audit hashes, privacy threshold status | Lets observers see outputs without exposing personal data. |

## One Payment Flow

**Flow:** municipal civic-work stipend or wage release.

1. Worker is enrolled with identity, eligibility, role, task category, wallet or
   payout account, and consent records.
2. Task is assigned by a municipal sponsor.
3. Evidence is submitted: photo, geotag, supervisor note, time window, material
   receipt, or public validation sample.
4. Supervisor verifies output and flags exceptions.
5. Payment is released through a controlled digital wallet or sandbox
   settlement account.
6. Dashboard records gross payment, tax/withholding if any, audit hash,
   grievance status, and payment exception rate.

Pilot limits:

- No national dividend.
- No oil-income lockbox.
- No CBDC issuance or sovereign monetary claim.
- No irreversible monetary claim.
- No payment without task evidence and supervisor signoff.
- No sensitive personal data in public dashboards.

Success metrics:

- Payment release time after verification.
- Share of payments with complete evidence.
- Exception rate by worker, task type, and supervisor.
- Worker grievance resolution time.
- Repeatable audit trace from assignment to payment.

## One Civic-Work Flow

**Flow:** visible municipal service tasks.

Task families:

- park maintenance and shade repair reporting;
- station-area cleaning, access mapping, and signage readiness;
- canal, ditch, and drainage cleanup;
- roadside cleaning and waste hot-spot reporting;
- drainage clearing and flood-prevention checks;
- lighting inspection and minor repair reporting;
- tree care, shade maintenance, and heat-risk mitigation;
- accessibility mapping for disabled and elderly residents;
- safety-hazard reporting for sidewalks, crossings, canals, and school routes.

Lifecycle:

1. `PendingApproval` - task proposed with location, purpose, safety class, payment rate,
   evidence requirements, and sponsor.
2. `Approved` - legal authority, budget, safeguards, and supervisor capacity are
   confirmed.
3. `OpenForEnrollment` - eligible workers can accept or be matched to tasks
   without coercive workfare.
4. `InProgress` - enrolled worker or team performs the assigned task.
5. `EvidenceSubmitted` - worker or team submits required proof.
6. `Verified` - supervisor and sampling checks accept output.
7. `PaymentReleased` - wage instruction is created only after verification.
8. `Suspended`, `Rejected`, or grievance record - disputed, unsafe, or failed
   work enters review and can be exported for audit.

Operational workflow:

1. Task creation records jurisdiction, category, title, worker slots, rate,
   evidence rules, and safety class.
2. Governance approval attaches the local authority and supervisor.
3. Enrollment creates a worker assignment under the approved task.
4. Evidence submission attaches photo/document URI, geotag bucket, supervisor
   marker, and timestamp.
5. Verification approves, rejects, or suspends the assignment.
6. Wage instruction is generated only if authority, supervisor, evidence,
   geotag, hours, duplicate-risk, safety, and verification checks pass.
7. Grievances can be opened and resolved against the assignment.
8. Audit export returns the task, assignment, payment instruction, grievance
   register, and ordered lifecycle events.

Controls:

- no payment if evidence is missing;
- hold if duplicate-worker, duplicate-location, or ghost-worker risk is high;
- block if task is unsafe, coercive, politically captured, or lacks legal
  authority;
- publish only aggregate outputs above privacy thresholds.

## One Procurement Flow

**Flow:** small local procurement package for municipal service inputs.

Example package: cleaning tools, reflective vests, water points, signage,
lighting consumables, drainage clearing materials, access-route materials,
mobile repair support, or verified local transport for civic-work teams.

The first package should use one supplier category only. Adding multiple
categories too early makes price benchmarking, beneficial-ownership screening,
delivery verification, and supplier-payment timing harder to interpret.

Lifecycle:

1. Municipality defines need and budget.
2. Vendor eligibility is checked: registration, beneficial ownership,
   sanctions/PEP screening, tax identifier, local content, delivery capacity.
3. Price benchmark is recorded.
4. Contract is awarded through competitive or justified small-value procedure.
5. Delivery evidence is recorded: invoice, receipt, supervisor acceptance,
   location, item count, quality note.
6. Payment is released only after milestone evidence.
7. Audit record links procurement to civic-work output and public benefit.

Success metrics:

- supplier payment time after verified delivery;
- price variance against benchmark;
- local supplier share;
- rejected invoice rate;
- delivery disputes;
- audit completeness.

## 90-Day Pilot

Purpose: prove the evidence loop without scale risk.

Scope:

- one municipality or service corridor;
- 250-500 workers;
- 20-40 local vendors;
- 5-8 task categories;
- one supervisor chain;
- one payment rail in sandbox or controlled fiat-backed settlement;
- one dashboard used by municipal operators, auditors, and observers.

Deliverables:

- legal pilot authority and local compact;
- worker eligibility and appeals policy;
- procurement rules and vendor onboarding checklist;
- task registry and evidence schema;
- dashboard for payments, tasks, procurement, audit, grievances, and public
  aggregate reporting;
- incident and rollback runbook.

Go/no-go gates:

- at least 90% of payments have complete evidence;
- audit trail reconstructs assignment-to-payment for sampled items;
- no unresolved severe privacy, safety, corruption, or coercion incidents;
- Redis/PostgreSQL session, role, and audit controls pass live tests;
- public aggregate dashboard hides personal data.

Executable screen:

- `MinimumViablePilotInput` captures the pilot boundary, explicit exclusions,
  legal/local authority, payment readiness, civic-work readiness, procurement
  readiness, dashboard readiness, OpenSourceRail reference confirmation,
  evidence quality, payment exceptions, supplier timing, grievances, capture
  risk, safety, privacy, and stop conditions.
- `MinimumViablePilotAssessment` computes scope, operations, evidence,
  integrity, readiness, stop conditions, required actions, and the stage
  decision.
- `PilotDecision` can return `not_ready`, `evidence_only`, `authorize_90_day`,
  `extend_to_180_day`, `extend_to_12_month`,
  `graduate_to_governorate_review`, `pause`, or `stop`.
- The engine hard-caps the pilot at `evidence_only` if the scope is no longer
  one municipality, one payment flow, one civic-work flow, one procurement
  flow, one supplier category, and one dashboard, or if CBDC issuance, oil
  lockbox, citizen dividends, ministry restructuring, or national macro claims
  enter the pilot scope.
- The engine can stop the pilot for missing legal authority, personal-data
  exposure, fabricated evidence, capture risk, coercion, severe safety
  incidents, or off-book arrears.
- The engine records a failed `opensource_rail_reference` gate when rail-enabling
  works are not tied back to the existing OpenSourceRail design/simulator/ops
  and safety-case artifacts.

Code surface:

- `crates/cs-analytics/src/minimum_viable_pilot.rs`
- `crates/cs-civic-work/src/lib.rs`
- `migrations/20260724000001_minimum_viable_pilot.sql`

## 180-Day Pilot

Purpose: test repeatability, procurement discipline, and service outcomes.

Expanded scope:

- 1,500-3,000 workers;
- 75-150 vendors;
- three municipal service areas;
- one tourism or visitor-service cluster;
- one verified local-material procurement lane;
- one bank or payment-service-provider integration in a supervised sandbox;
- monthly public report.

Deliverables:

- independent audit of first 90 days;
- procurement price benchmark table;
- worker grievance and appeal statistics;
- service-output indicators such as cleaned kilometers, drainage points
  cleared, lights inspected, accessibility issues mapped, visitor-support
  shifts delivered, and heat-risk mitigation assets maintained;
- fiscal report showing payments, procurement, admin cost, exceptions, and
  rejected claims.

Scale gates:

- administrative cost remains within pilot budget;
- supplier payment delays do not create hidden arrears;
- evidence quality improves rather than deteriorates with scale;
- grievance resolution is timely and documented;
- no political capture pattern dominates task assignment, vendor awards, or
  supervisor signoff.

## 12-Month Pilot

Purpose: decide whether the model can become a governorate-level operating
system component.

Expanded scope:

- 10,000-25,000 workers or rotating participants;
- 300-600 vendors;
- linked municipal, tourism, small infrastructure, and environmental tasks;
- procurement packages large enough to test competitive discipline;
- credit-history export for consenting workers and vendors;
- integration with a local public dashboard and independent audit body.

Deliverables:

- governorate pilot report;
- audited payment and procurement dataset;
- civic-work outcome report;
- local-content and vendor-development report;
- credit-readiness assessment for workers, vendors, and small contractors;
- legal and fiscal recommendations for controlled expansion or shutdown.

Expansion gates:

- audited public benefits are distinguishable from cash revenues;
- payment, procurement, and civic-work records are independently verifiable;
- no dividend or national claim is made from pilot evidence alone;
- macro, legal, privacy, security, and fiscal reviews approve the next stage;
- citizens can appeal identity, eligibility, payment, and task decisions.

## Stop Conditions

The pilot should pause or stop if any of the following occurs:

- legal authority is unclear or withdrawn;
- payment exceptions exceed agreed thresholds;
- evidence is routinely fabricated or unverifiable;
- vendor awards show capture or undisclosed beneficial ownership;
- worker participation becomes coercive;
- personal data appears in public reports;
- arrears or off-book guarantees emerge;
- supervisors approve work without inspection;
- local conflict, land, heritage, or safety risk cannot be controlled.

## What The Pilot Proves

A successful pilot proves only this:

Cylinder Seal can make a bounded public economic flow measurable, auditable,
and contestable across identity, payment, civic work, procurement, audit, and
dashboard layers.

It does not prove national CBDC readiness, national dividend affordability,
ministry restructuring, sovereign-credit improvement, or macroeconomic growth.
Those remain separate legal, fiscal, technical, and political decisions.
