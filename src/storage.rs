pub mod database;

use crate::core::{EddaConfig, EddaResult};
use std::path::PathBuf;

pub use database::{get_pool, init_database};

/// Storage manager for Edda
pub struct StorageManager {
    config: EddaConfig,
    data_dir: PathBuf,
}

impl StorageManager {
    /// Create a new storage manager
    pub fn new(config: EddaConfig) -> EddaResult<Self> {
        let data_dir = config.data_dir.clone();

        // Ensure data directory exists
        std::fs::create_dir_all(&data_dir).map_err(|e| {
            crate::core::StorageError::Initialization {
                message: format!("Failed to create data directory: {}", e),
            }
        })?;

        // Create subdirectories
        let subdirs = ["db", "logs", "backups", "cache"];
        for subdir in subdirs {
            let subdir_path = data_dir.join(subdir);
            std::fs::create_dir_all(&subdir_path).map_err(|e| {
                crate::core::StorageError::Initialization {
                    message: format!("Failed to create subdirectory {}: {}", subdir, e),
                }
            })?;
        }

        Ok(Self { config, data_dir })
    }

    /// Get the data directory path
    pub fn data_dir(&self) -> &PathBuf {
        &self.data_dir
    }

    /// Get the database path
    pub fn database_path(&self) -> PathBuf {
        self.data_dir.join("db").join("edda.db")
    }

    /// Get the logs directory path
    pub fn logs_dir(&self) -> PathBuf {
        self.data_dir.join("logs")
    }

    /// Get the backups directory path
    pub fn backups_dir(&self) -> PathBuf {
        self.data_dir.join("backups")
    }

    /// Get the cache directory path
    pub fn cache_dir(&self) -> PathBuf {
        self.data_dir.join("cache")
    }

    /// Initialize the database
    pub async fn init_database(&self) -> EddaResult<()> {
        database::init_database(self.database_path()).await
    }

    /// Check if the storage is properly initialized
    pub fn is_initialized(&self) -> bool {
        self.data_dir.exists()
            && self.data_dir.join("db").exists()
            && self.data_dir.join("logs").exists()
            && self.data_dir.join("backups").exists()
            && self.data_dir.join("cache").exists()
    }
}
