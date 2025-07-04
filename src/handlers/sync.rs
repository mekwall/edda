use crate::cli::{GitHubSyncCommands, SyncCommands};
use crate::core::{EddaConfig, EddaResult};
use crate::github::GitHubSyncProvider;
use crate::sync::SyncProvider;

pub async fn handle_sync_commands(
    subcommand: SyncCommands,
    config: &mut EddaConfig,
) -> EddaResult<()> {
    match subcommand {
        SyncCommands::GitHub { subcommand } => {
            handle_github_sync_commands(subcommand, config).await
        }
    }
}

pub async fn handle_github_sync_commands(
    command: GitHubSyncCommands,
    config: &mut EddaConfig,
) -> EddaResult<()> {
    match command {
        GitHubSyncCommands::Repository { repository } => {
            if let Some(repo) = repository {
                config.github.repository = Some(repo);
                crate::core::save_config(config, None)?;
                println!(
                    "GitHub repository configured: {}",
                    config.github.repository.as_ref().unwrap()
                );
            } else {
                println!("GitHub repository: {:?}", config.github.repository);
            }
            Ok(())
        }
        GitHubSyncCommands::Mode { mode } => {
            if let Some(sync_mode) = mode {
                if !["issues", "projects", "both"].contains(&sync_mode.as_str()) {
                    return Err(crate::core::EddaError::Sync(
                        crate::core::error::SyncError::Configuration {
                            message: "Sync mode must be 'issues', 'projects', or 'both'"
                                .to_string(),
                        },
                    ));
                }
                config.github.sync_mode = sync_mode;
                crate::core::save_config(config, None)?;
                println!("GitHub sync mode configured: {}", config.github.sync_mode);
            } else {
                println!("GitHub sync mode: {}", config.github.sync_mode);
            }
            Ok(())
        }
        GitHubSyncCommands::Projects { project_ids } => {
            if let Some(ids_str) = project_ids {
                let ids: Result<Vec<u64>, _> = ids_str
                    .split(',')
                    .map(|s| s.trim().parse::<u64>())
                    .collect();

                match ids {
                    Ok(ids_vec) => {
                        config.github.project_ids = ids_vec;
                        crate::core::save_config(config, None)?;
                        println!(
                            "GitHub project IDs configured: {:?}",
                            config.github.project_ids
                        );
                    }
                    Err(_) => {
                        return Err(crate::core::EddaError::Sync(
                            crate::core::error::SyncError::Configuration {
                                message: "Invalid project IDs format. Use comma-separated numbers."
                                    .to_string(),
                            },
                        ));
                    }
                }
            } else {
                println!("GitHub project IDs: {:?}", config.github.project_ids);
            }
            Ok(())
        }
        GitHubSyncCommands::Columns { column, status } => {
            if let (Some(col), Some(st)) = (column, status) {
                // Validate status
                if !["pending", "in_progress", "completed", "cancelled"].contains(&st.as_str()) {
                    return Err(crate::core::EddaError::Sync(crate::core::error::SyncError::Configuration {
                        message: "Status must be 'pending', 'in_progress', 'completed', or 'cancelled'".to_string(),
                    }));
                }

                config.github.column_mapping.insert(col.clone(), st.clone());
                crate::core::save_config(config, None)?;
                println!("Column mapping configured: '{}' -> '{}'", col, st);
            } else {
                println!("Column mappings:");
                for (col, st) in &config.github.column_mapping {
                    println!("  '{}' -> '{}'", col, st);
                }
            }
            Ok(())
        }
        GitHubSyncCommands::ListColumns => {
            println!("Configured column mappings:");
            if config.github.column_mapping.is_empty() {
                println!("  No column mappings configured");
            } else {
                for (col, st) in &config.github.column_mapping {
                    println!("  '{}' -> '{}'", col, st);
                }
            }
            Ok(())
        }
        GitHubSyncCommands::Status => {
            println!("GitHub Sync Status:");
            println!("  Repository: {:?}", config.github.repository);
            println!(
                "  Token: {}",
                if crate::core::config::get_github_token().is_some() {
                    "Configured (from environment)"
                } else {
                    "Not configured (set GITHUB_TOKEN env var)"
                }
            );
            println!("  Sync Mode: {}", config.github.sync_mode);
            println!("  Sync Interval: {} seconds", config.github.sync_interval);

            if config.github.sync_mode == "projects" || config.github.sync_mode == "both" {
                println!("  Project IDs: {:?}", config.github.project_ids);
                println!(
                    "  Column Mappings: {} configured",
                    config.github.column_mapping.len()
                );
            }

            // Test connection
            if let Some(repo) = &config.github.repository {
                if crate::core::config::get_github_token().is_some() {
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
                    println!("  Connection: ✗ No token configured (set GITHUB_TOKEN env var)");
                }
            } else {
                println!("  Connection: ✗ No repository configured");
            }

            Ok(())
        }
        GitHubSyncCommands::SetupToken => {
            println!("GitHub Token Setup:");
            println!("Set one of the following environment variables:");
            println!("  GITHUB_TOKEN=<your_token>");
            println!("  EDDA_GITHUB_TOKEN=<your_token>");
            println!("  GH_TOKEN=<your_token>");
            println!("  GITHUB_ACCESS_TOKEN=<your_token>");
            println!();
            println!("You can set this in your shell profile or use:");
            println!("  export GITHUB_TOKEN=<your_token>");
            println!();
            println!("To get a GitHub token:");
            println!("  1. Go to GitHub Settings > Developer settings > Personal access tokens");
            println!("  2. Generate a new token with 'repo' scope");
            println!("  3. Copy the token and set it as an environment variable");
            Ok(())
        }
    }
}
