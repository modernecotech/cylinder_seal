//! HTML page route handlers

use axum::{extract::State, response::Html};
use std::sync::Arc;
use sqlx::Row;
use crate::state::AppState;

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
        window.fetch = function(...args) {
            const token = sessionStorage.getItem('cbi_token');
            if (token && args[1]) {
                args[1].headers = args[1].headers || {};
                args[1].headers['Authorization'] = `Bearer ${token}`;
            }
            return originalFetch.apply(this, args);
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
            <a href="/compliance" class="hover:text-blue-200">Compliance</a>
            <button onclick="logout()" class="hover:text-blue-200">Logout</button>
        </div>
    </div>
</nav>
<main class="max-w-7xl mx-auto px-4 py-6">
<script>
function logout() {
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

pub async fn overview_page(
    State(app_state): State<Arc<AppState>>,
) -> Html<String> {
    let user_count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM users WHERE balance_owc > 0")
        .fetch_one(&app_state.db_pool)
        .await
        .unwrap_or(0);

    let gdp = user_count as f64 * 5500.0 / 1_000_000_000.0;

    let project_count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM industrial_projects")
        .fetch_one(&app_state.db_pool)
        .await
        .unwrap_or(0);

    let report_count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM regulatory_reports WHERE status IN ('Draft', 'UnderReview')")
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

pub async fn projects_page(
    State(app_state): State<Arc<AppState>>,
) -> Html<String> {
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
            <tbody>"#
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

pub async fn analytics_page(
    State(app_state): State<Arc<AppState>>,
) -> Html<String> {
    let rows = sqlx::query("SELECT sector, employment, gdp_contribution_usd FROM sector_economic_snapshots LIMIT 10")
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
            <tbody>"#
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
            sector, employment, gdp / 1_000_000.0
        ));
    }

    table.push_str("</tbody></table>");

    let html = format!(
        r#"{}<h1 class="text-3xl font-bold mb-6">Analytics & Trade Data</h1>{}{}"#,
        HTML_HEADER, table, HTML_FOOTER
    );
    Html(html)
}

pub async fn compliance_page(
    State(app_state): State<Arc<AppState>>,
) -> Html<String> {
    let rows = sqlx::query("SELECT report_id, report_type, status, risk_score FROM regulatory_reports LIMIT 20")
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
            <tbody>"#
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

pub async fn accounts_page(
    State(app_state): State<Arc<AppState>>,
) -> Html<String> {
    let rows = sqlx::query("SELECT display_name, kyc_tier, balance_owc, credit_score FROM users LIMIT 50")
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
            <tbody>"#
    );

    for row in rows {
        let display_name: String = row.get("display_name");
        let kyc_tier: String = row.get("kyc_tier");
        let balance: i64 = row.get("balance_owc");
        let credit_score: Option<f64> = row.get("credit_score");

        let score_str = credit_score.map(|s| format!("{:.0}", s)).unwrap_or_else(|| "N/A".to_string());
        let score_class = if let Some(s) = credit_score {
            if s > 700.0 { "text-green-600" } else { "text-red-600" }
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
