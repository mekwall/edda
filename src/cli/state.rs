use clap::Subcommand;

#[derive(Subcommand)]
pub enum StateCommands {
    /// Set state value
    Set { key: String, value: String },
    /// Get state value
    Get { key: String },
    /// List state keys
    List { prefix: Option<String> },
    /// Delete state value
    Delete { key: String },
    /// Clear all state
    Clear,
}
