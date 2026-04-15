use thiserror::Error;

#[derive(Error, Debug)]
pub enum CylinderSealError {
    #[error("Cryptographic error: {0}")]
    CryptoError(String),

    #[error("Invalid signature")]
    InvalidSignature,

    #[error("Invalid hash")]
    InvalidHash,

    #[error("Double spend detected")]
    DoubleSpend,

    #[error("Insufficient balance: required {required}, available {available}")]
    InsufficientBalance { required: i64, available: i64 },

    #[error("Invalid transaction: {0}")]
    InvalidTransaction(String),

    #[error("Invalid journal entry: {0}")]
    InvalidEntry(String),

    #[error("Out of sequence: expected {expected}, got {got}")]
    OutOfSequence { expected: u64, got: u64 },

    #[error("Entry not found")]
    EntryNotFound,

    #[error("User not found")]
    UserNotFound,

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("KYC tier limit exceeded")]
    KYCTierLimitExceeded,

    #[error("Replay attack detected")]
    ReplayDetected,

    #[error("Invalid KYC tier")]
    InvalidKYCTier,

    #[error("Invalid nonce: {0}")]
    InvalidNonce(String),

    #[error("Device ID mismatch: {0}")]
    DeviceIdMismatch(String),

    #[error("Device compromised (jailbroken/rooted)")]
    DeviceCompromised,

    #[error("Internal error: {0}")]
    InternalError(String),
}

pub type Result<T> = std::result::Result<T, CylinderSealError>;
