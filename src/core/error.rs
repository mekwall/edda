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
#[derive(Debug, thiserror::Error)]
pub enum TaskError {
    #[error("Task not found: {id}")]
    NotFound { id: String },

    #[error("Invalid status transition from {from} to {to}")]
    InvalidStatusTransition { from: String, to: String },

    #[error("Validation error: {message}")]
    Validation { message: String },

    #[error("Task already exists: {id}")]
    AlreadyExists { id: String },

    #[error("Storage error: {message}")]
    Storage { message: String },
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

    #[error("Backup error: {message}")]
    Backup { message: String },
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_error_display() {
        let error = TaskError::NotFound {
            id: "123".to_string(),
        };
        assert_eq!(error.to_string(), "Task not found: 123");
    }

    #[test]
    fn test_storage_error_display() {
        let error = StorageError::Connection {
            message: "test".to_string(),
        };
        assert_eq!(error.to_string(), "Database connection failed: test");
    }

    #[test]
    fn test_config_error_display() {
        let error = ConfigError::FileNotFound {
            path: "/test.toml".to_string(),
        };
        assert_eq!(
            error.to_string(),
            "Configuration file not found: /test.toml"
        );
    }

    #[test]
    fn test_sync_error_display() {
        let error = SyncError::ProviderNotFound {
            provider: "github".to_string(),
        };
        assert_eq!(error.to_string(), "Sync provider not found: github");
    }

    #[test]
    fn test_error_conversion() {
        let task_error = TaskError::NotFound {
            id: "123".to_string(),
        };
        let edda_error: EddaError = task_error.into();
        assert!(matches!(edda_error, EddaError::Task(_)));
    }

    #[test]
    fn test_edda_result_type() {
        let result: EddaResult<String> = Ok("test".to_string());
        assert!(result.is_ok());

        let error_result: EddaResult<String> = Err(EddaError::Logging("test error".to_string()));
        assert!(error_result.is_err());
    }
}
