use crate::cli::TaskCommands;
use crate::core::{EddaConfig, EddaResult, Priority, TaskEngine, TaskStatus};
use crate::storage::SqliteTaskStorage;
use std::path::PathBuf;
use std::str::FromStr;

/// Create a task engine instance for the given configuration
pub async fn create_task_engine(config: &EddaConfig) -> EddaResult<TaskEngine> {
    let db_path = if config.database.url.starts_with("sqlite:") {
        PathBuf::from(config.database.url.trim_start_matches("sqlite:"))
    } else {
        config.data_dir.join("edda.db")
    };

    let pool = crate::storage::get_pool(db_path).await?;
    let storage = SqliteTaskStorage::new(pool);
    Ok(TaskEngine::new(Box::new(storage)))
}

pub async fn handle_task_commands(
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
    let pool = crate::storage::get_pool(db_path).await?;
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
                crate::core::EddaError::Task(crate::core::TaskError::Validation {
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
                crate::core::EddaError::Task(crate::core::TaskError::Validation {
                    message: format!("Invalid task ID: {id}"),
                })
            })?;
            let mut task = task_engine.get_task(task_id).await?.ok_or_else(|| {
                crate::core::EddaError::Task(crate::core::TaskError::NotFound { id: id.clone() })
            })?;

            match field.to_lowercase().as_str() {
                "description" => task.description = value,
                "status" => {
                    task.status = TaskStatus::from_str(&value).map_err(|e| {
                        crate::core::EddaError::Task(crate::core::TaskError::Validation {
                            message: format!("Invalid status: {e}"),
                        })
                    })?
                }
                "priority" => {
                    task.priority = Some(Priority::from_str(&value).map_err(|e| {
                        crate::core::EddaError::Task(crate::core::TaskError::Validation {
                            message: format!("Invalid priority: {e}"),
                        })
                    })?)
                }
                "project" => task.project = Some(value),
                _ => {
                    return Err(crate::core::EddaError::Task(
                        crate::core::TaskError::Validation {
                            message: format!("Unknown field: {field}"),
                        },
                    ));
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
                crate::core::EddaError::Task(crate::core::TaskError::Validation {
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
                crate::core::EddaError::Task(crate::core::TaskError::Validation {
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
                crate::core::EddaError::Task(crate::core::TaskError::Validation {
                    message: format!("Invalid task ID: {id}"),
                })
            })?;
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
                crate::core::EddaError::Task(crate::core::TaskError::Validation {
                    message: format!("Invalid task ID: {id}"),
                })
            })?;
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
                crate::core::EddaError::Task(crate::core::TaskError::Validation {
                    message: format!("Invalid task ID: {id}"),
                })
            })?;
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
                crate::core::EddaError::Task(crate::core::TaskError::Validation {
                    message: format!("Invalid task ID: {id}"),
                })
            })?;
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
                crate::core::EddaError::Task(crate::core::TaskError::Validation {
                    message: format!("Invalid task ID: {id}"),
                })
            })?;
            let task = task_engine.remove_tag(task_id, &tag).await?;
            Ok(())
        }
    }
}
