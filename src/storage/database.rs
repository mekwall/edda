use crate::core::EddaResult;
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use std::path::PathBuf;

/// Initialize the SQLite database
pub async fn init_database(db_path: PathBuf) -> EddaResult<()> {
    // Create database directory if it doesn't exist
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| crate::core::StorageError::Initialization {
            message: format!("Failed to create database directory: {}", e),
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
            message: format!("Failed to connect to database: {}", e),
        })?;

    // Run migrations
    run_migrations(&pool).await?;

    Ok(())
}

/// Run database migrations
async fn run_migrations(pool: &SqlitePool) -> EddaResult<()> {
    // Create tasks table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            uuid TEXT UNIQUE NOT NULL,
            description TEXT NOT NULL,
            status TEXT NOT NULL DEFAULT 'pending',
            priority TEXT,
            project TEXT,
            due_date TEXT,
            scheduled_date TEXT,
            start_date TEXT,
            end_date TEXT,
            entry_date TEXT NOT NULL,
            modified_date TEXT NOT NULL,
            tags TEXT,
            annotations TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await
    .map_err(|e| crate::core::StorageError::Migration {
        message: format!("Failed to create tasks table: {}", e),
    })?;

    // Create documents table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS documents (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            uuid TEXT UNIQUE NOT NULL,
            title TEXT NOT NULL,
            content TEXT,
            content_type TEXT,
            file_path TEXT,
            metadata TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await
    .map_err(|e| crate::core::StorageError::Migration {
        message: format!("Failed to create documents table: {}", e),
    })?;

    // Create state table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS state (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await
    .map_err(|e| crate::core::StorageError::Migration {
        message: format!("Failed to create state table: {}", e),
    })?;

    // Create indexes
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_tasks_status ON tasks(status)")
        .execute(pool)
        .await
        .map_err(|e| crate::core::StorageError::Migration {
            message: format!("Failed to create tasks status index: {}", e),
        })?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_tasks_project ON tasks(project)")
        .execute(pool)
        .await
        .map_err(|e| crate::core::StorageError::Migration {
            message: format!("Failed to create tasks project index: {}", e),
        })?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_tasks_due_date ON tasks(due_date)")
        .execute(pool)
        .await
        .map_err(|e| crate::core::StorageError::Migration {
            message: format!("Failed to create tasks due date index: {}", e),
        })?;

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
            message: format!("Failed to connect to database: {}", e),
        })?)
}
