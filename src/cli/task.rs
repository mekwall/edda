use clap::Subcommand;

#[derive(Subcommand)]
pub enum TaskCommands {
    /// Add a new task
    Add { description: String },
    /// List tasks
    List { query: Option<String> },
    /// Get task information
    Get { id: String },
    /// Modify task
    Modify {
        id: String,
        field: String,
        value: String,
    },
    /// Mark task as done
    Done { id: String },
    /// Delete task
    Delete { id: String },
    /// Start time tracking
    Start { id: String },
    /// Stop time tracking
    Stop { id: String },
    /// Add annotation
    Annotate { id: String, note: String },
    /// Add tag
    Tag { id: String, tag: String },
    /// Remove tag
    Untag { id: String, tag: String },
}
