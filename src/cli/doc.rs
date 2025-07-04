use clap::Subcommand;
use std::path::PathBuf;

#[derive(Subcommand)]
pub enum DocCommands {
    /// Add a document
    Add {
        path: PathBuf,
        #[arg(long)]
        title: Option<String>,
    },
    /// List documents
    List { query: Option<String> },
    /// Get document
    Get { id: String },
    /// Update document
    Update {
        id: String,
        field: String,
        value: String,
    },
    /// Get document content
    Content { id: String },
    /// Delete document
    Delete { id: String },
}
