use crate::core::{Annotation, EddaError, EddaResult, Priority, Task, TaskError, TaskStatus};
use chrono::{DateTime, Utc};
use serde_json;
use sqlx::{Row, SqlitePool, sqlite::SqlitePoolOptions};
use uuid::Uuid;

/// Trait for task storage operations
#[async_trait::async_trait]
pub trait TaskStorage {
    /// Create a new task
    async fn create_task(&self, task: Task) -> EddaResult<Task>;

    /// Get a task by ID
    async fn get_task_by_id(&self, id: i64) -> EddaResult<Option<Task>>;

    /// Get a task by UUID
    async fn get_task_by_uuid(&self, uuid: Uuid) -> EddaResult<Option<Task>>;

    /// Update an existing task
    async fn update_task(&self, task: Task) -> EddaResult<Task>;

    /// Delete a task by ID
    async fn delete_task(&self, id: i64) -> EddaResult<bool>;

    /// List all tasks with optional filtering
    async fn list_tasks(&self, filter: Option<TaskFilter>) -> EddaResult<Vec<Task>>;

    /// Get task count
    async fn count_tasks(&self, filter: Option<TaskFilter>) -> EddaResult<u64>;
}

/// Task filter for querying tasks
#[derive(Debug, Clone)]
pub struct TaskFilter {
    pub status: Option<TaskStatus>,
    pub project: Option<String>,
    pub tags: Option<Vec<String>>,
    pub priority: Option<Priority>,
    pub include_deleted: bool,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

impl Default for TaskFilter {
    fn default() -> Self {
        Self {
            status: None,
            project: None,
            tags: None,
            priority: None,
            include_deleted: false,
            limit: None,
            offset: None,
        }
    }
}

/// SQLite implementation of task storage
pub struct SqliteTaskStorage {
    pool: SqlitePool,
}

impl SqliteTaskStorage {
    /// Create a new SQLite task storage
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl TaskStorage for SqliteTaskStorage {
    async fn create_task(&self, mut task: Task) -> EddaResult<Task> {
        // Ensure task has a UUID
        if task.uuid == Uuid::nil() {
            task.uuid = Uuid::new_v4();
        }

        // Update timestamps
        let now = Utc::now();
        task.entry_date = now;
        task.modified_date = now;

        // Serialize complex fields
        let tags_json = serde_json::to_string(&task.tags).map_err(|e| TaskError::Validation {
            message: format!("Failed to serialize tags: {}", e),
        })?;

        let annotations_json =
            serde_json::to_string(&task.annotations).map_err(|e| TaskError::Validation {
                message: format!("Failed to serialize annotations: {}", e),
            })?;

        let depends_json =
            serde_json::to_string(&task.depends).map_err(|e| TaskError::Validation {
                message: format!("Failed to serialize depends: {}", e),
            })?;

        let result = sqlx::query(
            r#"
            INSERT INTO tasks (
                uuid, description, status, priority, project, due_date, scheduled_date,
                start_date, end_date, entry_date, modified_date, tags, annotations,
                parent_uuid, depends, recurrence, effort, effort_spent, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(task.uuid.to_string())
        .bind(&task.description)
        .bind(task.status.to_string())
        .bind(task.priority.as_ref().map(|p| p.to_string()))
        .bind(&task.project)
        .bind(task.due_date.map(|d| d.to_rfc3339()))
        .bind(task.scheduled_date.map(|d| d.to_rfc3339()))
        .bind(task.start_date.map(|d| d.to_rfc3339()))
        .bind(task.end_date.map(|d| d.to_rfc3339()))
        .bind(task.entry_date.to_rfc3339())
        .bind(task.modified_date.to_rfc3339())
        .bind(&tags_json)
        .bind(&annotations_json)
        .bind(task.parent_uuid.map(|u| u.to_string()))
        .bind(&depends_json)
        .bind(&task.recurrence)
        .bind(task.effort)
        .bind(task.effort_spent)
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(|e| {
            EddaError::Task(TaskError::Storage {
                message: format!("Failed to create task: {}", e),
            })
        })?;

        // Set the ID from the insert result
        task.id = Some(result.last_insert_rowid());

        Ok(task)
    }

    async fn get_task_by_id(&self, id: i64) -> EddaResult<Option<Task>> {
        let row = sqlx::query("SELECT * FROM tasks WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| {
                EddaError::Task(TaskError::Storage {
                    message: format!("Failed to get task: {}", e),
                })
            })?;

        if let Some(row) = row {
            Ok(Some(row_to_task(row)?))
        } else {
            Ok(None)
        }
    }

    async fn get_task_by_uuid(&self, uuid: Uuid) -> EddaResult<Option<Task>> {
        let row = sqlx::query("SELECT * FROM tasks WHERE uuid = ?")
            .bind(uuid.to_string())
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| TaskError::Storage {
                message: format!("Failed to get task: {}", e),
            })?;

        if let Some(row) = row {
            Ok(Some(row_to_task(row)?))
        } else {
            Ok(None)
        }
    }

    async fn update_task(&self, mut task: Task) -> EddaResult<Task> {
        if task.id.is_none() {
            return Err(EddaError::Task(TaskError::Validation {
                message: "Task must have an ID to update".to_string(),
            }));
        }

        // Update modified timestamp
        task.modified_date = Utc::now();

        // Serialize complex fields
        let tags_json = serde_json::to_string(&task.tags).map_err(|e| TaskError::Validation {
            message: format!("Failed to serialize tags: {}", e),
        })?;

        let annotations_json =
            serde_json::to_string(&task.annotations).map_err(|e| TaskError::Validation {
                message: format!("Failed to serialize annotations: {}", e),
            })?;

        let depends_json =
            serde_json::to_string(&task.depends).map_err(|e| TaskError::Validation {
                message: format!("Failed to serialize depends: {}", e),
            })?;

        sqlx::query(
            r#"
            UPDATE tasks SET
                description = ?, status = ?, priority = ?, project = ?, due_date = ?,
                scheduled_date = ?, start_date = ?, end_date = ?, modified_date = ?,
                tags = ?, annotations = ?, parent_uuid = ?, depends = ?, recurrence = ?,
                effort = ?, effort_spent = ?, updated_at = ?
            WHERE id = ?
            "#,
        )
        .bind(&task.description)
        .bind(task.status.to_string())
        .bind(task.priority.as_ref().map(|p| p.to_string()))
        .bind(&task.project)
        .bind(task.due_date.map(|d| d.to_rfc3339()))
        .bind(task.scheduled_date.map(|d| d.to_rfc3339()))
        .bind(task.start_date.map(|d| d.to_rfc3339()))
        .bind(task.end_date.map(|d| d.to_rfc3339()))
        .bind(task.modified_date.to_rfc3339())
        .bind(&tags_json)
        .bind(&annotations_json)
        .bind(task.parent_uuid.map(|u| u.to_string()))
        .bind(&depends_json)
        .bind(&task.recurrence)
        .bind(task.effort)
        .bind(task.effort_spent)
        .bind(task.modified_date.to_rfc3339())
        .bind(task.id.unwrap())
        .execute(&self.pool)
        .await
        .map_err(|e| TaskError::Storage {
            message: format!("Failed to update task: {}", e),
        })?;

        Ok(task)
    }

    async fn delete_task(&self, id: i64) -> EddaResult<bool> {
        let result = sqlx::query("DELETE FROM tasks WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| TaskError::Storage {
                message: format!("Failed to delete task: {}", e),
            })?;

        Ok(result.rows_affected() > 0)
    }

    async fn list_tasks(&self, filter: Option<TaskFilter>) -> EddaResult<Vec<Task>> {
        let filter = filter.unwrap_or_default();

        let mut query = String::from("SELECT * FROM tasks WHERE 1=1");
        let mut conditions = Vec::new();

        if !filter.include_deleted {
            conditions.push("status != 'deleted'");
        }

        if let Some(_status) = &filter.status {
            conditions.push("status = ?");
        }

        if let Some(_project) = &filter.project {
            conditions.push("project = ?");
        }

        if let Some(_priority) = &filter.priority {
            conditions.push("priority = ?");
        }

        // Add conditions to query
        for condition in conditions {
            query.push_str(&format!(" AND {}", condition));
        }

        // Add ordering
        query.push_str(" ORDER BY modified_date DESC");

        // Add limit and offset
        if let Some(limit) = filter.limit {
            query.push_str(&format!(" LIMIT {}", limit));
        }

        if let Some(offset) = filter.offset {
            query.push_str(&format!(" OFFSET {}", offset));
        }

        // Execute query with parameters
        let mut query_builder = sqlx::query(&query);

        if let Some(status) = &filter.status {
            query_builder = query_builder.bind(status.to_string());
        }

        if let Some(project) = &filter.project {
            query_builder = query_builder.bind(project);
        }

        if let Some(priority) = &filter.priority {
            query_builder = query_builder.bind(priority.to_string());
        }

        let rows = query_builder
            .fetch_all(&self.pool)
            .await
            .map_err(|e| TaskError::Storage {
                message: format!("Failed to list tasks: {}", e),
            })?;

        let mut tasks = Vec::new();
        for row in rows {
            tasks.push(row_to_task(row)?);
        }

        Ok(tasks)
    }

    async fn count_tasks(&self, filter: Option<TaskFilter>) -> EddaResult<u64> {
        let filter = filter.unwrap_or_default();

        let mut query = String::from("SELECT COUNT(*) FROM tasks WHERE 1=1");
        let mut conditions = Vec::new();

        if !filter.include_deleted {
            conditions.push("status != 'deleted'");
        }

        if let Some(_status) = &filter.status {
            conditions.push("status = ?");
        }

        if let Some(_project) = &filter.project {
            conditions.push("project = ?");
        }

        if let Some(_priority) = &filter.priority {
            conditions.push("priority = ?");
        }

        // Add conditions to query
        for condition in conditions {
            query.push_str(&format!(" AND {}", condition));
        }

        // Execute query with parameters
        let mut query_builder = sqlx::query_scalar(&query);

        if let Some(status) = &filter.status {
            query_builder = query_builder.bind(status.to_string());
        }

        if let Some(project) = &filter.project {
            query_builder = query_builder.bind(project);
        }

        if let Some(priority) = &filter.priority {
            query_builder = query_builder.bind(priority.to_string());
        }

        let count: i64 =
            query_builder
                .fetch_one(&self.pool)
                .await
                .map_err(|e| TaskError::Storage {
                    message: format!("Failed to count tasks: {}", e),
                })?;

        Ok(count as u64)
    }
}

/// Convert a database row to a Task
fn row_to_task(row: sqlx::sqlite::SqliteRow) -> EddaResult<Task> {
    let id: i64 = row.get("id");
    let uuid_str: String = row.get("uuid");
    let description: String = row.get("description");
    let status_str: String = row.get("status");
    let priority_str: Option<String> = row.get("priority");
    let project: Option<String> = row.get("project");
    let due_date_str: Option<String> = row.get("due_date");
    let scheduled_date_str: Option<String> = row.get("scheduled_date");
    let start_date_str: Option<String> = row.get("start_date");
    let end_date_str: Option<String> = row.get("end_date");
    let entry_date_str: String = row.get("entry_date");
    let modified_date_str: String = row.get("modified_date");
    let tags_json: String = row.get("tags");
    let annotations_json: String = row.get("annotations");
    let parent_uuid_str: Option<String> = row.get("parent_uuid");
    let depends_json: String = row.get("depends");
    let recurrence: Option<String> = row.get("recurrence");
    let effort: Option<i64> = row.get("effort");
    let effort_spent: Option<i64> = row.get("effort_spent");

    // Parse UUID
    let uuid = Uuid::parse_str(&uuid_str).map_err(|e| TaskError::Validation {
        message: format!("Invalid UUID: {}", e),
    })?;

    // Parse status
    let status = status_str
        .parse::<TaskStatus>()
        .map_err(|e| TaskError::Validation {
            message: format!("Invalid status: {}", e),
        })?;

    // Parse priority
    let priority = if let Some(priority_str) = priority_str {
        Some(
            priority_str
                .parse::<Priority>()
                .map_err(|e| TaskError::Validation {
                    message: format!("Invalid priority: {}", e),
                })?,
        )
    } else {
        None
    };

    // Parse dates
    let due_date = if let Some(date_str) = due_date_str {
        Some(
            DateTime::parse_from_rfc3339(&date_str)
                .map_err(|e| TaskError::Validation {
                    message: format!("Invalid due date: {}", e),
                })?
                .with_timezone(&Utc),
        )
    } else {
        None
    };

    let scheduled_date = if let Some(date_str) = scheduled_date_str {
        Some(
            DateTime::parse_from_rfc3339(&date_str)
                .map_err(|e| TaskError::Validation {
                    message: format!("Invalid scheduled date: {}", e),
                })?
                .with_timezone(&Utc),
        )
    } else {
        None
    };

    let start_date = if let Some(date_str) = start_date_str {
        Some(
            DateTime::parse_from_rfc3339(&date_str)
                .map_err(|e| TaskError::Validation {
                    message: format!("Invalid start date: {}", e),
                })?
                .with_timezone(&Utc),
        )
    } else {
        None
    };

    let end_date = if let Some(date_str) = end_date_str {
        Some(
            DateTime::parse_from_rfc3339(&date_str)
                .map_err(|e| TaskError::Validation {
                    message: format!("Invalid end date: {}", e),
                })?
                .with_timezone(&Utc),
        )
    } else {
        None
    };

    let entry_date = DateTime::parse_from_rfc3339(&entry_date_str)
        .map_err(|e| TaskError::Validation {
            message: format!("Invalid entry date: {}", e),
        })?
        .with_timezone(&Utc);

    let modified_date = DateTime::parse_from_rfc3339(&modified_date_str)
        .map_err(|e| TaskError::Validation {
            message: format!("Invalid modified date: {}", e),
        })?
        .with_timezone(&Utc);

    // Parse JSON fields
    let tags: std::collections::HashSet<String> =
        serde_json::from_str(&tags_json).map_err(|e| TaskError::Validation {
            message: format!("Invalid tags JSON: {}", e),
        })?;

    let annotations: Vec<Annotation> =
        serde_json::from_str(&annotations_json).map_err(|e| TaskError::Validation {
            message: format!("Invalid annotations JSON: {}", e),
        })?;

    let parent_uuid = if let Some(uuid_str) = parent_uuid_str {
        Some(
            Uuid::parse_str(&uuid_str).map_err(|e| TaskError::Validation {
                message: format!("Invalid parent UUID: {}", e),
            })?,
        )
    } else {
        None
    };

    let depends: std::collections::HashSet<Uuid> =
        serde_json::from_str(&depends_json).map_err(|e| TaskError::Validation {
            message: format!("Invalid depends JSON: {}", e),
        })?;

    Ok(Task {
        id: Some(id),
        uuid,
        description,
        status,
        priority,
        project,
        due_date,
        scheduled_date,
        start_date,
        end_date,
        entry_date,
        modified_date,
        tags,
        annotations,
        parent_uuid,
        depends,
        recurrence,
        effort: effort.map(|e| e as u32),
        effort_spent: effort_spent.map(|e| e as u32),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    #[tokio::test]
    #[serial]
    async fn test_create_and_get_task() {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect("sqlite::memory:")
            .await
            .unwrap();

        // Run migrations
        crate::storage::database::run_migrations(&pool)
            .await
            .unwrap();

        let storage = SqliteTaskStorage::new(pool);

        // Create a task
        let task = Task::new("Test task".to_string());
        let created_task = storage.create_task(task).await.unwrap();

        assert!(created_task.id.is_some());
        assert_eq!(created_task.description, "Test task");

        // Get the task by ID
        let retrieved_task = storage
            .get_task_by_id(created_task.id.unwrap())
            .await
            .unwrap();
        assert!(retrieved_task.is_some());
        assert_eq!(retrieved_task.unwrap().description, "Test task");

        // Get the task by UUID
        let retrieved_task = storage.get_task_by_uuid(created_task.uuid).await.unwrap();
        assert!(retrieved_task.is_some());
        assert_eq!(retrieved_task.unwrap().description, "Test task");
    }

    #[tokio::test]
    #[serial]
    async fn test_update_task() {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect("sqlite::memory:")
            .await
            .unwrap();

        // Run migrations
        crate::storage::database::run_migrations(&pool)
            .await
            .unwrap();

        let storage = SqliteTaskStorage::new(pool);

        // Create a task
        let task = Task::new("Test task".to_string());
        let created_task = storage.create_task(task).await.unwrap();

        // Update the task
        let mut updated_task = created_task.clone();
        updated_task.description = "Updated task".to_string();
        updated_task.priority = Some(Priority::High);

        let result = storage.update_task(updated_task).await.unwrap();
        assert_eq!(result.description, "Updated task");
        assert_eq!(result.priority, Some(Priority::High));

        // Verify the update
        let retrieved_task = storage
            .get_task_by_id(created_task.id.unwrap())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(retrieved_task.description, "Updated task");
        assert_eq!(retrieved_task.priority, Some(Priority::High));
    }

    #[tokio::test]
    #[serial]
    async fn test_list_tasks() {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect("sqlite::memory:")
            .await
            .unwrap();

        // Run migrations
        crate::storage::database::run_migrations(&pool)
            .await
            .unwrap();

        let storage = SqliteTaskStorage::new(pool);

        // Create multiple tasks
        let task1 = Task::new("Task 1".to_string());
        let task2 = Task::new("Task 2".to_string());
        let task3 = Task::new("Task 3".to_string());

        storage.create_task(task1).await.unwrap();
        storage.create_task(task2).await.unwrap();
        storage.create_task(task3).await.unwrap();

        // List all tasks
        let tasks = storage.list_tasks(None).await.unwrap();
        assert_eq!(tasks.len(), 3);

        // List with filter
        let filter = TaskFilter {
            status: Some(TaskStatus::Pending),
            ..Default::default()
        };
        let tasks = storage.list_tasks(Some(filter)).await.unwrap();
        assert_eq!(tasks.len(), 3); // All tasks should be pending by default
    }

    #[tokio::test]
    #[serial]
    async fn test_delete_task() {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect("sqlite::memory:")
            .await
            .unwrap();

        // Run migrations
        crate::storage::database::run_migrations(&pool)
            .await
            .unwrap();

        let storage = SqliteTaskStorage::new(pool);

        // Create a task
        let task = Task::new("Test task".to_string());
        let created_task = storage.create_task(task).await.unwrap();

        // Delete the task
        let deleted = storage.delete_task(created_task.id.unwrap()).await.unwrap();
        assert!(deleted);

        // Verify the task is gone
        let retrieved_task = storage
            .get_task_by_id(created_task.id.unwrap())
            .await
            .unwrap();
        assert!(retrieved_task.is_none());
    }
}
