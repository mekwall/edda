mod config;
mod doc;
mod state;
mod sync;
mod system;
mod task;

use crate::core::{EddaConfig, EddaError, EddaResult, init_logging, load_config, validate_config};

pub use doc::DocCommands;
pub use state::StateCommands;
pub use sync::{GitHubSyncCommands, SyncCommands};
pub use system::{ConfigCommands, SystemCommands};
pub use task::TaskCommands;

use clap::Parser;
use std::path::PathBuf;

/// Edda: AI agent-native CLI for structured task and document management
#[derive(Parser)]
#[command(name = "edda")]
#[command(about = "AI agent-native CLI for structured task and document management")]
#[command(version)]
#[command(propagate_version = true)]
pub struct Cli {
    /// Configuration file path
    #[arg(long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// Data directory path
    #[arg(long, value_name = "DIR")]
    pub data_dir: Option<PathBuf>,

    /// Output format (text, json, yaml)
    #[arg(long, default_value = "text")]
    pub format: Option<String>,

    /// Suppress non-error output
    #[arg(long)]
    pub quiet: bool,

    /// Enable verbose logging
    #[arg(long)]
    pub verbose: bool,

    /// Subcommand to run
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(clap::Subcommand)]
pub enum Commands {
    /// Task management commands
    Task {
        #[command(subcommand)]
        subcommand: TaskCommands,
    },
    /// Document management commands
    Doc {
        #[command(subcommand)]
        subcommand: DocCommands,
    },
    /// State management commands
    State {
        #[command(subcommand)]
        subcommand: StateCommands,
    },
    /// Query engine
    Query { query: String },
    /// System commands
    System {
        #[command(subcommand)]
        subcommand: SystemCommands,
    },
    /// Sync commands
    Sync {
        #[command(subcommand)]
        subcommand: SyncCommands,
    },
}

/// Parse CLI arguments and return configuration
pub fn parse_args() -> EddaResult<(Cli, EddaConfig)> {
    let cli = Cli::parse();

    // Load configuration
    let mut config = load_config(cli.config.as_ref().cloned())?;

    // Override with CLI arguments
    if let Some(data_dir) = &cli.data_dir {
        config.data_dir = data_dir.clone();
    }

    if let Some(format) = &cli.format {
        config.output_format = format.clone();
    }

    if cli.verbose {
        config.log_level = "debug".to_string();
    }

    // Validate configuration
    validate_config(&config)?;

    Ok((cli, config))
}

/// Initialize the application with CLI arguments
pub fn init_app() -> EddaResult<(Cli, EddaConfig)> {
    let (cli, config) = parse_args()?;

    // Initialize logging
    init_logging(&config).map_err(|e| EddaError::Logging(e.to_string()))?;

    Ok((cli, config))
}
