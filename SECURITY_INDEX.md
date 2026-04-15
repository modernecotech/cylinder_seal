# CylinderSeal Documentation Index

## 📋 Start Here

**For VCs/Business:**
1. **[VC_PITCH.md](VC_PITCH.md)** — Investor deck (20 min read)
2. **[README.md](README.md)** — Project overview (5 min)
3. **[SECURITY_SUMMARY.md](SECURITY_SUMMARY.md)** — Security/defensibility (15 min)

**For Technical:**
1. **[README.md](README.md)** — Project overview (5 min)
2. **[docs/IRON_SECURITY.md](docs/IRON_SECURITY.md)** — 12 hardening layers (30 min)
3. **[IMPLEMENTATION_ROADMAP.md](IMPLEMENTATION_ROADMAP.md)** — Build plan (30 min)
4. **[docs/DEVELOPER_QUICK_REFERENCE.md](docs/DEVELOPER_QUICK_REFERENCE.md)** — Coding patterns (reference)

**For Everything:**
- **[SECURITY_SUMMARY.md](SECURITY_SUMMARY.md)** — What was hardened, why (15 min)
- **[TERMINOLOGY_REFACTORING.md](TERMINOLOGY_REFACTORING.md)** — Why "chainblock" → "journal entry" (5 min)

---

## 🔐 Security Documentation

### Architecture & Design
| Document | Purpose | Read If |
|----------|---------|---------|
| **[/.claude/plans/zazzy-finding-muffin.md](/.claude/plans/zazzy-finding-muffin.md)** | System architecture (3 tiers, stack choices) | You need to understand the overall design |
| **[/.claude/plans/security-hardening.md](/.claude/plans/security-hardening.md)** | 9 attack vectors + mitigations (before iron-hardening) | You want to understand incremental improvements |
| **[docs/IRON_SECURITY.md](docs/IRON_SECURITY.md)** | 12 hardening layers with code examples | You're implementing features |

### Validation & Operations
| Document | Purpose | Read If |
|----------|---------|---------|
| **[docs/SECURITY_VALIDATION.md](docs/SECURITY_VALIDATION.md)** | 4 defense layers: device, super-peer, consensus, anomaly | You're building validation logic |
| **[docs/DEVELOPER_QUICK_REFERENCE.md](docs/DEVELOPER_QUICK_REFERENCE.md)** | Common patterns, testing, debugging | You're actively coding |
| **[SECURITY_IMPROVEMENTS.md](SECURITY_IMPROVEMENTS.md)** | Before/after comparison, attack complexity matrix | You're reporting progress |

### Implementation & Deployment
| Document | Purpose | Read If |
|----------|---------|---------|
| **[IMPLEMENTATION_ROADMAP.md](IMPLEMENTATION_ROADMAP.md)** | 16-week plan, 4 phases, weekly deliverables | You're planning sprints or hiring |
| **[SECURITY_SUMMARY.md](SECURITY_SUMMARY.md)** | Executive summary of hardening | You're pitching to investors/stakeholders |

---

## 🎯 By Role

### Security Engineer
Start with:
1. [SECURITY_SUMMARY.md](SECURITY_SUMMARY.md)
2. [docs/IRON_SECURITY.md](docs/IRON_SECURITY.md) (all 12 layers)
3. [docs/SECURITY_VALIDATION.md](docs/SECURITY_VALIDATION.md) (4 defense layers)
4. [IMPLEMENTATION_ROADMAP.md](IMPLEMENTATION_ROADMAP.md) (audit planning)

### Rust Backend Engineer
Start with:
1. [docs/IRON_SECURITY.md](docs/IRON_SECURITY.md) (sections 1-11)
2. [crates/cs-core/src/models.rs](crates/cs-core/src/models.rs) (study updated fields)
3. [docs/DEVELOPER_QUICK_REFERENCE.md](docs/DEVELOPER_QUICK_REFERENCE.md) (patterns)
4. [IMPLEMENTATION_ROADMAP.md](IMPLEMENTATION_ROADMAP.md) (Weeks 1-4, 5-10)

### Android Engineer
Start with:
1. [docs/IRON_SECURITY.md](docs/IRON_SECURITY.md) (sections 3, 5, 6)
2. [android/core/core-crypto/](android/core/core-crypto/) (Keystore, nonce, attestation)
3. [docs/DEVELOPER_QUICK_REFERENCE.md](docs/DEVELOPER_QUICK_REFERENCE.md) (Kotlin patterns)
4. [IMPLEMENTATION_ROADMAP.md](IMPLEMENTATION_ROADMAP.md) (Weeks 1-4, 11-14)

### DevOps/Infrastructure
Start with:
1. [SECURITY_SUMMARY.md](SECURITY_SUMMARY.md)
2. [docs/IRON_SECURITY.md](docs/IRON_SECURITY.md) (Section 2: 5 Super-Peers)
3. [IMPLEMENTATION_ROADMAP.md](IMPLEMENTATION_ROADMAP.md) (Week 8: 5-super-peer setup)

### Product Manager
Start with:
1. [SECURITY_SUMMARY.md](SECURITY_SUMMARY.md)
2. [docs/IRON_SECURITY.md](docs/IRON_SECURITY.md) (Graduated Security Tiers section)
3. [IMPLEMENTATION_ROADMAP.md](IMPLEMENTATION_ROADMAP.md) (Phase 3: UX)

### Compliance/Legal
Start with:
1. [SECURITY_SUMMARY.md](SECURITY_SUMMARY.md) (Regulatory Status)
2. [docs/IRON_SECURITY.md](docs/IRON_SECURITY.md) (Section 10: Audit Logging)
3. [docs/DEVELOPER_QUICK_REFERENCE.md](docs/DEVELOPER_QUICK_REFERENCE.md) (Regulatory Compliance Checklist)

---

## 🔍 By Topic

### Key Management
- [docs/IRON_SECURITY.md](docs/IRON_SECURITY.md) - Section 1 (Device Key Rotation)
- [docs/IRON_SECURITY.md](docs/IRON_SECURITY.md) - Section 2 (Super-Peer Keys)
- [docs/IRON_SECURITY.md](docs/IRON_SECURITY.md) - Section 3 (User Key Recovery)

### Cryptography
- [docs/IRON_SECURITY.md](docs/IRON_SECURITY.md) - Section 4 (E2E Encryption)
- [crates/cs-core/src/crypto.rs](crates/cs-core/src/crypto.rs) - Ed25519, BLAKE2b
- [docs/IRON_SECURITY.md](docs/IRON_SECURITY.md) - Section 6 (Hardware-Bound Nonces)

### Consensus & Conflict Resolution
- [docs/IRON_SECURITY.md](docs/IRON_SECURITY.md) - Section 2 (5-Super-Peer BFT)
- [docs/IRON_SECURITY.md](docs/IRON_SECURITY.md) - Section 5 (Deterministic Resolution)
- [docs/SECURITY_VALIDATION.md](docs/SECURITY_VALIDATION.md) - Conflict Detection

### Offline Security
- [docs/IRON_SECURITY.md](docs/IRON_SECURITY.md) - Section 1 (Vector Clocks)
- [docs/IRON_SECURITY.md](docs/IRON_SECURITY.md) - Section 2 (Monotonic Clocks)
- [docs/SECURITY_VALIDATION.md](docs/SECURITY_VALIDATION.md) - Device-level validation

### Device & Hardware
- [docs/IRON_SECURITY.md](docs/IRON_SECURITY.md) - Section 3 (Strongbox)
- [docs/IRON_SECURITY.md](docs/IRON_SECURITY.md) - Section 6 (Hardware-Bound Nonces)
- [docs/IRON_SECURITY.md](docs/IRON_SECURITY.md) - Section 8 (Device Reputation)

### Audit & Compliance
- [docs/IRON_SECURITY.md](docs/IRON_SECURITY.md) - Section 10 (Immutable Audit Logs)
- [docs/DEVELOPER_QUICK_REFERENCE.md](docs/DEVELOPER_QUICK_REFERENCE.md) - Regulatory Checklist

### User Experience
- [docs/IRON_SECURITY.md](docs/IRON_SECURITY.md) - Section 7 (Graduated Security Tiers)
- [docs/IRON_SECURITY.md](docs/IRON_SECURITY.md) - Section 8 (Transaction Witnesses)
- [IMPLEMENTATION_ROADMAP.md](IMPLEMENTATION_ROADMAP.md) - Phase 3 (UX)

---

## 📊 Document Complexity

| Complexity | Documents |
|-----------|-----------|
| **Basic** | README.md, SECURITY_SUMMARY.md |
| **Intermediate** | docs/DEVELOPER_QUICK_REFERENCE.md, IMPLEMENTATION_ROADMAP.md |
| **Advanced** | docs/IRON_SECURITY.md, docs/SECURITY_VALIDATION.md |
| **Expert** | crates/cs-core/src/models.rs, proto/chain_sync.proto |

---

## 🚀 Quick Navigation

**"I need to understand the system quickly"**
→ [SECURITY_SUMMARY.md](SECURITY_SUMMARY.md) (15 min)

**"I'm implementing a feature"**
→ [docs/DEVELOPER_QUICK_REFERENCE.md](docs/DEVELOPER_QUICK_REFERENCE.md) (reference)

**"I need to debug an issue"**
→ [docs/DEVELOPER_QUICK_REFERENCE.md](docs/DEVELOPER_QUICK_REFERENCE.md) (Debugging Guide)

**"I'm reviewing a PR"**
→ [docs/SECURITY_VALIDATION.md](docs/SECURITY_VALIDATION.md) (Validation Rules)

**"I need to plan the build"**
→ [IMPLEMENTATION_ROADMAP.md](IMPLEMENTATION_ROADMAP.md) (16 weeks, 4 phases)

**"I'm pitching to investors"**
→ [SECURITY_SUMMARY.md](SECURITY_SUMMARY.md) (attack complexity + ROI)

**"I need regulatory approval"**
→ [docs/IRON_SECURITY.md](docs/IRON_SECURITY.md) (Section 10: Audit Logs)

---

## 📚 Complete File Reference

### Security Plans & Architecture
```
/.claude/plans/
├── zazzy-finding-muffin.md          ← System architecture (3 tiers)
├── security-hardening.md             ← 9 attack vectors + MVP improvements
└── (This file)
```

### Security Documentation
```
docs/
├── IRON_SECURITY.md                  ← 12 hardening layers (most important)
├── SECURITY_VALIDATION.md            ← 4 defense layers, validation rules
├── DEVELOPER_QUICK_REFERENCE.md      ← Patterns, debugging, compliance
└── (others)
```

### Implementation
```
SECURITY_IMPROVEMENTS.md              ← Before/after comparison
IMPLEMENTATION_ROADMAP.md             ← 16-week plan
SECURITY_SUMMARY.md                   ← Executive summary
SECURITY_INDEX.md                     ← This file
```

### Code Files (Updated for Hardening)
```
crates/cs-core/src/
├── models.rs                         ← Transaction, LedgerBlock (UPDATED)
├── crypto.rs                         ← Ed25519, BLAKE2b (UPDATED)
├── nonce.rs                          ← Deterministic RFC 6979 (NEW)
└── hardware_binding.rs               ← Device serial binding (NEW)

proto/
└── chain_sync.proto                  ← Protobuf schemas (UPDATED)
```

---

## ✅ Verification Checklist

Before going to production, verify you've read:

- [ ] [README.md](README.md) - Project overview
- [ ] [SECURITY_SUMMARY.md](SECURITY_SUMMARY.md) - What changed
- [ ] [docs/IRON_SECURITY.md](docs/IRON_SECURITY.md) - How to build it
- [ ] [docs/SECURITY_VALIDATION.md](docs/SECURITY_VALIDATION.md) - Validation rules
- [ ] [IMPLEMENTATION_ROADMAP.md](IMPLEMENTATION_ROADMAP.md) - Timeline
- [ ] [docs/DEVELOPER_QUICK_REFERENCE.md](docs/DEVELOPER_QUICK_REFERENCE.md) - Patterns
- [ ] [SECURITY_IMPROVEMENTS.md](SECURITY_IMPROVEMENTS.md) - Before/after

---

## 🎓 Learning Path

**Week 1 (Understanding):**
- Monday: [README.md](README.md) + [SECURITY_SUMMARY.md](SECURITY_SUMMARY.md)
- Tuesday: [docs/IRON_SECURITY.md](docs/IRON_SECURITY.md) sections 1-6
- Wednesday: [docs/IRON_SECURITY.md](docs/IRON_SECURITY.md) sections 7-12
- Thursday: [docs/SECURITY_VALIDATION.md](docs/SECURITY_VALIDATION.md)
- Friday: [IMPLEMENTATION_ROADMAP.md](IMPLEMENTATION_ROADMAP.md)

**Week 2 (Deep Dive):**
- By role: See "By Role" section above
- Study code: crates/cs-core/src/ + proto/chain_sync.proto
- Practice: [docs/DEVELOPER_QUICK_REFERENCE.md](docs/DEVELOPER_QUICK_REFERENCE.md) exercises

**Week 3+ (Building):**
- Reference: [docs/DEVELOPER_QUICK_REFERENCE.md](docs/DEVELOPER_QUICK_REFERENCE.md)
- Validate: [docs/SECURITY_VALIDATION.md](docs/SECURITY_VALIDATION.md)
- Track: [IMPLEMENTATION_ROADMAP.md](IMPLEMENTATION_ROADMAP.md)

---

## 📞 Questions?

**"Is this really iron-secure?"**
→ [SECURITY_SUMMARY.md](SECURITY_SUMMARY.md) - "The Bottom Line" section

**"How long to build?"**
→ [IMPLEMENTATION_ROADMAP.md](IMPLEMENTATION_ROADMAP.md) - 16 weeks

**"What could still fail?"**
→ [SECURITY_SUMMARY.md](SECURITY_SUMMARY.md) - "What Remains Open"

**"How do I debug X?"**
→ [docs/DEVELOPER_QUICK_REFERENCE.md](docs/DEVELOPER_QUICK_REFERENCE.md) - "Debugging Guide"

**"Am I building this right?"**
→ [docs/DEVELOPER_QUICK_REFERENCE.md](docs/DEVELOPER_QUICK_REFERENCE.md) - "Common Mistakes"

---

**Good luck building. You've got this. 🔐**
