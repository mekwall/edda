use crate::core::{EddaError, EddaResult, TaskError};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use uuid::Uuid;

/// Task status enum matching Taskwarrior statuses
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[derive(Default)]
pub enum TaskStatus {
    #[default]
    Pending,
    Completed,
    Deleted,
    Waiting,
}


impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskStatus::Pending => write!(f, "pending"),
            TaskStatus::Completed => write!(f, "completed"),
            TaskStatus::Deleted => write!(f, "deleted"),
            TaskStatus::Waiting => write!(f, "waiting"),
        }
    }
}

impl std::str::FromStr for TaskStatus {
    type Err = crate::core::TaskError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "pending" => Ok(TaskStatus::Pending),
            "completed" => Ok(TaskStatus::Completed),
            "deleted" => Ok(TaskStatus::Deleted),
            "waiting" => Ok(TaskStatus::Waiting),
            _ => Err(crate::core::TaskError::Validation {
                message: format!("Invalid task status: {s}"),
            }),
        }
    }
}

/// Task priority enum matching Taskwarrior priorities
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[derive(Default)]
pub enum Priority {
    High,
    #[default]
    Medium,
    Low,
    Number(u8), // 0-9
}


impl std::fmt::Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Priority::High => write!(f, "H"),
            Priority::Medium => write!(f, "M"),
            Priority::Low => write!(f, "L"),
            Priority::Number(n) => write!(f, "{n}"),
        }
    }
}

impl std::str::FromStr for Priority {
    type Err = crate::core::TaskError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "H" | "HIGH" => Ok(Priority::High),
            "M" | "MEDIUM" => Ok(Priority::Medium),
            "L" | "LOW" => Ok(Priority::Low),
            s => {
                if let Ok(n) = s.parse::<u8>() {
                    if n <= 9 {
                        Ok(Priority::Number(n))
                    } else {
                        Err(crate::core::TaskError::Validation {
                            message: format!("Priority number must be 0-9, got: {n}"),
                        })
                    }
                } else {
                    Err(crate::core::TaskError::Validation {
                        message: format!("Invalid priority: {s}"),
                    })
                }
            }
        }
    }
}

/// Task annotation (note/comment)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Annotation {
    pub entry: DateTime<Utc>,
    pub description: String,
}

/// Main Task struct with Taskwarrior-compatible fields
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Task {
    /// Internal database ID
    pub id: Option<i64>,

    /// Unique identifier (UUID)
    pub uuid: Uuid,

    /// Task description/title
    pub description: String,

    /// Task status
    pub status: TaskStatus,

    /// Task priority
    pub priority: Option<Priority>,

    /// Project name
    pub project: Option<String>,

    /// Due date
    pub due_date: Option<DateTime<Utc>>,

    /// Scheduled date (when task should be started)
    pub scheduled_date: Option<DateTime<Utc>>,

    /// Start date (when task was actually started)
    pub start_date: Option<DateTime<Utc>>,

    /// End date (when task was completed)
    pub end_date: Option<DateTime<Utc>>,

    /// Entry date (when task was created)
    pub entry_date: DateTime<Utc>,

    /// Modified date (when task was last modified)
    pub modified_date: DateTime<Utc>,

    /// Task tags
    pub tags: HashSet<String>,

    /// Task annotations/notes
    pub annotations: Vec<Annotation>,

    /// Parent task UUID (for subtasks)
    pub parent_uuid: Option<Uuid>,

    /// Dependencies (UUIDs of tasks this task depends on)
    pub depends: HashSet<Uuid>,

    /// Recurrence pattern (if task repeats)
    pub recurrence: Option<String>,

    /// Estimated effort (in minutes)
    pub effort: Option<u32>,

    /// Actual effort spent (in minutes)
    pub effort_spent: Option<u32>,
}

impl Task {
    /// Create a new task with the given description
    pub fn new(description: String) -> Self {
        let now = Utc::now();
        Self {
            id: None,
            uuid: Uuid::new_v4(),
            description,
            status: TaskStatus::Pending,
            priority: None,
            project: None,
            due_date: None,
            scheduled_date: None,
            start_date: None,
            end_date: None,
            entry_date: now,
            modified_date: now,
            tags: HashSet::new(),
            annotations: Vec::new(),
            parent_uuid: None,
            depends: HashSet::new(),
            recurrence: None,
            effort: None,
            effort_spent: None,
        }
    }

    /// Add a tag to the task
    pub fn add_tag(&mut self, tag: String) {
        self.tags.insert(tag);
        self.modified_date = Utc::now();
    }

    /// Remove a tag from the task
    pub fn remove_tag(&mut self, tag: &str) -> bool {
        let removed = self.tags.remove(tag);
        if removed {
            self.modified_date = Utc::now();
        }
        removed
    }

    /// Add an annotation to the task
    pub fn add_annotation(&mut self, description: String) {
        let annotation = Annotation {
            entry: Utc::now(),
            description,
        };
        self.annotations.push(annotation);
        self.modified_date = Utc::now();
    }

    /// Mark task as started
    pub fn start(&mut self) -> Result<(), crate::core::TaskError> {
        if self.status != TaskStatus::Pending && self.status != TaskStatus::Waiting {
            return Err(crate::core::TaskError::InvalidStatusTransition {
                from: self.status.to_string(),
                to: "started".to_string(),
            });
        }

        self.start_date = Some(Utc::now());
        self.modified_date = Utc::now();
        Ok(())
    }

    /// Mark task as completed
    pub fn complete(&mut self) -> Result<(), crate::core::TaskError> {
        if self.status == TaskStatus::Completed {
            return Err(crate::core::TaskError::InvalidStatusTransition {
                from: self.status.to_string(),
                to: "completed".to_string(),
            });
        }

        self.status = TaskStatus::Completed;
        self.end_date = Some(Utc::now());
        self.modified_date = Utc::now();
        Ok(())
    }

    /// Mark task as deleted
    pub fn delete(&mut self) -> Result<(), crate::core::TaskError> {
        if self.status == TaskStatus::Deleted {
            return Err(crate::core::TaskError::InvalidStatusTransition {
                from: self.status.to_string(),
                to: "deleted".to_string(),
            });
        }

        self.status = TaskStatus::Deleted;
        self.modified_date = Utc::now();
        Ok(())
    }

    /// Check if task is active (pending or waiting)
    pub fn is_active(&self) -> bool {
        matches!(self.status, TaskStatus::Pending | TaskStatus::Waiting)
    }

    /// Check if task is completed
    pub fn is_completed(&self) -> bool {
        self.status == TaskStatus::Completed
    }

    /// Check if task is deleted
    pub fn is_deleted(&self) -> bool {
        self.status == TaskStatus::Deleted
    }

    /// Check if task is overdue
    pub fn is_overdue(&self) -> bool {
        if let Some(due_date) = self.due_date {
            self.is_active() && Utc::now() > due_date
        } else {
            false
        }
    }

    /// Get task age in days
    pub fn age_days(&self) -> i64 {
        let now = Utc::now();
        (now - self.entry_date).num_days()
    }

    /// Get task urgency score (simplified version)
    pub fn urgency_score(&self) -> f64 {
        let mut score = 0.0;

        // Priority score
        if let Some(priority) = &self.priority {
            match priority {
                Priority::High => score += 10.0,
                Priority::Medium => score += 5.0,
                Priority::Low => score += 1.0,
                Priority::Number(n) => score += *n as f64,
            }
        }

        // Due date score
        if let Some(due_date) = self.due_date {
            let now = Utc::now();
            if now > due_date {
                // Overdue tasks get high urgency
                score += 15.0;
            } else {
                // Due soon tasks get moderate urgency
                let days_until_due = (due_date - now).num_days();
                if days_until_due <= 1 {
                    score += 10.0;
                } else if days_until_due <= 7 {
                    score += 5.0;
                }
            }
        }

        // Age score (older tasks get slightly higher urgency)
        let age_days = self.age_days();
        if age_days > 30 {
            score += 2.0;
        } else if age_days > 7 {
            score += 1.0;
        }

        score
    }
}

impl std::fmt::Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let id = self.id.unwrap_or(0);
        let priority = self
            .priority
            .as_ref()
            .map(|p| p.to_string())
            .unwrap_or_default();
        let project = self
            .project
            .as_ref()
            .map(|p| format!(" [{p}]"))
            .unwrap_or_default();
        let tags = if self.tags.is_empty() {
            String::new()
        } else {
            format!(
                " {}",
                self.tags
                    .iter()
                    .map(|t| format!("+{t}"))
                    .collect::<Vec<_>>()
                    .join(" ")
            )
        };

        write!(f, "{id}{priority}{project}{tags}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_new() {
        let task = Task::new("Test task".to_string());
        assert_eq!(task.description, "Test task");
        assert_eq!(task.status, TaskStatus::Pending);
        assert!(task.id.is_none());
        assert!(task.uuid != Uuid::nil());
        assert!(task.is_active());
        assert!(!task.is_completed());
        assert!(!task.is_deleted());
    }

    #[test]
    fn test_task_status_display() {
        assert_eq!(TaskStatus::Pending.to_string(), "pending");
        assert_eq!(TaskStatus::Completed.to_string(), "completed");
        assert_eq!(TaskStatus::Deleted.to_string(), "deleted");
        assert_eq!(TaskStatus::Waiting.to_string(), "waiting");
    }

    #[test]
    fn test_task_status_from_str() {
        assert_eq!(
            "pending".parse::<TaskStatus>().unwrap(),
            TaskStatus::Pending
        );
        assert_eq!(
            "COMPLETED".parse::<TaskStatus>().unwrap(),
            TaskStatus::Completed
        );
        assert_eq!(
            "Deleted".parse::<TaskStatus>().unwrap(),
            TaskStatus::Deleted
        );
        assert_eq!(
            "waiting".parse::<TaskStatus>().unwrap(),
            TaskStatus::Waiting
        );

        assert!("invalid".parse::<TaskStatus>().is_err());
    }

    #[test]
    fn test_priority_display() {
        assert_eq!(Priority::High.to_string(), "H");
        assert_eq!(Priority::Medium.to_string(), "M");
        assert_eq!(Priority::Low.to_string(), "L");
        assert_eq!(Priority::Number(5).to_string(), "5");
    }

    #[test]
    fn test_priority_from_str() {
        assert_eq!("H".parse::<Priority>().unwrap(), Priority::High);
        assert_eq!("high".parse::<Priority>().unwrap(), Priority::High);
        assert_eq!("M".parse::<Priority>().unwrap(), Priority::Medium);
        assert_eq!("medium".parse::<Priority>().unwrap(), Priority::Medium);
        assert_eq!("L".parse::<Priority>().unwrap(), Priority::Low);
        assert_eq!("low".parse::<Priority>().unwrap(), Priority::Low);
        assert_eq!("5".parse::<Priority>().unwrap(), Priority::Number(5));
        assert_eq!("0".parse::<Priority>().unwrap(), Priority::Number(0));
        assert_eq!("9".parse::<Priority>().unwrap(), Priority::Number(9));

        assert!("10".parse::<Priority>().is_err());
        assert!("invalid".parse::<Priority>().is_err());
    }

    #[test]
    fn test_task_add_remove_tag() {
        let mut task = Task::new("Test task".to_string());

        task.add_tag("urgent".to_string());
        assert!(task.tags.contains("urgent"));

        task.add_tag("work".to_string());
        assert!(task.tags.contains("work"));
        assert_eq!(task.tags.len(), 2);

        assert!(task.remove_tag("urgent"));
        assert!(!task.tags.contains("urgent"));
        assert!(task.tags.contains("work"));
        assert_eq!(task.tags.len(), 1);

        assert!(!task.remove_tag("nonexistent"));
    }

    #[test]
    fn test_task_add_annotation() {
        let mut task = Task::new("Test task".to_string());
        let original_modified = task.modified_date;

        task.add_annotation("Test note".to_string());

        assert_eq!(task.annotations.len(), 1);
        assert_eq!(task.annotations[0].description, "Test note");
        assert!(task.modified_date > original_modified);
    }

    #[test]
    fn test_task_start() {
        let mut task = Task::new("Test task".to_string());

        assert!(task.start().is_ok());
        assert!(task.start_date.is_some());

        // Can't start a completed task
        task.status = TaskStatus::Completed;
        assert!(task.start().is_err());
    }

    #[test]
    fn test_task_complete() {
        let mut task = Task::new("Test task".to_string());

        assert!(task.complete().is_ok());
        assert_eq!(task.status, TaskStatus::Completed);
        assert!(task.end_date.is_some());
        assert!(task.is_completed());

        // Can't complete an already completed task
        assert!(task.complete().is_err());
    }

    #[test]
    fn test_task_delete() {
        let mut task = Task::new("Test task".to_string());

        assert!(task.delete().is_ok());
        assert_eq!(task.status, TaskStatus::Deleted);
        assert!(task.is_deleted());

        // Can't delete an already deleted task
        assert!(task.delete().is_err());
    }

    #[test]
    fn test_task_urgency_score() {
        let mut task = Task::new("Test task".to_string());

        // Basic task has low urgency
        assert_eq!(task.urgency_score(), 0.0);

        // High priority increases urgency
        task.priority = Some(Priority::High);
        assert_eq!(task.urgency_score(), 10.0);

        // Due date increases urgency
        task.due_date = Some(Utc::now() + chrono::Duration::days(1));
        assert!(task.urgency_score() > 10.0);
    }

    #[test]
    fn test_task_is_overdue() {
        let mut task = Task::new("Test task".to_string());

        // No due date
        assert!(!task.is_overdue());

        // Future due date
        task.due_date = Some(Utc::now() + chrono::Duration::days(1));
        assert!(!task.is_overdue());

        // Past due date, active task
        task.due_date = Some(Utc::now() - chrono::Duration::days(1));
        assert!(task.is_overdue());

        // Past due date, completed task
        task.status = TaskStatus::Completed;
        assert!(!task.is_overdue());
    }
}

/// Task engine for high-level task management operations
pub struct TaskEngine {
    storage: Box<dyn crate::storage::TaskStorage + Send + Sync>,
}

impl TaskEngine {
    /// Create a new task engine with the given storage backend
    pub fn new(storage: Box<dyn crate::storage::TaskStorage + Send + Sync>) -> Self {
        Self { storage }
    }

    /// Create a new task with validation
    pub async fn create_task(&self, description: String) -> EddaResult<Task> {
        // Validate description
        if description.trim().is_empty() {
            return Err(EddaError::Task(TaskError::Validation {
                message: "Task description cannot be empty".to_string(),
            }));
        }

        let task = Task::new(description);
        self.storage.create_task(task).await
    }

    /// Get a task by ID with validation
    pub async fn get_task(&self, id: i64) -> EddaResult<Option<Task>> {
        if id <= 0 {
            return Err(EddaError::Task(TaskError::Validation {
                message: "Task ID must be positive".to_string(),
            }));
        }

        self.storage.get_task_by_id(id).await
    }

    /// Get a task by UUID
    pub async fn get_task_by_uuid(&self, uuid: Uuid) -> EddaResult<Option<Task>> {
        self.storage.get_task_by_uuid(uuid).await
    }

    /// Update a task with validation
    pub async fn update_task(&self, mut task: Task) -> EddaResult<Task> {
        // Validate description
        if task.description.trim().is_empty() {
            return Err(EddaError::Task(TaskError::Validation {
                message: "Task description cannot be empty".to_string(),
            }));
        }

        // Validate status transitions
        if let Some(existing_task) = self.storage.get_task_by_id(task.id.unwrap_or(0)).await? {
            if !self.is_valid_status_transition(&existing_task.status, &task.status) {
                return Err(EddaError::Task(TaskError::Validation {
                    message: format!(
                        "Invalid status transition from {} to {}",
                        existing_task.status, task.status
                    ),
                }));
            }
        }

        // Update timestamps
        task.modified_date = Utc::now();

        self.storage.update_task(task).await
    }

    /// Mark a task as completed
    pub async fn complete_task(&self, id: i64) -> EddaResult<Task> {
        let mut task = self
            .get_task(id)
            .await?
            .ok_or_else(|| EddaError::Task(TaskError::NotFound { id: id.to_string() }))?;

        task.complete()?;
        self.storage.update_task(task).await
    }

    /// Mark a task as deleted
    pub async fn delete_task(&self, id: i64) -> EddaResult<Task> {
        let mut task = self
            .get_task(id)
            .await?
            .ok_or_else(|| EddaError::Task(TaskError::NotFound { id: id.to_string() }))?;

        task.delete()?;
        self.storage.update_task(task).await
    }

    /// Start time tracking for a task
    pub async fn start_task(&self, id: i64) -> EddaResult<Task> {
        let mut task = self
            .get_task(id)
            .await?
            .ok_or_else(|| EddaError::Task(TaskError::NotFound { id: id.to_string() }))?;

        task.start()?;
        self.storage.update_task(task).await
    }

    /// Stop time tracking for a task
    pub async fn stop_task(&self, id: i64) -> EddaResult<Task> {
        let mut task = self
            .get_task(id)
            .await?
            .ok_or_else(|| EddaError::Task(TaskError::NotFound { id: id.to_string() }))?;

        // Stop time tracking by clearing start date
        task.start_date = None;
        task.modified_date = Utc::now();

        self.storage.update_task(task).await
    }

    /// Add an annotation to a task
    pub async fn annotate_task(&self, id: i64, description: String) -> EddaResult<Task> {
        if description.trim().is_empty() {
            return Err(EddaError::Task(TaskError::Validation {
                message: "Annotation description cannot be empty".to_string(),
            }));
        }

        let mut task = self
            .get_task(id)
            .await?
            .ok_or_else(|| EddaError::Task(TaskError::NotFound { id: id.to_string() }))?;

        task.add_annotation(description);
        self.storage.update_task(task).await
    }

    /// Add a tag to a task
    pub async fn add_tag(&self, id: i64, tag: String) -> EddaResult<Task> {
        if tag.trim().is_empty() {
            return Err(EddaError::Task(TaskError::Validation {
                message: "Tag cannot be empty".to_string(),
            }));
        }

        let mut task = self
            .get_task(id)
            .await?
            .ok_or_else(|| EddaError::Task(TaskError::NotFound { id: id.to_string() }))?;

        task.add_tag(tag);
        self.storage.update_task(task).await
    }

    /// Remove a tag from a task
    pub async fn remove_tag(&self, id: i64, tag: &str) -> EddaResult<Task> {
        let mut task = self
            .get_task(id)
            .await?
            .ok_or_else(|| EddaError::Task(TaskError::NotFound { id: id.to_string() }))?;

        task.remove_tag(tag);
        self.storage.update_task(task).await
    }

    /// List tasks with filtering
    pub async fn list_tasks(
        &self,
        filter: Option<crate::storage::TaskFilter>,
    ) -> EddaResult<Vec<Task>> {
        self.storage.list_tasks(filter).await
    }

    /// Count tasks with filtering
    pub async fn count_tasks(&self, filter: Option<crate::storage::TaskFilter>) -> EddaResult<u64> {
        self.storage.count_tasks(filter).await
    }

    /// Check if a status transition is valid
    fn is_valid_status_transition(&self, from: &TaskStatus, to: &TaskStatus) -> bool {
        match (from, to) {
            // Any status can transition to deleted
            (_, TaskStatus::Deleted) => true,
            // Deleted tasks cannot transition to other statuses
            (TaskStatus::Deleted, _) => false,
            // Pending can transition to any non-deleted status
            (TaskStatus::Pending, _) => true,
            // Completed can transition back to pending or waiting
            (TaskStatus::Completed, TaskStatus::Pending | TaskStatus::Waiting) => true,
            // Waiting can transition to pending or completed
            (TaskStatus::Waiting, TaskStatus::Pending | TaskStatus::Completed) => true,
            // Other transitions are invalid
            _ => false,
        }
    }

    /// Get child tasks of a parent task
    pub async fn get_child_tasks(&self, parent_id: i64) -> EddaResult<Vec<Task>> {
        let parent_task = self.get_task(parent_id).await?.ok_or_else(|| {
            EddaError::Task(TaskError::NotFound {
                id: parent_id.to_string(),
            })
        })?;

        let filter = crate::storage::TaskFilter::default();
        // Note: This would need to be implemented in the storage layer
        // For now, we'll get all tasks and filter in memory
        let all_tasks = self.storage.list_tasks(None).await?;
        let child_tasks: Vec<Task> = all_tasks
            .into_iter()
            .filter(|task| task.parent_uuid == Some(parent_task.uuid))
            .collect();

        Ok(child_tasks)
    }

    /// Get tasks that depend on a given task
    pub async fn get_dependent_tasks(&self, task_id: i64) -> EddaResult<Vec<Task>> {
        let task = self.get_task(task_id).await?.ok_or_else(|| {
            EddaError::Task(TaskError::NotFound {
                id: task_id.to_string(),
            })
        })?;

        let all_tasks = self.storage.list_tasks(None).await?;
        let dependent_tasks: Vec<Task> = all_tasks
            .into_iter()
            .filter(|t| t.depends.contains(&task.uuid))
            .collect();

        Ok(dependent_tasks)
    }
}

#[cfg(test)]
mod task_engine_tests {
    use super::*;
    use crate::storage::{SqliteTaskStorage, TaskStorage};
    use serial_test::serial;
    use sqlx::sqlite::SqlitePoolOptions;

    async fn create_test_engine() -> TaskEngine {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect("sqlite::memory:")
            .await
            .unwrap();

        crate::storage::database::run_migrations(&pool)
            .await
            .unwrap();

        let storage = SqliteTaskStorage::new(pool);
        TaskEngine::new(Box::new(storage))
    }

    #[tokio::test]
    #[serial]
    async fn test_create_task_with_validation() {
        let engine = create_test_engine().await;

        // Test valid task creation
        let task = engine.create_task("Valid task".to_string()).await.unwrap();
        assert_eq!(task.description, "Valid task");

        // Test invalid task creation
        let result = engine.create_task("".to_string()).await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            crate::core::EddaError::Task(TaskError::Validation { .. })
        ));
    }

    #[tokio::test]
    #[serial]
    async fn test_complete_task() {
        let engine = create_test_engine().await;

        // Create a task
        let task = engine.create_task("Test task".to_string()).await.unwrap();
        let task_id = task.id.unwrap();

        // Complete the task
        let completed_task = engine.complete_task(task_id).await.unwrap();
        assert_eq!(completed_task.status, TaskStatus::Completed);
        assert!(completed_task.end_date.is_some());
    }

    #[tokio::test]
    #[serial]
    async fn test_start_stop_task() {
        let engine = create_test_engine().await;

        // Create a task
        let task = engine.create_task("Test task".to_string()).await.unwrap();
        let task_id = task.id.unwrap();

        // Start the task
        let started_task = engine.start_task(task_id).await.unwrap();
        assert!(started_task.start_date.is_some());

        // Stop the task
        let stopped_task = engine.stop_task(task_id).await.unwrap();
        assert!(stopped_task.start_date.is_none());
    }

    #[tokio::test]
    #[serial]
    async fn test_add_remove_tag() {
        let engine = create_test_engine().await;

        // Create a task
        let task = engine.create_task("Test task".to_string()).await.unwrap();
        let task_id = task.id.unwrap();

        // Add a tag
        let task_with_tag = engine
            .add_tag(task_id, "important".to_string())
            .await
            .unwrap();
        assert!(task_with_tag.tags.contains("important"));

        // Remove the tag
        let task_without_tag = engine.remove_tag(task_id, "important").await.unwrap();
        assert!(!task_without_tag.tags.contains("important"));
    }

    #[tokio::test]
    #[serial]
    async fn test_annotate_task() {
        let engine = create_test_engine().await;

        // Create a task
        let task = engine.create_task("Test task".to_string()).await.unwrap();
        let task_id = task.id.unwrap();

        // Add an annotation
        let annotated_task = engine
            .annotate_task(task_id, "This is a note".to_string())
            .await
            .unwrap();
        assert_eq!(annotated_task.annotations.len(), 1);
        assert_eq!(annotated_task.annotations[0].description, "This is a note");
    }

    #[tokio::test]
    #[serial]
    async fn test_status_transitions() {
        let engine = create_test_engine().await;

        // Create a task
        let task = engine.create_task("Test task".to_string()).await.unwrap();
        let mut task = engine.get_task(task.id.unwrap()).await.unwrap().unwrap();

        // Test valid transitions
        task.status = TaskStatus::Completed;
        let updated_task = engine.update_task(task.clone()).await.unwrap();
        assert_eq!(updated_task.status, TaskStatus::Completed);

        // Test invalid transition (completed -> deleted should be valid)
        task.status = TaskStatus::Deleted;
        let updated_task = engine.update_task(task).await.unwrap();
        assert_eq!(updated_task.status, TaskStatus::Deleted);
    }
}
