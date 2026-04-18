//! Error types for analytics operations

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Project not found: {0}")]
    ProjectNotFound(uuid::Uuid),

    #[error("Invalid sector: {0}")]
    InvalidSector(String),

    #[error("Invalid period format: {0}")]
    InvalidPeriod(String),

    #[error("Computation error: {0}")]
    ComputationError(String),

    #[error("No data available for period: {0}")]
    NoDataAvailable(String),
}

pub type Result<T> = std::result::Result<T, Error>;
