mod cli;
mod core;
mod storage;
mod sync;

use cli::{
    Commands, ConfigCommands, DocCommands, GitHubSyncCommands, StateCommands, SyncCommands,
    SystemCommands, TaskCommands, init_app,
};
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
        Some(Commands::Sync { subcommand }) => handle_sync_commands(subcommand, &config).await,
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

async fn handle_task_commands(
    subcommand: TaskCommands,
    config: &EddaConfig,
    format: &str,
    quiet: bool,
) -> EddaResult<()> {
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
            let tasks = task_engine.list_tasks(None).await?;

            if tasks.is_empty() {
                if !quiet {
                    println!("No tasks found.");
                }
                return Ok(());
            }

            match format {
                "json" => {
                    let json = serde_json::json!({
                        "tasks": tasks,
                        "meta": {
                            "total": tasks.len(),
                            "pending": tasks.iter().filter(|t| t.status == TaskStatus::Pending).count(),
                            "completed": tasks.iter().filter(|t| t.status == TaskStatus::Completed).count(),
                        }
                    });
                    println!("{}", serde_json::to_string_pretty(&json).unwrap());
                }
                _ => {
                    // Text table output
                    println!(
                        "{:<4} {:<30} {:<10} {:<20} {:<20}",
                        "ID", "Description", "Status", "Created", "Modified"
                    );
                    for task in tasks {
                        println!(
                            "{:<4} {:<30} {:<10} {:<20} {:<20}",
                            task.id.unwrap_or(0),
                            task.description.chars().take(30).collect::<String>(),
                            task.status,
                            task.entry_date.format("%Y-%m-%d %H:%M"),
                            task.modified_date.format("%Y-%m-%d %H:%M")
                        );
                    }
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
                Some(task) => match format {
                    "json" => {
                        println!("{}", serde_json::to_string_pretty(&task).unwrap());
                    }
                    _ => {
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
                },
                None => {
                    if !quiet {
                        println!("Task {task_id} not found.");
                    }
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

async fn handle_sync_commands(subcommand: SyncCommands, config: &EddaConfig) -> EddaResult<()> {
    match subcommand {
        SyncCommands::GitHub { subcommand } => {
            handle_github_sync_commands(subcommand, config).await
        }
    }
}

async fn handle_github_sync_commands(
    subcommand: GitHubSyncCommands,
    config: &EddaConfig,
) -> EddaResult<()> {
    match subcommand {
        GitHubSyncCommands::Pull => {
            println!("Pulling tasks from GitHub Issues...");
            // TODO: Implement GitHub sync pull
            println!("GitHub sync pull not yet implemented");
            Ok(())
        }
        GitHubSyncCommands::Push => {
            println!("Pushing tasks to GitHub Issues...");
            // TODO: Implement GitHub sync push
            println!("GitHub sync push not yet implemented");
            Ok(())
        }
        GitHubSyncCommands::Status => {
            println!("GitHub Sync Status:");
            println!("  Repository: {:?}", config.github.repository);
            println!(
                "  Token: {}",
                if config.github.token.is_some() {
                    "Configured"
                } else {
                    "Not configured"
                }
            );
            println!("  Sync Interval: {} seconds", config.github.sync_interval);
            println!("  Last Sync: Not implemented yet");
            Ok(())
        }
        GitHubSyncCommands::Config { key, value } => {
            println!("Configuring GitHub sync: {} = {}", key, value);

            // Create a mutable copy of the config
            let mut config_copy = config.clone();

            // Set the GitHub configuration value
            let full_key = format!("github.{}", key);
            config_copy.set_value(&full_key, &value)?;

            // Save the configuration
            core::save_config(&config_copy, None)?;

            println!("GitHub sync configuration updated successfully");
            Ok(())
        }
    }
}

async fn handle_config_commands(subcommand: ConfigCommands, config: &EddaConfig) -> EddaResult<()> {
    match subcommand {
        ConfigCommands::Show => {
            println!("Current Configuration:");
            println!("  Data Directory: {:?}", config.data_dir);
            println!("  Log Level: {}", config.log_level);
            println!("  Output Format: {}", config.output_format);
            println!("  Database URL: {}", config.database.url);
            println!("  GitHub Repository: {:?}", config.github.repository);
            println!(
                "  GitHub Token: {}",
                if config.github.token.is_some() {
                    "***"
                } else {
                    "Not set"
                }
            );
            println!("  Sync Interval: {} seconds", config.github.sync_interval);
            Ok(())
        }
        ConfigCommands::Set { key, value } => {
            println!("Setting configuration key '{}' to '{}'", key, value);

            // Create a mutable copy of the config
            let mut config_copy = config.clone();

            // Set the value
            config_copy.set_value(&key, &value)?;

            // Save the configuration
            core::save_config(&config_copy, None)?;

            println!("Configuration updated and saved successfully");
            Ok(())
        }
        ConfigCommands::Get { key } => {
            match config.get_value(&key) {
                Some(value) => {
                    if key == "github.token" {
                        println!("***");
                    } else {
                        println!("{}", value);
                    }
                }
                None => println!("Unknown configuration key: {}", key),
            }
            Ok(())
        }
        ConfigCommands::Edit => {
            println!("Opening configuration file for editing...");
            // TODO: Implement configuration file editing
            println!("Configuration file editing not yet implemented");
            Ok(())
        }
        ConfigCommands::Validate => {
            println!("Validating configuration...");
            match core::validate_config(config) {
                Ok(()) => println!("Configuration is valid"),
                Err(e) => {
                    println!("Configuration validation failed: {}", e);
                    return Err(e);
                }
            }
            Ok(())
        }
        ConfigCommands::Reset => {
            println!("Resetting configuration to defaults...");

            // Create default configuration
            let default_config = EddaConfig::default();

            // Save the default configuration
            core::save_config(&default_config, None)?;

            println!("Configuration reset to defaults successfully");
            Ok(())
        }
    }
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
        SystemCommands::Config { subcommand } => handle_config_commands(subcommand, config).await,
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
