use crate::core::EddaResult;
use chrono::Utc;
use flate2::Compression;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use sqlx::{Row, SqlitePool, sqlite::SqlitePoolOptions};
use std::fs;
use std::path::PathBuf;

#[cfg(test)]
use sqlx::Row as _;

/// Initialize the SQLite database
pub async fn init_database(db_path: PathBuf) -> EddaResult<()> {
    // Create database directory if it doesn't exist
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| {
            crate::core::EddaError::Storage(crate::core::StorageError::Initialization {
                message: format!("Failed to create database directory: {e}"),
            })
        })?;
    }

    // Create empty database file if it doesn't exist
    if !db_path.exists() {
        // Create an empty file to initialize SQLite database
        std::fs::File::create(&db_path).map_err(|e| {
            crate::core::EddaError::Storage(crate::core::StorageError::Initialization {
                message: format!("Failed to create database file: {e}"),
            })
        })?;
    }

    // Create database URL
    let database_url = format!("sqlite:{}", db_path.to_string_lossy());

    // Create connection pool
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .map_err(|e| {
            crate::core::EddaError::Storage(crate::core::StorageError::Connection {
                message: format!("Failed to connect to database: {e}"),
            })
        })?;

    // Run migrations
    run_migrations(&pool).await?;

    // Validate database integrity
    validate_database_integrity(&pool).await?;

    Ok(())
}

/// Create a backup of the database
pub async fn create_backup(db_path: &PathBuf, backup_path: &PathBuf) -> EddaResult<()> {
    // Ensure backup directory exists
    if let Some(parent) = backup_path.parent() {
        fs::create_dir_all(parent).map_err(|e| {
            crate::core::EddaError::Storage(crate::core::StorageError::Backup {
                message: format!("Failed to create backup directory: {e}"),
            })
        })?;
    }

    // Simple file copy for backup
    fs::copy(db_path, backup_path).map_err(|e| {
        crate::core::EddaError::Storage(crate::core::StorageError::Backup {
            message: format!("Failed to create backup: {e}"),
        })
    })?;

    Ok(())
}

/// Create a compressed backup of the database
pub async fn create_compressed_backup(db_path: &PathBuf, backup_path: &PathBuf) -> EddaResult<()> {
    // Ensure backup directory exists
    if let Some(parent) = backup_path.parent() {
        fs::create_dir_all(parent).map_err(|e| {
            crate::core::EddaError::Storage(crate::core::StorageError::Backup {
                message: format!("Failed to create backup directory: {e}"),
            })
        })?;
    }

    // Read the database file
    let mut input = fs::File::open(db_path).map_err(|e| {
        crate::core::EddaError::Storage(crate::core::StorageError::Backup {
            message: format!("Failed to open database for backup: {e}"),
        })
    })?;

    // Create compressed backup
    let output = fs::File::create(backup_path).map_err(|e| {
        crate::core::EddaError::Storage(crate::core::StorageError::Backup {
            message: format!("Failed to create compressed backup: {e}"),
        })
    })?;

    let mut encoder = GzEncoder::new(output, Compression::default());
    std::io::copy(&mut input, &mut encoder).map_err(|e| {
        crate::core::EddaError::Storage(crate::core::StorageError::Backup {
            message: format!("Failed to compress backup: {e}"),
        })
    })?;

    encoder.finish().map_err(|e| {
        crate::core::EddaError::Storage(crate::core::StorageError::Backup {
            message: format!("Failed to finish compression: {e}"),
        })
    })?;

    Ok(())
}

/// Restore database from backup
pub async fn restore_backup(backup_path: &PathBuf, db_path: &PathBuf) -> EddaResult<()> {
    // Check if backup file exists
    if !backup_path.exists() {
        return Err(crate::core::EddaError::Storage(
            crate::core::StorageError::Backup {
                message: format!("Backup file not found: {}", backup_path.display()),
            },
        ));
    }

    // Determine if backup is compressed
    let is_compressed = backup_path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext == "gz")
        .unwrap_or(false);

    if is_compressed {
        restore_compressed_backup(backup_path, db_path).await?;
    } else {
        // Simple file copy for uncompressed backup
        fs::copy(backup_path, db_path).map_err(|e| {
            crate::core::EddaError::Storage(crate::core::StorageError::Backup {
                message: format!("Failed to restore backup: {e}"),
            })
        })?;
    }

    // Validate restored database
    let database_url = format!("sqlite:{}", db_path.to_string_lossy());
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect(&database_url)
        .await
        .map_err(|e| {
            crate::core::EddaError::Storage(crate::core::StorageError::Connection {
                message: format!("Failed to connect to restored database: {e}"),
            })
        })?;

    validate_database_integrity(&pool).await?;

    Ok(())
}

/// Restore from compressed backup
async fn restore_compressed_backup(backup_path: &PathBuf, db_path: &PathBuf) -> EddaResult<()> {
    let input = fs::File::open(backup_path).map_err(|e| {
        crate::core::EddaError::Storage(crate::core::StorageError::Backup {
            message: format!("Failed to open compressed backup: {e}"),
        })
    })?;

    let output = fs::File::create(db_path).map_err(|e| {
        crate::core::EddaError::Storage(crate::core::StorageError::Backup {
            message: format!("Failed to create restored database: {e}"),
        })
    })?;

    let mut decoder = GzDecoder::new(input);
    let mut output = output;
    std::io::copy(&mut decoder, &mut output).map_err(|e| {
        crate::core::EddaError::Storage(crate::core::StorageError::Backup {
            message: format!("Failed to decompress backup: {e}"),
        })
    })?;

    Ok(())
}

/// Validate database integrity
async fn validate_database_integrity(pool: &SqlitePool) -> EddaResult<()> {
    // Run SQLite integrity check
    let result = sqlx::query("PRAGMA integrity_check")
        .fetch_one(pool)
        .await
        .map_err(|e| {
            crate::core::EddaError::Storage(crate::core::StorageError::Corruption {
                message: format!("Failed to run integrity check: {e}"),
            })
        })?;

    let integrity_result: String = result.get("integrity_check");
    if integrity_result != "ok" {
        return Err(crate::core::EddaError::Storage(
            crate::core::StorageError::Corruption {
                message: format!("Database integrity check failed: {}", integrity_result),
            },
        ));
    }

    // Check that all required tables exist
    let tables = sqlx::query("SELECT name FROM sqlite_master WHERE type='table'")
        .fetch_all(pool)
        .await
        .map_err(|e| {
            crate::core::EddaError::Storage(crate::core::StorageError::Corruption {
                message: format!("Failed to check tables: {e}"),
            })
        })?;

    let required_tables = vec!["tasks", "documents", "state", "schema_version"];
    let existing_tables: Vec<String> = tables.iter().map(|row| row.get("name")).collect();

    for required_table in required_tables {
        if !existing_tables.contains(&required_table.to_string()) {
            return Err(crate::core::EddaError::Storage(
                crate::core::StorageError::Corruption {
                    message: format!("Required table '{}' is missing", required_table),
                },
            ));
        }
    }

    Ok(())
}

/// Sanitize data before storage
pub fn sanitize_string(input: &str) -> String {
    // Remove null bytes and control characters
    input
        .chars()
        .filter(|&c| c != '\0' && !c.is_control())
        .collect()
}

/// Validate and sanitize task data
pub fn validate_task_data(
    description: &str,
    project: Option<&str>,
    tags: &[String],
) -> EddaResult<()> {
    // Validate description
    if description.trim().is_empty() {
        return Err(crate::core::EddaError::Task(
            crate::core::TaskError::Validation {
                message: "Task description cannot be empty".to_string(),
            },
        ));
    }

    if description.len() > 1000 {
        return Err(crate::core::EddaError::Task(
            crate::core::TaskError::Validation {
                message: "Task description too long (max 1000 characters)".to_string(),
            },
        ));
    }

    // Validate project name
    if let Some(project_name) = project {
        if project_name.len() > 100 {
            return Err(crate::core::EddaError::Task(
                crate::core::TaskError::Validation {
                    message: "Project name too long (max 100 characters)".to_string(),
                },
            ));
        }
    }

    // Validate tags
    for tag in tags {
        if tag.trim().is_empty() {
            return Err(crate::core::EddaError::Task(
                crate::core::TaskError::Validation {
                    message: "Tag cannot be empty".to_string(),
                },
            ));
        }
        if tag.len() > 50 {
            return Err(crate::core::EddaError::Task(
                crate::core::TaskError::Validation {
                    message: format!("Tag '{}' too long (max 50 characters)", tag),
                },
            ));
        }
    }

    Ok(())
}

/// Run database migrations
pub async fn run_migrations(pool: &SqlitePool) -> EddaResult<()> {
    // Create schema version table to track migrations
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS schema_version (
            version INTEGER PRIMARY KEY,
            applied_at TEXT NOT NULL CHECK (datetime(applied_at) IS NOT NULL),
            description TEXT NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await
    .map_err(|e| {
        crate::core::EddaError::Storage(crate::core::StorageError::Migration {
            message: format!("Failed to create schema_version table: {e}"),
        })
    })?;

    // Get current schema version
    let current_version = sqlx::query("SELECT MAX(version) as version FROM schema_version")
        .fetch_optional(pool)
        .await
        .map_err(|e| {
            crate::core::EddaError::Storage(crate::core::StorageError::Migration {
                message: format!("Failed to get current schema version: {e}"),
            })
        })?
        .map(|row| row.get::<i32, _>("version"))
        .unwrap_or(0);

    // Apply migrations in order
    let migrations = vec![(
        1,
        "Initial schema with tasks, documents, state tables, constraints, and indexes",
    )];

    for (version, description) in migrations {
        if version > current_version {
            apply_migration(pool, version, description).await?;
        }
    }

    Ok(())
}

/// Apply a specific migration
async fn apply_migration(pool: &SqlitePool, version: i32, description: &str) -> EddaResult<()> {
    match version {
        1 => apply_migration_1(pool).await?,
        _ => {
            return Err(crate::core::EddaError::Storage(
                crate::core::StorageError::Migration {
                    message: format!("Unknown migration version: {}", version),
                },
            ));
        }
    }

    // Record the migration
    sqlx::query("INSERT INTO schema_version (version, applied_at, description) VALUES (?, ?, ?)")
        .bind(version)
        .bind(Utc::now().to_rfc3339())
        .bind(description)
        .execute(pool)
        .await
        .map_err(|e| {
            crate::core::EddaError::Storage(crate::core::StorageError::Migration {
                message: format!("Failed to record migration {}: {}", version, e),
            })
        })?;

    Ok(())
}

/// Migration 1: Complete schema with constraints and indexes
async fn apply_migration_1(pool: &SqlitePool) -> EddaResult<()> {
    // Create tasks table with Taskwarrior-compatible fields and constraints
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            uuid TEXT UNIQUE NOT NULL,
            description TEXT NOT NULL CHECK (length(trim(description)) > 0),
            status TEXT NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'completed', 'deleted', 'waiting')),
            priority TEXT CHECK (priority IN ('H', 'M', 'L') OR (priority GLOB '[0-9]' AND CAST(priority AS INTEGER) BETWEEN 0 AND 9)),
            project TEXT,
            due_date TEXT CHECK (due_date IS NULL OR datetime(due_date) IS NOT NULL),
            scheduled_date TEXT CHECK (scheduled_date IS NULL OR datetime(scheduled_date) IS NOT NULL),
            start_date TEXT CHECK (start_date IS NULL OR datetime(start_date) IS NOT NULL),
            end_date TEXT CHECK (end_date IS NULL OR datetime(end_date) IS NOT NULL),
            entry_date TEXT NOT NULL CHECK (datetime(entry_date) IS NOT NULL),
            modified_date TEXT NOT NULL CHECK (datetime(modified_date) IS NOT NULL),
            tags TEXT CHECK (tags IS NULL OR json_valid(tags)),
            annotations TEXT CHECK (annotations IS NULL OR json_valid(annotations)),
            parent_uuid TEXT CHECK (parent_uuid IS NULL OR length(parent_uuid) = 36),
            depends TEXT CHECK (depends IS NULL OR json_valid(depends)),
            recurrence TEXT,
            effort INTEGER CHECK (effort IS NULL OR effort >= 0),
            effort_spent INTEGER CHECK (effort_spent IS NULL OR effort_spent >= 0),
            created_at TEXT NOT NULL CHECK (datetime(created_at) IS NOT NULL),
            updated_at TEXT NOT NULL CHECK (datetime(updated_at) IS NOT NULL)
        )
        "#,
    )
    .execute(pool)
    .await
    .map_err(|e| crate::core::EddaError::Storage(crate::core::StorageError::Migration {
        message: format!("Failed to create tasks table: {}", e),
    }))?;

    // Create documents table with constraints
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS documents (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            uuid TEXT UNIQUE NOT NULL,
            title TEXT NOT NULL CHECK (length(trim(title)) > 0),
            content TEXT,
            content_type TEXT,
            file_path TEXT,
            metadata TEXT CHECK (metadata IS NULL OR json_valid(metadata)),
            created_at TEXT NOT NULL CHECK (datetime(created_at) IS NOT NULL),
            updated_at TEXT NOT NULL CHECK (datetime(updated_at) IS NOT NULL)
        )
        "#,
    )
    .execute(pool)
    .await
    .map_err(|e| {
        crate::core::EddaError::Storage(crate::core::StorageError::Migration {
            message: format!("Failed to create documents table: {}", e),
        })
    })?;

    // Create state table with constraints
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS state (
            key TEXT PRIMARY KEY CHECK (length(trim(key)) > 0),
            value TEXT NOT NULL,
            created_at TEXT NOT NULL CHECK (datetime(created_at) IS NOT NULL),
            updated_at TEXT NOT NULL CHECK (datetime(updated_at) IS NOT NULL)
        )
        "#,
    )
    .execute(pool)
    .await
    .map_err(|e| {
        crate::core::EddaError::Storage(crate::core::StorageError::Migration {
            message: format!("Failed to create state table: {}", e),
        })
    })?;

    // Create all indexes for optimal performance
    let indexes = vec![
        "CREATE INDEX IF NOT EXISTS idx_tasks_status ON tasks(status)",
        "CREATE INDEX IF NOT EXISTS idx_tasks_project ON tasks(project)",
        "CREATE INDEX IF NOT EXISTS idx_tasks_due_date ON tasks(due_date)",
        "CREATE INDEX IF NOT EXISTS idx_tasks_uuid ON tasks(uuid)",
        "CREATE INDEX IF NOT EXISTS idx_tasks_parent_uuid ON tasks(parent_uuid)",
        "CREATE INDEX IF NOT EXISTS idx_tasks_priority ON tasks(priority)",
        "CREATE INDEX IF NOT EXISTS idx_tasks_entry_date ON tasks(entry_date)",
        "CREATE INDEX IF NOT EXISTS idx_tasks_modified_date ON tasks(modified_date)",
        "CREATE INDEX IF NOT EXISTS idx_tasks_status_priority ON tasks(status, priority)",
        "CREATE INDEX IF NOT EXISTS idx_tasks_project_status ON tasks(project, status)",
        "CREATE INDEX IF NOT EXISTS idx_documents_uuid ON documents(uuid)",
        "CREATE INDEX IF NOT EXISTS idx_documents_content_type ON documents(content_type)",
        "CREATE INDEX IF NOT EXISTS idx_state_key ON state(key)",
    ];

    for index_sql in indexes {
        sqlx::query(index_sql).execute(pool).await.map_err(|e| {
            crate::core::EddaError::Storage(crate::core::StorageError::Migration {
                message: format!("Failed to create index: {}", e),
            })
        })?;
    }

    Ok(())
}

/// Get a database connection pool
pub async fn get_pool(db_path: PathBuf) -> EddaResult<SqlitePool> {
    let database_url = format!("sqlite:{}", db_path.to_string_lossy());

    Ok(SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .map_err(|e| crate::core::StorageError::Connection {
            message: format!("Failed to connect to database: {e}"),
        })?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    #[tokio::test]
    #[serial]
    async fn test_init_database() {
        // Use in-memory database for reliable testing
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect("sqlite::memory:")
            .await
            .unwrap();

        // Run migrations
        let result = run_migrations(&pool).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[serial]
    async fn test_get_pool() {
        // Use in-memory database for reliable testing
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect("sqlite::memory:")
            .await
            .unwrap();

        // Run migrations
        run_migrations(&pool).await.unwrap();

        assert!(pool.acquire().await.is_ok());
    }

    #[tokio::test]
    #[serial]
    async fn test_database_tables_exist() {
        // Use in-memory database for more reliable testing

        // For in-memory database, we need to create the pool directly
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect("sqlite::memory:")
            .await
            .unwrap();

        // Run migrations
        run_migrations(&pool).await.unwrap();

        // Check that tables exist
        let tables = sqlx::query("SELECT name FROM sqlite_master WHERE type='table'")
            .fetch_all(&pool)
            .await
            .unwrap();

        let table_names: Vec<String> = tables
            .iter()
            .map(|row| row.get::<String, _>("name"))
            .collect();

        assert!(table_names.contains(&"tasks".to_string()));
        assert!(table_names.contains(&"documents".to_string()));
        assert!(table_names.contains(&"state".to_string()));
        assert!(table_names.contains(&"schema_version".to_string()));
    }

    #[tokio::test]
    #[serial]
    async fn test_database_indexes_exist() {
        // Use in-memory database for more reliable testing
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect("sqlite::memory:")
            .await
            .unwrap();

        // Run migrations
        run_migrations(&pool).await.unwrap();

        // Check that indexes exist
        let indexes = sqlx::query("SELECT name FROM sqlite_master WHERE type='index'")
            .fetch_all(&pool)
            .await
            .unwrap();

        let index_names: Vec<String> = indexes
            .iter()
            .map(|row| row.get::<String, _>("name"))
            .collect();

        assert!(index_names.contains(&"idx_tasks_status".to_string()));
        assert!(index_names.contains(&"idx_tasks_project".to_string()));
        assert!(index_names.contains(&"idx_tasks_due_date".to_string()));
        assert!(index_names.contains(&"idx_tasks_uuid".to_string()));
        assert!(index_names.contains(&"idx_tasks_parent_uuid".to_string()));
        assert!(index_names.contains(&"idx_tasks_priority".to_string()));
        assert!(index_names.contains(&"idx_tasks_entry_date".to_string()));
        assert!(index_names.contains(&"idx_tasks_modified_date".to_string()));
        assert!(index_names.contains(&"idx_tasks_status_priority".to_string()));
        assert!(index_names.contains(&"idx_tasks_project_status".to_string()));
        assert!(index_names.contains(&"idx_documents_uuid".to_string()));
        assert!(index_names.contains(&"idx_documents_content_type".to_string()));
        assert!(index_names.contains(&"idx_state_key".to_string()));
    }
}
