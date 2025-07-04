use crate::core::EddaResult;
use chrono::Utc;
use sqlx::{Row, SqlitePool, sqlite::SqlitePoolOptions};
use std::path::PathBuf;

#[cfg(test)]
use sqlx::Row as _;

/// Initialize the SQLite database
pub async fn init_database(db_path: PathBuf) -> EddaResult<()> {
    // Create database directory if it doesn't exist
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| crate::core::StorageError::Initialization {
            message: format!("Failed to create database directory: {e}"),
        })?;
    }

    // Create empty database file if it doesn't exist
    if !db_path.exists() {
        // Create an empty file to initialize SQLite database
        std::fs::File::create(&db_path).map_err(|e| crate::core::StorageError::Initialization {
            message: format!("Failed to create database file: {e}"),
        })?;
    }

    // Create database URL
    let database_url = format!("sqlite:{}", db_path.to_string_lossy());

    // Create connection pool
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .map_err(|e| crate::core::StorageError::Connection {
            message: format!("Failed to connect to database: {e}"),
        })?;

    // Run migrations
    run_migrations(&pool).await?;

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
