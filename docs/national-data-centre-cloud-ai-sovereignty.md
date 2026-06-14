# National Data Centre, Cloud, Social Media, And AI Sovereignty System

## Purpose

Iraq should treat domestic compute as national infrastructure, alongside power,
water, rail, ports, banking, and industrial capacity. Cylinder Seal depends on
trusted digital infrastructure for payments, audit logs, procurement evidence,
public dashboards, citizen services, industrial planning, and AI-assisted
administration. Those functions should not depend permanently on foreign
proprietary cloud, social media, advertising, identity, or AI API systems.

The goal is not digital isolation. The goal is operational independence:

- Iraqi public institutions, banks, universities, producers, media, and citizens
  can run essential workloads inside Iraq.
- The core software stack is exclusively open source, locally mirrored,
  reproducibly deployed, and auditable.
- Public-interest social media, messaging, education, cultural archives, and AI
  services can continue operating even if foreign platforms change prices,
  terms, sanctions exposure, moderation rules, API access, or availability.
- Private Iraqi cloud companies, telecom operators, banks, universities, and
  startups can build on top of a domestic infrastructure base instead of
  renting the national digital future from foreign platforms.

This chapter is a scenario plan, not a procurement decision. It uses public
digital, macro, power, and data-centre references as anchors, then applies
explicit scenario assumptions for Iraq-specific costs and revenues.

## Current Demand Signal

Iraq already has the demand profile for a domestic digital infrastructure
strategy. DataReportal estimated 38.0 million internet users in Iraq at the
start of 2025, equal to 81.7% penetration, and 34.3 million social media user
identities, equal to 73.8% of the population. The World Bank reported Iraq's
2024 GDP at about $279.64 billion. This means the user base exists before the
infrastructure strategy is built; the missing piece is domestic ownership,
service quality, developer capacity, and institutional trust.

Power is the binding constraint. IEA country data highlights Iraq's large oil
and gas resources and strong solar PV potential, while also showing that power
system planning remains central to Iraq's development path. A data-centre plan
must therefore be paired with dedicated power, waste-heat, water, and grid
resilience design from the first phase.

Industry cost anchors are volatile. Uptime Institute has reported that data
centre costs rose after the pre-COVID period, when some projects were reported
around $6-8 million per MW in favorable cases, while 2025 industry commentary
often places Tier III construction around $7-9 million per MW before local
contingency, power, land, security, and IT hardware. For Iraq, this plan uses
conservative scenario bands rather than a single point estimate.

## Strategic Boundary

The system should be independent of US proprietary systems, not disconnected
from the global open-source commons.

Required boundary:

- no dependency on AWS, Azure, Google Cloud, Oracle Cloud, Meta, X, TikTok,
  OpenAI, Anthropic, Palantir, Snowflake, Datadog, Splunk, Microsoft 365, Google
  Workspace, or similar proprietary platforms for essential sovereign services;
- no closed-source identity provider, audit-log core, payment evidence store,
  public dashboard stack, state messaging stack, or AI inference dependency;
- all critical software can be built from source, mirrored inside Iraq, scanned,
  deployed offline, and maintained by Iraqi teams;
- all citizen-facing services include export, portability, appeal, and privacy
  controls;
- all state access to private data requires law, audit, and independent
  oversight.

Practical limitation:

- servers, storage devices, network switches, accelerators, firmware, optical
  gear, and power equipment will still contain imported components for years;
- GPU and accelerator supply chains are not fully open and not fully domestic;
- therefore the first sovereignty target is software, data, operations, and
  procurement optionality, while a later industrial programme develops assembly,
  repair, firmware review, open hardware pilots, and regional component
  partnerships.

## Open-Source Reference Stack

The reference stack should be boring, inspectable, and replaceable:

| Layer | Open-source reference stack | Use |
| --- | --- | --- |
| Operating base | Debian GNU/Linux or comparable fully open Linux distribution, hardened kernel profiles, reproducible package mirrors | Host base, package control, offline rebuilds. |
| Infrastructure cloud | OpenStack for IaaS, bare-metal provisioning, virtual networks, block/object integration | Government, bank, telecom, university, and enterprise cloud. |
| Container platform | Kubernetes, Cilium, Argo CD or Flux, Helm, Open Policy Agent | Cloud-native services and repeatable deployment. |
| Storage | Ceph object/block/file storage, PostgreSQL, Apache Kafka or Redpanda-compatible open stack where license permits, Valkey-compatible cache | Evidence stores, object storage, logs, queues, analytics. |
| Identity and access | Keycloak, OpenLDAP-compatible directories, hardware-backed MFA, local PKI | Role control, citizen/worker/operator auth boundaries. |
| Observability | Prometheus, Grafana, Loki-compatible logs, OpenTelemetry, OpenSearch | Audit, monitoring, search, incident response. |
| Social and messaging | ActivityPub/Mastodon-compatible microblogging, Matrix messaging, PeerTube-style video, Mobilizon-style events | Domestic public-interest social services without platform lock-in. |
| Collaboration | Nextcloud-compatible files, Collabora/LibreOffice Online, Jitsi-compatible video, Zulip or Mattermost-compatible team chat | Government, schools, universities, SMEs, civic organizations. |
| AI infrastructure | PyTorch, Hugging Face Transformers, vLLM, llama.cpp, Ollama-compatible local serving, open-weight Arabic/Kurdish-capable models where licenses allow | Inference, retrieval, translation, summarization, coding, document review, education. |
| Data governance | Apache Iceberg-compatible lakehouse, CKAN-compatible open data catalogues, privacy-preserving analytics, signed audit exports | Public data, procurement evidence, economic dashboards, research. |

Open source does not automatically mean sovereign. The stack must be paired
with Iraqi maintainers, local mirrors, security review, incident response,
procurement discipline, and training.

## Physical Architecture

The national design should be distributed from the start.

| Component | Recommended role |
| --- | --- |
| Central sovereign cloud region | Main government, banking, education, audit, and AI services. Candidate locations should be selected by power quality, fiber, security, flood/heat risk, land title, and operator availability, not politics. |
| Southern energy/data region | Basra or nearby energy corridor for power-linked compute, port/logistics, oil and gas services, industrial data, and disaster recovery from the central region. |
| Northern continuity region | Mosul or Erbil-area candidate subject to federal/KRG compact, power/fiber resilience, and legal authority. Provides continuity and national geographic redundancy. |
| Samawah / Al-Muthanna edge node | Pilot edge node aligned with OpenSourceRail enabling works, municipal evidence, local dashboards, and training. It should not host national secrets. |
| University and hospital edge nodes | Smaller edge nodes for education, health records, research computing, language datasets, and telemedicine. |
| Telecom edge cache | Local caching, content delivery, emergency communications, and latency reduction in partnership with Iraqi telecom operators. |

Each region should have:

- dual fiber routes and exchange-point connectivity;
- dedicated power with grid connection, gas or solar-plus-storage backup, and
  strict generator emissions controls;
- water-aware cooling, including dry or hybrid cooling where feasible;
- physical security and transparent access logs;
- independent audit of uptime, outages, power use, water use, and incident
  response.

## Energy And Water Model

Data centres are electricity infrastructure. A 100 MW IT load at PUE 1.45 uses:

`100 MW x 1.45 x 8,760 hours = 1,270,200 MWh/year`, or about `1.27 TWh/year`.

Scenario energy demand:

| IT load | PUE assumption | Annual facility electricity | Planning implication |
| ---: | ---: | ---: | --- |
| 10 MW | 1.45-1.60 | 127-140 GWh/year | Pilot can be supplied with dedicated grid contract plus backup power. |
| 50 MW | 1.45-1.55 | 635-679 GWh/year | Requires power-development coordination, not only a building permit. |
| 150 MW | 1.40-1.50 | 1.84-1.97 TWh/year | Needs dedicated generation, transmission, and heat/water strategy. |
| 300 MW | 1.35-1.45 | 3.55-3.81 TWh/year | Becomes a national power-sector project. |

Rules:

- no large AI training campus without dedicated generation and grid impact
  review;
- use captured gas, solar, storage, and high-efficiency cooling where feasible;
- require water budget and cooling design before site approval;
- publish PUE, water-use effectiveness, outages, and carbon intensity by site;
- prioritize inference, retrieval, and small-model fine-tuning before
  large-scale frontier-model training.

## Ten-Year Build Plan

### Phase 0: 0-6 Months - Authority, Design, And Procurement Discipline

Deliverables:

- national open-infrastructure charter;
- data classification and public-sector cloud migration policy;
- open-source software bill of materials for every critical layer;
- Iraqi mirror of source packages, container images, model weights, and
  documentation;
- site-screening process for power, fiber, heat, water, flood, security, and
  land-title risk;
- procurement rule: no sovereign dependency on a proprietary foreign cloud,
  identity, audit, social, or AI API service;
- training pipeline with universities and diaspora engineers.

Estimated cost: `$15-30 million`.

### Phase 1: 6-24 Months - Pilot Sovereign Cloud And Samawah Edge

Target:

- 8-12 MW IT load across two small regions and one Samawah edge node;
- first government workloads: Cylinder Seal evidence, pilot dashboards,
  procurement records, public aggregate dashboards, civic-work audit exports,
  document management, and training systems;
- first social layer: public-interest ActivityPub and Matrix services for
  municipalities, universities, cultural institutions, and civic-work pilot
  communications;
- first AI layer: Arabic/Kurdish document search, translation assistance,
  procurement review support, legal-document summarization, coding support for
  Iraqi developers, and school/university tutoring pilots.

Estimated capex:

| Cost item | Scenario range |
| --- | ---: |
| Data-centre shell, MEP, security, cooling, fit-out | $80-140 million |
| General compute, storage, network, backup | $35-70 million |
| AI inference/fine-tuning hardware | $25-80 million |
| Power resilience, solar/storage/generator integration | $25-55 million |
| Software integration, cyber, training, source mirrors | $15-30 million |
| Contingency | $20-45 million |
| Total | **$200-420 million** |

Year-2 revenue target: `$30-80 million/year`.

### Phase 2: Years 2-5 - National Cloud Utility

Target:

- 40-60 MW IT load across three Iraqi regions;
- migrate non-secret government workloads, education platforms, public
  procurement, municipal dashboards, selected health and university systems,
  bank disaster-recovery workloads, telecom edge services, and SME cloud;
- create Iraqi cloud marketplace for hosting, managed databases, cybersecurity,
  backup, analytics, AI inference, and software-as-a-service providers;
- require every ministry and state company to publish a migration/retirement
  plan for foreign SaaS where domestic open-source alternatives meet service
  requirements.

Estimated cumulative capex by Year 5: `$0.9-1.6 billion`.

Annual revenue target by Year 5:

| Revenue lane | Year-5 range |
| --- | ---: |
| Government cloud and collaboration subscriptions | $140-260 million |
| Banking, telecom, insurance, and regulated hosting | $120-260 million |
| SME cloud, backup, cybersecurity, and managed databases | $50-120 million |
| AI inference, translation, search, and document services | $50-180 million |
| Domestic social/media subscriptions, ads, creator services, and public-service contracts | $20-80 million |
| Disaster recovery, CDN/cache, and university/HPC services | $40-90 million |
| Total | **$420-990 million/year** |

### Phase 3: Years 5-10 - Sovereign AI And Regional Digital Services

Target:

- 150-300 MW IT load if power and demand gates pass;
- national Arabic/Kurdish AI services for education, legal aid, health triage
  support, agriculture extension, procurement review, anti-corruption red flags,
  industrial maintenance, tourism translation, heritage archives, and local
  software engineering;
- regional export of Iraqi cloud services to nearby markets where latency,
  price, Arabic language, legal neutrality, or Iraqi diaspora channels create a
  niche;
- content creator, social commerce, e-learning, cultural media, and tourism
  platforms with domestic payments and tax evidence.

Estimated cumulative capex by Year 10: `$4.0-8.5 billion`.

Annual revenue target by Year 10:

| Revenue lane | Year-10 range |
| --- | ---: |
| Government cloud and digital public infrastructure | $350-650 million |
| Regulated private cloud and disaster recovery | $450-950 million |
| AI inference, fine-tuning, domain models, and data services | $300-900 million |
| Domestic social/media, creator economy, education, and collaboration | $100-350 million |
| Regional hosting, CDN, cybersecurity, and managed services | $200-650 million |
| Research, university, health, tourism, and industrial data services | $100-250 million |
| Total | **$1.5-3.75 billion/year** |

### Phase 4: Years 10-15 - Optional Export-Scale Digital Industry

This phase should happen only if Phase 3 has proven power discipline, revenue,
operator competence, and social trust.

Target:

- 300-500 MW IT load;
- regional AI/cloud export business;
- Iraqi hardware assembly, repair, optical module, rack integration, battery,
  cooling, and data-centre construction supply chains;
- open hardware pilots where feasible, including RISC-V education and embedded
  systems rather than unrealistic near-term frontier GPU independence.

Estimated cumulative capex by Year 15: `$8-14 billion`.

Annual revenue target by Year 15: `$3-6 billion/year`, with the higher end
requiring export competitiveness, not only domestic public contracts.

## Financing Model

Funding should blend oil-equity allocation, loans, domestic capital markets,
and anchor-customer contracts.

| Source | Use | Controls |
| --- | --- | --- |
| INDHC retained earnings / oil-equity allocation | Phase 0-1 equity and public-interest services | Capped allocation; no dividends from capex; audited capital account. |
| MDB/development finance digital loans | Education, health, public procurement, open data, cyber resilience, energy efficiency | Safeguards, procurement transparency, data-rights review. |
| Green / transition loans | Solar, storage, captured-gas efficiency, cooling upgrades, waste-heat reuse | Energy and water KPIs. |
| Domestic bond or sukuk | Data-centre facilities with contracted government/bank/telecom demand | DSCR gate; no hidden guarantee; published tariff model. |
| Bank and telecom precommitments | Regulated hosting, disaster recovery, edge cache, network exchange | Multi-year offtake, service-level agreements, exit rights. |
| Diaspora technology bond | Training, AI datasets, creator platforms, university compute | Transparent reporting; no speculative dividend claim. |
| Private Iraqi cloud providers | SaaS, managed services, cybersecurity, content, education apps | Open marketplace and competition rules. |

The state should not create a monopoly cloud bureaucracy. It should create a
wholesale sovereign infrastructure utility with open APIs, regulated pricing,
private-sector resale, and anti-capture controls.

## Operating Cashflow Logic

The business should be judged like infrastructure plus software, not like a
subsidized IT department.

Illustrative annual operating profile:

| Phase | Annual revenue | Annual opex excluding depreciation | EBITDA range | Comment |
| --- | ---: | ---: | ---: | --- |
| Year 2 pilot | $30-80m | $45-95m | -$65m to $20m | Expected to be near break-even or loss-making while workloads migrate. |
| Year 5 utility | $420-990m | $260-650m | $100-340m | Requires committed public, bank, telecom, and SME workloads. |
| Year 10 AI/cloud | $1.5-3.75b | $950m-2.4b | $400m-1.35b | Requires AI monetization, regional hosting, and strong power discipline. |
| Year 15 export scale | $3-6b | $1.9-3.8b | $800m-2.2b | Only if Iraq becomes a credible regional provider. |

Operating assumptions:

- electricity at `$0.05-0.09/kWh` equivalent, depending on dedicated generation,
  grid cost, and fuel accounting;
- non-energy opex at 8-14% of cumulative active capex for staff, maintenance,
  security, bandwidth, replacement, audits, and software support;
- accelerator refresh every 3-5 years;
- storage refresh every 4-6 years;
- no cash dividend from this sector until maintenance, power, refresh capex,
  debt service, cyber reserve, and service obligations are funded.

## Economic Benefits

### Direct Benefits

- Domestic cloud revenue substitutes for foreign cloud, SaaS, backup,
  cybersecurity, data analytics, AI API, content hosting, and collaboration
  services.
- Iraqi engineers, operators, cybersecurity analysts, data-centre technicians,
  AI specialists, language-data teams, designers, creator-platform operators,
  and support firms become a new services workforce.
- Banks, telecoms, universities, hospitals, producers, media firms, and
  municipalities gain lower-latency domestic infrastructure.
- Cylinder Seal audit logs, civic-work evidence, procurement records, and
  dashboards gain a domestic operating base.

### Second-Order Benefits

- faster government services and lower paper-processing cost;
- better procurement transparency and fraud detection;
- local AI for Arabic, Iraqi Arabic, Kurdish, Turkmen, Syriac, and heritage
  material;
- domestic creator economy and tourism marketing channels;
- reduced vulnerability to foreign platform censorship, shutdowns, price
  shocks, API closure, or legal pressure;
- university research capacity and software startup formation;
- stronger cyber incident response and data-residency discipline;
- local digital tax base from cloud, ads, subscriptions, creator income, and
  platform commerce.

### Growth Contribution

Scenario contribution to non-oil GDP growth:

| Horizon | Conservative case | Base case | Strategic case |
| --- | ---: | ---: | ---: |
| Year 2 | 0.05-0.10 percentage points | 0.10-0.20 pp | 0.20-0.30 pp |
| Year 5 | 0.15-0.30 pp | 0.30-0.60 pp | 0.60-0.90 pp |
| Year 10 | 0.30-0.60 pp | 0.60-1.20 pp | 1.20-1.80 pp |

The strategic case is not a forecast. It requires public-sector migration,
private-sector trust, reliable power, Iraqi staffing, open-source competence,
regional exports, and strong privacy/civil-rights governance.

## Social Media And Public Digital Sphere

Iraq should not try to clone foreign social media as a state propaganda system.
That would destroy trust. The better model is a public-interest protocol layer:

- ActivityPub-compatible public accounts for ministries, municipalities,
  universities, cultural institutions, public broadcasters, sports clubs,
  tourism boards, and verified local businesses;
- Matrix-compatible secure messaging for public-sector teams, universities,
  emergency coordination, and civic-work pilot operators;
- PeerTube-style video for lectures, cultural archives, tourism promotion,
  ministry hearings, procurement explainers, and local media;
- creator payments through domestic regulated payment rails;
- portability and export tools so citizens are not trapped;
- clear moderation law, appeal rights, transparency reports, and court-order
  rules.

Social benefits:

- Iraqi cultural production is discoverable and monetizable;
- tourism campaigns can be distributed through domestic and diaspora channels;
- local businesses can advertise without relying entirely on foreign platforms;
- schools and universities get domestic education media;
- public institutions have resilient channels during crises.

Red line:

- the domestic social layer must not become mandatory for private speech, a
  surveillance dragnet, or a tool for shutting off the open internet.

## AI Infrastructure

The first AI target should be useful public and commercial tools, not a vanity
frontier model.

Priority use cases:

- Arabic/Kurdish translation and summarization for public services;
- procurement-document risk flags and price-benchmark support;
- legal aid search and administrative appeal assistance;
- school tutoring and teacher support;
- health triage support with strict clinical limits;
- agriculture extension and irrigation advice;
- industrial maintenance manuals and technician training;
- tourism translation, itinerary support, and cultural-site explanation;
- coding assistant for Iraqi open-source developers;
- archive restoration, OCR, speech-to-text, and cultural heritage indexing.

AI governance requirements:

- no secret training on citizen private data;
- no biometric mass-surveillance model as a default public-service feature;
- model cards, data lineage, bias testing, and red-team reports;
- Arabic/Kurdish language evaluation benchmarks;
- human appeal and review for rights-impacting decisions;
- export controls and cyber controls for sensitive models.

## Integration With Cylinder Seal

This system becomes the compute and data layer for:

- civic-work task evidence and audit exports;
- procurement integrity and beneficial-ownership screening;
- project pipeline, capex, DSCR, and benefit-realization dashboards;
- citizen rights, grievance, and appeals logs;
- local-content evidence and producer registries;
- social/tourism/diaspora channels for Iraqi services and products;
- AI-assisted analysis of spending, fraud, maintenance, and public benefits.

The data-centre system should be treated as a productive sector inside the
INDHC/economic model. It earns revenue, pays taxes, employs Iraqis, trains
engineers, supports other sectors, and distributes cash only from audited
surplus after power, maintenance, refresh capex, debt service, and cyber reserve
requirements are met.

## Stop Conditions

The programme should pause or redesign if any of these occur:

- power demand is approved without dedicated supply and grid-impact review;
- personal data is centralized without privacy law, minimization, access logs,
  and appeal rights;
- proprietary cloud dependency is reintroduced into a critical sovereign layer;
- one state-controlled platform becomes mandatory for private speech;
- procurement awards create a protected monopoly without open APIs and private
  resale rights;
- AI systems are used for rights-impacting decisions without human review and
  appeal;
- water use threatens local communities or agriculture;
- data-centre debt is guaranteed off-book or paid from unfunded public-service
  obligations;
- skilled Iraqi staffing is replaced by permanent foreign operator dependence.

## Source Anchors

- DataReportal, [Digital 2025: Iraq](https://datareportal.com/reports/digital-2025-iraq), for internet and social media user estimates.
- World Bank, [Iraq Open Data](https://data.worldbank.org/country/iraq), for 2024 GDP context.
- IEA, [Iraq country profile](https://www.iea.org/countries/iraq), for energy-resource and power-system context.
- Uptime Institute, [data-centre cost commentary](https://journal.uptimeinstitute.com/data-center-costs-set-to-rise-and-rise/) and [2025 survey](https://uptimeinstitute.com/resources/research-and-reports/uptime-institute-global-data-center-survey-results-2025), for cost and PUE context.
- OpenStack, [open-source cloud infrastructure](https://www.openstack.org/), Kubernetes, [container orchestration](https://kubernetes.io/), Ceph, [open-source distributed storage](https://ceph.io/en/), OpenSearch, [open-source search and analytics](https://opensearch.org/), Mastodon, [free and open-source decentralized social media](https://joinmastodon.org/), Hugging Face Transformers, [open model tooling](https://huggingface.co/docs/transformers/en/index), and vLLM, [open-source LLM inference](https://docs.vllm.ai/), for reference stack viability.
