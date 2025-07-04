# Edda - Configuration Specification

## Overview

This specification defines the standardized configuration patterns, configuration structures, and configuration management for Edda. All components should reference this specification for consistent configuration handling across the application.

## Configuration Principles

### 1. Hierarchical Configuration

Configuration follows a hierarchical structure with clear precedence:

1. **Command-line arguments** (highest priority)
2. **Environment variables**
3. **Configuration files**
4. **Default values** (lowest priority)

### 2. Configuration Sources

```rust
#[derive(Debug, Clone)]
pub enum ConfigSource {
    /// Command-line argument
    CommandLine,
    /// Environment variable
    Environment,
    /// Configuration file
    ConfigFile(PathBuf),
    /// Default value
    Default,
}
```

### 3. Configuration Validation

All configuration values should be validated at load time:

```rust
pub trait ConfigValidator {
    fn validate(&self) -> Result<(), ConfigError>;
}
```

## Core Configuration Structure

### 1. Main Configuration

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EddaConfig {
    /// Application settings
    pub app: AppConfig,
    /// Storage configuration
    pub storage: StorageConfig,
    /// CLI configuration
    pub cli: CLIConfig,
    /// Logging configuration
    pub logging: LoggingConfig,
    /// AI integration configuration
    pub ai: AIConfig,
    /// Sync configuration
    pub sync: SyncConfig,
    /// Security configuration
    pub security: SecurityConfig,
}
```

### 2. Application Configuration

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// Application name
    pub name: String,
    /// Application version
    pub version: String,
    /// Data directory path
    pub data_dir: PathBuf,
    /// Cache directory path
    pub cache_dir: PathBuf,
    /// Temporary directory path
    pub temp_dir: PathBuf,
    /// Maximum memory usage (bytes)
    pub max_memory: Option<u64>,
    /// Enable debug mode
    pub debug: bool,
    /// Enable verbose logging
    pub verbose: bool,
}
```

### 3. Storage Configuration

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    /// Storage backend type
    pub backend: StorageBackend,
    /// Database file path (for SQLite)
    pub database_path: PathBuf,
    /// RocksDB directory (for RocksDB)
    pub rocksdb_path: Option<PathBuf>,
    /// File storage directory
    pub file_storage_path: PathBuf,
    /// Connection pool size
    pub connection_pool_size: usize,
    /// Cache configuration
    pub cache: CacheConfig,
    /// Backup configuration
    pub backup: BackupConfig,
    /// Encryption settings
    pub encryption: Option<EncryptionConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StorageBackend {
    SQLite,
    RocksDB,
    Hybrid,
}
```

### 4. CLI Configuration

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CLIConfig {
    /// Default output format
    pub default_format: OutputFormat,
    /// Enable color output
    pub color: bool,
    /// Enable progress indicators
    pub progress: bool,
    /// Command timeout (seconds)
    pub timeout: u64,
    /// Maximum concurrent operations
    pub max_concurrent: usize,
    /// Enable shell completion
    pub shell_completion: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutputFormat {
    Text,
    Json,
    Yaml,
    Table,
}
```

### 5. Logging Configuration

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log level
    pub level: LogLevel,
    /// Log file path
    pub file: Option<PathBuf>,
    /// Enable console output
    pub console: bool,
    /// Log format
    pub format: LogFormat,
    /// Enable structured logging
    pub structured: bool,
    /// Log rotation settings
    pub rotation: LogRotationConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogFormat {
    Simple,
    Detailed,
    Json,
}
```

### 6. AI Integration Configuration

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIConfig {
    /// Enable AI integration
    pub enabled: bool,
    /// Agent configuration
    pub agents: Vec<AgentConfig>,
    /// Context management
    pub context: ContextConfig,
    /// Workflow configuration
    pub workflow: WorkflowConfig,
    /// Communication settings
    pub communication: CommunicationConfig,
}
```

### 7. Sync Configuration

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncConfig {
    /// Enable synchronization
    pub enabled: bool,
    /// Sync interval (seconds)
    pub interval: u64,
    /// Remote storage configuration
    pub remote: RemoteConfig,
    /// Conflict resolution strategy
    pub conflict_strategy: ConflictStrategy,
    /// Compression settings
    pub compression: CompressionConfig,
    /// Retry configuration
    pub retry: RetryConfig,
}
```

### 8. Security Configuration

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Enable encryption
    pub encryption_enabled: bool,
    /// Encryption algorithm
    pub encryption_algorithm: EncryptionAlgorithm,
    /// Key derivation settings
    pub key_derivation: KeyDerivationConfig,
    /// Access control
    pub access_control: AccessControlConfig,
    /// Audit logging
    pub audit: AuditConfig,
}
```

## Configuration Loading

### 1. Configuration Loader

```rust
pub struct ConfigLoader {
    /// Configuration sources
    sources: Vec<ConfigSource>,
    /// Configuration validators
    validators: Vec<Box<dyn ConfigValidator>>,
}

impl ConfigLoader {
    /// Load configuration from all sources
    pub fn load(&self) -> Result<EddaConfig, ConfigError> {
        let mut config = EddaConfig::default();

        // Load from each source in order of precedence
        for source in &self.sources {
            self.load_from_source(&mut config, source)?;
        }

        // Validate configuration
        self.validate_config(&config)?;

        Ok(config)
    }

    /// Load configuration from specific source
    fn load_from_source(&self, config: &mut EddaConfig, source: &ConfigSource) -> Result<(), ConfigError> {
        match source {
            ConfigSource::CommandLine => self.load_from_command_line(config),
            ConfigSource::Environment => self.load_from_environment(config),
            ConfigSource::ConfigFile(path) => self.load_from_file(config, path),
            ConfigSource::Default => self.load_defaults(config),
        }
    }
}
```

### 2. Environment Variable Mapping

```rust
impl ConfigLoader {
    /// Load configuration from environment variables
    fn load_from_environment(&self, config: &mut EddaConfig) -> Result<(), ConfigError> {
        // App configuration
        if let Ok(data_dir) = std::env::var("EDDA_DATA_DIR") {
            config.app.data_dir = PathBuf::from(data_dir);
        }

        if let Ok(debug) = std::env::var("EDDA_DEBUG") {
            config.app.debug = debug.parse().unwrap_or(false);
        }

        // Storage configuration
        if let Ok(database_path) = std::env::var("EDDA_DATABASE_PATH") {
            config.storage.database_path = PathBuf::from(database_path);
        }

        // CLI configuration
        if let Ok(format) = std::env::var("EDDA_OUTPUT_FORMAT") {
            config.cli.default_format = format.parse().unwrap_or(OutputFormat::Text);
        }

        // Logging configuration
        if let Ok(level) = std::env::var("EDDA_LOG_LEVEL") {
            config.logging.level = level.parse().unwrap_or(LogLevel::Info);
        }

        Ok(())
    }
}
```

### 3. Configuration File Loading

```rust
impl ConfigLoader {
    /// Load configuration from file
    fn load_from_file(&self, config: &mut EddaConfig, path: &Path) -> Result<(), ConfigError> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| ConfigError::FileReadError(path.to_path_buf(), e))?;

        let file_config: EddaConfig = match path.extension().and_then(|s| s.to_str()) {
            Some("toml") => toml::from_str(&content)
                .map_err(|e| ConfigError::ParseError("TOML".to_string(), e.to_string()))?,
            Some("yaml") | Some("yml") => serde_yaml::from_str(&content)
                .map_err(|e| ConfigError::ParseError("YAML".to_string(), e.to_string()))?,
            Some("json") => serde_json::from_str(&content)
                .map_err(|e| ConfigError::ParseError("JSON".to_string(), e.to_string()))?,
            _ => return Err(ConfigError::UnsupportedFormat(path.to_path_buf())),
        };

        // Merge configuration
        self.merge_config(config, &file_config);

        Ok(())
    }
}
```

## Configuration Validation

### 1. Configuration Validator

```rust
impl ConfigValidator for EddaConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        // Validate app configuration
        self.app.validate()?;

        // Validate storage configuration
        self.storage.validate()?;

        // Validate CLI configuration
        self.cli.validate()?;

        // Validate logging configuration
        self.logging.validate()?;

        // Validate AI configuration
        self.ai.validate()?;

        // Validate sync configuration
        self.sync.validate()?;

        // Validate security configuration
        self.security.validate()?;

        Ok(())
    }
}
```

### 2. Component Validation

```rust
impl ConfigValidator for AppConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        // Validate data directory
        if !self.data_dir.exists() {
            std::fs::create_dir_all(&self.data_dir)
                .map_err(|e| ConfigError::DirectoryCreationError(self.data_dir.clone(), e))?;
        }

        // Validate cache directory
        if !self.cache_dir.exists() {
            std::fs::create_dir_all(&self.cache_dir)
                .map_err(|e| ConfigError::DirectoryCreationError(self.cache_dir.clone(), e))?;
        }

        // Validate memory limit
        if let Some(max_memory) = self.max_memory {
            if max_memory < 1024 * 1024 { // Minimum 1MB
                return Err(ConfigError::InvalidValue("max_memory".to_string(), "must be at least 1MB".to_string()));
            }
        }

        Ok(())
    }
}
```

## Configuration Management

### 1. Configuration Manager

```rust
pub struct ConfigManager {
    /// Current configuration
    config: Arc<RwLock<EddaConfig>>,
    /// Configuration loader
    loader: ConfigLoader,
    /// Configuration watchers
    watchers: Vec<Box<dyn ConfigWatcher>>,
}

impl ConfigManager {
    /// Initialize configuration manager
    pub fn new() -> Result<Self, ConfigError> {
        let loader = ConfigLoader::new()?;
        let config = Arc::new(RwLock::new(loader.load()?));

        Ok(Self {
            config,
            loader,
            watchers: Vec::new(),
        })
    }

    /// Reload configuration
    pub fn reload(&self) -> Result<(), ConfigError> {
        let new_config = self.loader.load()?;

        {
            let mut config = self.config.write().unwrap();
            *config = new_config;
        }

        // Notify watchers
        self.notify_watchers();

        Ok(())
    }

    /// Get current configuration
    pub fn get_config(&self) -> EddaConfig {
        self.config.read().unwrap().clone()
    }

    /// Update configuration
    pub fn update_config(&self, updates: &EddaConfig) -> Result<(), ConfigError> {
        let mut config = self.config.write().unwrap();

        // Merge updates
        self.merge_config(&mut config, updates);

        // Validate updated configuration
        config.validate()?;

        // Notify watchers
        self.notify_watchers();

        Ok(())
    }
}
```

### 2. Configuration Watching

```rust
pub trait ConfigWatcher: Send + Sync {
    fn on_config_changed(&self, config: &EddaConfig);
}

impl ConfigManager {
    /// Add configuration watcher
    pub fn add_watcher(&mut self, watcher: Box<dyn ConfigWatcher>) {
        self.watchers.push(watcher);
    }

    /// Notify all watchers
    fn notify_watchers(&self) {
        let config = self.get_config();
        for watcher in &self.watchers {
            watcher.on_config_changed(&config);
        }
    }
}
```

## Configuration File Formats

### 1. TOML Configuration

```toml
# edda.toml
[app]
name = "edda"
version = "0.1.0"
data_dir = "~/.edda"
cache_dir = "~/.edda/cache"
temp_dir = "~/.edda/temp"
debug = false
verbose = false

[storage]
backend = "sqlite"
database_path = "~/.edda/edda.db"
file_storage_path = "~/.edda/files"
connection_pool_size = 10

[cli]
default_format = "text"
color = true
progress = true
timeout = 30
max_concurrent = 4

[logging]
level = "info"
console = true
format = "detailed"
structured = true

[ai]
enabled = true

[sync]
enabled = false
interval = 300

[security]
encryption_enabled = false
```

### 2. YAML Configuration

```yaml
# edda.yaml
app:
  name: edda
  version: 0.1.0
  data_dir: ~/.edda
  cache_dir: ~/.edda/cache
  temp_dir: ~/.edda/temp
  debug: false
  verbose: false

storage:
  backend: sqlite
  database_path: ~/.edda/edda.db
  file_storage_path: ~/.edda/files
  connection_pool_size: 10

cli:
  default_format: text
  color: true
  progress: true
  timeout: 30
  max_concurrent: 4

logging:
  level: info
  console: true
  format: detailed
  structured: true

ai:
  enabled: true

sync:
  enabled: false
  interval: 300

security:
  encryption_enabled: false
```

## Environment Variables

### 1. Standard Environment Variables

```bash
# Application settings
export EDDA_DATA_DIR="/path/to/data"
export EDDA_DEBUG="true"
export EDDA_VERBOSE="true"

# Storage settings
export EDDA_DATABASE_PATH="/path/to/database.db"
export EDDA_FILE_STORAGE_PATH="/path/to/files"

# CLI settings
export EDDA_OUTPUT_FORMAT="json"
export EDDA_COLOR="true"
export EDDA_TIMEOUT="60"

# Logging settings
export EDDA_LOG_LEVEL="debug"
export EDDA_LOG_FILE="/path/to/logs/edda.log"

# AI settings
export EDDA_AI_ENABLED="true"

# Sync settings
export EDDA_SYNC_ENABLED="false"
export EDDA_SYNC_INTERVAL="300"

# Security settings
export EDDA_ENCRYPTION_ENABLED="false"
```

## Configuration Best Practices

### 1. Sensible Defaults

```rust
impl Default for EddaConfig {
    fn default() -> Self {
        Self {
            app: AppConfig {
                name: "edda".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
                data_dir: dirs::home_dir().unwrap().join(".edda"),
                cache_dir: dirs::cache_dir().unwrap().join("edda"),
                temp_dir: std::env::temp_dir().join("edda"),
                max_memory: None,
                debug: false,
                verbose: false,
            },
            storage: StorageConfig::default(),
            cli: CLIConfig::default(),
            logging: LoggingConfig::default(),
            ai: AIConfig::default(),
            sync: SyncConfig::default(),
            security: SecurityConfig::default(),
        }
    }
}
```

### 2. Configuration Validation

```rust
// Always validate configuration at load time
let config = ConfigLoader::new()?.load()?;
config.validate()?;
```

### 3. Configuration Documentation

```rust
/// Configuration documentation should be comprehensive
/// and include examples for all settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExampleConfig {
    /// The path to the data directory
    ///
    /// This directory stores all application data including
    /// databases, files, and configuration.
    ///
    /// Example: "/home/user/.edda"
    pub data_dir: PathBuf,
}
```

## Integration with Other Specs

This configuration specification should be referenced by:

- `SPEC_ARCHITECTURE.md` - For configuration architecture patterns
- `SPEC_STORAGE_ENGINE.md` - For storage configuration
- `SPEC_CLI_DESIGN.md` - For CLI configuration
- `SPEC_AI_INTEGRATION.md` - For AI integration configuration
- `SPEC_SYNC_ENGINE.md` - For sync configuration
- `SPEC_DEVELOPMENT.md` - For development configuration
- `SPEC_DEPLOYMENT.md` - For deployment configuration

All other specifications should reference this document instead of defining their own configuration patterns.
