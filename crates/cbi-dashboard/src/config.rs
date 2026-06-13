//! Configuration management

use std::net::SocketAddr;

#[derive(Clone, Debug)]
pub struct Config {
    pub bind_addr: SocketAddr,
    pub database_url: String,
    pub redis_url: String,
    pub db_max_connections: u32,
    pub session_ttl_secs: u64,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let bind_addr = std::env::var("BIND_ADDR")
            .unwrap_or_else(|_| "127.0.0.1:8081".to_string())
            .parse::<SocketAddr>()?;

        // The dashboard is PostgreSQL-only. POS-local SQLite remains separate
        // as a terminal/device store and is not a dashboard runtime path.
        let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            let host = std::env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string());
            let port = std::env::var("DB_PORT").unwrap_or_else(|_| "5432".to_string());
            let name = std::env::var("DB_NAME").unwrap_or_else(|_| "cylinder_seal".to_string());
            let user = std::env::var("DB_USER").unwrap_or_else(|_| "postgres".to_string());
            let password =
                std::env::var("DB_PASSWORD").unwrap_or_else(|_| "change-me-dev-only".to_string());
            format!("postgresql://{user}:{password}@{host}:{port}/{name}")
        });

        let redis_url =
            std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string());

        let db_max_connections = std::env::var("DB_MAX_CONNECTIONS")
            .ok()
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(20);

        let session_ttl_secs = std::env::var("SESSION_TTL_SECS")
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(43_200); // 12 hours

        Ok(Self {
            bind_addr,
            database_url,
            redis_url,
            db_max_connections,
            session_ttl_secs,
        })
    }
}
