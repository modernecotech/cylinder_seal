//! Application state holding all shared resources

use sqlx::PgPool;
use deadpool_redis::Pool as RedisPool;

/// Central application state
/// Holds database pools and shared resources
pub struct AppState {
    pub db_pool: PgPool,
    pub redis_pool: RedisPool,
}

impl AppState {
    pub async fn new(db_pool: PgPool, redis_pool: RedisPool) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            db_pool,
            redis_pool,
        })
    }
}
