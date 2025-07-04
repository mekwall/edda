use clap::Subcommand;

#[derive(Subcommand)]
pub enum SyncCommands {
    /// GitHub sync commands
    #[command(name = "github")]
    GitHub {
        #[command(subcommand)]
        subcommand: GitHubSyncCommands,
    },
}

#[derive(Subcommand)]
pub enum GitHubSyncCommands {
    /// Pull tasks from GitHub Issues
    Pull,
    /// Push tasks to GitHub Issues
    Push,
    /// Show sync status
    Status,
    /// Configure GitHub sync
    Config { key: String, value: String },
}
