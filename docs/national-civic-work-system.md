# National Civic Work System

As Iraq formalizes, automates, and industrializes through Digital IQD, INDHC,
and digitally governed industrial champions, some low-productivity work will
disappear. Cylinder Seal therefore includes a National Civic Work System: a
digitally verified, locally administered, dignity-preserving labor platform that
pays citizens for measurable social, environmental, cultural, sport, care,
education, municipal, food-security, and disaster-resilience work.

Status: policy-design scenario. It is not a welfare law, labor-market forecast,
budget appropriation, or implemented software module.

The objective is not to hide unemployment. The objective is to convert spare
labor capacity into public value.

## Source Discipline

| Public fact | Use in this design | Source |
| --- | --- | --- |
| World Bank WDI reports Iraq youth unemployment near 32% in recent modeled ILO estimates, including 31.8% in 2022 and 32.0% in 2025. | Civic work is designed as a transition and participation layer for youth and underemployed workers, not as a cosmetic add-on. | [World Bank WDI youth unemployment indicator](https://data.worldbank.org/indicator/SL.UEM.1524.ZS?locations=IQ) |
| IMF staff identify informality, lack of diversification, low financial inclusion, high reliance on cash, labor-market challenges, gender gaps, and structural obstacles to revenue mobilization in Iraq. | Civic work must create formal records, training history, payment evidence, and pathways into private employment rather than trapping people in low-value public work. | [IMF Iraq Selected Issues, 2024](https://www.imf.org/en/publications/cr/issues/2024/05/15/iraq-selected-issues-549033) |
| UNDP's Climate Vulnerability Index for Iraq integrates climate, socio-economic, and spatial data across all governorates and highlights impacts on water, agriculture, health, and infrastructure. | Civic work should prioritize climate adaptation, water resilience, heat response, food security, and local restoration. | [UNDP Climate Vulnerability Index of Iraq](https://www.undp.org/iraq/publications/climate-vulnerability-index-iraq) |
| UNDP describes climate-resilient agriculture in Iraq as reducing pressure on shared water resources and preventing disputes, and frames environmental action as a pathway to social cohesion. | Civic work should treat environmental restoration as public value, peacebuilding, and employability infrastructure. | [UNDP environmental action story, June 4, 2026](https://www.undp.org/stories/environmental-action-climate-peace-and-security) |

These sources support the need for a transition system. They do not validate the
payment levels, budget envelope, or institutional design below.

## Core Principle

The system must not feel like:

```text
You are unemployed, so go clean streets.
```

It should feel like:

```text
You are part of rebuilding Iraq, and your contribution is measured, paid,
respected, and visible.
```

This is a national participation economy, not old-style ministry payroll
expansion and not punitive workfare.

## Why Not Welfare Alone

Plain cash transfers can reduce poverty, but they do not by themselves create
purpose, skills, community repair, or a visible route from informal labor into
formal economic records.

The civic-work system is therefore designed to create several outputs at once:

- income for people who would otherwise be excluded from productivity gains;
- dignity through paid, visible, socially useful contribution;
- training records and certifications that improve employability;
- measurable community improvement in parks, schools, sport, care, heritage,
  water, food security, and environmental restoration;
- lower unrest risk by giving young people a respected participation ladder;
- formal income and reliability histories for workers with thin financial files.

The system should complement welfare, dividends, and ordinary employment. It
should not replace legal social protection, and it should not trap people in
permanent low-wage public tasks.

## Module Boundary

Proposed software module:

```text
cs-civic-work
```

Purpose:

- civic labor registry;
- task marketplace;
- verification engine;
- civic wage and credit payments;
- civic reputation and training records;
- public-impact dashboard;
- fraud and ghost-worker controls;
- privacy-bounded aggregate reporting.

The module connects to Digital IQD wallets but should not expose full payment
history to local supervisors. Task verification, wage payment, identity, and
reputation need separate permissions.

## Civic Work Flow

1. Municipality, school, sports club, NGO, environmental agency, health clinic,
   heritage authority, or approved community institution posts a task.
2. The task is checked against approved categories, wage rules, safety rules,
   budget availability, and verifier requirements.
3. Citizen accepts the task through a Civic Work Wallet linked to Digital IQD.
4. Citizen completes the work.
5. Evidence is submitted: supervisor approval, GPS check-in where lawful,
   timestamped photo, sensor evidence, peer validation, institutional sign-off,
   or output count.
6. Cylinder Seal calculates wage, credit bonus, training record, and reputation
   update.
7. Payment is released in Digital IQD.
8. Public dashboards show aggregate public value by district and category.

## Work Categories

| Sector | Examples of meaningful work | Public value signal |
| --- | --- | --- |
| Environment | Tree planting, riverbank cleanup, marsh restoration, anti-desertification, recycling, canal maintenance. | Survival rate, hectares restored, waste removed, canals cleared, heat-island reduction. |
| Social care | Elderly visits, disability support, childcare support, school meals, community health outreach. | Verified visits, care hours, referrals, meals delivered, missed-visit rate. |
| Sport | Local football coaching, girls' sport programs, youth leagues, public fitness events. | Teams supported, attendance, coach certification, female participation, safe-play compliance. |
| Culture | Heritage restoration, tourism guides, museum support, traditional crafts, local festivals. | Sites maintained, tours delivered, visitor ratings, craft income, preservation tasks. |
| Education | Literacy tutoring, after-school STEM clubs, vocational mentoring, homework support. | Tutoring hours, learner attendance, assessment gains, certification completions. |
| Municipal work | Street cleaning, park maintenance, public-space repair, lighting reports, pothole reports. | Streets cleaned, assets repaired, response time, citizen complaints resolved. |
| Food security | Urban farming, greenhouse support, date-palm care, irrigation monitoring, storage support. | Crop survival, irrigation checks, spoilage reduction, farmer support visits. |
| Disaster resilience | Flood response training, heatwave support teams, emergency supply distribution, first-aid teams. | Training completions, response drills, vulnerable-household checks, supplies delivered. |

## Payment Design

There are three payment types.

| Payment | Purpose | Guardrail |
| --- | --- | --- |
| Civic wage | Direct Digital IQD payment for verified work. | Paid only after task evidence passes; capped by hours, category, and local budget. |
| Civic credit bonus | Extra credit for transport, training, sports memberships, childcare, local goods, or housing deposits. | Spend categories are transparent and appealable; credits must not become hidden patronage. |
| Progression wage | Higher pay after verified certification. | Requires training certificate, task quality record, and periodic reassessment. |

Suggested progression ladder:

| Level | Requirement | Example wage logic |
| --- | --- | --- |
| Entry | Identity verified, task safety briefing completed. | Base civic wage. |
| Reliable | 40 verified hours, low dispute rate, supervisor or peer validation. | Base wage plus small reliability bonus. |
| Certified | First aid, coaching, irrigation, restoration, care, tutoring, construction safety, or tourism-guide certificate. | Progression wage for certified task categories. |
| Team lead | 200 verified hours, certification, no fraud flags, ability to supervise small crews. | Higher wage, but capped crew size and random audits. |
| Bridge-to-work | Employer, INDHC subsidiary, SME, school, municipality, or NGO apprenticeship offer. | Temporary wage support while transitioning to normal job or enterprise. |

The system should be a bridge into better work, not a permanent low-wage holding
pen.

## Civic Service Year

Iraq could create a voluntary Civic Service Year for people aged 18-30, with
paths for older participants, women returning to work, people with disabilities,
and displaced people where appropriate.

| Track | Work |
| --- | --- |
| Green Iraq Corps | Trees, marshes, canals, recycling, heat resilience, riverbank work. |
| Sports Iraq Corps | Coaching, school leagues, women's sport, community clubs, public fitness. |
| Care Iraq Corps | Elderly care, disability support, child services, health outreach. |
| Skills Iraq Corps | Tutoring, coding clubs, vocational workshops, apprenticeship support. |
| Heritage Iraq Corps | Archaeology support, tourism, culture, old-city restoration, festivals. |
| Municipal Iraq Corps | Parks, streets, public assets, neighborhood reporting, maintenance. |
| Food And Water Corps | Irrigation, farming support, greenhouses, water-saving campaigns. |
| Disaster Resilience Corps | Heatwave teams, flood drills, first aid, emergency supply distribution. |

After 12 months, participants receive:

- Digital IQD income history;
- verified work record;
- training certificates;
- preferential access to apprenticeships;
- mortgage or rent-support points where lawful;
- SME credit score boost;
- public recognition badge;
- optional transition interview with banks, SMEs, INDHC subsidiaries, schools,
  municipalities, or NGOs.

## Funding Model

Civic work should be funded by explicit appropriations and project budgets, not
by quietly raiding the citizen dividend pool.

Potential funding sources:

| Source | Use |
| --- | --- |
| Treasury social-transition allocation | Base civic wage, platform operations, verifier training. |
| Gross-profit levy share | Productivity gains from INDHC and champions can fund social-transition work. |
| Municipal service budgets | Parks, streets, waste, lighting, local repairs, community sports. |
| Climate adaptation and MDB grants | Water, heat, agriculture, restoration, disaster resilience. |
| INDHC project community budgets | Local maintenance, training, supplier outreach, environmental mitigation. |
| NGO or school co-funding | Care, tutoring, sports, heritage, local civic programs. |

Suggested accounting identity:

```text
Civic Work Budget
  = Treasury Social Transition Allocation
  + Eligible Municipal Service Budgets
  + Climate / MDB / Donor Co-Funding
  + Approved INDHC Community Benefit Budgets
  - Platform Operations
  - Verification And Audit Reserve
```

Policy rule:

```text
Citizen dividend funds are not civic-work payroll funds.
```

Dividends distribute capital returns. Civic wages pay verified public work.

## Verification Design

Verification should scale with task risk.

| Task risk | Example | Evidence |
| --- | --- | --- |
| Low | Park cleanup, public fitness event, festival support. | Supervisor sign-off, random photo sample, peer validation. |
| Medium | Tutoring, canal cleaning, tree maintenance, sports coaching. | Attendance log, location check, supervisor sign-off, output count, random audit. |
| High | Elderly care, childcare, disability support, disaster response, water infrastructure. | Certified worker, institutional sign-off, safety checklist, two-person verification, audit sampling. |
| Sensitive | Heritage sites, domestic violence support, child protection, protected wetlands, security-adjacent areas. | Restricted verifier list, privacy controls, no public location disclosure, specialized oversight. |

Photo, GPS, and biometric evidence must be lawful, proportionate, and
privacy-bounded. The system should not normalize surveillance for low-risk work.

## Governance Safeguards

The civic-work system can become corrupt unless designed carefully.

Required controls:

- no ghost workers;
- wallet-based attendance and lawful identity checks;
- random audits;
- public dashboards by district and category;
- NGO, school, university, and civil-society verification roles;
- photo, GPS, sensor, or supervisor evidence only where appropriate;
- grievance and appeal system;
- anti-nepotism controls;
- caps on local political appointments;
- verifier rotation;
- conflict-of-interest disclosure;
- worker safety rules and incident reporting;
- disability and gender-access review;
- independent audit by civil society, universities, and the supreme audit
  authority or equivalent.

## Dashboard Metrics

| Metric | Example |
| --- | --- |
| Active workers | Civic workers active this month, by governorate, age band, gender, and track. |
| Work completed | Verified civic hours, tasks completed, dispute rate, rejection rate. |
| Environmental output | Trees planted and maintained, canals cleared, waste removed, hectares restored. |
| Sport output | Youth teams supported, girls' sport sessions, coach certificates, attendance. |
| Care output | Elderly visits, disability support hours, childcare sessions, referrals. |
| Municipal output | Parks, streets, lighting reports, repairs, citizen complaints resolved. |
| Food and water output | Irrigation checks, greenhouse support, farmer visits, storage support. |
| Training output | Certificates issued, apprenticeships started, bridge-to-work placements. |
| Integrity output | Audit flags, ghost-worker attempts, verifier sanctions, appeal outcomes. |
| Fiscal output | Civic wage paid, cost per verified output, budget remaining, co-funding. |

## Data Model

Suggested primitives:

| Model | Purpose |
| --- | --- |
| `CivicWorkWallet` | Links a participant to Digital IQD payments, civic reputation, training, and work limits. |
| `CivicTask` | Approved unit of work with category, location rules, verifier, wage, budget, safety level, and expiry. |
| `CivicTaskPosting` | Institution request to create tasks, with budget source and approval state. |
| `CivicWorkClaim` | Worker claim that a task was performed. |
| `CivicEvidenceBundle` | Supervisor sign-off, photo, sensor, GPS, attendance, peer, or institutional evidence. |
| `CivicVerificationDecision` | Approved, rejected, held, disputed, or audit-required. |
| `CivicWagePayment` | Digital IQD payment for verified work. |
| `CivicCreditBonus` | Spend-limited bonus for transport, training, childcare, housing deposit, sport, or local goods. |
| `CivicReputationScore` | Reliability, certification, dispute, safety, and task-completion history. |
| `CivicCertificate` | Training or skill credential. |
| `CivicImpactMetric` | Public output measure linked to verified work. |
| `CivicAppeal` | Worker, verifier, or institution appeal. |
| `VerifierRegistry` | Approved supervisors, NGOs, schools, clubs, universities, agencies, and auditors. |

Suggested events:

| Event | Meaning |
| --- | --- |
| `TaskPosted` | Institution proposes work. |
| `TaskApproved` | Budget, safety, category, and verifier checks pass. |
| `TaskAccepted` | Worker accepts a task. |
| `EvidenceSubmitted` | Worker or verifier submits proof. |
| `TaskVerified` | Evidence passes. |
| `TaskRejected` | Evidence fails or task is invalid. |
| `PaymentReleased` | Civic wage or credit is paid. |
| `CertificateIssued` | Training credential is recorded. |
| `ReputationUpdated` | Civic score changes. |
| `AuditFlagRaised` | Fraud, ghost-worker, nepotism, or verifier abuse is suspected. |
| `AppealResolved` | Dispute is closed with reason code. |

## Privacy And Safety Boundaries

- Public dashboards show aggregates, not individual identities.
- Care, child, disability, domestic-violence, and sensitive heritage tasks need
  strict privacy controls.
- GPS should be coarse or time-limited unless high-risk work requires precision.
- Biometric use requires explicit legal authority, minimization, and appeal.
- Workers must be able to dispute false attendance or supervisor abuse.
- Safety training is mandatory before environmental, municipal, disaster, care,
  or child-facing work.
- No participant should lose ordinary welfare, dividend, or legal rights for
  refusing civic work.

## Ten-Year Rollout

| Phase | Years | Goal | Main work |
| --- | --- | --- | --- |
| Foundation | 0-1 | Define law, task categories, wage rules, privacy rules, and verifier registry. | Pilot in 3 governorates with municipal cleanup, sport, tutoring, and tree-care tasks. |
| Pilot | 1-2 | Prove verification, payments, anti-ghost-worker controls, and grievance process. | Add care, irrigation monitoring, heritage support, and disaster drills. |
| Scale | 3-5 | Launch Civic Service Year and connect training to apprenticeships. | Expand to all participating governorates; integrate banks, SMEs, INDHC Academy, and municipalities. |
| Productivity transition | 5-8 | Absorb workers displaced from low-productivity admin, informal middlemen, and inefficient logistics. | Add bridge-to-work wage support, certification ladders, and sector-specific civic corps. |
| Mature | 8-10 | Turn civic work into a permanent participation layer, not emergency relief. | Renew only programs with audited impact; retire low-value tasks and fund higher-skill tracks. |

## First 180 Days

| Month | Work |
| --- | --- |
| 1 | Draft civic-work charter, define dignity-of-work rules, ban punitive workfare, and list eligible institutions. |
| 2 | Define task taxonomy, wage bands, evidence tiers, privacy rules, and appeal process. |
| 3 | Build `CivicTask`, `CivicWorkWallet`, `CivicEvidenceBundle`, and `VerifierRegistry` schema proposals. |
| 4 | Select pilot districts and institutions: municipality, school, sports club, NGO, environmental agency, and health clinic. |
| 5 | Launch limited pilots for park maintenance, tutoring, sport coaching, tree care, canal cleanup, and elderly visits. |
| 6 | Publish first public dashboard with aggregate hours, outputs, payments, audits, and appeals. |

## Integration With Other Pillars

| Pillar | Integration |
| --- | --- |
| Digital IQD | Pays civic wages and credits, records income history, supports category-limited bonuses. |
| INDHC | Funds community-benefit work and creates bridge-to-work apprenticeships. |
| Industrial champions | Offer progression routes from civic training into supplier jobs, maintenance, tourism, food, water, and green sectors. |
| Ministry transition | Absorbs some staff and citizens into measurable public-value work rather than hidden payroll. |
| Credit scoring | Verified civic income, certificates, and reliability improve thin-file worker profiles. |
| Tourism | Heritage, guide, festival, and public-space work improve visitor experience and local income. |
| Green and rail | Civic work supports station-area maintenance, urban cooling, tree care, recycling, and public-space safety. |
| Dividend system | Dividends provide capital income; civic work provides paid participation and skill development. |

## Risks

| Risk | Mitigation |
| --- | --- |
| Becomes fake jobs. | Output metrics, task expiry, random audit, no payment without evidence. |
| Becomes punitive workfare. | Voluntary participation, no loss of legal rights for refusal, dignity charter, grievance path. |
| Becomes patronage. | Verifier rotation, anti-nepotism rules, public dashboards, local appointment caps. |
| Ghost workers appear. | Wallet-based attendance, evidence bundles, random audits, duplicate detection. |
| Supervisors abuse workers. | Appeals, worker ratings of verifiers, civil-society audit, sanctions. |
| Low-value tasks crowd out real jobs. | Wage bands below skilled market roles, bridge-to-work design, SME/private placement targets. |
| Privacy harms vulnerable groups. | Aggregate public reporting, sensitive-task controls, minimal GPS/photo use. |
| Budget becomes open-ended. | Explicit appropriation, task caps, cost-per-output review, sunset rules. |

## Build Sequence

1. Keep the civic-work architecture in policy-design status until legal,
   privacy, labor, and fiscal review are complete.
2. Add `CivicTask`, `VerifierRegistry`, `CivicEvidenceBundle`,
   `CivicWorkClaim`, and `CivicWagePayment` design models.
3. Add route-level prototype tests for task posting, evidence submission,
   verification, payment release, and appeal.
4. Add dashboard projections: active workers, verified hours, public outputs,
   audit flags, payments, and bridge-to-work outcomes.
5. Integrate with credit scoring only after privacy review.
6. Add legal review for labor law, child protection, care work, biometric use,
   data protection, municipal authority, and anti-corruption enforcement.

## Bottom Line

Productivity gains are socially legitimate only if citizens see a pathway from
lost low-value work into income, dignity, skill, and visible contribution.

The National Civic Work System makes that pathway measurable:

```text
productivity gains
  -> fiscal space and higher surplus
  -> verified civic work, training, care, restoration, sport, culture, and municipal repair
  -> income history, skills, public value, and social cohesion
  -> better private-sector and INDHC employability
```
