use clap::Subcommand;
use std::path::PathBuf;

#[derive(Subcommand)]
pub enum SystemCommands {
    /// Initialize Edda data directory
    Init,
    /// Create backup of data
    Backup,
    /// Restore from backup
    Restore { backup: PathBuf },
    /// Configuration management
    Config {
        #[command(subcommand)]
        subcommand: ConfigCommands,
    },
    /// Show system status
    Status,
    /// Clean up temporary files
    Cleanup,
}

#[derive(Subcommand)]
pub enum ConfigCommands {
    /// Show current configuration
    Show,
    /// Set configuration value
    Set { key: String, value: String },
    /// Get configuration value
    Get { key: String },
    /// Edit configuration file
    Edit,
    /// Validate configuration
    Validate,
    /// Reset configuration to defaults
    Reset,
}
