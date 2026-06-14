//! HTML page route handlers

use crate::state::AppState;
use axum::{extract::State, response::Html};
use sqlx::Row;
use std::sync::Arc;

const HTML_HEADER: &str = r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>CBI Dashboard</title>
    <script src="https://cdn.tailwindcss.com"></script>
    <script>
        // Check auth on page load
        if (!sessionStorage.getItem('cbi_token')) {
            window.location.href = '/login';
        }

        // Add token to all fetch requests
        const originalFetch = window.fetch;
        window.fetch = function(input, init = {}) {
            const token = sessionStorage.getItem('cbi_token');
            if (token) {
                const headers = new Headers(init.headers || {});
                headers.set('Authorization', `Bearer ${token}`);
                headers.set('X-CSRF-Token', token);
                init = {...init, headers};
            }
            return originalFetch.call(this, input, init);
        };
    </script>
</head>
<body class="bg-gray-50">
<nav class="bg-blue-900 text-white p-4 mb-6">
    <div class="max-w-7xl mx-auto flex justify-between items-center">
        <h1 class="text-2xl font-bold">CBI Dashboard</h1>
        <div class="space-x-4">
            <a href="/overview" class="hover:text-blue-200">Overview</a>
            <a href="/projects" class="hover:text-blue-200">Projects</a>
            <a href="/accounts" class="hover:text-blue-200">Accounts</a>
            <a href="/analytics" class="hover:text-blue-200">Analytics</a>
            <a href="/monetary" class="hover:text-blue-200">Monetary</a>
            <a href="/compliance" class="hover:text-blue-200">Compliance</a>
            <button onclick="logout()" class="hover:text-blue-200">Logout</button>
        </div>
    </div>
</nav>
<main class="max-w-7xl mx-auto px-4 py-6">
<script>
async function logout() {
    const token = sessionStorage.getItem('cbi_token');
    if (token) {
        try {
            await fetch('/auth/logout', {method: 'POST'});
        } catch (_) {}
    }
    sessionStorage.removeItem('cbi_token');
    sessionStorage.removeItem('cbi_username');
    window.location.href = '/login';
}
</script>"#;

const HTML_FOOTER: &str = "</main></body></html>";

pub async fn root_redirect() -> axum::response::Redirect {
    axum::response::Redirect::permanent("/overview")
}

pub async fn login_page() -> Html<&'static str> {
    Html(include_str!("../../templates/login.html"))
}

pub async fn overview_page(State(app_state): State<Arc<AppState>>) -> Html<String> {
    let user_count =
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM users WHERE balance_owc > 0")
            .fetch_one(&app_state.db_pool)
            .await
            .unwrap_or(0);

    let gdp = user_count as f64 * 5500.0 / 1_000_000_000.0;

    let project_count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM industrial_projects")
        .fetch_one(&app_state.db_pool)
        .await
        .unwrap_or(0);

    let report_count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM regulatory_reports WHERE status IN ('Draft', 'UnderReview')",
    )
    .fetch_one(&app_state.db_pool)
    .await
    .unwrap_or(0);

    let html = format!(
        r#"{}<h1 class="text-3xl font-bold mb-6">Economic Command Center</h1>
        <div class="grid grid-cols-1 md:grid-cols-4 gap-4 mb-8">
            <div class="bg-white p-6 rounded-lg shadow">
                <h3 class="text-gray-600 text-sm font-medium">GDP Estimate</h3>
                <p class="text-3xl font-bold text-blue-900 mt-2">${:.2}B</p>
            </div>
            <div class="bg-white p-6 rounded-lg shadow">
                <h3 class="text-gray-600 text-sm font-medium">Active Users</h3>
                <p class="text-3xl font-bold text-green-600 mt-2">{}</p>
            </div>
            <div class="bg-white p-6 rounded-lg shadow">
                <h3 class="text-gray-600 text-sm font-medium">Industrial Projects</h3>
                <p class="text-3xl font-bold text-orange-600 mt-2">{}</p>
            </div>
            <div class="bg-white p-6 rounded-lg shadow">
                <h3 class="text-gray-600 text-sm font-medium">Pending Reports</h3>
                <p class="text-3xl font-bold text-red-600 mt-2">{}</p>
            </div>
        </div>{}"#,
        HTML_HEADER, gdp, user_count, project_count, report_count, HTML_FOOTER
    );
    Html(html)
}

pub async fn monetary_page() -> Html<String> {
    let mut html = String::from(HTML_HEADER);
    html.push_str(
        r#"
<div class="flex items-center justify-between mb-6">
    <h1 class="text-3xl font-bold text-gray-900">Broad Money & Civic Wage Envelope</h1>
    <span id="policy-status" class="px-3 py-1 rounded bg-gray-200 text-gray-800 text-sm font-semibold">Loading</span>
</div>

<div class="grid grid-cols-1 md:grid-cols-4 gap-4 mb-8">
    <div class="bg-white p-5 rounded-lg shadow">
        <h3 class="text-gray-600 text-sm font-medium">Current M2</h3>
        <p id="current-m2" class="text-2xl font-bold text-blue-900 mt-2">0 IQD</p>
        <p id="m2-period" class="text-xs text-gray-500 mt-1"></p>
    </div>
    <div class="bg-white p-5 rounded-lg shadow">
        <h3 class="text-gray-600 text-sm font-medium">Broad-Money Headroom</h3>
        <p id="headroom" class="text-2xl font-bold text-green-700 mt-2">0 IQD</p>
    </div>
    <div class="bg-white p-5 rounded-lg shadow">
        <h3 class="text-gray-600 text-sm font-medium">Civic Wage Budget</h3>
        <p id="wage-budget" class="text-2xl font-bold text-indigo-800 mt-2">0 IQD</p>
    </div>
    <div class="bg-white p-5 rounded-lg shadow">
        <h3 class="text-gray-600 text-sm font-medium">Non-USD Coverage</h3>
        <p id="non-usd-coverage" class="text-2xl font-bold text-slate-800 mt-2">0%</p>
    </div>
</div>

<div class="grid grid-cols-1 xl:grid-cols-3 gap-6">
    <section class="xl:col-span-2 bg-white rounded-lg shadow p-6">
        <h2 class="text-xl font-bold text-gray-900 mb-4">Set Active Envelope</h2>
        <form id="budget-form" class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <label class="block">
                <span class="text-sm font-medium text-gray-700">Period</span>
                <input id="period-code" name="period_code" class="mt-1 w-full border rounded px-3 py-2" value="2026-Q4" required>
            </label>
            <label class="block">
                <span class="text-sm font-medium text-gray-700">Broad Money Ceiling IQD</span>
                <input id="broad-money-ceiling" name="broad_money_ceiling_iqd" type="number" min="0" step="1000" class="mt-1 w-full border rounded px-3 py-2" required>
            </label>
            <label class="block">
                <span class="text-sm font-medium text-gray-700">Civic Worker Budget IQD</span>
                <input id="civic-worker-budget" name="civic_worker_budget_iqd" type="number" min="0" step="1000" class="mt-1 w-full border rounded px-3 py-2" required>
            </label>
            <label class="block">
                <span class="text-sm font-medium text-gray-700">Non-USD Floor %</span>
                <input id="non-usd-floor" name="non_usd_origin_floor_pct" type="number" min="0" max="100" step="0.01" class="mt-1 w-full border rounded px-3 py-2" value="100" required>
            </label>
            <label class="block">
                <span class="text-sm font-medium text-gray-700">Non-USD Allocation IQD</span>
                <input id="non-usd-allocation" name="non_usd_origin_allocated_iqd" type="number" min="0" step="1000" class="mt-1 w-full border rounded px-3 py-2" required>
            </label>
            <label class="block">
                <span class="text-sm font-medium text-gray-700">Planned Workers</span>
                <input id="planned-worker-count" name="planned_worker_count" type="number" min="0" step="1" class="mt-1 w-full border rounded px-3 py-2" value="0" required>
            </label>
            <label class="block">
                <span class="text-sm font-medium text-gray-700">Average Monthly Wage IQD</span>
                <input id="average-monthly-wage" name="average_monthly_wage_iqd" type="number" min="0" step="1000" class="mt-1 w-full border rounded px-3 py-2" value="0" required>
            </label>
            <label class="block md:col-span-2">
                <span class="text-sm font-medium text-gray-700">Notes</span>
                <textarea id="budget-notes" name="notes" class="mt-1 w-full border rounded px-3 py-2" rows="3"></textarea>
            </label>
            <div class="md:col-span-2 flex items-center justify-between gap-4">
                <p id="form-message" class="text-sm text-gray-600"></p>
                <button type="submit" class="bg-blue-900 text-white px-5 py-2 rounded hover:bg-blue-800 font-semibold">Activate</button>
            </div>
        </form>
    </section>

    <aside class="bg-white rounded-lg shadow p-6">
        <h2 class="text-xl font-bold text-gray-900 mb-4">Civic Work Readiness</h2>
        <dl class="space-y-4">
            <div class="flex justify-between border-b pb-2">
                <dt class="text-gray-600">Assessed Programs</dt>
                <dd id="assessed-programs" class="font-bold">0</dd>
            </div>
            <div class="flex justify-between border-b pb-2">
                <dt class="text-gray-600">Eligible Programs</dt>
                <dd id="eligible-programs" class="font-bold">0</dd>
            </div>
            <div class="flex justify-between border-b pb-2">
                <dt class="text-gray-600">Payable Hours</dt>
                <dd id="payable-hours" class="font-bold">0</dd>
            </div>
            <div class="flex justify-between">
                <dt class="text-gray-600">Held Hours</dt>
                <dd id="held-hours" class="font-bold">0</dd>
            </div>
        </dl>
        <form id="payroll-form" class="mt-6 border-t pt-5 space-y-3">
            <h3 class="text-sm font-bold text-gray-900">Draft Payroll Batch</h3>
            <label class="block">
                <span class="text-sm font-medium text-gray-700">Hourly Wage IQD</span>
                <input id="payroll-hourly-wage" type="number" min="1" step="100" class="mt-1 w-full border rounded px-3 py-2" value="0" required>
            </label>
            <label class="block">
                <span class="text-sm font-medium text-gray-700">Batch Notes</span>
                <textarea id="payroll-notes" class="mt-1 w-full border rounded px-3 py-2" rows="2"></textarea>
            </label>
            <button type="submit" class="w-full bg-slate-800 text-white px-4 py-2 rounded hover:bg-slate-700 font-semibold">Draft Batch</button>
            <p id="payroll-message" class="text-sm text-gray-600"></p>
        </form>
        <div class="mt-6 border-t pt-5">
            <h3 class="text-sm font-bold text-gray-900 mb-2">Recent Batches</h3>
            <div id="recent-payroll-batches" class="space-y-2 text-sm text-gray-700"></div>
        </div>
    </aside>
</div>

<script>
const formatIqd = new Intl.NumberFormat('en-US', { maximumFractionDigits: 0 });
const formatPct = new Intl.NumberFormat('en-US', { maximumFractionDigits: 2 });

function setText(id, value) {
    document.getElementById(id).textContent = value;
}

function numberValue(id) {
    return Number(document.getElementById(id).value || 0);
}

async function loadPolicy() {
    const response = await fetch('/api/monetary/broad-money-budget');
    const data = await response.json();
    const policy = data.latest_policy;

    setText('current-m2', `${formatIqd.format(data.monetary_snapshot.current_m2_iqd)} IQD`);
    setText('m2-period', data.monetary_snapshot.period || 'No monetary snapshot');
    setText('headroom', `${formatIqd.format(data.broad_money_headroom_iqd)} IQD`);
    setText('non-usd-coverage', `${formatPct.format(data.non_usd_coverage_pct)}%`);
    setText('policy-status', data.policy_binding ? 'Binding' : 'No Active Policy');
    document.getElementById('policy-status').className = data.policy_binding
        ? 'px-3 py-1 rounded bg-green-100 text-green-800 text-sm font-semibold'
        : 'px-3 py-1 rounded bg-gray-200 text-gray-800 text-sm font-semibold';

    setText('assessed-programs', data.civic_work.assessed_programs.toLocaleString());
    setText('eligible-programs', data.civic_work.eligible_programs.toLocaleString());
    setText('payable-hours', formatIqd.format(data.civic_work.payable_hours));
    setText('held-hours', formatIqd.format(data.civic_work.held_hours));

    if (policy) {
        document.getElementById('period-code').value = policy.period_code;
        document.getElementById('broad-money-ceiling').value = Math.round(policy.broad_money_ceiling_iqd);
        document.getElementById('civic-worker-budget').value = Math.round(policy.civic_worker_budget_iqd);
        document.getElementById('non-usd-floor').value = policy.non_usd_origin_floor_pct;
        document.getElementById('non-usd-allocation').value = Math.round(policy.non_usd_origin_allocated_iqd);
        document.getElementById('planned-worker-count').value = policy.planned_worker_count;
        document.getElementById('average-monthly-wage').value = Math.round(policy.average_monthly_wage_iqd);
        document.getElementById('budget-notes').value = policy.notes || '';
        document.getElementById('payroll-hourly-wage').value = Math.max(1, Math.round(policy.average_monthly_wage_iqd / 160));
        setText('wage-budget', `${formatIqd.format(policy.civic_worker_budget_iqd)} IQD`);
    } else {
        document.getElementById('broad-money-ceiling').value = Math.round(data.monetary_snapshot.current_m2_iqd);
        setText('wage-budget', '0 IQD');
    }

    const batches = document.getElementById('recent-payroll-batches');
    batches.innerHTML = '';
    if (!data.recent_payroll_batches.length) {
        batches.textContent = 'No draft batches';
    } else {
        for (const batch of data.recent_payroll_batches) {
            const item = document.createElement('div');
            item.className = 'flex justify-between gap-3 border-b pb-2';
            item.innerHTML = `<span>${batch.period_code} · ${batch.status}</span><strong>${formatIqd.format(batch.batch_amount_iqd)} IQD</strong>`;
            batches.appendChild(item);
        }
    }
}

document.getElementById('budget-form').addEventListener('submit', async (event) => {
    event.preventDefault();
    const payload = {
        period_code: document.getElementById('period-code').value.trim(),
        broad_money_ceiling_iqd: numberValue('broad-money-ceiling'),
        civic_worker_budget_iqd: numberValue('civic-worker-budget'),
        non_usd_origin_floor_pct: numberValue('non-usd-floor'),
        non_usd_origin_allocated_iqd: numberValue('non-usd-allocation'),
        planned_worker_count: Math.trunc(numberValue('planned-worker-count')),
        average_monthly_wage_iqd: numberValue('average-monthly-wage'),
        notes: document.getElementById('budget-notes').value.trim() || null
    };

    const message = document.getElementById('form-message');
    message.textContent = 'Submitting';
    message.className = 'text-sm text-gray-600';

    const response = await fetch('/api/monetary/broad-money-budget', {
        method: 'POST',
        headers: {'Content-Type': 'application/json'},
        body: JSON.stringify(payload)
    });

    if (!response.ok) {
        message.textContent = await response.text();
        message.className = 'text-sm text-red-700';
        return;
    }

    const result = await response.json();
    message.textContent = `Activated ${result.period_code}`;
    message.className = 'text-sm text-green-700';
    await loadPolicy();
});

document.getElementById('payroll-form').addEventListener('submit', async (event) => {
    event.preventDefault();
    const message = document.getElementById('payroll-message');
    const payload = {
        period_code: document.getElementById('period-code').value.trim(),
        hourly_wage_iqd: numberValue('payroll-hourly-wage'),
        notes: document.getElementById('payroll-notes').value.trim() || null
    };

    message.textContent = 'Drafting';
    message.className = 'text-sm text-gray-600';

    const response = await fetch('/api/monetary/civic-payroll-batches', {
        method: 'POST',
        headers: {'Content-Type': 'application/json'},
        body: JSON.stringify(payload)
    });

    if (!response.ok) {
        message.textContent = await response.text();
        message.className = 'text-sm text-red-700';
        return;
    }

    const result = await response.json();
    message.textContent = `Drafted ${formatIqd.format(result.batch_amount_iqd)} IQD`;
    message.className = 'text-sm text-green-700';
    await loadPolicy();
});

loadPolicy().catch(() => {
    setText('policy-status', 'Unavailable');
    document.getElementById('policy-status').className = 'px-3 py-1 rounded bg-red-100 text-red-800 text-sm font-semibold';
});
</script>"#
    );
    html.push_str(HTML_FOOTER);
    Html(html)
}

pub async fn projects_page(State(app_state): State<Arc<AppState>>) -> Html<String> {
    let rows = sqlx::query("SELECT name, sector, status, employment_count, capacity_pct_utilized FROM industrial_projects LIMIT 20")
        .fetch_all(&app_state.db_pool)
        .await
        .unwrap_or_default();

    let mut table = String::from(
        r#"<table class="w-full border-collapse border border-gray-300">
            <thead class="bg-gray-200">
                <tr>
                    <th class="border p-2 text-left">Project Name</th>
                    <th class="border p-2 text-left">Sector</th>
                    <th class="border p-2 text-left">Status</th>
                    <th class="border p-2 text-right">Employment</th>
                    <th class="border p-2 text-right">Capacity %</th>
                </tr>
            </thead>
            <tbody>"#,
    );

    for row in rows {
        let name: String = row.get("name");
        let sector: String = row.get("sector");
        let status: String = row.get("status");
        let employment: i32 = row.get("employment_count");
        let capacity: i32 = row.get("capacity_pct_utilized");

        table.push_str(&format!(
            r#"<tr class="hover:bg-gray-100">
                <td class="border p-2">{}</td>
                <td class="border p-2">{}</td>
                <td class="border p-2"><span class="px-2 py-1 rounded text-white bg-blue-600 text-sm">{}</span></td>
                <td class="border p-2 text-right">{}</td>
                <td class="border p-2 text-right">{}%</td>
            </tr>"#,
            name, sector, status, employment, capacity
        ));
    }

    table.push_str("</tbody></table>");

    let html = format!(
        r#"{}<h1 class="text-3xl font-bold mb-6">Industrial Projects Registry</h1>{}{}"#,
        HTML_HEADER, table, HTML_FOOTER
    );
    Html(html)
}

pub async fn analytics_page(State(app_state): State<Arc<AppState>>) -> Html<String> {
    let rows = sqlx::query(
        "SELECT sector, employment, gdp_contribution_usd FROM sector_economic_snapshots LIMIT 10",
    )
    .fetch_all(&app_state.db_pool)
    .await
    .unwrap_or_default();

    let mut table = String::from(
        r#"<table class="w-full border-collapse border border-gray-300">
            <thead class="bg-gray-200">
                <tr>
                    <th class="border p-2 text-left">Sector</th>
                    <th class="border p-2 text-right">Employment</th>
                    <th class="border p-2 text-right">GDP Contribution</th>
                </tr>
            </thead>
            <tbody>"#,
    );

    for row in rows {
        let sector: String = row.get("sector");
        let employment: i32 = row.get("employment");
        let gdp: f64 = row.get("gdp_contribution_usd");

        table.push_str(&format!(
            r#"<tr class="hover:bg-gray-100">
                <td class="border p-2">{}</td>
                <td class="border p-2 text-right">{}</td>
                <td class="border p-2 text-right">${:.2}M</td>
            </tr>"#,
            sector,
            employment,
            gdp / 1_000_000.0
        ));
    }

    table.push_str("</tbody></table>");

    let html = format!(
        r#"{}<h1 class="text-3xl font-bold mb-6">Analytics & Trade Data</h1>{}{}"#,
        HTML_HEADER, table, HTML_FOOTER
    );
    Html(html)
}

pub async fn compliance_page(State(app_state): State<Arc<AppState>>) -> Html<String> {
    let rows = sqlx::query(
        "SELECT report_id, report_type, status, risk_score FROM regulatory_reports LIMIT 20",
    )
    .fetch_all(&app_state.db_pool)
    .await
    .unwrap_or_default();

    let mut table = String::from(
        r#"<table class="w-full border-collapse border border-gray-300">
            <thead class="bg-gray-200">
                <tr>
                    <th class="border p-2 text-left">Report ID</th>
                    <th class="border p-2 text-left">Type</th>
                    <th class="border p-2 text-left">Status</th>
                    <th class="border p-2 text-right">Risk Score</th>
                </tr>
            </thead>
            <tbody>"#,
    );

    for row in rows {
        let report_id: String = row.get("report_id");
        let report_type: String = row.get("report_type");
        let status: String = row.get("status");
        let risk_score: i32 = row.get("risk_score");

        table.push_str(&format!(
            r#"<tr class="hover:bg-gray-100">
                <td class="border p-2 font-mono text-sm">{}</td>
                <td class="border p-2"><span class="px-2 py-1 rounded text-white text-sm bg-purple-600">{}</span></td>
                <td class="border p-2">{}</td>
                <td class="border p-2 text-right font-bold">{}</td>
            </tr>"#,
            &report_id[..8.min(report_id.len())], report_type, status, risk_score
        ));
    }

    table.push_str("</tbody></table>");

    let html = format!(
        r#"{}<h1 class="text-3xl font-bold mb-6">Compliance Operations</h1>{}{}"#,
        HTML_HEADER, table, HTML_FOOTER
    );
    Html(html)
}

pub async fn accounts_page(State(app_state): State<Arc<AppState>>) -> Html<String> {
    let rows =
        sqlx::query("SELECT display_name, kyc_tier, balance_owc, credit_score FROM users LIMIT 50")
            .fetch_all(&app_state.db_pool)
            .await
            .unwrap_or_default();

    let mut table = String::from(
        r#"<table class="w-full border-collapse border border-gray-300">
            <thead class="bg-gray-200">
                <tr>
                    <th class="border p-2 text-left">User</th>
                    <th class="border p-2 text-left">KYC Tier</th>
                    <th class="border p-2 text-right">Balance</th>
                    <th class="border p-2 text-right">Credit Score</th>
                </tr>
            </thead>
            <tbody>"#,
    );

    for row in rows {
        let display_name: String = row.get("display_name");
        let kyc_tier: String = row.get("kyc_tier");
        let balance: i64 = row.get("balance_owc");
        let credit_score: Option<f64> = row.get("credit_score");

        let score_str = credit_score
            .map(|s| format!("{:.0}", s))
            .unwrap_or_else(|| "N/A".to_string());
        let score_class = if let Some(s) = credit_score {
            if s > 700.0 {
                "text-green-600"
            } else {
                "text-red-600"
            }
        } else {
            "text-gray-600"
        };

        table.push_str(&format!(
            r#"<tr class="hover:bg-gray-100">
                <td class="border p-2">{}</td>
                <td class="border p-2"><span class="px-2 py-1 rounded text-white text-sm bg-blue-600">{}</span></td>
                <td class="border p-2 text-right">{} OWC</td>
                <td class="border p-2 text-right font-bold {}">{}</td>
            </tr>"#,
            display_name, kyc_tier, balance, score_class, score_str
        ));
    }

    table.push_str("</tbody></table>");

    let html = format!(
        r#"{}<h1 class="text-3xl font-bold mb-6">Account Management</h1>{}{}"#,
        HTML_HEADER, table, HTML_FOOTER
    );
    Html(html)
}
