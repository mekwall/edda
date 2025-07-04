use crate::cli::{ConfigCommands, SystemCommands};
use crate::core::{EddaConfig, EddaResult};
use std::path::PathBuf;

pub async fn handle_system_commands(
    subcommand: SystemCommands,
    config: &EddaConfig,
) -> EddaResult<()> {
    match subcommand {
        SystemCommands::Init => {
            println!("Initializing Edda data directory...");

            // Create a default .edda.toml in the current directory if none exists in the path
            let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
            let local_config = cwd.join(".edda.toml");
            let config_exists = crate::core::config::find_config_file().is_some();
            if !config_exists {
                let default_config = EddaConfig::default();
                let toml_string = toml::to_string_pretty(&default_config)
                    .expect("Failed to serialize default config");
                std::fs::write(&local_config, toml_string).expect("Failed to write .edda.toml");
                println!("Created default .edda.toml in {:?}", cwd);
            }

            // Create data directory if it doesn't exist
            if !config.data_dir.exists() {
                std::fs::create_dir_all(&config.data_dir).map_err(|e| {
                    crate::core::EddaError::Storage(crate::core::StorageError::Initialization {
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
                        crate::core::EddaError::Storage(crate::core::StorageError::Initialization {
                            message: format!("Failed to create database directory: {e}"),
                        })
                    })?;
                }
            }

            // Initialize database with migrations
            crate::storage::init_database(db_path).await?;
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
            crate::core::save_config(&config_copy, None)?;

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
            match crate::core::validate_config(config) {
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
            crate::core::save_config(&default_config, None)?;

            println!("Configuration reset to defaults successfully");
            Ok(())
        }
    }
}
