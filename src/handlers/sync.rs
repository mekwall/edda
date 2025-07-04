use crate::cli::{GitHubSyncCommands, SyncCommands};
use crate::core::{EddaConfig, EddaResult};

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
            crate::core::save_config(&config_copy, None)?;

            println!("GitHub sync configuration updated successfully");
            Ok(())
        }
    }
}
