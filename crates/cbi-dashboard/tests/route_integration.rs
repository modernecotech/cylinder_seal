use axum::{
    body::Body,
    http::{header, request::Builder, Request, StatusCode},
    Router,
};
use cbi_dashboard::{
    build_app,
    state::{AppState, MemoryAuditRecorder, MemorySessionStore, SessionStore},
};
use serde_json::json;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tower::ServiceExt;

const TOKEN: &str = "test-session-token";

struct TestApp {
    router: Router,
    audit_recorder: Arc<MemoryAuditRecorder>,
}

async fn app_with_role(role: &str) -> TestApp {
    let db_pool = PgPoolOptions::new()
        .max_connections(1)
        .connect_lazy("postgresql://postgres:postgres@localhost/cylinder_seal")
        .expect("lazy PostgreSQL pool");

    let session_store = Arc::new(MemorySessionStore::new());
    let session_json = json!({
        "operator_id": "op-test",
        "username": "test-operator",
        "role": role,
    })
    .to_string();
    session_store
        .set_session(TOKEN, &session_json, 3_600)
        .await
        .expect("seed in-memory session");

    let store: Arc<dyn SessionStore> = session_store;
    let audit_recorder = Arc::new(MemoryAuditRecorder::new());
    let state = Arc::new(
        AppState::new(db_pool, store, audit_recorder.clone(), 3_600)
            .await
            .expect("app state"),
    );

    TestApp {
        router: build_app(state),
        audit_recorder,
    }
}

fn request(method: &str, uri: &str) -> Builder {
    Request::builder().method(method).uri(uri)
}

fn bearer_request(method: &str, uri: &str) -> Builder {
    request(method, uri).header(header::AUTHORIZATION, format!("Bearer {TOKEN}"))
}

fn cookie_request(method: &str, uri: &str) -> Builder {
    request(method, uri).header(header::COOKIE, format!("cs_dash_session={TOKEN}"))
}

#[tokio::test]
async fn health_endpoint_is_public() {
    let app = app_with_role("auditor").await;

    let response = app
        .router
        .oneshot(request("GET", "/health").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn protected_api_requires_session() {
    let app = app_with_role("auditor").await;

    let response = app
        .router
        .oneshot(request("GET", "/api/projects").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn protected_read_route_accepts_bearer_session() {
    let app = app_with_role("auditor").await;

    let response = app
        .router
        .oneshot(
            bearer_request("GET", "/api/projects")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn protected_read_route_accepts_cookie_session() {
    let app = app_with_role("auditor").await;

    let response = app
        .router
        .oneshot(
            cookie_request("GET", "/api/projects")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn unsafe_cookie_only_request_requires_csrf_token() {
    let app = app_with_role("officer").await;
    let body = serde_json::to_vec(&project_payload()).unwrap();

    let response = app
        .router
        .oneshot(
            cookie_request("POST", "/api/projects")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn unsafe_cookie_request_with_csrf_token_is_allowed() {
    let app = app_with_role("officer").await;
    let body = serde_json::to_vec(&project_payload()).unwrap();

    let response = app
        .router
        .oneshot(
            cookie_request("POST", "/api/projects")
                .header("x-csrf-token", TOKEN)
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn officer_can_create_project_with_bearer_session() {
    let app = app_with_role("officer").await;
    let body = serde_json::to_vec(&project_payload()).unwrap();

    let response = app
        .router
        .oneshot(
            bearer_request("POST", "/api/projects")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let records = app.audit_recorder.records().await;
    assert_eq!(records.len(), 1);
    assert_eq!(records[0].action, "project.create");
    assert_eq!(
        records[0].target_kind.as_deref(),
        Some("industrial_project")
    );
    assert_eq!(records[0].result, "ok");
}

#[tokio::test]
async fn auditor_cannot_create_project() {
    let app = app_with_role("auditor").await;
    let body = serde_json::to_vec(&project_payload()).unwrap();

    let response = app
        .router
        .oneshot(
            bearer_request("POST", "/api/projects")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::FORBIDDEN);
    let records = app.audit_recorder.records().await;
    assert_eq!(records.len(), 1);
    assert_eq!(records[0].action, "project.create");
    assert_eq!(records[0].result, "denied");
}

#[tokio::test]
async fn supervisor_can_freeze_account() {
    let app = app_with_role("supervisor").await;
    let body = serde_json::to_vec(&json!({ "reason": "test" })).unwrap();

    let response = app
        .router
        .oneshot(
            bearer_request(
                "POST",
                "/api/accounts/550e8400-e29b-41d4-a716-446655440000/freeze",
            )
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(body))
            .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let records = app.audit_recorder.records().await;
    assert_eq!(records.len(), 1);
    assert_eq!(records[0].action, "account.freeze");
    assert_eq!(records[0].target_kind.as_deref(), Some("user"));
    assert_eq!(records[0].result, "ok");
}

#[tokio::test]
async fn officer_can_create_compliance_report_with_audit_record() {
    let app = app_with_role("officer").await;
    let body = serde_json::to_vec(&json!({
        "report_type": "SAR",
        "subject_user_id": "550e8400-e29b-41d4-a716-446655440000",
        "narrative": "test report"
    }))
    .unwrap();

    let response = app
        .router
        .oneshot(
            bearer_request("POST", "/api/compliance/reports")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let records = app.audit_recorder.records().await;
    assert_eq!(records.len(), 1);
    assert_eq!(records[0].action, "report.create");
    assert_eq!(records[0].target_kind.as_deref(), Some("regulatory_report"));
    assert_eq!(records[0].result, "ok");
}

#[tokio::test]
async fn supervisor_can_create_directive_with_audit_record() {
    let app = app_with_role("supervisor").await;
    let body = serde_json::to_vec(&json!({
        "directive_type": "velocity_cap",
        "description": "test directive"
    }))
    .unwrap();

    let response = app
        .router
        .oneshot(
            bearer_request("POST", "/api/audit/directives")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let records = app.audit_recorder.records().await;
    assert_eq!(records.len(), 1);
    assert_eq!(records[0].action, "directive.create");
    assert_eq!(
        records[0].target_kind.as_deref(),
        Some("emergency_directive")
    );
    assert_eq!(records[0].result, "ok");
}

#[tokio::test]
async fn officer_cannot_freeze_account() {
    let app = app_with_role("officer").await;
    let body = serde_json::to_vec(&json!({ "reason": "test" })).unwrap();

    let response = app
        .router
        .oneshot(
            bearer_request(
                "POST",
                "/api/accounts/550e8400-e29b-41d4-a716-446655440000/freeze",
            )
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(body))
            .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::FORBIDDEN);
    let records = app.audit_recorder.records().await;
    assert_eq!(records.len(), 1);
    assert_eq!(records[0].action, "account.freeze");
    assert_eq!(records[0].result, "denied");
}

#[tokio::test]
async fn logout_deletes_session() {
    let app = app_with_role("auditor").await;

    let logout_response = app
        .router
        .clone()
        .oneshot(
            bearer_request("POST", "/auth/logout")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(logout_response.status(), StatusCode::OK);

    let response = app
        .router
        .oneshot(
            bearer_request("GET", "/api/projects")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

fn project_payload() -> serde_json::Value {
    json!({
        "name": "Test Project",
        "sector": "Cement",
        "governorate": "Najaf",
        "estimated_capex_usd": 1000000.0,
        "expected_revenue_usd_annual": 250000.0,
        "employment_count": 25
    })
}
