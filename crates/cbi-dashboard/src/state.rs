//! Application state holding all shared resources

use crate::auth::{AuthenticatedOperator, OperatorRole};
use async_trait::async_trait;
use axum::http::StatusCode;
use deadpool_redis::Pool as RedisPool;
use redis::AsyncCommands;
use sqlx::PgPool;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Debug)]
pub struct SessionStoreError;

#[async_trait]
pub trait SessionStore: Send + Sync {
    async fn set_session(
        &self,
        token: &str,
        session_json: &str,
        ttl_secs: u64,
    ) -> Result<(), SessionStoreError>;

    async fn get_session(&self, token: &str) -> Result<Option<String>, SessionStoreError>;

    async fn delete_session(&self, token: &str) -> Result<(), SessionStoreError>;
}

pub struct RedisSessionStore {
    redis_pool: RedisPool,
}

impl RedisSessionStore {
    pub fn new(redis_pool: RedisPool) -> Self {
        Self { redis_pool }
    }
}

#[async_trait]
impl SessionStore for RedisSessionStore {
    async fn set_session(
        &self,
        token: &str,
        session_json: &str,
        ttl_secs: u64,
    ) -> Result<(), SessionStoreError> {
        let mut conn = self.redis_pool.get().await.map_err(|_| SessionStoreError)?;
        let _: () = conn
            .set_ex(format!("session:{token}"), session_json, ttl_secs)
            .await
            .map_err(|_| SessionStoreError)?;
        Ok(())
    }

    async fn get_session(&self, token: &str) -> Result<Option<String>, SessionStoreError> {
        let mut conn = self.redis_pool.get().await.map_err(|_| SessionStoreError)?;
        conn.get(format!("session:{token}"))
            .await
            .map_err(|_| SessionStoreError)
    }

    async fn delete_session(&self, token: &str) -> Result<(), SessionStoreError> {
        let mut conn = self.redis_pool.get().await.map_err(|_| SessionStoreError)?;
        let _: i32 = conn
            .del(format!("session:{token}"))
            .await
            .map_err(|_| SessionStoreError)?;
        Ok(())
    }
}

#[derive(Default)]
pub struct MemorySessionStore {
    sessions: RwLock<HashMap<String, String>>,
}

impl MemorySessionStore {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl SessionStore for MemorySessionStore {
    async fn set_session(
        &self,
        token: &str,
        session_json: &str,
        _ttl_secs: u64,
    ) -> Result<(), SessionStoreError> {
        self.sessions
            .write()
            .await
            .insert(token.to_string(), session_json.to_string());
        Ok(())
    }

    async fn get_session(&self, token: &str) -> Result<Option<String>, SessionStoreError> {
        Ok(self.sessions.read().await.get(token).cloned())
    }

    async fn delete_session(&self, token: &str) -> Result<(), SessionStoreError> {
        self.sessions.write().await.remove(token);
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AdminAuditRecord {
    pub operator_id: String,
    pub operator_username: String,
    pub action: String,
    pub target_kind: Option<String>,
    pub target_id: Option<String>,
    pub request_payload: serde_json::Value,
    pub result: String,
}

impl AdminAuditRecord {
    fn new(
        operator: &AuthenticatedOperator,
        action: &str,
        target_kind: Option<&str>,
        target_id: Option<String>,
        request_payload: serde_json::Value,
        result: &str,
    ) -> Self {
        Self {
            operator_id: operator.operator_id.clone(),
            operator_username: operator.username.clone(),
            action: action.to_string(),
            target_kind: target_kind.map(str::to_string),
            target_id,
            request_payload,
            result: result.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct AuditRecorderError;

#[async_trait]
pub trait AuditRecorder: Send + Sync {
    async fn record(&self, record: AdminAuditRecord) -> Result<(), AuditRecorderError>;
}

pub struct PostgresAuditRecorder {
    db_pool: PgPool,
}

impl PostgresAuditRecorder {
    pub fn new(db_pool: PgPool) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl AuditRecorder for PostgresAuditRecorder {
    async fn record(&self, record: AdminAuditRecord) -> Result<(), AuditRecorderError> {
        let operator_id = Uuid::parse_str(&record.operator_id).ok();
        let payload = record.request_payload.to_string();

        sqlx::query(
            r#"
            INSERT INTO admin_audit_log (
                operator_id,
                operator_username,
                action,
                target_kind,
                target_id,
                request_payload,
                result
            )
            VALUES ($1, $2, $3, $4, $5, CAST($6 AS JSONB), $7)
            "#,
        )
        .bind(operator_id)
        .bind(&record.operator_username)
        .bind(&record.action)
        .bind(&record.target_kind)
        .bind(&record.target_id)
        .bind(payload)
        .bind(&record.result)
        .execute(&self.db_pool)
        .await
        .map_err(|_| AuditRecorderError)?;

        Ok(())
    }
}

#[derive(Default)]
pub struct MemoryAuditRecorder {
    records: RwLock<Vec<AdminAuditRecord>>,
}

impl MemoryAuditRecorder {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn records(&self) -> Vec<AdminAuditRecord> {
        self.records.read().await.clone()
    }
}

#[async_trait]
impl AuditRecorder for MemoryAuditRecorder {
    async fn record(&self, record: AdminAuditRecord) -> Result<(), AuditRecorderError> {
        self.records.write().await.push(record);
        Ok(())
    }
}

/// Central application state
/// Holds database pools and shared resources
pub struct AppState {
    pub db_pool: PgPool,
    pub session_store: Arc<dyn SessionStore>,
    pub audit_recorder: Arc<dyn AuditRecorder>,
    pub session_ttl_secs: u64,
}

impl AppState {
    pub async fn new(
        db_pool: PgPool,
        session_store: Arc<dyn SessionStore>,
        audit_recorder: Arc<dyn AuditRecorder>,
        session_ttl_secs: u64,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            db_pool,
            session_store,
            audit_recorder,
            session_ttl_secs,
        })
    }

    pub async fn require_role_and_audit(
        &self,
        operator: &AuthenticatedOperator,
        required_role: OperatorRole,
        action: &str,
        target_kind: Option<&str>,
        target_id: Option<String>,
        request_payload: serde_json::Value,
    ) -> Result<(), StatusCode> {
        match operator.require_role(required_role) {
            Ok(()) => {
                self.record_admin_action(AdminAuditRecord::new(
                    operator,
                    action,
                    target_kind,
                    target_id,
                    request_payload,
                    "ok",
                ))
                .await
            }
            Err(status) => {
                self.record_admin_action(AdminAuditRecord::new(
                    operator,
                    action,
                    target_kind,
                    target_id,
                    request_payload,
                    "denied",
                ))
                .await?;
                Err(status)
            }
        }
    }

    async fn record_admin_action(&self, record: AdminAuditRecord) -> Result<(), StatusCode> {
        self.audit_recorder
            .record(record)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
    }
}
