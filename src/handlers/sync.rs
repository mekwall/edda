use crate::cli::{GitHubSyncCommands, SyncCommands};
use crate::core::{EddaConfig, EddaResult};
use crate::github::GitHubSyncProvider;
use crate::handlers::task::create_task_engine;
use crate::storage::TaskStorage;
use crate::sync::SyncProvider;

pub async fn handle_sync_commands(subcommand: SyncCommands, config: &EddaConfig) -> EddaResult<()> {
    match subcommand {
        SyncCommands::GitHub { subcommand } => {
            handle_github_sync_commands(subcommand, config).await
        }
    }
}

pub async fn handle_github_sync_commands(
    subcommand: GitHubSyncCommands,
    config: &EddaConfig,
) -> EddaResult<()> {
    match subcommand {
        GitHubSyncCommands::Pull => {
            println!("Pulling tasks from GitHub Issues...");

            // Create GitHub sync provider
            let provider = GitHubSyncProvider::new(config.github.clone())?;

            // Test connection first
            provider.test_connection().await?;
            println!("✓ Connected to GitHub successfully");

            // Pull tasks from GitHub
            let github_tasks = provider.pull_tasks().await?;
            println!("✓ Retrieved {} tasks from GitHub", github_tasks.len());

            // Get local task engine
            let task_engine = create_task_engine(config).await?;

            // Import tasks that don't exist locally
            let mut imported_count = 0;
            for task in github_tasks {
                // Check if task already exists (by description for now)
                let existing_tasks = task_engine.list_tasks(None).await?;
                let exists = existing_tasks
                    .iter()
                    .any(|t| t.description == task.description);

                if !exists {
                    task_engine.create_task(task.description).await?;
                    imported_count += 1;
                }
            }

            println!("✓ Imported {} new tasks from GitHub", imported_count);
            Ok(())
        }
        GitHubSyncCommands::Push => {
            println!("Pushing tasks to GitHub Issues...");

            // Create GitHub sync provider
            let provider = GitHubSyncProvider::new(config.github.clone())?;

            // Test connection first
            provider.test_connection().await?;
            println!("✓ Connected to GitHub successfully");

            // Get local tasks
            let task_engine = create_task_engine(config).await?;
            let local_tasks = task_engine.list_tasks(None).await?;
            println!("✓ Found {} local tasks", local_tasks.len());

            // Push tasks to GitHub
            provider.push_tasks(&local_tasks).await?;
            println!("✓ Pushed {} tasks to GitHub", local_tasks.len());

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

            // Test connection
            if let Some(repo) = &config.github.repository {
                if let Some(_token) = &config.github.token {
                    match GitHubSyncProvider::new(config.github.clone()) {
                        Ok(provider) => match provider.test_connection().await {
                            Ok(()) => {
                                println!("  Connection: ✓ Connected");
                                match provider.get_status().await {
                                    Ok(status) => {
                                        println!("  Status: {:?}", status);
                                    }
                                    Err(e) => {
                                        println!("  Status: Error - {}", e);
                                    }
                                }
                            }
                            Err(e) => {
                                println!("  Connection: ✗ Failed - {}", e);
                            }
                        },
                        Err(e) => {
                            println!("  Connection: ✗ Configuration error - {}", e);
                        }
                    }
                } else {
                    println!("  Connection: ✗ No token configured");
                }
            } else {
                println!("  Connection: ✗ No repository configured");
            }

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
            crate::core::save_config(&config_copy, None)?;

            println!("GitHub sync configuration updated successfully");
            Ok(())
        }
    }
}
