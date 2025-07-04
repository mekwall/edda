use clap::Subcommand;

#[derive(Subcommand)]
pub enum SyncCommands {
    /// GitHub sync commands
    GitHub {
        #[command(subcommand)]
        subcommand: GitHubSyncCommands,
    },
}

#[derive(Subcommand)]
pub enum GitHubSyncCommands {
    /// Configure GitHub repository
    #[command(name = "repo")]
    Repository {
        /// Repository in owner/repo format
        repository: Option<String>,
    },

    /// Configure GitHub sync mode
    #[command(name = "mode")]
    Mode {
        /// Sync mode: issues, projects, or both
        mode: Option<String>,
    },

    /// Configure project board IDs
    #[command(name = "projects")]
    Projects {
        /// Project board IDs (comma-separated)
        project_ids: Option<String>,
    },

    /// Configure column mappings
    #[command(name = "columns")]
    Columns {
        /// Column name
        column: Option<String>,
        /// Task status to map to
        status: Option<String>,
    },

    /// List configured column mappings
    #[command(name = "list-columns")]
    ListColumns,

    /// Show GitHub sync status
    Status,

    /// Setup GitHub token
    #[command(name = "setup-token")]
    SetupToken,
}
