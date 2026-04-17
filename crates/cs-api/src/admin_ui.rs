//! Server-rendered admin UI (HTMX) — scaffold.
//!
//! Why HTMX over a JS SPA?
//!
//! - The compliance dashboard is read-mostly with low-volume mutations
//!   (approve / reject / file SAR). It does not need client-side state
//!   management or offline-first behaviour.
//! - A server-rendered surface keeps the trust boundary at the same
//!   process that already enforces RBAC; the API and UI share the
//!   `require_admin` middleware, including the audit log.
//! - No JS toolchain in CI, no node_modules — the deploy artefact stays
//!   the single Rust binary.
//!
//! This module is a deliberate **scaffold**: a working login and
//! dashboard page, plus the conventions for adding more pages. Full
//! coverage (rule proposals, UBO management, etc.) is left as
//! follow-up. Templates here are inline string concatenation rather
//! than askama/maud — same reason as above, the scope is tiny.

use axum::extract::{Path, State};
use axum::http::header::CONTENT_TYPE;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Extension;
use uuid::Uuid;

use crate::beneficial_owners::{list_owners, BeneficialOwnerState};
use crate::compliance::{dashboard, ComplianceState, DashboardResponse};
use crate::middleware::AdminPrincipal;
use crate::rule_governance::{list_pending, RuleGovernanceState};
use crate::travel_rule::{get_payload, TravelRuleState};

const BASE_HEAD: &str = r#"<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>CylinderSeal Admin</title>
<script src="https://unpkg.com/htmx.org@1.9.12"></script>
<style>
  body { font: 14px/1.5 system-ui, sans-serif; max-width: 1100px; margin: 2rem auto; padding: 0 1rem; color: #222; }
  header { display: flex; justify-content: space-between; border-bottom: 1px solid #ccc; padding-bottom: .5rem; margin-bottom: 1.5rem; }
  h1 { margin: 0; font-size: 1.25rem; }
  .grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(180px, 1fr)); gap: 1rem; margin-bottom: 1.5rem; }
  .card { border: 1px solid #ddd; border-radius: 6px; padding: .75rem 1rem; }
  .card h2 { margin: 0 0 .25rem; font-size: .85rem; color: #666; font-weight: 500; text-transform: uppercase; letter-spacing: .03em; }
  .card .v { font-size: 1.5rem; font-weight: 600; }
  table { width: 100%; border-collapse: collapse; }
  th, td { text-align: left; padding: .35rem .5rem; border-bottom: 1px solid #eee; }
  th { color: #666; font-weight: 500; }
  .ok { color: #1a7f37; }
  .warn { color: #b08800; }
  .err { color: #cf222e; }
</style>
</head>"#;

/// `GET /admin/` — landing page. Renders dashboard JSON server-side.
pub async fn index(
    Extension(actor): Extension<AdminPrincipal>,
    State(state): State<ComplianceState>,
) -> Response {
    let body = match dashboard(State(state)).await {
        Ok(axum::Json(d)) => render_dashboard(&actor, &d),
        Err((status, msg)) => {
            return error_page(status, &msg);
        }
    };
    html(body)
}

/// `GET /admin/rules/proposals` — list pending rule proposals with
/// approve / reject controls. Approve+reject buttons hit the JSON API
/// (which already enforces four-eyes + supervisor role) via HTMX.
pub async fn rule_proposals_page(
    Extension(actor): Extension<AdminPrincipal>,
    State(state): State<RuleGovernanceState>,
) -> Response {
    let rows = match list_pending(State(state)).await {
        Ok(axum::Json(rows)) => rows,
        Err((status, msg)) => return error_page(status, &msg),
    };
    let table = if rows.is_empty() {
        "<p>No pending proposals.</p>".to_string()
    } else {
        let body_rows = rows
            .iter()
            .map(|r| {
                format!(
                    r#"<tr>
                       <td>{code}</td><td>v{ver}</td><td>{name}</td>
                       <td>{cat}</td><td>{sev}</td><td>{act}</td>
                       <td>
                         <button hx-post="/v1/governance/rules/proposals/{id}/approve"
                                 hx-ext="json-enc" hx-vals='{{}}'
                                 hx-target="closest tr" hx-swap="outerHTML"
                                 hx-confirm="Approve this rule version?">Approve</button>
                         <button hx-post="/v1/governance/rules/proposals/{id}/reject"
                                 hx-ext="json-enc"
                                 hx-vals='{{"reason": "rejected via admin UI"}}'
                                 hx-target="closest tr" hx-swap="outerHTML"
                                 hx-confirm="Reject this rule version?">Reject</button>
                       </td></tr>"#,
                    id = r.version_id,
                    code = r.rule_code,
                    ver = r.version,
                    name = r.name,
                    cat = r.category,
                    sev = r.severity,
                    act = r.action,
                )
            })
            .collect::<Vec<_>>()
            .join("");
        format!(
            r#"<table>
              <tr><th>Rule</th><th>Ver</th><th>Name</th><th>Category</th>
                  <th>Severity</th><th>Action</th><th></th></tr>
              {body_rows}
            </table>"#,
        )
    };
    let body = format!(
        r#"{head}
        <body>
          {nav}
          <header><h1>Rule proposals (pending)</h1>
            <div>{user} ({role})</div></header>
          {table}
        </body></html>"#,
        head = BASE_HEAD,
        nav = nav(),
        user = actor.username,
        role = actor.role,
    );
    html(body)
}

/// `GET /admin/businesses/:user_id/owners` — list beneficial owners for a
/// business. Verify button hits the JSON API.
pub async fn ubo_page(
    Extension(actor): Extension<AdminPrincipal>,
    Path(business_user_id): Path<Uuid>,
    State(state): State<BeneficialOwnerState>,
) -> Response {
    let rows = match list_owners(State(state), Path(business_user_id)).await {
        Ok(axum::Json(rows)) => rows,
        Err((status, msg)) => return error_page(status, &msg),
    };
    let total: rust_decimal::Decimal = rows.iter().map(|o| o.ownership_pct).sum();
    let threshold_class = if total >= rust_decimal::Decimal::from(75) {
        "ok"
    } else {
        "warn"
    };
    let table_rows = rows
        .iter()
        .map(|o| {
            let pep_badge = if o.is_pep {
                r#"<span class="warn">PEP</span>"#
            } else {
                ""
            };
            let verified_badge = if o.verified {
                r#"<span class="ok">verified</span>"#.to_string()
            } else {
                format!(
                    r#"<button hx-post="/v1/businesses/{biz}/beneficial-owners/{oid}/verify"
                              hx-target="closest td" hx-swap="innerHTML"
                              hx-confirm="Mark this owner verified?">Verify</button>"#,
                    biz = business_user_id,
                    oid = o.owner_id,
                )
            };
            format!(
                r#"<tr>
                   <td>{name}</td><td>{nat}</td><td>{pct}%</td>
                   <td>{ctrl}</td><td>{pep}</td><td>{verified}</td>
                   </tr>"#,
                name = o.full_name,
                nat = o.nationality,
                pct = o.ownership_pct,
                ctrl = o.control_type,
                pep = pep_badge,
                verified = verified_badge,
            )
        })
        .collect::<Vec<_>>()
        .join("");
    let body = format!(
        r#"{head}
        <body>
          {nav}
          <header><h1>Beneficial owners</h1>
            <div>{user} ({role})</div></header>
          <p>Business: <code>{biz}</code></p>
          <p>Total disclosed: <span class="{cls}">{total}%</span>
             (threshold: 75%)</p>
          <table>
            <tr><th>Name</th><th>Nationality</th><th>Ownership</th>
                <th>Control</th><th>PEP</th><th>Verified</th></tr>
            {table_rows}
          </table>
        </body></html>"#,
        head = BASE_HEAD,
        nav = nav(),
        user = actor.username,
        role = actor.role,
        biz = business_user_id,
        total = total,
        cls = threshold_class,
    );
    html(body)
}

/// `GET /admin/travel-rule/:tx_id` — view the FATF Rec 16 originator /
/// beneficiary payload for a single transaction.
pub async fn travel_rule_page(
    Extension(actor): Extension<AdminPrincipal>,
    Path(tx_id): Path<Uuid>,
    State(state): State<TravelRuleState>,
) -> Response {
    let payload = match get_payload(State(state), Path(tx_id)).await {
        Ok(axum::Json(p)) => p,
        Err((status, msg)) => return error_page(status, &msg),
    };
    let body = format!(
        r#"{head}
        <body>
          {nav}
          <header><h1>Travel Rule payload</h1>
            <div>{user} ({role})</div></header>
          <p>Transaction: <code>{tx}</code></p>
          <table>
            <tr><th>Originator name</th><td>{on}</td></tr>
            <tr><th>Originator country</th><td>{oc}</td></tr>
            <tr><th>Beneficiary name</th><td>{bn}</td></tr>
            <tr><th>Beneficiary country</th><td>{bc}</td></tr>
            <tr><th>VASP originator</th><td>{vo}</td></tr>
            <tr><th>VASP beneficiary</th><td>{vb}</td></tr>
            <tr><th>Amount (μOWC)</th><td>{amt}</td></tr>
            <tr><th>Currency</th><td>{ccy}</td></tr>
            <tr><th>Purpose code</th><td>{purpose}</td></tr>
          </table>
        </body></html>"#,
        head = BASE_HEAD,
        nav = nav(),
        user = actor.username,
        role = actor.role,
        tx = payload.transaction_id,
        on = payload.originator_name,
        oc = payload.originator_country,
        bn = payload.beneficiary_name,
        bc = payload.beneficiary_country,
        vo = payload.vasp_originator,
        vb = payload.vasp_beneficiary,
        amt = payload.amount_micro_owc,
        ccy = payload.currency,
        purpose = payload.purpose_code.unwrap_or_else(|| "—".into()),
    );
    html(body)
}

fn nav() -> &'static str {
    r#"<nav style="margin-bottom:1rem;">
       <a href="/admin/">Dashboard</a> &middot;
       <a href="/admin/rules/proposals">Rule proposals</a>
       </nav>"#
}

/// `GET /admin/login` — login form. Posts to `/v1/admin/auth/login`
/// via HTMX; on success the cookie is set and we redirect to /admin/.
pub async fn login_page() -> Response {
    let body = format!(
        r##"{head}
        <body>
          <header><h1>CylinderSeal Admin</h1></header>
          <form hx-post="/v1/admin/auth/login"
                hx-ext="json-enc"
                hx-target="#err"
                hx-on::after-request="if(event.detail.successful) window.location='/admin/'">
            <p><label>Username <input name="username" required></label></p>
            <p><label>Password <input name="password" type="password" required></label></p>
            <button type="submit">Sign in</button>
            <p id="err" class="err"></p>
          </form>
        </body></html>"##,
        head = BASE_HEAD,
    );
    html(body)
}

fn render_dashboard(actor: &AdminPrincipal, d: &DashboardResponse) -> String {
    let rules_table = d
        .top_triggered_rules
        .iter()
        .map(|r| format!("<tr><td>{}</td><td>{}</td></tr>", r.rule_code, r.hit_count))
        .collect::<Vec<_>>()
        .join("");
    let feeds_table = d
        .feeds
        .iter()
        .map(|f| {
            let cls = match f.status.as_str() {
                "ok" => "ok",
                "running" => "warn",
                _ => "err",
            };
            format!(
                r#"<tr><td>{}</td><td class="{}">{}</td><td>{}</td><td>{}</td></tr>"#,
                f.feed_name,
                cls,
                f.status,
                f.records_added,
                f.error.clone().unwrap_or_default()
            )
        })
        .collect::<Vec<_>>()
        .join("");
    format!(
        r#"{head}
        <body>
          <header>
            <h1>Compliance dashboard</h1>
            <div>{user} ({role}) <a href="/admin/login">sign out</a></div>
          </header>

          <h2 style="font-size: 1rem; margin: 1rem 0 .5rem;">Reports</h2>
          <div class="grid">
            <div class="card"><h2>SAR draft</h2><div class="v">{sar_d}</div></div>
            <div class="card"><h2>SAR review</h2><div class="v">{sar_r}</div></div>
            <div class="card"><h2>SAR filed</h2><div class="v">{sar_f}</div></div>
            <div class="card"><h2>STR draft</h2><div class="v">{str_d}</div></div>
            <div class="card"><h2>CTR filed</h2><div class="v">{ctr_f}</div></div>
            <div class="card"><h2>EDD active</h2><div class="v">{edd}</div></div>
          </div>

          <h2 style="font-size: 1rem; margin: 1rem 0 .5rem;">Risk distribution (users)</h2>
          <div class="grid">
            <div class="card"><h2>Low</h2><div class="v">{low}</div></div>
            <div class="card"><h2>Med-low</h2><div class="v">{ml}</div></div>
            <div class="card"><h2>Medium</h2><div class="v">{med}</div></div>
            <div class="card"><h2>High</h2><div class="v">{hi}</div></div>
            <div class="card"><h2>Critical</h2><div class="v err">{crit}</div></div>
          </div>

          <h2 style="font-size: 1rem; margin: 1rem 0 .5rem;">Top triggered rules (30d)</h2>
          <table>
            <tr><th>Rule</th><th>Hits</th></tr>
            {rules_table}
          </table>

          <h2 style="font-size: 1rem; margin: 1rem 0 .5rem;">Feed health</h2>
          <table>
            <tr><th>Feed</th><th>Status</th><th>Entries</th><th>Error</th></tr>
            {feeds_table}
          </table>

          <p style="margin-top:2rem; color:#888;">
            CBI policy rate: {cbi}% &middot; IQD/USD: {iqd}
          </p>
        </body></html>"#,
        head = BASE_HEAD,
        user = actor.username,
        role = actor.role,
        sar_d = d.report_counts.sar_draft,
        sar_r = d.report_counts.sar_review,
        sar_f = d.report_counts.sar_filed,
        str_d = d.report_counts.str_draft,
        ctr_f = d.report_counts.ctr_filed,
        edd = d.report_counts.edd_active,
        low = d.risk_distribution.low,
        ml = d.risk_distribution.medium_low,
        med = d.risk_distribution.medium,
        hi = d.risk_distribution.high,
        crit = d.risk_distribution.critical,
        rules_table = rules_table,
        feeds_table = feeds_table,
        cbi = d.cbi_policy_rate,
        iqd = d.iqd_usd_rate,
    )
}

fn html(body: String) -> Response {
    (
        StatusCode::OK,
        [(CONTENT_TYPE, "text/html; charset=utf-8")],
        body,
    )
        .into_response()
}

fn error_page(status: StatusCode, msg: &str) -> Response {
    let body = format!(
        r#"{head}<body><header><h1>Error</h1></header><p class="err">{}</p></body></html>"#,
        msg,
        head = BASE_HEAD,
    );
    (
        status,
        [(CONTENT_TYPE, "text/html; charset=utf-8")],
        body,
    )
        .into_response()
}
