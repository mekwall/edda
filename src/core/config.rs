use crate::core::error::{ConfigError, EddaResult};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Main configuration structure for Edda
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EddaConfig {
    /// Data directory for storing Edda data
    #[serde(default = "default_data_dir")]
    pub data_dir: PathBuf,

    /// Log level for tracing
    #[serde(default = "default_log_level")]
    pub log_level: String,

    /// Output format (text, json, yaml)
    #[serde(default = "default_output_format")]
    pub output_format: String,

    /// GitHub sync configuration
    #[serde(default)]
    pub github: GitHubConfig,

    /// Database configuration
    #[serde(default)]
    pub database: DatabaseConfig,
}

/// GitHub-specific configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubConfig {
    /// GitHub personal access token
    pub token: Option<String>,

    /// GitHub repository (owner/repo format)
    pub repository: Option<String>,

    /// Sync interval in seconds
    #[serde(default = "default_sync_interval")]
    pub sync_interval: u64,
}

/// Database-specific configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    /// Database URL (for SQLite, this is the file path)
    #[serde(default = "default_database_url")]
    pub url: String,

    /// Maximum number of database connections
    #[serde(default = "default_max_connections")]
    pub max_connections: u32,
}

impl Default for EddaConfig {
    fn default() -> Self {
        Self {
            data_dir: default_data_dir(),
            log_level: default_log_level(),
            output_format: default_output_format(),
            github: GitHubConfig::default(),
            database: DatabaseConfig::default(),
        }
    }
}

impl Default for GitHubConfig {
    fn default() -> Self {
        Self {
            token: None,
            repository: None,
            sync_interval: default_sync_interval(),
        }
    }
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: default_database_url(),
            max_connections: default_max_connections(),
        }
    }
}

/// Load configuration from file and environment variables
pub fn load_config(config_path: Option<PathBuf>) -> EddaResult<EddaConfig> {
    let mut config = if let Some(path) = config_path {
        load_config_from_file(&path)?
    } else {
        EddaConfig::default()
    };

    // Override with environment variables
    override_from_env(&mut config);

    Ok(config)
}

/// Load configuration from TOML file
fn load_config_from_file(path: &PathBuf) -> EddaResult<EddaConfig> {
    if !path.exists() {
        return Err(ConfigError::FileNotFound {
            path: path.to_string_lossy().to_string(),
        }
        .into());
    }

    let content = std::fs::read_to_string(path).map_err(|e| ConfigError::InvalidFormat {
        message: format!("Failed to read config file: {}", e),
    })?;

    toml::from_str(&content)
        .map_err(|e| ConfigError::InvalidFormat {
            message: format!("Failed to parse TOML: {}", e),
        })
        .map_err(Into::into)
}

/// Override configuration with environment variables
fn override_from_env(config: &mut EddaConfig) {
    if let Ok(data_dir) = std::env::var("EDDA_DATA_DIR") {
        config.data_dir = PathBuf::from(data_dir);
    }

    if let Ok(log_level) = std::env::var("EDDA_LOG_LEVEL") {
        config.log_level = log_level;
    }

    if let Ok(output_format) = std::env::var("EDDA_OUTPUT_FORMAT") {
        config.output_format = output_format;
    }

    if let Ok(token) = std::env::var("EDDA_GITHUB_TOKEN") {
        config.github.token = Some(token);
    }

    if let Ok(repo) = std::env::var("EDDA_GITHUB_REPOSITORY") {
        config.github.repository = Some(repo);
    }

    if let Ok(db_url) = std::env::var("EDDA_DATABASE_URL") {
        config.database.url = db_url;
    }
}

/// Validate configuration
pub fn validate_config(config: &EddaConfig) -> EddaResult<()> {
    // Validate data directory
    if !config.data_dir.exists() {
        std::fs::create_dir_all(&config.data_dir).map_err(|e| ConfigError::Validation {
            message: format!("Failed to create data directory: {}", e),
        })?;
    }

    // Validate log level
    let valid_log_levels = ["trace", "debug", "info", "warn", "error"];
    if !valid_log_levels.contains(&config.log_level.as_str()) {
        return Err(ConfigError::Validation {
            message: format!("Invalid log level: {}", config.log_level),
        }
        .into());
    }

    // Validate output format
    let valid_formats = ["text", "json", "yaml"];
    if !valid_formats.contains(&config.output_format.as_str()) {
        return Err(ConfigError::Validation {
            message: format!("Invalid output format: {}", config.output_format),
        }
        .into());
    }

    Ok(())
}

// Default value functions
fn default_data_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("edda")
}

fn default_log_level() -> String {
    "info".to_string()
}

fn default_output_format() -> String {
    "text".to_string()
}

fn default_sync_interval() -> u64 {
    300 // 5 minutes
}

fn default_database_url() -> String {
    "sqlite:edda.db".to_string()
}

fn default_max_connections() -> u32 {
    5
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    #[serial]
    fn test_default_config() {
        let config = EddaConfig::default();
        assert_eq!(config.log_level, "info");
        assert_eq!(config.output_format, "text");
        assert_eq!(config.github.sync_interval, 300);
        assert_eq!(config.database.max_connections, 5);
    }

    #[test]
    #[serial]
    fn test_load_config_without_file() {
        // Clear env vars that may affect config
        unsafe {
            std::env::remove_var("EDDA_LOG_LEVEL");
            std::env::remove_var("EDDA_OUTPUT_FORMAT");
            std::env::remove_var("EDDA_GITHUB_TOKEN");
        }
        let config = load_config(None).unwrap();
        assert_eq!(config.log_level, "info");
        assert_eq!(config.output_format, "text");
    }

    #[test]
    #[serial]
    fn test_load_config_from_file() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("test.toml");

        let config_content = r#"
            log_level = "debug"
            output_format = "json"
            [github]
            token = "test-token"
            repository = "test/repo"
            sync_interval = 600
        "#;

        fs::write(&config_path, config_content).unwrap();

        let config = load_config(Some(config_path)).unwrap();
        assert_eq!(config.log_level, "debug");
        assert_eq!(config.output_format, "json");
        assert_eq!(config.github.token, Some("test-token".to_string()));
        assert_eq!(config.github.repository, Some("test/repo".to_string()));
        assert_eq!(config.github.sync_interval, 600);
    }

    #[test]
    #[serial]
    fn test_load_config_file_not_found() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("nonexistent.toml");

        let result = load_config(Some(config_path));
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            crate::core::EddaError::Config(ConfigError::FileNotFound { .. })
        ));
    }

    #[test]
    #[serial]
    fn test_load_config_invalid_toml() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("invalid.toml");

        fs::write(&config_path, "invalid toml content").unwrap();

        let result = load_config(Some(config_path));
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            crate::core::EddaError::Config(ConfigError::InvalidFormat { .. })
        ));
    }

    #[test]
    #[serial]
    fn test_environment_variable_override() {
        unsafe {
            std::env::set_var("EDDA_LOG_LEVEL", "debug");
            std::env::set_var("EDDA_OUTPUT_FORMAT", "json");
            std::env::set_var("EDDA_GITHUB_TOKEN", "env-token");
        }

        let config = load_config(None).unwrap();
        assert_eq!(config.log_level, "debug");
        assert_eq!(config.output_format, "json");
        assert_eq!(config.github.token, Some("env-token".to_string()));

        // Clean up
        unsafe {
            std::env::remove_var("EDDA_LOG_LEVEL");
            std::env::remove_var("EDDA_OUTPUT_FORMAT");
            std::env::remove_var("EDDA_GITHUB_TOKEN");
        }
    }

    #[test]
    fn test_validate_config_success() {
        let temp_dir = TempDir::new().unwrap();
        let mut config = EddaConfig::default();
        config.data_dir = temp_dir.path().to_path_buf();

        let result = validate_config(&config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_config_invalid_log_level() {
        let mut config = EddaConfig::default();
        config.log_level = "invalid".to_string();

        let result = validate_config(&config);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            crate::core::EddaError::Config(ConfigError::Validation { .. })
        ));
    }

    #[test]
    fn test_validate_config_invalid_output_format() {
        let mut config = EddaConfig::default();
        config.output_format = "invalid".to_string();

        let result = validate_config(&config);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            crate::core::EddaError::Config(ConfigError::Validation { .. })
        ));
    }

    #[test]
    fn test_github_config_default() {
        let config = GitHubConfig::default();
        assert_eq!(config.token, None);
        assert_eq!(config.repository, None);
        assert_eq!(config.sync_interval, 300);
    }

    #[test]
    fn test_database_config_default() {
        let config = DatabaseConfig::default();
        assert_eq!(config.url, "sqlite:edda.db");
        assert_eq!(config.max_connections, 5);
    }
}
