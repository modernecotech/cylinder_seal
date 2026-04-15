use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Environment {
    Development,
    Staging,
    Production,
}

impl Environment {
    fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "production" | "prod" => Environment::Production,
            "staging" | "stage" => Environment::Staging,
            _ => Environment::Development,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub environment: Environment,
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub super_peer: SuperPeerConfig,
    pub logging: LoggingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub grpc_port: u16,
    pub http_port: u16,
    pub node_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub name: String,
    pub user: String,
    pub password: String,
    pub max_connections: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisConfig {
    pub host: String,
    pub port: u16,
    pub db: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuperPeerConfig {
    /// Number of super-peers in Byzantine quorum (typically 5)
    pub quorum_size: u8,
    /// Minimum confirmations required (typically 3 of 5)
    pub min_confirmations: u8,
    /// Key rotation interval in days
    pub key_rotation_days: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log level: TRACE, DEBUG, INFO, WARN, ERROR
    pub level: String,
    /// JSON structured logs (true) or human-readable (false)
    pub json: bool,
}

impl Config {
    /// Load configuration from environment variables with sensible defaults.
    /// CLI arguments override environment variables when provided.
    pub fn load(config_path: &Option<String>, environment: &Option<String>) -> Result<Self> {
        // CLI --environment flag overrides ENVIRONMENT env var
        let env_str = environment
            .clone()
            .or_else(|| env::var("ENVIRONMENT").ok())
            .unwrap_or_else(|| "development".to_string());
        let environment = Environment::from_str(&env_str);

        if let Some(path) = config_path {
            tracing::info!("Config file loading not yet implemented, ignoring: {}", path);
        }

        let server = ServerConfig {
            grpc_port: env::var("GRPC_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(50051),
            http_port: env::var("HTTP_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(8080),
            node_id: env::var("NODE_ID").unwrap_or_else(|_| "node-1".to_string()),
        };

        let database = DatabaseConfig {
            host: env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string()),
            port: env::var("DB_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(5432),
            name: env::var("DB_NAME").unwrap_or_else(|_| "cylinder_seal".to_string()),
            user: env::var("DB_USER").unwrap_or_else(|_| "postgres".to_string()),
            password: env::var("DB_PASSWORD").unwrap_or_else(|_| "password".to_string()),
            max_connections: env::var("DB_MAX_CONNECTIONS")
                .ok()
                .and_then(|c| c.parse().ok())
                .unwrap_or(10),
        };

        let redis = RedisConfig {
            host: env::var("REDIS_HOST").unwrap_or_else(|_| "localhost".to_string()),
            port: env::var("REDIS_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(6379),
            db: env::var("REDIS_DB")
                .ok()
                .and_then(|d| d.parse().ok())
                .unwrap_or(0),
        };

        let super_peer = SuperPeerConfig {
            quorum_size: env::var("SUPER_PEER_QUORUM_SIZE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(5),
            min_confirmations: env::var("SUPER_PEER_MIN_CONFIRMATIONS")
                .ok()
                .and_then(|c| c.parse().ok())
                .unwrap_or(3),
            key_rotation_days: env::var("KEY_ROTATION_DAYS")
                .ok()
                .and_then(|d| d.parse().ok())
                .unwrap_or(30),
        };

        let logging = LoggingConfig {
            level: env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()),
            json: env::var("LOG_JSON")
                .map(|v| v.to_lowercase() == "true")
                .unwrap_or(false),
        };

        Ok(Self {
            environment,
            server,
            database,
            redis,
            super_peer,
            logging,
        })
    }
}
