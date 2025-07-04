mod cli;
mod core;
mod storage;
mod sync;

use cli::{Commands, DocCommands, StateCommands, SystemCommands, TaskCommands, init_app};
use core::EddaResult;

#[tokio::main]
async fn main() {
    // Initialize application
    let (cli, _config) = match init_app() {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Failed to initialize application: {}", e);
            std::process::exit(1);
        }
    };

    // Handle commands
    let result = match cli.command {
        Some(Commands::Task { subcommand }) => handle_task_commands(subcommand).await,
        Some(Commands::Doc { subcommand }) => handle_doc_commands(subcommand).await,
        Some(Commands::State { subcommand }) => handle_state_commands(subcommand).await,
        Some(Commands::Query { query }) => handle_query_command(query).await,
        Some(Commands::System { subcommand }) => handle_system_commands(subcommand).await,
        None => {
            // Show help if no command provided
            println!("Edda: AI agent-native CLI for structured task and document management");
            println!("Use 'edda --help' for more information");
            Ok(())
        }
    };

    // Handle result
    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

async fn handle_task_commands(_subcommand: TaskCommands) -> EddaResult<()> {
    // TODO: Implement task commands
    println!("Task commands not yet implemented");
    Ok(())
}

async fn handle_doc_commands(_subcommand: DocCommands) -> EddaResult<()> {
    // TODO: Implement document commands
    println!("Document commands not yet implemented");
    Ok(())
}

async fn handle_state_commands(_subcommand: StateCommands) -> EddaResult<()> {
    // TODO: Implement state commands
    println!("State commands not yet implemented");
    Ok(())
}

async fn handle_query_command(_query: String) -> EddaResult<()> {
    // TODO: Implement query engine
    println!("Query engine not yet implemented");
    Ok(())
}

async fn handle_system_commands(subcommand: SystemCommands) -> EddaResult<()> {
    match subcommand {
        SystemCommands::Init => {
            println!("Initializing Edda data directory...");
            // TODO: Implement initialization
            Ok(())
        }
        SystemCommands::Backup => {
            println!("Creating backup...");
            // TODO: Implement backup
            Ok(())
        }
        SystemCommands::Restore { backup } => {
            println!("Restoring from backup: {:?}", backup);
            // TODO: Implement restore
            Ok(())
        }
        SystemCommands::Config => {
            println!("Configuration management not yet implemented");
            Ok(())
        }
        SystemCommands::Status => {
            println!("System status not yet implemented");
            Ok(())
        }
        SystemCommands::Cleanup => {
            println!("Cleanup not yet implemented");
            Ok(())
        }
    }
}
