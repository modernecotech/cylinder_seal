use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Environment {
    Development,
    Staging,
    Production,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub environment: Environment,
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
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

impl Config {
    pub fn load(config_path: &Option<String>, environment: &Option<String>) -> Result<Self> {
        // TODO: load from config file or environment variables
        // Default development config for now
        Ok(Self {
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
                password: "password".to_string(),
                max_connections: 10,
            },
            redis: RedisConfig {
                host: "localhost".to_string(),
                port: 6379,
                db: 0,
            },
        })
    }
}
