use crate::core::error::SyncError;
use crate::core::task::Task;
use crate::core::{EddaError, EddaResult};
use crate::storage::TaskStorage;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Trait for sync providers (GitHub, GitLab, etc.)
#[async_trait::async_trait]
pub trait SyncProvider: Send + Sync {
    /// Get the name of the provider
    fn name(&self) -> &str;

    /// Pull tasks from the remote provider
    async fn pull_tasks(&self) -> EddaResult<Vec<Task>>;

    /// Push tasks to the remote provider
    async fn push_tasks(&self, tasks: &[Task]) -> EddaResult<()>;

    /// Get sync status
    async fn get_status(&self) -> EddaResult<SyncStatus>;

    /// Test connection to the provider
    async fn test_connection(&self) -> EddaResult<()>;
}

/// Represents a sync operation that can be queued for later execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SyncOperation {
    CreateTask {
        task: Task,
        timestamp: DateTime<Utc>,
    },
    UpdateTask {
        task: Task,
        timestamp: DateTime<Utc>,
    },
    DeleteTask {
        task_id: i64,
        timestamp: DateTime<Utc>,
    },
    CreateDocument {
        document: Document,
        timestamp: DateTime<Utc>,
    },
    UpdateDocument {
        document: Document,
        timestamp: DateTime<Utc>,
    },
    DeleteDocument {
        document_id: i64,
        timestamp: DateTime<Utc>,
    },
}

/// Represents a document for sync operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub id: Option<i64>,
    pub uuid: Uuid,
    pub title: String,
    pub content: Option<String>,
    pub content_type: Option<String>,
    pub file_path: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Conflict resolution strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConflictResolution {
    LocalWins,
    RemoteWins,
    Manual,
    Merge,
}

/// Sync status for tracking sync state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SyncStatus {
    Pending,
    InProgress,
    Completed,
    Failed { error: String },
}

/// Offline operation queue for pending sync operations
#[derive(Debug)]
pub struct OfflineQueue {
    operations: Arc<RwLock<Vec<SyncOperation>>>,
    max_operations: usize,
}

impl OfflineQueue {
    pub fn new(max_operations: usize) -> Self {
        Self {
            operations: Arc::new(RwLock::new(Vec::new())),
            max_operations,
        }
    }

    /// Add an operation to the offline queue
    pub async fn enqueue(&self, operation: SyncOperation) -> EddaResult<()> {
        let mut ops = self.operations.write().await;

        if ops.len() >= self.max_operations {
            // Remove oldest operation to make room
            ops.remove(0);
        }

        ops.push(operation);
        Ok(())
    }

    /// Get all pending operations
    pub async fn get_pending_operations(&self) -> Vec<SyncOperation> {
        let ops = self.operations.read().await;
        ops.clone()
    }

    /// Remove operations from the queue (after successful sync)
    pub async fn remove_operations(&self, indices: &[usize]) {
        let mut ops = self.operations.write().await;
        let mut sorted_indices: Vec<usize> = indices.to_vec();
        sorted_indices.sort_by(|a, b| b.cmp(a)); // Sort in descending order

        for &index in &sorted_indices {
            if index < ops.len() {
                ops.remove(index);
            }
        }
    }

    /// Clear all operations from the queue
    pub async fn clear(&self) {
        let mut ops = self.operations.write().await;
        ops.clear();
    }

    /// Get the number of pending operations
    pub async fn len(&self) -> usize {
        let ops = self.operations.read().await;
        ops.len()
    }

    /// Check if the queue is empty
    pub async fn is_empty(&self) -> bool {
        let ops = self.operations.read().await;
        ops.is_empty()
    }
}

/// Local cache for offline-first operations
#[derive(Debug)]
pub struct LocalCache {
    tasks: Arc<RwLock<HashMap<i64, CachedTask>>>,
    documents: Arc<RwLock<HashMap<i64, CachedDocument>>>,
    last_sync: Arc<RwLock<Option<DateTime<Utc>>>>,
}

/// Cached task with sync metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedTask {
    pub task: Task,
    pub last_modified: DateTime<Utc>,
    pub sync_status: SyncStatus,
    pub version: u64,
}

/// Cached document with sync metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedDocument {
    pub document: Document,
    pub last_modified: DateTime<Utc>,
    pub sync_status: SyncStatus,
    pub version: u64,
}

impl LocalCache {
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(RwLock::new(HashMap::new())),
            documents: Arc::new(RwLock::new(HashMap::new())),
            last_sync: Arc::new(RwLock::new(None)),
        }
    }

    /// Cache a task locally
    pub async fn cache_task(&self, task: Task, version: u64) {
        let cached_task = CachedTask {
            task: task.clone(),
            last_modified: Utc::now(),
            sync_status: SyncStatus::Pending,
            version,
        };

        let mut tasks = self.tasks.write().await;
        tasks.insert(task.id.unwrap_or(0), cached_task);
    }

    /// Get a cached task
    pub async fn get_cached_task(&self, task_id: i64) -> Option<CachedTask> {
        let tasks = self.tasks.read().await;
        tasks.get(&task_id).cloned()
    }

    /// Update sync status for a cached task
    pub async fn update_task_sync_status(&self, task_id: i64, status: SyncStatus) {
        let mut tasks = self.tasks.write().await;
        if let Some(cached_task) = tasks.get_mut(&task_id) {
            cached_task.sync_status = status;
            cached_task.last_modified = Utc::now();
        }
    }

    /// Cache a document locally
    pub async fn cache_document(&self, document: Document, version: u64) {
        let cached_document = CachedDocument {
            document: document.clone(),
            last_modified: Utc::now(),
            sync_status: SyncStatus::Pending,
            version,
        };

        let mut documents = self.documents.write().await;
        documents.insert(document.id.unwrap_or(0), cached_document);
    }

    /// Get a cached document
    pub async fn get_cached_document(&self, document_id: i64) -> Option<CachedDocument> {
        let documents = self.documents.read().await;
        documents.get(&document_id).cloned()
    }

    /// Update sync status for a cached document
    pub async fn update_document_sync_status(&self, document_id: i64, status: SyncStatus) {
        let mut documents = self.documents.write().await;
        if let Some(cached_document) = documents.get_mut(&document_id) {
            cached_document.sync_status = status;
            cached_document.last_modified = Utc::now();
        }
    }

    /// Set the last sync timestamp
    pub async fn set_last_sync(&self, timestamp: DateTime<Utc>) {
        let mut last_sync = self.last_sync.write().await;
        *last_sync = Some(timestamp);
    }

    /// Get the last sync timestamp
    pub async fn get_last_sync(&self) -> Option<DateTime<Utc>> {
        let last_sync = self.last_sync.read().await;
        *last_sync
    }

    /// Get all cached tasks
    pub async fn get_all_cached_tasks(&self) -> Vec<CachedTask> {
        let tasks = self.tasks.read().await;
        tasks.values().cloned().collect()
    }

    /// Get all cached documents
    pub async fn get_all_cached_documents(&self) -> Vec<CachedDocument> {
        let documents = self.documents.read().await;
        documents.values().cloned().collect()
    }

    /// Clear all cached data
    pub async fn clear(&self) {
        let mut tasks = self.tasks.write().await;
        let mut documents = self.documents.write().await;
        let mut last_sync = self.last_sync.write().await;

        tasks.clear();
        documents.clear();
        *last_sync = None;
    }
}

/// Conflict resolver for handling sync conflicts
#[derive(Debug)]
pub struct ConflictResolver {
    default_strategy: ConflictResolution,
}

impl ConflictResolver {
    pub fn new(default_strategy: ConflictResolution) -> Self {
        Self { default_strategy }
    }

    /// Resolve a conflict between local and remote data
    pub fn resolve_task_conflict(
        &self,
        local_task: &Task,
        remote_task: &Task,
        strategy: Option<ConflictResolution>,
    ) -> Task {
        let strategy = strategy.unwrap_or(self.default_strategy.clone());

        match strategy {
            ConflictResolution::LocalWins => local_task.clone(),
            ConflictResolution::RemoteWins => remote_task.clone(),
            ConflictResolution::Manual => {
                // For manual resolution, return the most recently modified task
                if local_task.modified_date > remote_task.modified_date {
                    local_task.clone()
                } else {
                    remote_task.clone()
                }
            }
            ConflictResolution::Merge => self.merge_tasks(local_task, remote_task),
        }
    }

    /// Merge two tasks, combining their properties
    fn merge_tasks(&self, local_task: &Task, remote_task: &Task) -> Task {
        let mut merged_task = local_task.clone();

        // Use the most recent modification date
        if remote_task.modified_date > local_task.modified_date {
            merged_task.modified_date = remote_task.modified_date.clone();
        }

        // Merge tags (union of both sets)
        let mut local_tags = local_task.tags.clone();
        let remote_tags = remote_task.tags.clone();
        for tag in remote_tags {
            if !local_tags.contains(&tag) {
                local_tags.insert(tag);
            }
        }
        merged_task.tags = local_tags;

        // Merge annotations (combine both lists)
        let mut local_annotations = local_task.annotations.clone();
        let remote_annotations = remote_task.annotations.clone();
        local_annotations.extend(remote_annotations);
        merged_task.annotations = local_annotations;

        // Use the higher priority if different
        if let (Some(local_priority), Some(remote_priority)) =
            (&local_task.priority, &remote_task.priority)
        {
            if remote_priority > local_priority {
                merged_task.priority = remote_task.priority.clone();
            }
        }

        merged_task
    }

    /// Resolve a document conflict
    pub fn resolve_document_conflict(
        &self,
        local_document: &Document,
        remote_document: &Document,
        strategy: Option<ConflictResolution>,
    ) -> Document {
        let strategy = strategy.unwrap_or(self.default_strategy.clone());

        match strategy {
            ConflictResolution::LocalWins => local_document.clone(),
            ConflictResolution::RemoteWins => remote_document.clone(),
            ConflictResolution::Manual => {
                if local_document.updated_at > remote_document.updated_at {
                    local_document.clone()
                } else {
                    remote_document.clone()
                }
            }
            ConflictResolution::Merge => self.merge_documents(local_document, remote_document),
        }
    }

    /// Merge two documents
    fn merge_documents(&self, local_document: &Document, remote_document: &Document) -> Document {
        let mut merged_document = local_document.clone();

        // Use the most recent update time
        if remote_document.updated_at > local_document.updated_at {
            merged_document.updated_at = remote_document.updated_at;
        }

        // Merge metadata if both have it
        if let (Some(local_metadata), Some(remote_metadata)) =
            (&local_document.metadata, &remote_document.metadata)
        {
            if let (Some(local_obj), Some(remote_obj)) =
                (local_metadata.as_object(), remote_metadata.as_object())
            {
                let mut merged_obj = local_obj.clone();
                merged_obj.extend(remote_obj.clone());
                merged_document.metadata = Some(serde_json::Value::Object(merged_obj));
            }
        }

        merged_document
    }
}

/// Sync manager for coordinating offline and online operations
pub struct SyncManager {
    queue: OfflineQueue,
    cache: LocalCache,
    resolver: ConflictResolver,
    storage: Arc<dyn TaskStorage + Send + Sync>,
}

impl SyncManager {
    pub fn new(
        storage: Arc<dyn TaskStorage + Send + Sync>,
        max_queue_size: usize,
        default_conflict_strategy: ConflictResolution,
    ) -> Self {
        Self {
            queue: OfflineQueue::new(max_queue_size),
            cache: LocalCache::new(),
            resolver: ConflictResolver::new(default_conflict_strategy),
            storage,
        }
    }

    /// Create a task with offline support
    pub async fn create_task(&self, task: Task) -> EddaResult<Task> {
        // Store locally first
        let created_task = self.storage.create_task(task).await?;

        // Cache the task
        self.cache.cache_task(created_task.clone(), 1).await;

        // Queue for sync
        let operation = SyncOperation::CreateTask {
            task: created_task.clone(),
            timestamp: Utc::now(),
        };
        self.queue.enqueue(operation).await?;

        Ok(created_task)
    }

    /// Update a task with offline support
    pub async fn update_task(&self, task: Task) -> EddaResult<Task> {
        // Update locally first
        let updated_task = self.storage.update_task(task).await?;

        // Update cache
        self.cache.cache_task(updated_task.clone(), 1).await;

        // Queue for sync
        let operation = SyncOperation::UpdateTask {
            task: updated_task.clone(),
            timestamp: Utc::now(),
        };
        self.queue.enqueue(operation).await?;

        Ok(updated_task)
    }

    /// Delete a task with offline support
    pub async fn delete_task(&self, task_id: i64) -> EddaResult<bool> {
        // Delete locally first
        let deleted = self.storage.delete_task(task_id).await?;

        if deleted {
            // Queue for sync
            let operation = SyncOperation::DeleteTask {
                task_id,
                timestamp: Utc::now(),
            };
            self.queue.enqueue(operation).await?;
        }

        Ok(deleted)
    }

    /// Get tasks with offline support (from cache if available)
    pub async fn get_tasks(&self) -> EddaResult<Vec<Task>> {
        // Try to get from cache first
        let cached_tasks = self.cache.get_all_cached_tasks().await;

        if !cached_tasks.is_empty() {
            // Return cached tasks
            Ok(cached_tasks.into_iter().map(|ct| ct.task).collect())
        } else {
            // Fall back to storage
            let tasks = self.storage.list_tasks(None).await?;

            // Cache the tasks
            for task in &tasks {
                self.cache.cache_task(task.clone(), 1).await;
            }

            Ok(tasks)
        }
    }

    /// Perform a sync operation (when online)
    pub async fn sync(&self) -> EddaResult<()> {
        // Get pending operations
        let operations = self.queue.get_pending_operations().await;

        if operations.is_empty() {
            return Ok(());
        }

        // Mark sync as in progress
        self.cache.set_last_sync(Utc::now()).await;

        // Process operations (this would typically involve a sync provider)
        // For now, we'll just mark them as completed
        for (_index, operation) in operations.iter().enumerate() {
            match operation {
                SyncOperation::CreateTask { task, .. } => {
                    self.cache
                        .update_task_sync_status(task.id.unwrap_or(0), SyncStatus::Completed)
                        .await;
                }
                SyncOperation::UpdateTask { task, .. } => {
                    self.cache
                        .update_task_sync_status(task.id.unwrap_or(0), SyncStatus::Completed)
                        .await;
                }
                SyncOperation::DeleteTask { task_id, .. } => {
                    self.cache
                        .update_task_sync_status(*task_id, SyncStatus::Completed)
                        .await;
                }
                _ => {
                    // Handle document operations similarly
                }
            }
        }

        // Remove completed operations from queue
        let indices: Vec<usize> = (0..operations.len()).collect();
        self.queue.remove_operations(&indices).await;

        Ok(())
    }

    /// Check if there are pending sync operations
    pub async fn has_pending_operations(&self) -> bool {
        !self.queue.is_empty().await
    }

    /// Get the number of pending operations
    pub async fn pending_operation_count(&self) -> usize {
        self.queue.len().await
    }

    /// Get the last sync timestamp
    pub async fn get_last_sync_time(&self) -> Option<DateTime<Utc>> {
        self.cache.get_last_sync().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::task::Task;
    use crate::storage::SqliteTaskStorage;
    use crate::storage::database::get_pool;
    use std::path::PathBuf;

    #[tokio::test]
    async fn test_offline_queue_operations() {
        let queue = OfflineQueue::new(10);

        // Test enqueue
        let task = Task::new("Test task".to_string());
        let operation = SyncOperation::CreateTask {
            task: task.clone(),
            timestamp: Utc::now(),
        };

        assert!(queue.enqueue(operation).await.is_ok());
        assert_eq!(queue.len().await, 1);
        assert!(!queue.is_empty().await);

        // Test get pending operations
        let operations = queue.get_pending_operations().await;
        assert_eq!(operations.len(), 1);

        // Test clear
        queue.clear().await;
        assert!(queue.is_empty().await);
    }

    #[tokio::test]
    async fn test_local_cache_operations() {
        let cache = LocalCache::new();

        // Test cache task
        let task = Task::new("Test task".to_string());
        cache.cache_task(task.clone(), 1).await;

        // Test get cached task
        let cached_task = cache.get_cached_task(task.id.unwrap_or(0)).await;
        assert!(cached_task.is_some());

        // Test update sync status
        cache
            .update_task_sync_status(task.id.unwrap_or(0), SyncStatus::Completed)
            .await;
        let cached_task = cache.get_cached_task(task.id.unwrap_or(0)).await;
        assert!(matches!(
            cached_task.unwrap().sync_status,
            SyncStatus::Completed
        ));
    }

    #[tokio::test]
    async fn test_conflict_resolution() {
        let resolver = ConflictResolver::new(ConflictResolution::LocalWins);

        let local_task = Task::new("Local task".to_string());
        let mut remote_task = Task::new("Remote task".to_string());
        remote_task.description = "Remote description".to_string();

        // Test local wins strategy
        let resolved = resolver.resolve_task_conflict(&local_task, &remote_task, None);
        assert_eq!(resolved.description, local_task.description);

        // Test remote wins strategy
        let resolved = resolver.resolve_task_conflict(
            &local_task,
            &remote_task,
            Some(ConflictResolution::RemoteWins),
        );
        assert_eq!(resolved.description, remote_task.description);
    }

    #[tokio::test]
    async fn test_sync_manager_operations() {
        // Create a temporary database for testing
        let db_path = PathBuf::from(":memory:");
        let pool = get_pool(db_path).await.unwrap();

        // Run migrations to create tables
        crate::storage::database::run_migrations(&pool)
            .await
            .unwrap();

        let storage = Arc::new(SqliteTaskStorage::new(pool));
        let manager = SyncManager::new(storage, 100, ConflictResolution::LocalWins);

        // Test create task
        let task = Task::new("Test task".to_string());
        let created_task = manager.create_task(task).await.unwrap();
        assert!(created_task.id.is_some());

        // Test get tasks
        let tasks = manager.get_tasks().await.unwrap();
        assert!(!tasks.is_empty());

        // Test pending operations
        assert!(manager.has_pending_operations().await);
        assert_eq!(manager.pending_operation_count().await, 1);

        // Test sync
        assert!(manager.sync().await.is_ok());
        assert!(!manager.has_pending_operations().await);
    }
}
