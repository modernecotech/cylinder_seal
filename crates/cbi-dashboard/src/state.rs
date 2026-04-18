//! Application state holding all shared resources

use std::sync::Arc;
use sqlx::PgPool;
use deadpool_redis::Pool as RedisPool;

use cs_analytics::{AnalyticsRepository, SqlxAnalyticsRepository};

/// Central application state
/// Holds database pools, repositories, and business logic engines
pub struct AppState {
    pub db_pool: PgPool,
    pub redis_pool: RedisPool,

    // Analytics
    pub analytics_repo: Arc<dyn AnalyticsRepository>,
}

impl AppState {
    pub async fn new(db_pool: PgPool, redis_pool: RedisPool) -> Result<Self, Box<dyn std::error::Error>> {
        let analytics_repo = Arc::new(SqlxAnalyticsRepository::new(db_pool.clone()));

        Ok(Self {
            db_pool,
            redis_pool,
            analytics_repo,
        })
    }
}
