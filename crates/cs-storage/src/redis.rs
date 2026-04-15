// Redis connection and utilities
use deadpool_redis::{Config, Pool};

#[derive(Clone)]
pub struct RedisConfig {
    pub host: String,
    pub port: u16,
    pub database: u8,
}

impl RedisConfig {
    pub fn from_env() -> Self {
        Self {
            host: std::env::var("REDIS_HOST").unwrap_or_else(|_| "localhost".to_string()),
            port: std::env::var("REDIS_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(6379),
            database: std::env::var("REDIS_DB")
                .ok()
                .and_then(|d| d.parse().ok())
                .unwrap_or(0),
        }
    }

    pub async fn connect(&self) -> Result<Pool, deadpool_redis::CreatePoolError> {
        let cfg = Config::from_url(&format!(
            "redis://{}:{}/{}",
            self.host, self.port, self.database
        ));
        cfg.create_pool(None)
    }
}
