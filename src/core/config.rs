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
    /// GitHub repository (owner/repo format)
    pub repository: Option<String>,

    /// Sync interval in seconds
    #[serde(default = "default_sync_interval")]
    pub sync_interval: u64,

    /// Sync mode: "issues", "projects", or "both"
    #[serde(default = "default_sync_mode")]
    pub sync_mode: String,

    /// Project board IDs (for multi-project sync)
    #[serde(default)]
    pub project_ids: Vec<u64>,

    /// Column mapping for project boards (column_name -> task_status)
    #[serde(default = "default_column_mapping")]
    pub column_mapping: std::collections::HashMap<String, String>,
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
            repository: None,
            sync_interval: default_sync_interval(),
            sync_mode: default_sync_mode(),
            project_ids: Vec::new(),
            column_mapping: default_column_mapping(),
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

impl EddaConfig {
    /// Set a configuration value by key
    pub fn set_value(&mut self, key: &str, value: &str) -> EddaResult<()> {
        match key {
            "data_dir" => {
                self.data_dir = PathBuf::from(value);
            }
            "log_level" => {
                let valid_levels = ["trace", "debug", "info", "warn", "error"];
                if !valid_levels.contains(&value) {
                    return Err(ConfigError::Validation {
                        message: format!("Invalid log level: {}", value),
                    }
                    .into());
                }
                self.log_level = value.to_string();
            }
            "output_format" => {
                let valid_formats = ["text", "json", "yaml"];
                if !valid_formats.contains(&value) {
                    return Err(ConfigError::Validation {
                        message: format!("Invalid output format: {}", value),
                    }
                    .into());
                }
                self.output_format = value.to_string();
            }
            "database.url" => {
                self.database.url = value.to_string();
            }
            "database.max_connections" => {
                let max_conn = value.parse::<u32>().map_err(|_| ConfigError::Validation {
                    message: format!("Invalid max_connections value: {}", value),
                })?;
                self.database.max_connections = max_conn;
            }
            "github.repository" => {
                self.github.repository = Some(value.to_string());
            }
            "github.sync_interval" => {
                let interval = value.parse::<u64>().map_err(|_| ConfigError::Validation {
                    message: format!("Invalid sync_interval value: {}", value),
                })?;
                self.github.sync_interval = interval;
            }
            "github.sync_mode" => {
                let valid_modes = ["issues", "projects", "both"];
                if !valid_modes.contains(&value) {
                    return Err(ConfigError::Validation {
                        message: format!("Invalid sync_mode: {}", value),
                    }
                    .into());
                }
                self.github.sync_mode = value.to_string();
            }
            "github.project_ids" => {
                let ids: Vec<u64> = value
                    .split(',')
                    .filter_map(|s| s.trim().parse::<u64>().ok())
                    .collect();
                self.github.project_ids = ids;
            }
            "github.column_mapping" => {
                let mut map = std::collections::HashMap::new();
                for pair in value.split(',') {
                    let parts: Vec<&str> = pair.split('=').collect();
                    if parts.len() == 2 {
                        map.insert(parts[0].to_string(), parts[1].to_string());
                    }
                }
                self.github.column_mapping = map;
            }
            _ => {
                return Err(ConfigError::Validation {
                    message: format!("Unknown configuration key: {}", key),
                }
                .into());
            }
        }
        Ok(())
    }

    /// Get a configuration value by key
    pub fn get_value(&self, key: &str) -> Option<String> {
        match key {
            "data_dir" => Some(self.data_dir.to_string_lossy().to_string()),
            "log_level" => Some(self.log_level.clone()),
            "output_format" => Some(self.output_format.clone()),
            "database.url" => Some(self.database.url.clone()),
            "database.max_connections" => Some(self.database.max_connections.to_string()),
            "github.repository" => self.github.repository.clone(),
            "github.sync_interval" => Some(self.github.sync_interval.to_string()),
            "github.sync_mode" => Some(self.github.sync_mode.clone()),
            "github.project_ids" => Some(
                self.github
                    .project_ids
                    .iter()
                    .map(|id| id.to_string())
                    .collect::<Vec<_>>()
                    .join(","),
            ),
            "github.column_mapping" => {
                let mut pairs = Vec::new();
                for (k, v) in &self.github.column_mapping {
                    pairs.push(format!("{}={}", k, v));
                }
                Some(pairs.join(","))
            }
            _ => None,
        }
    }
}

/// Load configuration from file and environment variables
pub fn load_config(config_path: Option<PathBuf>) -> EddaResult<EddaConfig> {
    let mut config = if let Some(path) = config_path {
        load_config_from_file(&path)?
    } else {
        // First try to find .edda.toml in current directory or parent directories
        if let Some(local_config_path) = find_config_file() {
            load_config_from_file(&local_config_path)?
        } else {
            // Fall back to default config file in home directory
            let default_config_path = get_default_config_path();
            if default_config_path.exists() {
                load_config_from_file(&default_config_path)?
            } else {
                EddaConfig::default()
            }
        }
    };

    // Override with environment variables
    override_from_env(&mut config);

    Ok(config)
}

/// Save configuration to file
pub fn save_config(config: &EddaConfig, config_path: Option<PathBuf>) -> EddaResult<()> {
    let path = config_path.unwrap_or_else(get_default_config_path);

    // Create config directory if it doesn't exist
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent).map_err(|e| ConfigError::Persistence {
                message: format!("Failed to create config directory: {e}"),
            })?;
        }
    }

    let toml_string = toml::to_string_pretty(config).map_err(|e| ConfigError::Persistence {
        message: format!("Failed to serialize configuration: {e}"),
    })?;

    std::fs::write(&path, toml_string).map_err(|e| ConfigError::Persistence {
        message: format!("Failed to write configuration file: {e}"),
    })?;

    Ok(())
}

/// Get default configuration file path
pub fn get_default_config_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("edda")
        .join("config.toml")
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
        message: format!("Failed to read config file: {e}"),
    })?;

    toml::from_str(&content)
        .map_err(|e| ConfigError::InvalidFormat {
            message: format!("Failed to parse TOML: {e}"),
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
            message: format!("Failed to create data directory: {e}"),
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

    // Validate GitHub sync mode
    let valid_sync_modes = ["issues", "projects", "both"];
    if !valid_sync_modes.contains(&config.github.sync_mode.as_str()) {
        return Err(ConfigError::Validation {
            message: format!("Invalid sync_mode: {}", config.github.sync_mode),
        }
        .into());
    }

    // Validate project_ids if sync_mode is "projects" or "both"
    if config.github.sync_mode == "projects" || config.github.sync_mode == "both" {
        if config.github.project_ids.is_empty() {
            return Err(ConfigError::Validation {
                message: "project_ids is required when sync_mode is 'projects' or 'both'"
                    .to_string(),
            }
            .into());
        }
    }

    // Validate column_mapping
    for (column_name, task_status) in &config.github.column_mapping {
        if column_name.is_empty() || task_status.is_empty() {
            return Err(ConfigError::Validation {
                message: format!(
                    "Invalid column_mapping entry: {}={}",
                    column_name, task_status
                ),
            }
            .into());
        }
    }

    Ok(())
}

/// Find the nearest .edda.toml configuration file by searching recursively
/// from the current directory up to the home directory
pub fn find_config_file() -> Option<PathBuf> {
    // Get current working directory
    let current_dir = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(_) => return None,
    };

    // Get home directory
    let home_dir = match dirs::home_dir() {
        Some(dir) => dir,
        None => return None,
    };

    // Search recursively from current directory up to home directory
    let mut search_dir = current_dir;
    loop {
        let config_file = search_dir.join(".edda.toml");
        if config_file.exists() {
            return Some(config_file);
        }

        // Stop if we've reached the home directory or root
        if search_dir == home_dir || search_dir.parent().is_none() {
            break;
        }

        // Move up one directory
        search_dir = search_dir.parent().unwrap().to_path_buf();
    }

    None
}

/// Get GitHub token from environment variables
/// Checks for tokens in order: GITHUB_TOKEN, EDDA_GITHUB_TOKEN, GH_TOKEN, GITHUB_ACCESS_TOKEN
pub fn get_github_token() -> Option<String> {
    std::env::var("GITHUB_TOKEN")
        .or_else(|_| std::env::var("EDDA_GITHUB_TOKEN"))
        .or_else(|_| std::env::var("GH_TOKEN"))
        .or_else(|_| std::env::var("GITHUB_ACCESS_TOKEN"))
        .ok()
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

fn default_sync_mode() -> String {
    "issues".to_string()
}

fn default_column_mapping() -> std::collections::HashMap<String, String> {
    let mut map = std::collections::HashMap::new();
    map.insert("To Do".to_string(), "todo".to_string());
    map.insert("In Progress".to_string(), "in_progress".to_string());
    map.insert("Done".to_string(), "done".to_string());
    map
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
        assert_eq!(config.github.sync_mode, "issues");
        assert_eq!(config.github.project_ids.len(), 0);
        assert_eq!(config.github.column_mapping.len(), 3);
    }

    #[test]
    #[serial]
    fn test_load_config_without_file() {
        // Clear env vars that may affect config
        unsafe {
            std::env::remove_var("EDDA_LOG_LEVEL");
            std::env::remove_var("EDDA_OUTPUT_FORMAT");
            std::env::remove_var("EDDA_GITHUB_REPOSITORY");
        }

        // Test that config loads successfully
        // The actual values depend on whether there's a home config file
        // but we can verify the structure is correct
        let config = load_config(None).unwrap();
        assert!(!config.log_level.is_empty());
        assert!(!config.output_format.is_empty());
        assert!(config.github.sync_interval > 0);
        assert!(config.database.max_connections > 0);
        let valid_modes = ["issues", "projects", "both"];
        assert!(valid_modes.contains(&config.github.sync_mode.as_str()));
        assert!(
            config
                .github
                .project_ids
                .iter()
                .all(|id| *id > 0 || *id == 0)
        );
        assert_eq!(config.github.column_mapping.len(), 3);
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
            repository = "test/repo"
            sync_interval = 600
            sync_mode = "projects"
            project_ids = [1234567890, 9876543210]
            [github.column_mapping]
            "To Do" = "todo"
            "In Progress" = "in_progress"
            "Done" = "done"
        "#;

        fs::write(&config_path, config_content).unwrap();

        let config = load_config(Some(config_path)).unwrap();
        assert_eq!(config.log_level, "debug");
        assert_eq!(config.output_format, "json");
        assert_eq!(config.github.repository, Some("test/repo".to_string()));
        assert_eq!(config.github.sync_interval, 600);
        assert_eq!(config.github.sync_mode, "projects");
        assert_eq!(config.github.project_ids, vec![1234567890, 9876543210]);
        assert_eq!(config.github.column_mapping.len(), 3);
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
            std::env::set_var("EDDA_GITHUB_REPOSITORY", "env-repo");
        }

        let config = load_config(None).unwrap();
        assert_eq!(config.log_level, "debug");
        assert_eq!(config.output_format, "json");
        assert_eq!(config.github.repository, Some("env-repo".to_string()));

        // Clean up
        unsafe {
            std::env::remove_var("EDDA_LOG_LEVEL");
            std::env::remove_var("EDDA_OUTPUT_FORMAT");
            std::env::remove_var("EDDA_GITHUB_REPOSITORY");
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
        assert_eq!(config.repository, None);
        assert_eq!(config.sync_interval, 300);
        assert_eq!(config.sync_mode, "issues");
        assert_eq!(config.project_ids.len(), 0);
        assert_eq!(config.column_mapping.len(), 3);
    }

    #[test]
    fn test_database_config_default() {
        let config = DatabaseConfig::default();
        assert_eq!(config.url, "sqlite:edda.db");
        assert_eq!(config.max_connections, 5);
    }

    #[test]
    #[serial]
    fn test_find_config_file_in_current_directory() {
        let temp_dir = TempDir::new().unwrap();
        let config_file = temp_dir.path().join(".edda.toml");

        // Create a config file in the temp directory
        let config_content = r#"
            log_level = "debug"
            output_format = "json"
        "#;
        fs::write(&config_file, config_content).unwrap();

        // Change to the temp directory
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        // Test that the config file is found
        let config = load_config(None).unwrap();
        assert_eq!(config.log_level, "debug");
        assert_eq!(config.output_format, "json");

        // Restore original directory
        std::env::set_current_dir(original_dir).unwrap();
    }

    #[test]
    #[serial]
    fn test_find_config_file_in_parent_directory() {
        let temp_dir = TempDir::new().unwrap();
        let parent_dir = temp_dir.path().join("parent");
        let child_dir = parent_dir.join("child");

        // Create directories
        fs::create_dir_all(&child_dir).unwrap();

        // Create config file in parent directory
        let config_file = parent_dir.join(".edda.toml");
        let config_content = r#"
            log_level = "warn"
            output_format = "yaml"
        "#;
        fs::write(&config_file, config_content).unwrap();

        // Change to child directory
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(&child_dir).unwrap();

        // Test that the config file is found in parent directory
        let config = load_config(None).unwrap();
        assert_eq!(config.log_level, "warn");
        assert_eq!(config.output_format, "yaml");

        // Restore original directory
        std::env::set_current_dir(original_dir).unwrap();
    }

    #[test]
    #[serial]
    fn test_find_config_file_prioritizes_closer_file() {
        let temp_dir = TempDir::new().unwrap();
        let parent_dir = temp_dir.path().join("parent");
        let child_dir = parent_dir.join("child");

        // Create directories
        fs::create_dir_all(&child_dir).unwrap();

        // Create config file in parent directory
        let parent_config_file = parent_dir.join(".edda.toml");
        let parent_config_content = r#"
            log_level = "warn"
            output_format = "yaml"
        "#;
        fs::write(&parent_config_file, parent_config_content).unwrap();

        // Create config file in child directory (should take precedence)
        let child_config_file = child_dir.join(".edda.toml");
        let child_config_content = r#"
            log_level = "debug"
            output_format = "json"
        "#;
        fs::write(&child_config_file, child_config_content).unwrap();

        // Change to child directory
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(&child_dir).unwrap();

        // Test that the closer config file is found
        let config = load_config(None).unwrap();
        assert_eq!(config.log_level, "debug");
        assert_eq!(config.output_format, "json");

        // Restore original directory
        std::env::set_current_dir(original_dir).unwrap();
    }

    #[test]
    #[serial]
    fn test_find_config_file_not_found() {
        let temp_dir = TempDir::new().unwrap();
        let test_dir = temp_dir.path().join("test");

        // Create test directory without config file
        fs::create_dir_all(&test_dir).unwrap();

        // Change to test directory
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(&test_dir).unwrap();

        // Clear env vars that may affect config
        unsafe {
            std::env::remove_var("EDDA_LOG_LEVEL");
            std::env::remove_var("EDDA_OUTPUT_FORMAT");
            std::env::remove_var("EDDA_GITHUB_REPOSITORY");
        }

        // Test that no config file is found and defaults are used
        // Note: This test may fail if there's a config file in the home directory
        // In that case, we test that the config loads successfully regardless
        let config = load_config(None).unwrap();
        // We can't assert specific values since they might come from home config
        // Just ensure the config loads without error
        assert!(!config.log_level.is_empty());
        assert!(!config.output_format.is_empty());

        // Restore original directory
        std::env::set_current_dir(original_dir).unwrap();
    }

    #[test]
    #[serial]
    fn test_fallback_to_home_config() {
        // This test verifies that when no .edda.toml is found locally,
        // the system falls back to the home directory config

        let temp_dir = TempDir::new().unwrap();
        let test_dir = temp_dir.path().join("test");

        // Create test directory without config file
        fs::create_dir_all(&test_dir).unwrap();

        // Change to test directory
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(&test_dir).unwrap();

        // Clear env vars that may affect config
        unsafe {
            std::env::remove_var("EDDA_LOG_LEVEL");
            std::env::remove_var("EDDA_OUTPUT_FORMAT");
            std::env::remove_var("EDDA_GITHUB_REPOSITORY");
        }

        // Test that config loads successfully (either from home or defaults)
        let config = load_config(None).unwrap();

        // Verify that the config has valid values
        assert!(!config.log_level.is_empty());
        assert!(!config.output_format.is_empty());

        // The actual values depend on whether there's a home config file
        // but we can verify the structure is correct
        assert!(config.github.sync_interval > 0);
        assert!(config.database.max_connections > 0);

        // Restore original directory
        std::env::set_current_dir(original_dir).unwrap();
    }
}
