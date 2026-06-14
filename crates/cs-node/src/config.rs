use ::config::{Config as SourceConfig, File};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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
    /// Number of super-peers in the Raft cluster (typically 5)
    pub quorum_size: u8,
    /// Minimum Raft quorum (typically 3 of 5)
    pub min_confirmations: u8,
    /// Key rotation interval in days
    pub key_rotation_days: u32,
    /// Peers in the Raft cluster, by node id (e.g. "sp-basra", "sp-erbil").
    /// Empty = single-node loopback (development mode).
    pub peers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log level: TRACE, DEBUG, INFO, WARN, ERROR
    pub level: String,
    /// JSON structured logs (true) or human-readable (false)
    pub json: bool,
}

#[derive(Debug, Default, Deserialize)]
struct ConfigFile {
    environment: Option<String>,
    server: Option<ServerConfigFile>,
    database: Option<DatabaseConfigFile>,
    redis: Option<RedisConfigFile>,
    super_peer: Option<SuperPeerConfigFile>,
    logging: Option<LoggingConfigFile>,
}

#[derive(Debug, Default, Deserialize)]
struct ServerConfigFile {
    grpc_port: Option<u16>,
    http_port: Option<u16>,
    node_id: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
struct DatabaseConfigFile {
    host: Option<String>,
    port: Option<u16>,
    name: Option<String>,
    user: Option<String>,
    password: Option<String>,
    max_connections: Option<u32>,
}

#[derive(Debug, Default, Deserialize)]
struct RedisConfigFile {
    host: Option<String>,
    port: Option<u16>,
    db: Option<u8>,
}

#[derive(Debug, Default, Deserialize)]
struct SuperPeerConfigFile {
    quorum_size: Option<u8>,
    min_confirmations: Option<u8>,
    key_rotation_days: Option<u32>,
    peers: Option<Vec<String>>,
}

#[derive(Debug, Default, Deserialize)]
struct LoggingConfigFile {
    level: Option<String>,
    json: Option<bool>,
}

impl Config {
    /// Load configuration from defaults, optional file, environment variables,
    /// and CLI overrides, in that order.
    pub fn load(config_path: &Option<String>, environment: &Option<String>) -> Result<Self> {
        let mut cfg = Self::defaults();
        if let Some(path) = config_path {
            let file_cfg: ConfigFile = SourceConfig::builder()
                .add_source(File::with_name(path).required(true))
                .build()
                .with_context(|| format!("load config file {path}"))?
                .try_deserialize()
                .with_context(|| format!("parse config file {path}"))?;
            cfg.apply_file(file_cfg);
        }

        cfg.apply_env_overrides();
        if let Some(env_str) = environment.clone().or_else(|| env::var("ENVIRONMENT").ok()) {
            cfg.environment = Environment::from_str(&env_str);
        }

        Ok(cfg)
    }

    fn defaults() -> Self {
        Self {
            environment: Environment::Development,
            server: ServerConfig {
                grpc_port: 50051,
                http_port: 8080,
                node_id: "node-1".to_string(),
            },
            database: DatabaseConfig {
                host: "localhost".to_string(),
                port: 5432,
                name: "cylinder_seal".to_string(),
                user: "postgres".to_string(),
                password: "change-me-dev-only".to_string(),
                max_connections: 10,
            },
            redis: RedisConfig {
                host: "localhost".to_string(),
                port: 6379,
                db: 0,
            },
            super_peer: SuperPeerConfig {
                quorum_size: 5,
                min_confirmations: 3,
                key_rotation_days: 365,
                peers: Vec::new(),
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                json: false,
            },
        }
    }

    fn apply_file(&mut self, file: ConfigFile) {
        if let Some(environment) = file.environment {
            self.environment = Environment::from_str(&environment);
        }

        if let Some(server) = file.server {
            if let Some(value) = server.grpc_port {
                self.server.grpc_port = value;
            }
            if let Some(value) = server.http_port {
                self.server.http_port = value;
            }
            if let Some(value) = server.node_id {
                self.server.node_id = value;
            }
        }

        if let Some(database) = file.database {
            if let Some(value) = database.host {
                self.database.host = value;
            }
            if let Some(value) = database.port {
                self.database.port = value;
            }
            if let Some(value) = database.name {
                self.database.name = value;
            }
            if let Some(value) = database.user {
                self.database.user = value;
            }
            if let Some(value) = database.password {
                self.database.password = value;
            }
            if let Some(value) = database.max_connections {
                self.database.max_connections = value;
            }
        }

        if let Some(redis) = file.redis {
            if let Some(value) = redis.host {
                self.redis.host = value;
            }
            if let Some(value) = redis.port {
                self.redis.port = value;
            }
            if let Some(value) = redis.db {
                self.redis.db = value;
            }
        }

        if let Some(super_peer) = file.super_peer {
            if let Some(value) = super_peer.quorum_size {
                self.super_peer.quorum_size = value;
            }
            if let Some(value) = super_peer.min_confirmations {
                self.super_peer.min_confirmations = value;
            }
            if let Some(value) = super_peer.key_rotation_days {
                self.super_peer.key_rotation_days = value;
            }
            if let Some(value) = super_peer.peers {
                self.super_peer.peers = value;
            }
        }

        if let Some(logging) = file.logging {
            if let Some(value) = logging.level {
                self.logging.level = value;
            }
            if let Some(value) = logging.json {
                self.logging.json = value;
            }
        }
    }

    fn apply_env_overrides(&mut self) {
        if let Ok(value) = env::var("GRPC_PORT") {
            if let Ok(value) = value.parse() {
                self.server.grpc_port = value;
            }
        }
        if let Ok(value) = env::var("HTTP_PORT") {
            if let Ok(value) = value.parse() {
                self.server.http_port = value;
            }
        }
        if let Ok(value) = env::var("NODE_ID") {
            self.server.node_id = value;
        }

        if let Ok(value) = env::var("DB_HOST") {
            self.database.host = value;
        }
        if let Ok(value) = env::var("DB_PORT") {
            if let Ok(value) = value.parse() {
                self.database.port = value;
            }
        }
        if let Ok(value) = env::var("DB_NAME") {
            self.database.name = value;
        }
        if let Ok(value) = env::var("DB_USER") {
            self.database.user = value;
        }
        if let Ok(value) = env::var("DB_PASSWORD") {
            self.database.password = value;
        }
        if let Ok(value) = env::var("DB_MAX_CONNECTIONS") {
            if let Ok(value) = value.parse() {
                self.database.max_connections = value;
            }
        }

        if let Ok(value) = env::var("REDIS_HOST") {
            self.redis.host = value;
        }
        if let Ok(value) = env::var("REDIS_PORT") {
            if let Ok(value) = value.parse() {
                self.redis.port = value;
            }
        }
        if let Ok(value) = env::var("REDIS_DB") {
            if let Ok(value) = value.parse() {
                self.redis.db = value;
            }
        }

        if let Ok(value) = env::var("SUPER_PEER_QUORUM_SIZE") {
            if let Ok(value) = value.parse() {
                self.super_peer.quorum_size = value;
            }
        }
        if let Ok(value) = env::var("SUPER_PEER_MIN_CONFIRMATIONS") {
            if let Ok(value) = value.parse() {
                self.super_peer.min_confirmations = value;
            }
        }
        if let Ok(value) = env::var("KEY_ROTATION_DAYS") {
            if let Ok(value) = value.parse() {
                self.super_peer.key_rotation_days = value;
            }
        }
        if let Ok(value) = env::var("SUPER_PEER_PEERS") {
            self.super_peer.peers = value
                .split(',')
                .map(|p| p.trim().to_string())
                .filter(|p| !p.is_empty())
                .collect();
        }

        if let Ok(value) = env::var("RUST_LOG") {
            self.logging.level = value;
        }
        if let Ok(value) = env::var("LOG_JSON") {
            self.logging.json = value.eq_ignore_ascii_case("true");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn config_file_overrides_defaults() {
        let path = temp_config_path("cs-node-config-file");
        fs::write(
            &path,
            r#"
environment = "staging"

[server]
grpc_port = 60051
node_id = "sp-basra"

[database]
password = "from-file"
max_connections = 22

[super_peer]
peers = ["sp-baghdad", "sp-erbil"]
"#,
        )
        .unwrap();

        let cfg = Config::load(&Some(path.to_string_lossy().to_string()), &None).unwrap();
        fs::remove_file(path).ok();

        assert_eq!(cfg.environment, Environment::Staging);
        assert_eq!(cfg.server.grpc_port, 60051);
        assert_eq!(cfg.server.http_port, 8080);
        assert_eq!(cfg.server.node_id, "sp-basra");
        assert_eq!(cfg.database.password, "from-file");
        assert_eq!(cfg.database.max_connections, 22);
        assert_eq!(
            cfg.super_peer.peers,
            vec!["sp-baghdad".to_string(), "sp-erbil".to_string()]
        );
    }

    #[test]
    fn cli_environment_overrides_file() {
        let path = temp_config_path("cs-node-config-env");
        fs::write(&path, "environment = \"development\"\n").unwrap();

        let cfg = Config::load(
            &Some(path.to_string_lossy().to_string()),
            &Some("production".to_string()),
        )
        .unwrap();
        fs::remove_file(path).ok();

        assert_eq!(cfg.environment, Environment::Production);
    }

    fn temp_config_path(prefix: &str) -> std::path::PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("{prefix}-{nanos}.toml"))
    }
}
