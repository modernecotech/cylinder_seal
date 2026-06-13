# Fiscal Stress And Contingent Liability Model

Status: downside-control model. This is not a sovereign-debt recommendation,
budget forecast, financing offer, rating view, or investment prospectus.

This document adds the missing pessimistic logic to Cylinder Seal. The unified
economic model is rational only if it keeps working when oil revenue falls,
collections disappoint, debt costs rise, FX exposure opens, capex overruns, or
project guarantees become hidden fiscal claims.

## Source Discipline

| Source | Use in this model |
| --- | --- |
| [IMF Iraq 2025 Article IV](https://www.imf.org/en/news/articles/2025/07/08/pr-25243-iraq-imf-executive-board-concludes-2025-article-iv-consultation) | Baseline fiscal pressure, deficit, debt, reserve, public-finance, and consolidation discipline. |
| [Iraq Quantified Affordability And Cashflow Model](iraq-quantified-affordability-model.md) | Defines the fiscal-safe, constrained-base, and strategic-upper envelopes and the existing stress-test table. |
| [National Economic Operating Logic](national-economic-operating-logic.md) | Defines the capital, productive asset, booked cash, public benefit, distribution, and risk ledgers. |
| [Project Pipeline And Investment Gates](project-pipeline-and-investment-gates.md) | Defines DSCR, FX, guarantees, project-stage gates, stop conditions, and bankability package requirements. |

These sources support the control framework. They do not validate the values
used in any scenario.

## Core Rule

```text
When stress appears:
  protect maintenance,
  protect debt service,
  protect essential services,
  protect legal reserves,
  disclose contingent liabilities,
  suspend dividends,
  stop scale-up.
```

The model should never solve a fiscal problem by pretending that project debt,
availability payments, or guarantees are not state exposure.

## Stress Inputs

| Input | Why it matters |
| --- | --- |
| GDP, oil revenue, public capex, deficit, debt, and reserves | Shows whether the sovereign balance sheet can support oil-equity allocation. |
| Oil-equity draw | Tests the rule that oil equity must fit inside stressed oil revenue and public-capex limits. |
| New project debt and debt service | Tests whether projects can service debt after shocks. |
| Foreign-currency debt service, FX revenue, and approved buffer | Exposes currency mismatch before it becomes a bailout claim. |
| Operating cash after maintenance | Keeps debt and dividends tied to cash, not benefit estimates. |
| Maintenance reserve required and funded | Prevents asset stripping and premature distributions. |
| Gross-profit levy and retained earnings | Senior claims before dividends. |
| Dividend pool | Residual distribution that must fail first under stress. |
| Government guarantees and availability payments | Contingent liabilities that can migrate back onto Treasury. |
| Collection efficiency | Distinguishes invoiced revenue from settled cash. |
| Capex overrun, oil shock, revenue shortfall, interest shock, FX shock, and delay | Makes downside assumptions explicit and testable. |

## Fiscal Stress Modes

| Mode | Meaning | Required posture |
| --- | --- | --- |
| Stable | Stress gates pass and dividends remain affordable after senior claims. | Continue only within normal gates. |
| Watch | Debt, collections, contingent liabilities, debt/GDP, or delays are weakening. | Slow new commitments and tighten monitoring. |
| Defensive | DSCR, deficit, overruns, or contingent liabilities are materially stressed. | Freeze non-critical capex; protect water, food, power, maintenance, and debt service. |
| Stop scale-up | Oil-equity rule, DSCR, maintenance, FX, collections, or dividend affordability fails. | Stop new scale-up, suspend dividends, restructure or cancel weak projects. |

## Gate Logic

| Gate | Pass | Warn | Fail |
| --- | --- | --- | --- |
| Oil-equity fiscal rule | Draw fits the stressed cap. | N/A | Draw exceeds the stressed cap. |
| Debt-service cover | Stressed DSCR at or above 1.30. | 1.10 to 1.30. | Below 1.10. |
| FX cover | FX debt service is covered by FX revenue or approved buffer. | Mismatch is limited. | Mismatch exceeds tolerance. |
| Maintenance coverage | Maintenance reserve is fully funded. | N/A | Maintenance reserve has a funding gap. |
| Contingent liability | Guarantees and availability payments below 2% of GDP. | 2-5% of GDP. | Above 5% of GDP. |
| Collection efficiency | At least 85%. | 70-85%. | Below 70%. |
| Capex overrun | At or below 10%. | 10-20%. | Above 20%. |
| Dividend affordability | Dividend is zero or fully covered after senior claims. | N/A | Dividend is not covered after senior claims. |

## Dividend Suspension Rule

Dividends are not a political promise. They are a residual cash distribution.

Under stress, dividends are suspended before:

- maintenance reserves;
- debt service;
- FX buffers;
- gross-profit levy or lawful tax obligations;
- retained earnings required for asset renewal;
- essential public-service continuity;
- cybersecurity, safety, audit, and legal compliance.

This prevents the citizen dividend from becoming a disguised asset-stripping
mechanism.

## Contingent Liability Rule

Guarantees and availability payments are not free because they are not paid
today. They must be shown as fiscal exposure.

| Exposure | Required treatment |
| --- | --- |
| Sovereign guarantee | Explicit, capped, budgeted, disclosed, and legally approved. |
| Project-company guarantee | Counted against the contingent-liability dashboard even if not called. |
| Availability payment | Treated as a future public payment obligation. |
| Minimum revenue guarantee | Counted as public exposure unless private demand risk is real. |
| FX support | Counted as reserve or fiscal exposure. |
| Political rescue expectation | Flagged as implicit guarantee risk even without signed documents. |

## Software And Data Surface

The executable implementation is:

- `crates/cs-analytics/src/fiscal_stress.rs`
- `migrations/20260713000001_fiscal_stress_projection.sql`

Core objects:

| Object | Purpose |
| --- | --- |
| `FiscalStressInput` | Captures macro baseline, oil-equity draw, project debt, cashflow, FX, maintenance, dividends, guarantees, collections, overruns, and shocks. |
| `FiscalStressProjection` | Computes stressed oil revenue, fiscal-rule breach, DSCR, FX mismatch, maintenance gap, contingent liability, dividend gap, and recommended mode. |
| `FiscalStressGateResult` | Records pass/warn/fail state for fiscal, debt, FX, maintenance, contingent-liability, collection, capex-overrun, and dividend gates. |

## Dashboard Implications

The public dashboard should show:

- stressed oil-equity capacity;
- oil-equity breach amount;
- stressed DSCR;
- FX mismatch;
- maintenance gap;
- guarantees and availability payments as percent of GDP;
- collection efficiency;
- capex overrun;
- dividend affordability gap;
- recommended mode.

The dashboard should not show a project, ministry transition, dividend formula,
or INDHC expansion as scalable when the fiscal stress mode is defensive or stop
scale-up.

## Bottom Line

The model becomes more credible when it can say no.

```text
If stress breaks the cashflow, stop scale-up.
If guarantees become hidden debt, disclose and cap them.
If maintenance is underfunded, block distributions.
If FX revenue cannot cover FX debt, do not borrow in FX.
If collections are weak, do not count invoices as cash.
If dividends compete with solvency, dividends lose.
```
