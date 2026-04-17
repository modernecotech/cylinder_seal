//! Redis-backed implementations of [`NonceStore`] and [`SessionStore`].

use async_trait::async_trait;
use deadpool_redis::Pool;
use redis::AsyncCommands;
use uuid::Uuid;

use cs_core::error::{CylinderSealError, Result};

use crate::repository::{NonceStore, SessionStore};

const NONCE_KEY_PREFIX: &str = "cs:nonce:";
const SESSION_KEY_PREFIX: &str = "cs:session:";
const ADMIN_SESSION_KEY_PREFIX: &str = "cs:adm:session:";

pub struct RedisNonceStore {
    pool: Pool,
}

impl RedisNonceStore {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    fn key(nonce: &[u8; 32]) -> String {
        format!("{}{}", NONCE_KEY_PREFIX, hex::encode(nonce))
    }
}

#[async_trait]
impl NonceStore for RedisNonceStore {
    /// Atomically reserve a nonce with TTL. Uses `SET key 1 NX EX ttl`.
    /// Returns `true` on first-sight (accepted); `false` if the nonce was
    /// already present (replay attempt).
    async fn check_and_set(&self, nonce: &[u8; 32], ttl_hours: u32) -> Result<bool> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| CylinderSealError::DatabaseError(format!("redis pool: {e}")))?;

        let key = Self::key(nonce);
        let ttl_secs = (ttl_hours as u64) * 3600;

        // SET key value NX EX ttl — returns "OK" if set, nil if key exists.
        let res: Option<String> = redis::cmd("SET")
            .arg(&key)
            .arg(1)
            .arg("NX")
            .arg("EX")
            .arg(ttl_secs)
            .query_async(&mut *conn)
            .await
            .map_err(|e| CylinderSealError::DatabaseError(format!("redis SET NX: {e}")))?;

        Ok(res.is_some())
    }

    async fn exists(&self, nonce: &[u8; 32]) -> Result<bool> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| CylinderSealError::DatabaseError(format!("redis pool: {e}")))?;
        let key = Self::key(nonce);
        let exists: bool = conn
            .exists(&key)
            .await
            .map_err(|e| CylinderSealError::DatabaseError(format!("redis EXISTS: {e}")))?;
        Ok(exists)
    }
}

pub struct RedisSessionStore {
    pool: Pool,
}

impl RedisSessionStore {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    fn key(token: &str) -> String {
        format!("{}{}", SESSION_KEY_PREFIX, token)
    }
}

#[async_trait]
impl SessionStore for RedisSessionStore {
    async fn create(&self, user_id: Uuid, token: &str, ttl_hours: u32) -> Result<()> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| CylinderSealError::DatabaseError(format!("redis pool: {e}")))?;
        let key = Self::key(token);
        let ttl_secs = (ttl_hours as u64) * 3600;
        let _: () = redis::cmd("SET")
            .arg(&key)
            .arg(user_id.to_string())
            .arg("EX")
            .arg(ttl_secs)
            .query_async(&mut *conn)
            .await
            .map_err(|e| CylinderSealError::DatabaseError(format!("redis SET session: {e}")))?;
        Ok(())
    }

    async fn get_user(&self, token: &str) -> Result<Option<Uuid>> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| CylinderSealError::DatabaseError(format!("redis pool: {e}")))?;
        let key = Self::key(token);
        let val: Option<String> = conn
            .get(&key)
            .await
            .map_err(|e| CylinderSealError::DatabaseError(format!("redis GET session: {e}")))?;
        match val {
            Some(s) => Ok(Uuid::parse_str(&s).ok()),
            None => Ok(None),
        }
    }

    async fn invalidate(&self, token: &str) -> Result<()> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| CylinderSealError::DatabaseError(format!("redis pool: {e}")))?;
        let key = Self::key(token);
        let _: i64 = conn
            .del(&key)
            .await
            .map_err(|e| CylinderSealError::DatabaseError(format!("redis DEL session: {e}")))?;
        Ok(())
    }
}

pub struct RedisAdminSessionStore {
    pool: Pool,
}

impl RedisAdminSessionStore {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    fn key(token: &str) -> String {
        format!("{}{}", ADMIN_SESSION_KEY_PREFIX, token)
    }
}

#[async_trait]
impl crate::compliance::AdminSessionStore for RedisAdminSessionStore {
    async fn create(
        &self,
        token: &str,
        session: &crate::compliance::AdminSession,
        ttl_hours: u32,
    ) -> Result<()> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| CylinderSealError::DatabaseError(format!("redis pool: {e}")))?;
        let key = Self::key(token);
        let value = format!(
            "{}|{}|{}",
            session.operator_id, session.username, session.role
        );
        let ttl_secs = (ttl_hours as u64) * 3600;
        let _: () = redis::cmd("SET")
            .arg(&key)
            .arg(&value)
            .arg("EX")
            .arg(ttl_secs)
            .query_async(&mut *conn)
            .await
            .map_err(|e| CylinderSealError::DatabaseError(format!("redis SET adm session: {e}")))?;
        Ok(())
    }

    async fn get(&self, token: &str) -> Result<Option<crate::compliance::AdminSession>> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| CylinderSealError::DatabaseError(format!("redis pool: {e}")))?;
        let key = Self::key(token);
        let val: Option<String> = conn
            .get(&key)
            .await
            .map_err(|e| CylinderSealError::DatabaseError(format!("redis GET adm session: {e}")))?;
        match val {
            None => Ok(None),
            Some(s) => {
                let mut parts = s.splitn(3, '|');
                let id = parts.next().and_then(|s| Uuid::parse_str(s).ok());
                let user = parts.next().map(str::to_string);
                let role = parts.next().map(str::to_string);
                match (id, user, role) {
                    (Some(operator_id), Some(username), Some(role)) => {
                        Ok(Some(crate::compliance::AdminSession {
                            operator_id,
                            username,
                            role,
                        }))
                    }
                    _ => Ok(None),
                }
            }
        }
    }

    async fn invalidate(&self, token: &str) -> Result<()> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| CylinderSealError::DatabaseError(format!("redis pool: {e}")))?;
        let key = Self::key(token);
        let _: i64 = conn
            .del(&key)
            .await
            .map_err(|e| CylinderSealError::DatabaseError(format!("redis DEL adm session: {e}")))?;
        Ok(())
    }
}
