mod cli;
mod core;
mod storage;
mod sync;

use edda::cli::{
    Commands, ConfigCommands, DocCommands, GitHubSyncCommands, StateCommands, SyncCommands,
    SystemCommands, TaskCommands, init_app,
};
use edda::core::{EddaConfig, EddaResult};
use edda::handlers::{
    handle_doc_commands, handle_github_sync_commands, handle_query_command, handle_state_commands,
    handle_sync_commands, handle_system_commands, handle_task_commands,
};

#[tokio::main]
async fn main() {
    // Initialize application
    let (cli, config) = match init_app() {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Failed to initialize application: {e}");
            std::process::exit(1);
        }
    };

    // Handle commands
    let result = match cli.command {
        Some(Commands::Task { subcommand }) => {
            handle_task_commands(
                subcommand,
                &config,
                cli.format.as_deref().unwrap_or("text"),
                cli.quiet,
            )
            .await
        }
        Some(Commands::Doc { subcommand }) => handle_doc_commands(subcommand).await,
        Some(Commands::State { subcommand }) => handle_state_commands(subcommand).await,
        Some(Commands::Query { query }) => handle_query_command(query).await,
        Some(Commands::System { subcommand }) => handle_system_commands(subcommand, &config).await,
        Some(Commands::Sync { subcommand }) => {
            let mut config = config;
            handle_sync_commands(subcommand, &mut config).await
        }
        None => {
            // Show help if no command provided
            println!("Edda: AI agent-native CLI for structured task and document management");
            println!("Use 'edda --help' for more information");
            Ok(())
        }
    };

    // Handle result
    if let Err(e) = result {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
