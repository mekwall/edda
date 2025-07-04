mod cli;
mod core;
mod storage;
mod sync;

use cli::{Commands, DocCommands, StateCommands, SystemCommands, TaskCommands, init_app};
use core::{EddaConfig, EddaResult, Priority, TaskEngine, TaskStatus};
use std::path::PathBuf;
use std::str::FromStr;
use storage::SqliteTaskStorage;

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
        Some(Commands::Task { subcommand }) => handle_task_commands(subcommand, &config).await,
        Some(Commands::Doc { subcommand }) => handle_doc_commands(subcommand).await,
        Some(Commands::State { subcommand }) => handle_state_commands(subcommand).await,
        Some(Commands::Query { query }) => handle_query_command(query).await,
        Some(Commands::System { subcommand }) => handle_system_commands(subcommand, &config).await,
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

async fn handle_task_commands(subcommand: TaskCommands, config: &EddaConfig) -> EddaResult<()> {
    // Initialize storage and task engine
    let db_path = if config.database.url.starts_with("sqlite:") {
        PathBuf::from(config.database.url.trim_start_matches("sqlite:"))
    } else {
        config.data_dir.join("edda.db")
    };
    println!("[DEBUG] Using database path: {db_path:?}");
    let pool = storage::get_pool(db_path).await?;
    let storage = SqliteTaskStorage::new(pool);
    let task_engine = TaskEngine::new(Box::new(storage));

    match subcommand {
        TaskCommands::Add { description } => {
            let task = task_engine.create_task(description).await?;
            println!(
                "Created task {}: {}",
                task.id.unwrap_or(0),
                task.description
            );
            Ok(())
        }
        TaskCommands::List { query: _ } => {
            // For now, just list all tasks without filtering
            let tasks = task_engine.list_tasks(None).await?;

            if tasks.is_empty() {
                println!("No tasks found.");
            } else {
                println!("Tasks:");
                for task in tasks {
                    println!(
                        "  {}: {} [{}]",
                        task.id.unwrap_or(0),
                        task.description,
                        task.status
                    );
                }
            }
            Ok(())
        }
        TaskCommands::Get { id } => {
            let task_id = id.parse::<i64>().map_err(|_| {
                core::EddaError::Task(core::TaskError::Validation {
                    message: format!("Invalid task ID: {id}"),
                })
            })?;
            let task = task_engine.get_task(task_id).await?;

            match task {
                Some(task) => {
                    println!("Task {}: {}", task.id.unwrap_or(0), task.description);
                    println!("  Status: {}", task.status);
                    println!(
                        "  Priority: {}",
                        task.priority
                            .as_ref()
                            .map(|p| p.to_string())
                            .unwrap_or_else(|| "None".to_string())
                    );
                    println!("  Project: {}", task.project.as_deref().unwrap_or("None"));
                    println!(
                        "  Tags: {}",
                        if task.tags.is_empty() {
                            "None".to_string()
                        } else {
                            task.tags
                                .iter()
                                .map(|t| format!("+{t}"))
                                .collect::<Vec<_>>()
                                .join(" ")
                        }
                    );
                    println!("  Created: {}", task.entry_date);
                    println!("  Modified: {}", task.modified_date);
                }
                None => {
                    println!("Task {task_id} not found.");
                }
            }
            Ok(())
        }
        TaskCommands::Modify { id, field, value } => {
            let task_id = id.parse::<i64>().map_err(|_| {
                core::EddaError::Task(core::TaskError::Validation {
                    message: format!("Invalid task ID: {id}"),
                })
            })?;
            let mut task = task_engine.get_task(task_id).await?.ok_or_else(|| {
                core::EddaError::Task(core::TaskError::NotFound { id: id.clone() })
            })?;

            match field.to_lowercase().as_str() {
                "description" => task.description = value,
                "status" => {
                    task.status = TaskStatus::from_str(&value).map_err(|e| {
                        core::EddaError::Task(core::TaskError::Validation {
                            message: format!("Invalid status: {e}"),
                        })
                    })?
                }
                "priority" => {
                    task.priority = Some(Priority::from_str(&value).map_err(|e| {
                        core::EddaError::Task(core::TaskError::Validation {
                            message: format!("Invalid priority: {e}"),
                        })
                    })?)
                }
                "project" => task.project = Some(value),
                _ => {
                    return Err(core::EddaError::Task(core::TaskError::Validation {
                        message: format!("Unknown field: {field}"),
                    }));
                }
            }

            let updated_task = task_engine.update_task(task).await?;
            println!(
                "Updated task {}: {}",
                updated_task.id.unwrap_or(0),
                updated_task.description
            );
            Ok(())
        }
        TaskCommands::Done { id } => {
            let task_id = id.parse::<i64>().map_err(|_| {
                core::EddaError::Task(core::TaskError::Validation {
                    message: format!("Invalid task ID: {id}"),
                })
            })?;
            let task = task_engine.complete_task(task_id).await?;
            println!(
                "Completed task {}: {}",
                task.id.unwrap_or(0),
                task.description
            );
            Ok(())
        }
        TaskCommands::Delete { id } => {
            let task_id = id.parse::<i64>().map_err(|_| {
                core::EddaError::Task(core::TaskError::Validation {
                    message: format!("Invalid task ID: {id}"),
                })
            })?;
            let task = task_engine.delete_task(task_id).await?;
            println!(
                "Deleted task {}: {}",
                task.id.unwrap_or(0),
                task.description
            );
            Ok(())
        }
        TaskCommands::Start { id } => {
            let task_id = id.parse::<i64>().map_err(|_| {
                core::EddaError::Task(core::TaskError::Validation {
                    message: format!("Invalid task ID: {id}"),
                })
            })?;
            // TODO: Implement start_task in TaskEngine
            let task = task_engine.start_task(task_id).await?;
            println!(
                "Started task {}: {}",
                task.id.unwrap_or(0),
                task.description
            );
            Ok(())
        }
        TaskCommands::Stop { id } => {
            let task_id = id.parse::<i64>().map_err(|_| {
                core::EddaError::Task(core::TaskError::Validation {
                    message: format!("Invalid task ID: {id}"),
                })
            })?;
            // TODO: Implement stop_task in TaskEngine
            let task = task_engine.stop_task(task_id).await?;
            println!(
                "Stopped task {}: {}",
                task.id.unwrap_or(0),
                task.description
            );
            Ok(())
        }
        TaskCommands::Annotate { id, note } => {
            let task_id = id.parse::<i64>().map_err(|_| {
                core::EddaError::Task(core::TaskError::Validation {
                    message: format!("Invalid task ID: {id}"),
                })
            })?;
            // TODO: Implement annotate_task in TaskEngine
            let task = task_engine.annotate_task(task_id, note).await?;
            println!(
                "Annotated task {}: {}",
                task.id.unwrap_or(0),
                task.description
            );
            Ok(())
        }
        TaskCommands::Tag { id, tag } => {
            let task_id = id.parse::<i64>().map_err(|_| {
                core::EddaError::Task(core::TaskError::Validation {
                    message: format!("Invalid task ID: {id}"),
                })
            })?;
            // TODO: Implement add_tag in TaskEngine
            let task = task_engine.add_tag(task_id, tag).await?;
            println!(
                "Added tag to task {}: {}",
                task.id.unwrap_or(0),
                task.description
            );
            Ok(())
        }
        TaskCommands::Untag { id, tag } => {
            let task_id = id.parse::<i64>().map_err(|_| {
                core::EddaError::Task(core::TaskError::Validation {
                    message: format!("Invalid task ID: {id}"),
                })
            })?;
            // TODO: Implement remove_tag in TaskEngine
            let task = task_engine.remove_tag(task_id, &tag).await?;
            println!(
                "Removed tag from task {}: {}",
                task.id.unwrap_or(0),
                task.description
            );
            Ok(())
        }
    }
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

async fn handle_system_commands(subcommand: SystemCommands, config: &EddaConfig) -> EddaResult<()> {
    match subcommand {
        SystemCommands::Init => {
            println!("Initializing Edda data directory...");

            // Create data directory if it doesn't exist
            if !config.data_dir.exists() {
                std::fs::create_dir_all(&config.data_dir).map_err(|e| {
                    core::EddaError::Storage(core::StorageError::Initialization {
                        message: format!("Failed to create data directory: {e}"),
                    })
                })?;
                println!("Created data directory: {:?}", config.data_dir);
            }

            // Initialize database
            let db_path = if config.database.url.starts_with("sqlite:") {
                PathBuf::from(config.database.url.trim_start_matches("sqlite:"))
            } else {
                config.data_dir.join("edda.db")
            };
            println!("[DEBUG] Using database path: {db_path:?}");

            // Create database directory if needed
            if let Some(parent) = db_path.parent() {
                if !parent.exists() {
                    std::fs::create_dir_all(parent).map_err(|e| {
                        core::EddaError::Storage(core::StorageError::Initialization {
                            message: format!("Failed to create database directory: {e}"),
                        })
                    })?;
                }
            }

            // Initialize database with migrations
            storage::init_database(db_path).await?;
            println!("Database initialized successfully");

            Ok(())
        }
        SystemCommands::Backup => {
            println!("Creating backup...");
            // TODO: Implement backup
            Ok(())
        }
        SystemCommands::Restore { backup } => {
            println!("Restoring from backup: {backup:?}");
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
