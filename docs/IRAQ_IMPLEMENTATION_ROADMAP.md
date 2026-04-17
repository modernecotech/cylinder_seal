# Iraq Implementation Roadmap: Digital Iraqi Dinar

## Executive Summary

**Project**: Digital Iraqi Dinar (Digital IQD) - Central Bank of Iraq CBDC infrastructure
**Duration**: 18 months (Months 1-18 from parliament approval)
**Investment**: $5-7 million
**ROI**: Seigniorage capture of $4-8 billion annually by Year 2
**Phases**: 5 (Design → Pilot → Regional → National → Maturity)

---

## Phase 1: Design & Legal Framework (Months 1-2)

### 1.1 Parliamentary Legislation

**Deliverables:**
- Draft Digital Currency Act (define legal status, CBI authority, consumer protections)
- Parliamentary briefing slides (5-slide overview for finance committee)
- Impact analysis (effect on banking system, monetary policy, financial inclusion)

**Timeline:**
- Week 1-2: Draft bill presented to parliament finance committee
- Week 3-4: Committee debates and amendments
- Week 5-6: Full parliament debate and vote (target: simple majority)
- Week 7-8: Law signed and published

**Success Criteria:**
- Act passes with >50% parliamentary support
- Act provides clear legal foundation for CBI authority
- Act includes data privacy and consumer protection clauses

**Key Risks:**
- Political opposition from commercial banks (lose fee income)
- Rural parliament members skeptical of technology
- **Mitigation**: Emphasize financial inclusion benefit and job creation in rural areas

---

### 1.2 CBI Internal Organization

**Deliverables:**
- CBI Oversight Board established (5 members)
- Digital Currency Task Force assembled (technical + legal team, 20+ people)
- Budget allocation ($5-7M over 18 months)
- Implementation roadmap finalized
- Technical requirements document (specifications for super-peers, apps, APIs)

**Timeline:**
- Week 1: CBI Governor appoints Oversight Board
- Week 2: Task Force recruits begin (target: 20 people by Week 4)
- Week 3-4: Budget presented to Ministry of Finance
- Week 5-6: Technical architecture finalized
- Week 7-8: RFP drafted (if outsourcing) or team assignment (if internal)

**Success Criteria:**
- Oversight Board functioning (weekly meetings)
- Task Force has critical roles filled: CTO, security lead, ops lead, legal lead
- Budget secured
- Technical requirements reviewed by external experts (no critical gaps)

**Key Risks:**
- Difficulty recruiting qualified engineers in Iraq (shortage of blockchain/distributed systems expertise)
- **Mitigation**: Hire international consultants for critical roles; train local staff

---

### 1.3 Legal & Regulatory Framework

**Deliverables:**
- Data Privacy Law (CBI transaction data protected, not shared with other agencies)
- KYC/AML Procedures (FATF-compliant)
- Tax Reporting Requirements (how Digital IQD income is taxed)
- Emergency Authority Procedures (CBI can freeze accounts, pause issuance in crisis)
- Inter-agency Agreements (with tax authority, law enforcement, banks)

**Timeline:**
- Month 1: Draft all legal documents
- Month 1.5: Legal review (internal + external counsel)
- Month 2: Parliament approves; CBI publishes procedures

**Success Criteria:**
- All legal instruments passed by parliament or CBI Board
- Procedures aligned with FATF standards
- Law enforcement has clear path to access transaction data (with warrants)
- Privacy protected; no unauthorized access

---

### 1.4 Vendor Selection (If Outsourcing)

**Deliverables:**
- RFP published (specifies technical requirements, timeline, budget)
- Vendor shortlist created (3-5 candidates)
- Vendor selected via competitive bidding

**Timeline:**
- Week 6-8: RFP drafted and published
- Week 9-12: Vendors submit proposals
- Week 13: CBI evaluates (technical capability, cost, schedule)
- Week 14: Vendor selected and contract signed

**Success Criteria:**
- RFP is technically rigorous (no vendor can claim they don't understand requirements)
- Selected vendor has prior CBDC or blockchain experience
- Contract includes strict penalty clauses for schedule slippage or quality failures
- Vendor commits to ongoing support (5-year minimum)

**Alternative:** CBI assembles internal team (requires hiring 15-20 engineers by Month 2)

---

## Phase 2: Baghdad Pilot (Months 2-5, overlaps with Phase 1)

### 2.1 Infrastructure Setup

**Deliverables:**
- Data center setup in CBI Baghdad facility (secure, redundant power, backup)
- PostgreSQL 16 database installed and configured
- Redis 7 cache configured (nonce deduplication, rate limiting)
- Monitoring and alerting system deployed (prometheus, grafana, etc.)
- Backup procedures tested (hourly snapshots, off-site replication)

**Timeline:**
- Month 2: RFP for data center infrastructure issued (or internal team assigns resources)
- Month 2.5: Hardware arrives and installed
- Month 3: Database and cache configured; backup procedures tested
- Month 3.5: Monitoring system deployed and tested

**Success Criteria:**
- 99.5%+ uptime of infrastructure
- <1 second database query latency for transaction lookups
- Backup procedures tested monthly (can recover from full data center failure)
- All infrastructure changes logged and auditable

**Key Risks:**
- Power outages (Iraq has unstable grid)
- **Mitigation**: Battery backup (UPS) for 4+ hours; generator for 48+ hours

---

### 2.2 Software Development - Backend (Rust)

**Deliverables:**
- cs-core: Transaction, JournalEntry, User types (with BLAKE2 hashing, Ed25519 signing)
- cs-storage: PostgreSQL repositories (users, transactions, ledger state)
- cs-sync: gRPC ChainSync service (device ↔ super-peer sync)
- cs-api: REST API (KYC callbacks, admin endpoints)
- cs-exchange: OWC→IQD rate converter (simplified: just use fixed rate for pilot)
- Docker Compose: Local development environment

**Timeline:**
- Month 2: Types and data models finalized
- Month 2.5: PostgreSQL schema and migrations
- Month 3: Core storage layer (repos, queries, transactions)
- Month 3.5: gRPC sync service and conflict detection
- Month 4: REST API and KYC integration stubs
- Month 4.5: Docker setup; local test environment ready

**Success Criteria:**
- All services compile without warnings
- Unit tests cover >80% of code
- Performance benchmarks pass (1M transactions/second throughput for super-peer)
- Docker Compose brings up entire backend with single `docker-compose up`

**Key Risks:**
- Schedule slippage due to requirement changes
- **Mitigation**: Freeze requirements after Month 1.5; use agile sprints with weekly demos

---

### 2.3 Software Development - Android App

**Deliverables:**
- feature-onboarding: Key generation, PIN setup, KYC tier selection
- feature-wallet: Home screen showing balance, recent transactions
- feature-pay: NFC payment initiation (payer side)
- feature-receive: HCE setup, receipt of NFC payment (receiver side)
- feature-history: Transaction history with filtering and export
- core-database: Room database with SQLCipher encryption
- core-crypto: Tink integration for key management
- core-network: Retrofit + gRPC client for super-peer sync

**Timeline:**
- Month 2: Project setup, basic UI scaffolding
- Month 2.5: Onboarding flow (key generation, PIN)
- Month 3: Wallet and transaction history UI
- Month 3.5: NFC pay/receive flow
- Month 4: Database and encryption integration
- Month 4.5: Super-peer sync and conflict resolution UI

**Success Criteria:**
- App installs and runs on Android 7.0+ (min SDK 24)
- Key generation works with Android Keystore
- NFC payment can be initiated and received (two phones can transact)
- Transaction history persists and syncs to super-peer
- APK size <20MB (important for low-storage devices)

**Key Risks:**
- NFC hardware differences between devices (some phones lack NFC)
- **Mitigation**: Implement BLE fallback; test on 5+ device models

---

### 2.4 Integration & Testing

**Deliverables:**
- E2E test: Device A pays Device B offline → both sync to super-peer → super-peer confirms
- Load testing: Super-peer handles 10K concurrent devices
- Security testing: Penetration test of gRPC API
- User acceptance testing with 100 government employees (Baghdad IT department)

**Timeline:**
- Month 4: E2E test infrastructure setup
- Month 4.5: Load and security testing
- Month 5: UAT with government employees

**Success Criteria:**
- E2E test passes 100 times in a row (zero failures)
- Super-peer handles 10K concurrent devices without latency degradation
- Security audit finds no critical vulnerabilities (0 or 1 medium-severity issues acceptable)
- 100 UAT users give >4.5/5 satisfaction rating

---

### 2.5 Government Employee Pilot (Month 5)

**Rollout:**
- Week 1: 1,000 government IT employees (easy to train, understand tech)
- Week 2-3: 10,000 government employees (expand to 2-3 more agencies)
- Week 4: 100,000 government employees (full expansion to all Baghdad agencies)

**Workflow:**
1. Employee downloads app, generates key, sets PIN
2. Employee receives salary from CBI as Digital IQD (CBI makes one bulk transfer to each agency)
3. Employees transact with each other (peer-to-peer)
4. CBI monitors for issues (double-spend, crashes, fraud)
5. Feedback loop: employees report issues via app, CBI fixes

**Daily Monitoring:**
- System uptime (target: >95%)
- Transaction latency (target: <2 seconds from tap to confirmation)
- Double-spend rate (target: <0.001%)
- User complaints (target: <5 per day for 100K users)

**Success Criteria:**
- Pilot runs 4+ weeks with zero major incidents
- System uptime stays >95%
- <1% user churn (employees stick with app)
- Media coverage is positive ("Iraq launches revolutionary digital currency")

---

## Phase 3: Regional Expansion (Months 5-9)

### 3.1 Multi-Super-Peer Infrastructure

**Deliverables:**
- Basra super-peer node (2nd data center)
- Erbil super-peer node (3rd data center, Kurdish region)
- Inter-super-peer gossip protocol (super-peers sync hourly via gRPC)
- Byzantine consensus (3-of-5 quorum voting; 3 super-peers for now)
- Data replication (ledger replicated to all 3 super-peers)

**Timeline:**
- Month 5: Basra + Erbil data centers provisioned
- Month 5.5: Gossip protocol implementation
- Month 6: Byzantine consensus integrated
- Month 6.5: Replication tested (simulate super-peer failures, verify recovery)

**Success Criteria:**
- 3 super-peers operate independently
- Gossip protocol syncs all super-peers within 10 minutes
- Byzantine consensus (3-of-5) works with 1-2 super-peers down
- Ledger is consistent across all 3 super-peers (verified by hash comparison)

---

### 3.2 Automated AML/CFT System

**Deliverables:**
- Transaction flagging rules (large transfers, structuring, geographic anomalies)
- Blacklist integration (OFAC sanctions list, Iraqi wanted-persons list)
- SAR/CTR reporting to Ministry of Finance
- Automated account freeze (for confirmed violations)
- Manual review queue (for suspicious but not confirmed violations)

**Timeline:**
- Month 6: Flagging rules defined
- Month 6.5: Integration with blacklist sources
- Month 7: Automated freeze mechanism
- Month 7.5: SAR/CTR reporting

**Success Criteria:**
- <1% false positive rate (less than 1 in 100 flagged transactions are actually suspicious)
- 100% detection of blacklist matches
- Flagged accounts reviewed within 24 hours
- All SAR/CTR reports submitted on schedule

---

### 3.3 Credit Scoring Engine

**Deliverables:**
- Credit score calculation (based on payment history, transaction volume, counterparty overlap)
- Score publication to users
- Credit score used for lending limits

**Timeline:**
- Month 7: Scoring algorithm defined and tested
- Month 7.5: Batch job runs daily (computes scores for all users)
- Month 8: Scores published to users in app
- Month 8.5: Lending system uses scores (Tier A: score >70, limit 10M IQD, etc.)

**Success Criteria:**
- Credit score predicts loan default risk (>70% accuracy)
- Scores are transparent (users understand how score is calculated)
- Scoring is fair (no bias based on demographic characteristics)

---

### 3.4 Merchant Classification System (Iraqi Made Preference)

**Deliverables:**
- Tier 1/2/3/4 definitions published (what counts as Iraqi made)
- Merchant registry opens (retailers register products + origins)
- Product barcode system (origin code scanned at checkout)
- App spending tier enforcement (automatic limits per merchant tier)
- Spot audit procedures (5% of merchants verified monthly)
- Community reporting system (users report false classifications)

**Timeline:**
- Month 6: CBI publishes tier definitions + merchant registry
- Month 6.5: Initial merchant onboarding (Tier 1 retailers sign up)
- Month 7: Product barcode system integrated into app
- Month 7.5: Audit procedures operational

**Success Criteria:**
- 1,000+ Tier 1 (local) merchants registered
- 500+ Tier 2 (mixed) merchants registered
- Product origin data for 10,000+ items
- <1% false declaration rate in audits
- Community reports credible (fraud reports lead to merchant ban)

**Market Effect:**
- Tier 1 merchants see revenue surge (customers must spend salary budget here first)
- Tier 4 (pure import) merchants lose Digital Dinar customers
- Retailers begin stocking more local goods (to access Digital Dinar market)

### 3.5 Peer-to-Peer Lending System

**Deliverables:**
- Loan request flow (borrower requests loan, specifies amount + tenor)
- Lender discovery (lenders can search for loans to fund)
- Loan origination (lender approves, funds transfer to borrower)
- Repayment tracking (borrower makes monthly payments)
- Default handling (if borrower misses payment, dispute resolution)

**Timeline:**
- Month 7.5: Lending system architecture defined
- Month 8: Loan request and discovery UI
- Month 8.5: Repayment tracking and default handling

**Success Criteria:**
- 1,000+ active loans by end of phase
- <5% default rate (borrowers pay back)
- Market interest rates emerge (8-20% APR range)
- Lenders earn returns (users willing to lend at market rates)

---

### 3.6 Regional Rollout (Months 8-9)

**Timing:**
- Month 8: Announce expansion to Basra + Erbil
- Month 8.5: Super-peers go live in both cities
- Month 9: Users in both cities can download app and transact with tier spending limits active

**Merchant Expansion:**
- Basra merchants register Tier 1/2/3 (expansion of registry from Baghdad)
- Erbil merchants register (KRG-specific local goods identification)
- Cross-regional Tier definitions (what counts as local in each region)

**User Growth Target:**
- End of Month 5: 100K users (Baghdad pilot)
- End of Month 6: 500K users (Baghdad expansion)
- End of Month 7: 1M users (Basra + Erbil opening)
- End of Month 9: 2-5M users (regional adoption)

**Success Criteria:**
- 2-5M users by end of phase
- System uptime stays >99.5% across all regions
- Transaction throughput increases to 100K/day (from 10K/day in pilot)
- Credit scores are being used for lending decisions
- Tier spending limits working correctly (users can't overspend tier budgets)
- Local merchants report revenue increase (Tier 1 surge)

---

## Phase 4: National Scale (Months 9-18)

### 4.1 Super-Peer Federation Expansion

**Timeline:**
- Month 9: Plan expansion to 10+ super-peers (one per CBI regional branch)
- Month 10-12: Provision data centers and deploy super-peers in all branches
- Month 13-15: Integrate all super-peers into federation
- Month 15-18: Scale to handle national transaction load

**Super-Peer Locations (Target):**
- Baghdad (primary)
- Basra (southern)
- Erbil (north/KRG)
- Mosul (northern plains) - after security stabilization
- Karbala (central)
- Najaf (south-central)
- Diwaniyah (central)
- Wasit (east-central)
- Anbar (western) - if security permits
- Sulaymaniyah (KRG) - if KRG approves
- Kirkuk (disputed region) - handle separately

**Byzantine Consensus Update:**
- Month 9: Still 3-of-5 consensus (3 super-peers needed to confirm)
- Month 12: Upgrade to 5-of-9 consensus (5 super-peers needed)
- Month 15: Upgrade to 7-of-15 consensus (7 super-peers needed for confirmation)

**Rationale:** As more super-peers come online, increase quorum requirement to maintain security.

---

### 4.2 Full Android App Features

**Deliverables (all features enabled nationwide):**
- Offline NFC/BLE peer-to-peer payments (fully working)
- Credit scoring and lending marketplace
- Peer-to-peer lending (borrower/lender matching)
- Merchant accounts (business users) with Tier 1/2/3/4 classification
- Marketplace for services (taxi, food, cleaning, etc.) - separated by tier
- Tier spending allocation enforcement (user can't exceed tier budgets)
- Merchant filtering by tier (users can search "Show me Tier 1 local merchants")
- Tax reporting (automatic tax withholding on merchant income)
- Account recovery (social recovery via trusted contacts - deploy by Month 12)

**Spending Tier Features:**
- Home screen shows spending allocation breakdown (Tier 1: unlimited, Tier 2: 500K used of 500K, etc.)
- Merchants display tier badge (🇮🇶 Made in Iraq, ⚙️ Assembled Local, 📦 Mixed Import, ❌ Import Only)
- Transaction blocking if tier budget exceeded (app shows: "You've used your import allowance")
- Monthly allocation reset (on government salary receipt)
- Tier preference filtering (users can sort by "Local First")

**Timeline:**
- Month 9-12: Beta test all features with 1M+ users
- Month 13-15: Bug fixes and performance optimization
- Month 15-18: Full feature rollout nationwide

---

### 4.3 Bank Integration

**Deliverables:**
- Banks can accept Digital IQD deposits (convert to reserves)
- Banks can offer loans in Digital IQD
- Banks can see customer credit scores (to inform lending decisions)
- Inter-bank settlement in Digital IQD (banks transact with each other)

**Timeline:**
- Month 10: Technical API published (banks can integrate)
- Month 12: First 5 banks integrate
- Month 15: 10+ banks integrated
- Month 18: 20+ banks integrated

**Success Criteria:**
- Banks actively using Digital IQD for customer deposits/loans
- Banks no longer gatekeep access to payment system (Digital IQD exists independently)
- Credit scores from Digital IQD inform bank lending (reduces reliance on collateral)

---

### 4.4 Physical ↔ Digital Conversion

**Deliverables:**
- Citizens can exchange physical IQD for Digital IQD at any CBI branch (1:1)
- Citizens can exchange Digital IQD for physical IQD at any CBI branch (1:1)
- Merchants accept both (payment systems handle both)

**Timeline:**
- Month 9: Conversion infrastructure deployed
- Month 10: All CBI branches can do conversions
- Month 15: Most merchants accept both (incentivized)
- Month 18: Physical + Digital in parallel (both circulate freely)

---

### 4.5 Trade Policy Effects & Merchant Ecosystem

**Expected market effects (Months 9-18):**

**Month 9-12 (Early Effects):**
- Government salaries trigger surge in Tier 1 demand (retailers must stock local to capture salary market)
- Local producers begin scaling up (know there's guaranteed customer base)
- Tier 4 (pure import) merchants report revenue loss (can't capture Digital Dinar market)
- Tier 2 (mixed) merchants add local content to move up to Tier 1
- Imports decline measurably (20-30% reduction in consumer goods imports)

**Month 12-18 (Medium-term Effects):**
- Local supply chains develop (packaging, distribution, inputs produced domestically)
- Prices for local goods drop (competition increases as more producers enter)
- Trade balance visibly improves (imports down, local production up)
- Small businesses flourishing (local merchants report strong sales growth)
- New employment in local production and distribution

**Expected KPI:**
- Tier 1 (local) merchants: 3,000+ nationwide
- Tier 2 (mixed) merchants: 2,000+ nationwide
- Tier 1+2 revenue share: 60%+ of all Digital Dinar spending
- Tier 4 (pure import) market share: <10% (most converted to Tier 2/3)
- Trade deficit reduction: 15-25% improvement by Month 18

### 4.6 National User Growth

**Target (End of Month 18):**
- 40M+ active users (80% of Iraqi population)
- 1M+ daily transactions
- 50+ billion IQD circulating digitally
- 50+ billion IQD still in physical form (optional)
- Financial inclusion improved from 30% → 85%+

**Demographics:**
- Government employees (1M)
- Private sector workers (5M)
- Self-employed merchants (3M)
- Rural farmers (10M)
- Urban workers (15M+)
- Remainder: inactive accounts (5M)

**Trade Impact:**
- Imports (Tier 4): 5-10 billion IQD of Digital spending
- Tier 3 (mixed): 10-15 billion IQD of Digital spending
- Tier 2 (mixed local): 15-20 billion IQD of Digital spending
- Tier 1 (local): 20-25 billion IQD of Digital spending (50% of total)
- Net effect: Majority of government salary money flowing to local producers

---

## Phase 5: Maturity & Evolution (Year 2+, Optional)

### 5.1 Optional: Phase Out Physical IQD

**Decision point (Month 18):**
- CBI Board votes whether to phase out physical IQD
- If yes: 10-year gradual phase-out (reduce printing each year)
- If no: Maintain dual system indefinitely

**Advantages of digital-only:**
- Eliminate counterfeiting
- Real-time monetary policy (impossible with physical cash)
- Reduced CBI printing costs
- But: Requires full population adoption (excludes unbanked)

---

### 5.2 Optional: Open Super-Peer Operation

**Decision point (Month 18):**
- CBI Board votes whether to license banks/NGOs to operate super-peers
- If yes: Publish licensing requirements, solicit applications
- If no: Keep super-peers as CBI-operated only

**Licensing requirements:**
- Must be licensed financial institution (bank, NGO, telecom)
- Must pass security audit
- Must comply with CBI policy
- Must maintain 99.5%+ uptime
- Subject to Oversight Board audit

---

### 5.3 Optional: Open Source Code Release

**Decision point (Month 18):**
- CBI Board votes whether to publish source code
- If yes: Release under AGPL-3.0 (prevents proprietary capture)
- If no: Keep code proprietary to CBI

**Advantages:**
- Community can audit for backdoors
- Other countries can adopt (would need their own central bank)
- Prevents vendor lock-in
- But: Exposes implementation details to attackers

---

### 5.4 Year 2-3: International Expansion

**Optional (if CBI chooses):**
- Other countries adopt same platform (UAE, Saudi Arabia, Egypt)
- Each issues own currency
- Could federate governance (optional)
- Iraq maintains full sovereignty over IQD policy

**Timeline:**
- Year 2: Engagement with 2-3 other central banks
- Year 2.5: Technical sharing and training
- Year 3+: Other countries launch own Digital [Currency]

---

## Resource Requirements

### Team Composition

**Rust Backend (10-15 people):**
- CTO/Lead Architect (1)
- Senior engineers (3-4)
- Mid-level engineers (4-5)
- Junior engineers (2-3)
- QA/test engineers (2)

**Android Development (8-12 people):**
- Android Lead (1)
- Senior engineers (2-3)
- Mid-level engineers (3-4)
- Junior engineers (1-2)
- QA/test engineers (2)

**Operations & DevOps (4-6 people):**
- DevOps lead (1)
- Infrastructure engineers (2-3)
- Database admin (1)
- Security engineer (1)

**Project Management & Support (3-5 people):**
- Project manager (1)
- Business analyst (1)
- Legal/compliance (1)
- Communications (1)

**Total: 25-40 people**

---

## Budget Allocation

| Phase | Salaries | Infrastructure | Contracting | Contingency | Total |
|-------|----------|-----------------|-------------|------------|-------|
| **Phase 1 (Months 1-2)** | $300K | $100K | $200K | $100K | $700K |
| **Phase 2 (Months 2-5)** | $800K | $300K | $400K | $200K | $1.7M |
| **Phase 3 (Months 5-9)** | $1M | $500K | $300K | $200K | $2M |
| **Phase 4 (Months 9-18)** | $1.2M | $400K | $200K | $200K | $2M |
| **TOTAL** | **$3.3M** | **$1.3M** | **$1.1M** | **$0.7M** | **$6.4M** |

---

## Success Metrics & Exit Criteria

### Phase 1 Completion
- [ ] Parliament passes Digital Currency Act
- [ ] CBI Oversight Board operational
- [ ] Technical architecture finalized and reviewed
- [ ] Team of 10+ engineers hired
- [ ] Budget secured

### Phase 2 Completion
- [ ] 100K Baghdad pilot users
- [ ] 95%+ system uptime
- [ ] <30s average latency
- [ ] <0.01% double-spend rate
- [ ] Media coverage positive
- [ ] CBI Board approves Phase 3

### Phase 3 Completion
- [ ] 2-5M users across 3 regions
- [ ] 99.5%+ system uptime
- [ ] 3-of-5 Byzantine consensus stable
- [ ] Credit scoring active (scores help predict default risk)
- [ ] 500+ active peer-to-peer loans
- [ ] Bank integration begun

### Phase 4 Completion
- [ ] 40M+ users nationwide
- [ ] 99.9%+ system uptime
- [ ] 1M+ daily transactions
- [ ] 50+ billion IQD circulating
- [ ] 20+ banks integrated
- [ ] Full financial inclusion achieved
- [ ] Seigniorage projections validated

### Phase 5 (Maturity)
- [ ] CBI decides on optional features (phase-out physical, open super-peers, open source)
- [ ] System operates sustainably (costs <$1B/year to operate)
- [ ] Iraq becomes model for other CBDC deployments
- [ ] Monetary policy becomes precision instrument (real-time control of inflation)

---

## Risk Log & Mitigations

| Risk | Probability | Impact | Mitigation |
|------|-----------|--------|-----------|
| Schedule slippage (Phase 2) | High | High | Agile sprints, weekly demos, freeze requirements |
| Security vulnerability in Phase 2 | Medium | Critical | Third-party security audit, penetration testing |
| Low adoption (citizens don't trust digital) | Medium | High | Government salary incentive, merchant discounts, PR campaign |
| Bank opposition delays rollout | Medium | Medium | Include banks in governance, maintain open APIs |
| Power outages disrupt service | High | Medium | UPS backup, generator, distributed super-peers |
| Political pressure for inflation | Medium | High | Parliament oversight, public dashboards, independent audit |
| Regional conflict (Anbar, Mosul) | Low | Medium | Delay super-peers in unstable regions; federate carefully |
| Vendor fails to deliver (if outsourcing) | Low | Critical | Competitive bidding, strict contract terms, escrow funds |

---

## Conclusion

The Digital Iraqi Dinar is achievable in 18 months with disciplined execution. The project is **critical path item** (many sub-projects must complete on schedule) with **high visibility** (political and media attention).

**Success means:**
- Iraq has modern CBDC infrastructure
- Financial inclusion increases from 30% → 85%
- CBI captures $40B+ annually in seigniorage
- Commercial banks compete fairly instead of gatekeeping
- Monetary policy becomes precise instrument

**Failure means:**
- Months of work and $6M investment wasted
- Political crisis (parliament questioning CBI competence)
- Citizens lose trust in government technology
- Must wait 5+ years before attempting again

**Therefore:** Treat this as **mission critical**. Assign best people, remove obstacles, report weekly progress to CBI Board.

