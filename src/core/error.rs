use thiserror::Error;

/// Core error types for Edda
#[derive(Error, Debug)]
pub enum EddaError {
    #[error("Task error: {0}")]
    Task(#[from] TaskError),

    #[error("Storage error: {0}")]
    Storage(#[from] StorageError),

    #[error("Configuration error: {0}")]
    Config(#[from] ConfigError),

    #[error("Sync error: {0}")]
    Sync(#[from] SyncError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Logging error: {0}")]
    Logging(String),
}

/// Task-specific errors
#[derive(Error, Debug)]
pub enum TaskError {
    #[error("Task not found: {id}")]
    NotFound { id: String },

    #[error("Invalid task status transition: {from} -> {to}")]
    InvalidStatusTransition { from: String, to: String },

    #[error("Task validation failed: {message}")]
    Validation { message: String },

    #[error("Task already exists: {id}")]
    AlreadyExists { id: String },
}

/// Storage-specific errors
#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Database connection failed: {message}")]
    Connection { message: String },

    #[error("Database migration failed: {message}")]
    Migration { message: String },

    #[error("Data corruption detected: {message}")]
    Corruption { message: String },

    #[error("Storage initialization failed: {message}")]
    Initialization { message: String },
}

/// Configuration-specific errors
#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Configuration file not found: {path}")]
    FileNotFound { path: String },

    #[error("Invalid configuration format: {message}")]
    InvalidFormat { message: String },

    #[error("Missing required configuration: {key}")]
    MissingRequired { key: String },

    #[error("Configuration validation failed: {message}")]
    Validation { message: String },
}

/// Sync-specific errors
#[derive(Error, Debug)]
pub enum SyncError {
    #[error("Sync provider not found: {provider}")]
    ProviderNotFound { provider: String },

    #[error("Authentication failed: {message}")]
    Authentication { message: String },

    #[error("Network error: {message}")]
    Network { message: String },

    #[error("Conflict resolution failed: {message}")]
    Conflict { message: String },
}

/// Result type for Edda operations
pub type EddaResult<T> = Result<T, EddaError>;
