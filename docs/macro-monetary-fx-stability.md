# Macro, Monetary, Inflation, And FX Stability

Status: macro-stability control model. This is not a central-bank rulebook,
exchange-rate forecast, monetary-policy recommendation, debt-sustainability
analysis, IMF program, or official Central Bank of Iraq position.

The Cylinder Seal model can be fiscally solvent and still fail if it pushes too
much local liquidity into an economy that cannot absorb it. Dividends, civic
wages, project payrolls, bank credit, procurement payments, and Digital IQD
settlement must be phased against inflation, food prices, exchange-rate
pressure, reserve cover, domestic supply growth, import leakage, and central
bank independence.

## Core Rule

```text
No dividend growth, local-currency project surge, bank-credit expansion, or
Digital IQD liquidity injection should scale unless inflation, FX, reserves,
credit growth, domestic absorption, import leakage, and CBI governance gates
pass.
```

Fiscal stress asks whether the public sector and holding company can afford the
program. Macro stability asks whether the whole economy can absorb it without
turning productive policy into inflation, import leakage, or exchange-rate
pressure.

## What This Layer Controls

| Risk area | Control requirement |
| --- | --- |
| Headline and core inflation | Broad distributions and project spending slow when inflation rises above tolerance. |
| Food inflation | Broad dividends do not grow into food-price pressure; food logistics and targeted support take priority. |
| FX premium | Market exchange-rate pressure triggers FX source tagging, project phasing, and intervention transparency. |
| Reserve cover | Foreign-currency commitments are phased when import cover is thin. |
| Liquidity injection | Digital IQD issuance, dividends, civic wages, and local project spending are tested against nominal GDP. |
| Sterilization capacity | Treasury deposit buffers and monetary instruments must offset large injections. |
| Credit growth | Bank lending and broad money growth stay below overheating thresholds. |
| Domestic absorption | Domestic supply growth and non-oil FX receipts must rise before demand is pushed too hard. |
| Import leakage | Spending that leaks into imports is not treated as a domestic productivity loop. |
| Non-oil FX cover | Tourism, exports, services, and diaspora channels must create real FX receipts before FX-sensitive expansion. |
| Distribution phasing | Monthly and quarterly distribution calendars prevent political lump-sum shocks. |
| Policy coordination | Fiscal authorities, INDHC, Treasury, and CBI need a rule-based coordination and veto path. |
| CBI independence | The model cannot use Digital IQD to pressure the central bank into financing fiscal promises. |
| FX transparency | Auction, intervention, allocation, and source-tagging rules need audit visibility. |

## Operating Modes

| Mode | Meaning | Required response |
| --- | --- | --- |
| Stable | Inflation, FX, reserves, credit, and liquidity gates are inside tolerance. | Proceed with routine monitoring. |
| Watch | Risks are rising but not yet destabilizing. | Slow optional distributions and publish weekly/monthly macro dashboard. |
| Tighten liquidity | Money, credit, or macro-risk scores are high. | Increase sterilization, tighten credit, slow project drawdowns, and protect reserves. |
| Pause distributions | Inflation, FX premium, or liquidity injection breaches distribution tolerance. | Pause dividend growth and non-critical broad transfers. |
| Stop scale-up | Severe inflation, food inflation, FX premium, reserve-cover weakness, or unsterilized injection appears. | Stop new scale-up until macro conditions recover. |

## Why This Is Separate From Fiscal Stress

Fiscal stress can say:

```text
The project can pay debt.
The dividend is funded by audited surplus.
The reserve and maintenance rules pass.
```

Macro stability can still say:

```text
Do not release this money yet.
The economy cannot absorb the liquidity without inflation or FX leakage.
Phase the spending, sterilize the injection, or wait for domestic supply and
non-oil FX receipts to catch up.
```

Both gates are required. Fiscal solvency is not the same as price stability.

## Use In The Unified Model

This layer should run before:

- monthly citizen dividend growth;
- large Digital IQD liquidity injections;
- civic wage expansion;
- project payroll and procurement surges;
- bank-credit expansion programs;
- domestic bond or sukuk placement that might crowd out credit;
- FX-sensitive imports for rail, energy, water, electronics, HVAC, or defense;
- public announcements that imply guaranteed dividend growth.

## Software And Data Surface

The executable implementation is:

- `crates/cs-analytics/src/macro_stability.rs`
- `migrations/20260719000001_macro_stability.sql`

Core objects:

| Object | Purpose |
| --- | --- |
| `MacroStabilityInput` | Captures nominal GDP, inflation, food inflation, FX premium, reserves, import cover, FX demand, non-oil FX receipts, broad money, credit growth, bank liquidity, domestic supply, import leakage, Digital IQD injections, dividends, civic wages, project local spend, sterilization, treasury buffers, distribution phasing, CBI independence, policy coordination, and FX transparency. |
| `MacroStabilityAssessment` | Scores unsterilized liquidity, inflation pressure, FX pressure, credit heat, absorption capacity, macro risk, recommended mode, and required actions. |
| `MacroStabilityGateResult` | Records pass/warn/fail states for inflation, food inflation, FX premium, reserves, liquidity, sterilization, credit, domestic absorption, import leakage, non-oil FX cover, distribution phasing, policy coordination, CBI independence, and FX transparency. |

## Dashboard Requirements

The macro dashboard should show:

- headline, core, and food inflation;
- market FX premium;
- reserves and import-cover months;
- import bill, FX demand, and non-oil FX receipts;
- broad money and private credit growth;
- bank liquidity and loan/deposit ratio;
- domestic supply growth and import leakage;
- Digital IQD net injection, dividends, civic wages, and local project spend;
- sterilization capacity and treasury deposit buffer;
- unsterilized liquidity as a share of nominal GDP;
- recommended mode and required actions.

## Governance Boundary

This layer should be conservative:

```text
If inflation is high, slow injections.
If food inflation is high, protect supply before broad dividends.
If the FX premium widens, phase FX demand and publish source tags.
If import cover is weak, protect reserves.
If unsterilized liquidity is large, sterilize or delay distribution.
If CBI independence or policy coordination is missing, do not scale.
```

## Bottom Line

The model should not turn oil reform into a demand shock. A citizen dividend is
valuable only if its purchasing power survives. Digital IQD visibility helps the
state see liquidity and prices earlier, but it does not repeal monetary
constraints.
