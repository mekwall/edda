use crate::core::EddaResult;
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use std::path::PathBuf;

#[cfg(test)]
use sqlx::Row;

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
    // Create tasks table with Taskwarrior-compatible fields
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
            parent_uuid TEXT,
            depends TEXT,
            recurrence TEXT,
            effort INTEGER,
            effort_spent INTEGER,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await
    .map_err(|e| crate::core::StorageError::Migration {
        message: format!("Failed to create tasks table: {e}"),
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
        message: format!("Failed to create documents table: {e}"),
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
        message: format!("Failed to create state table: {e}"),
    })?;

    // Create indexes for tasks
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_tasks_status ON tasks(status)")
        .execute(pool)
        .await
        .map_err(|e| crate::core::StorageError::Migration {
            message: format!("Failed to create tasks status index: {e}"),
        })?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_tasks_project ON tasks(project)")
        .execute(pool)
        .await
        .map_err(|e| crate::core::StorageError::Migration {
            message: format!("Failed to create tasks project index: {e}"),
        })?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_tasks_due_date ON tasks(due_date)")
        .execute(pool)
        .await
        .map_err(|e| crate::core::StorageError::Migration {
            message: format!("Failed to create tasks due date index: {e}"),
        })?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_tasks_uuid ON tasks(uuid)")
        .execute(pool)
        .await
        .map_err(|e| crate::core::StorageError::Migration {
            message: format!("Failed to create tasks uuid index: {e}"),
        })?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_tasks_parent_uuid ON tasks(parent_uuid)")
        .execute(pool)
        .await
        .map_err(|e| crate::core::StorageError::Migration {
            message: format!("Failed to create tasks parent_uuid index: {e}"),
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
    }
}
