use crate::core::{EddaConfig, EddaError, EddaResult, init_logging, load_config, validate_config};
use clap::{Parser, Subcommand};
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

#[derive(Subcommand)]
pub enum Commands {
    /// Task management commands
    Task {
        /// Task subcommand
        #[command(subcommand)]
        subcommand: TaskCommands,
    },

    /// Document management commands
    Doc {
        /// Document subcommand
        #[command(subcommand)]
        subcommand: DocCommands,
    },

    /// State management commands
    State {
        /// State subcommand
        #[command(subcommand)]
        subcommand: StateCommands,
    },

    /// Query engine
    Query {
        /// SQL-like query string
        query: String,
    },

    /// System commands
    System {
        /// System subcommand
        #[command(subcommand)]
        subcommand: SystemCommands,
    },
}

#[derive(Subcommand)]
pub enum TaskCommands {
    /// Add a new task
    Add {
        /// Task description
        description: String,
    },

    /// List tasks
    List {
        /// Optional query filter
        query: Option<String>,
    },

    /// Get task information
    Get {
        /// Task ID
        id: String,
    },

    /// Modify task
    Modify {
        /// Task ID
        id: String,
        /// Field to modify
        field: String,
        /// New value
        value: String,
    },

    /// Mark task as done
    Done {
        /// Task ID
        id: String,
    },

    /// Delete task
    Delete {
        /// Task ID
        id: String,
    },

    /// Start time tracking
    Start {
        /// Task ID
        id: String,
    },

    /// Stop time tracking
    Stop {
        /// Task ID
        id: String,
    },

    /// Add annotation
    Annotate {
        /// Task ID
        id: String,
        /// Annotation note
        note: String,
    },

    /// Add tag
    Tag {
        /// Task ID
        id: String,
        /// Tag to add
        tag: String,
    },

    /// Remove tag
    Untag {
        /// Task ID
        id: String,
        /// Tag to remove
        tag: String,
    },
}

#[derive(Subcommand)]
pub enum DocCommands {
    /// Add a document
    Add {
        /// Document path
        path: PathBuf,
        /// Document title
        #[arg(long)]
        title: Option<String>,
    },

    /// List documents
    List {
        /// Optional query filter
        query: Option<String>,
    },

    /// Get document
    Get {
        /// Document ID
        id: String,
    },

    /// Update document
    Update {
        /// Document ID
        id: String,
        /// Field to update
        field: String,
        /// New value
        value: String,
    },

    /// Get document content
    Content {
        /// Document ID
        id: String,
    },

    /// Delete document
    Delete {
        /// Document ID
        id: String,
    },
}

#[derive(Subcommand)]
pub enum StateCommands {
    /// Set state value
    Set {
        /// State key
        key: String,
        /// State value
        value: String,
    },

    /// Get state value
    Get {
        /// State key
        key: String,
    },

    /// List state keys
    List {
        /// Optional prefix filter
        prefix: Option<String>,
    },

    /// Delete state value
    Delete {
        /// State key
        key: String,
    },

    /// Clear all state
    Clear,
}

#[derive(Subcommand)]
pub enum SystemCommands {
    /// Initialize Edda data directory
    Init,

    /// Create backup of data
    Backup,

    /// Restore from backup
    Restore {
        /// Backup path
        backup: PathBuf,
    },

    /// Show/edit configuration
    Config,

    /// Show system status
    Status,

    /// Clean up temporary files
    Cleanup,
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
