// PostgreSQL connection and utilities
use sqlx::postgres::PgPool;

pub type Database = PgPool;

#[derive(Clone)]
pub struct PostgresConfig {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: String,
    pub max_connections: u32,
}

impl PostgresConfig {
    pub fn from_env() -> Self {
        Self {
            host: std::env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string()),
            port: std::env::var("DB_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(5432),
            database: std::env::var("DB_NAME").unwrap_or_else(|_| "cylinder_seal".to_string()),
            username: std::env::var("DB_USER").unwrap_or_else(|_| "postgres".to_string()),
            password: std::env::var("DB_PASSWORD").unwrap_or_else(|_| "password".to_string()),
            max_connections: std::env::var("DB_MAX_CONNECTIONS")
                .ok()
                .and_then(|c| c.parse().ok())
                .unwrap_or(10),
        }
    }

    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database
        )
    }
}

pub async fn connect(config: &PostgresConfig) -> Result<Database, sqlx::Error> {
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(config.max_connections)
        .connect(&config.connection_string())
        .await
}
