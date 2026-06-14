# Cash Formalization And Demonetization Window

Status: cash-transition control model. This is not a monetary-law instruction,
AML approval, tax amnesty, sanctions determination, or official demonetization
plan.

The cash window is the highest-abuse-risk bridge between the old economy and
the formal Digital IQD / INDHC model. It must pull legitimate physical cash into
auditable channels without laundering corruption, sanctions exposure, theft,
terror finance, or unexplained public-office wealth.

## Core Rule

```text
No physical cash should become a transition balance, supplemental entitlement,
or dividend-adjacent claim unless the window is legally open, the deposit is
supervised, the depositor is identified, the amount fits caps, source-of-funds
risk is controlled, high-risk cases receive EDD, the receipt is signed, the
audit hash exists, appeals are live, and post-window rejection authority exists.
```

The one-year window is not an amnesty. It is a time-limited conversion process
with risk scoring, holds, referrals, caps, tax settlement, and public aggregate
reporting.

## What This Layer Controls

| Risk area | Control requirement |
| --- | --- |
| Legal authority | Cash conversion, post-window rejection, holds, referrals, tax settlement, and appeals have a lawful basis. |
| Window timing | Deposits are accepted only during the announced window, normally 365 days. |
| Supervision | Conversion points are licensed or otherwise supervised and staffed by trained operators. |
| Identity | Depositors are matched to eligible citizen or legal-person records before any receipt becomes convertible. |
| Cash authenticity | Physical notes are authenticated before any value is credited. |
| Per-citizen cap | Conversion is capped so unexplained cash cannot buy disproportionate benefit. |
| Source of funds | Deposits receive source-of-funds confidence scoring. Low-confidence cases go to enhanced due diligence. |
| Public-office risk | Politically exposed persons, public officials, related parties, and high-risk actors face stricter EDD. |
| Sanctions/watchlist | Hits are referred; they do not quietly convert. |
| Structuring | Split deposits, repeated small deposits, and pattern avoidance trigger EDD or referral. |
| Tax settlement | Required settlement, haircut, quarantine, or disclosure must complete before conversion. |
| Quarantine | Held funds sit in a controlled account until EDD, tax, or referral status is resolved. |
| Receipt and audit | Every deposit has a signed receipt, operator identity, location, timestamp, risk status, and audit hash. |
| Appeal path | Citizens can challenge holds, rejections, identity errors, and post-window disputes. |
| Public dashboard | Aggregate accepted, held, referred, rejected, and expired values are published without exposing private data. |

## Decision States

| Decision | Meaning | Required response |
| --- | --- | --- |
| Blocked | Legal authority or post-window rule is missing. | Do not open the cash window. |
| Not yet open | Deposit arrived before the legal start date. | Reject or record as pre-window inquiry only. |
| Window expired | Deposit arrived after the statutory window. | Reject under the post-window rule and report aggregate expiry metrics. |
| Rejected | Identity, supervision, cash authentication, receipt, audit, appeal, amount, or cap controls fail. | Reject or correct the failed control before re-submission. |
| Referred | Sanctions/watchlist or critical AML risk requires competent-authority review. | Freeze or quarantine under law; do not convert. |
| Hold for EDD | Source-of-funds, PEP, adverse media, structuring, suspicious activity, or tax settlement needs review. | Hold in quarantine until EDD/tax/referral outcome. |
| Accepted with settlement | Required tax or settlement has been collected. | Issue locked receipt after settlement evidence. |
| Accepted partial | Amount exceeds the cap, but a capped portion can convert. | Convert eligible amount and reject or quarantine excess. |
| Accepted | Deposit passes timing, identity, cap, provenance, receipt, audit, appeal, and dashboard gates. | Issue locked transition balance or supplemental entitlement receipt. |

## Why It Matters

The model originally needs a way to bring legacy cash into the new formal
system. Without this layer, the cash window can become the easiest way for
unknown provenance money to become state-recognized wealth. With this layer,
cash conversion becomes a controlled migration path:

```text
Physical cash
  -> supervised intake
  -> identity and cash authentication
  -> source-of-funds and AML screening
  -> cap, tax, quarantine, or referral decision
  -> signed receipt and audit hash
  -> locked transition balance or supplemental entitlement
  -> post-window rejection after expiry
```

## Conversion Boundaries

- Base citizen shares remain equal and non-saleable.
- Cash deposits do not buy control rights.
- Cash deposits do not bypass sanctions, AML, tax, or corruption review.
- Supplemental entitlements or transition balances are capped, locked, and
  auditable.
- Excess, suspicious, or post-window cash is rejected, held, or referred.
- Physical cash after the window has no conversion value unless explicit law
  creates a narrow appeal or correction exception.

## Software And Data Surface

The executable implementation is:

- `crates/cs-analytics/src/cash_formalization.rs`
- `migrations/20260721000001_cash_formalization.sql`

Core objects:

| Object | Purpose |
| --- | --- |
| `CashFormalizationInput` | Captures deposit reference, citizen reference, window day, legal authority, conversion-point supervision, operator training, identity, cash authentication, amount, cap, source-of-funds score, PEP/sanctions/adverse-media/structuring/suspicious flags, EDD, tax settlement, receipt, audit, quarantine, appeal, and dashboard status. |
| `CashFormalizationAssessment` | Computes remaining cap, eligible amount, converted value, quarantined amount, rejected amount, identity score, provenance score, operator-control score, AML risk score, settlement-readiness score, decision, and required actions. |
| `CashFormalizationGateResult` | Records pass/warn/fail state for legal, timing, supervision, identity, authenticity, cap, source-of-funds, PEP, sanctions, adverse media, structuring, suspicious activity, EDD, tax, receipt, audit, quarantine, appeal, and dashboard gates. |

## Dashboard Requirements

The cash-window dashboard should show aggregate metrics only:

- window start and expiry status;
- number and value of accepted deposits;
- number and value of partial acceptances;
- number and value of held deposits;
- number and value of referrals;
- number and value of rejections;
- number and value of expired/post-window attempts;
- source-of-funds confidence distribution;
- EDD backlog and resolution rate;
- tax settlement status;
- appeal backlog and resolution rate;
- conversion point performance and operator exception rates;
- cap-excess metrics;
- audit coverage.

## Governance Boundary

This layer should be stricter than ordinary retail wallet onboarding:

```text
If the legal window is missing, block.
If the window has expired, reject.
If identity or cash authentication fails, reject.
If sanctions or critical AML risk appears, refer.
If provenance is weak, hold for EDD.
If caps are exceeded, convert only the eligible portion.
If receipts or audit hashes are missing, reject.
If appeal paths or dashboards are missing, do not scale.
```

## Bottom Line

The cash formalization window is credible only if it makes unexplained cash
less powerful, not more powerful. The system should welcome legitimate cash
into formal channels while making corruption, coercion, sanctions evasion, and
post-window arbitrage harder to hide.
