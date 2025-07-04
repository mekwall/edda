# Edda - Storage Engine Specification

## Overview

This specification defines the storage engine architecture for Edda, providing efficient, reliable, and scalable data persistence for AI agent task and document management. The storage engine is designed to handle both local and remote data with strong consistency guarantees.

## Architecture Context

This component operates within the Edda architecture as defined in [SPEC_ARCHITECTURE.md](./SPEC_ARCHITECTURE.md):

- **Layer**: Data Storage Layer
- **Component**: SQLite Database, RocksDB Store, File System, Backup Manager, Sync Engine, Migration System
- **Responsibilities**: Provide reliable data persistence, handle data synchronization across devices, manage backup and recovery operations, ensure data integrity and consistency, support schema evolution and migrations, optimize storage performance
- **Dependencies**: Core Engine Layer (for data access patterns), CLI Interface Layer (for configuration), AI Agent Integration Layer (for AI agent data access)

## Architecture Overview

### Storage Layer Design

The storage layer implements the Data Storage Layer responsibilities defined in the master architecture:

- **SQLite Database**: Primary storage for structured data
- **RocksDB Store**: High-performance key-value storage
- **File System**: Document content and configuration storage
- **Backup Manager**: Automated backup and restore functionality
- **Sync Engine**: Remote synchronization capabilities
- **Migration System**: Database schema evolution

## Core Storage Components

### Storage Interface

```rust
/// Main storage interface for Edda
pub trait StorageEngine {
    /// Initialize the storage engine
    fn initialize(&mut self, config: &StorageConfig) -> Result<(), StorageError>;

    /// Create a new task
    fn create_task(&self, task: &Task) -> Result<TaskId, StorageError>;

    /// Update an existing task
    fn update_task(&self, task: &Task) -> Result<(), StorageError>;

    /// Delete a task
    fn delete_task(&self, task_id: &TaskId) -> Result<(), StorageError>;

    /// Get a task by ID
    fn get_task(&self, task_id: &TaskId) -> Result<Option<Task>, StorageError>;

    /// Query tasks with filters
    fn query_tasks(&self, query: &TaskQuery) -> Result<Vec<Task>, StorageError>;

    /// Create a new document
    fn create_document(&self, document: &Document) -> Result<DocumentId, StorageError>;

    /// Update an existing document
    fn update_document(&self, document: &Document) -> Result<(), StorageError>;

    /// Delete a document
    fn delete_document(&self, document_id: &DocumentId) -> Result<(), StorageError>;

    /// Get a document by ID
    fn get_document(&self, document_id: &DocumentId) -> Result<Option<Document>, StorageError>;

    /// Query documents with filters
    fn query_documents(&self, query: &DocumentQuery) -> Result<Vec<Document>, StorageError>;

    /// Begin a transaction
    fn begin_transaction(&self) -> Result<Transaction, StorageError>;

    /// Close the storage engine
    fn close(&self) -> Result<(), StorageError>;
}
```

### Storage Configuration

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    /// Storage backend type
    pub backend: StorageBackend,
    /// Database file path (for SQLite)
    pub database_path: PathBuf,
    /// RocksDB directory (for RocksDB)
    pub rocksdb_path: Option<PathBuf>,
    /// File storage directory
    pub file_storage_path: PathBuf,
    /// Connection pool size
    pub connection_pool_size: usize,
    /// Cache configuration
    pub cache_config: CacheConfig,
    /// Backup configuration
    pub backup_config: BackupConfig,
    /// Encryption settings
    pub encryption_config: Option<EncryptionConfig>,
}
```

## Storage Backends

### SQLite Database

Primary relational storage for structured data.

#### Database Schema

```sql
-- Core tables
CREATE TABLE tasks (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL,
    priority INTEGER NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    completed_at TEXT,
    due_date TEXT,
    parent_id TEXT,
    metadata TEXT,
    agent_context TEXT,
    FOREIGN KEY (parent_id) REFERENCES tasks(id)
);

CREATE TABLE documents (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    content_type TEXT NOT NULL,
    content TEXT,
    doc_type TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    version TEXT NOT NULL,
    metadata TEXT,
    agent_context TEXT
);

-- Relationship tables
CREATE TABLE task_documents (
    task_id TEXT NOT NULL,
    document_id TEXT NOT NULL,
    relationship_type TEXT NOT NULL,
    created_at TEXT NOT NULL,
    PRIMARY KEY (task_id, document_id),
    FOREIGN KEY (task_id) REFERENCES tasks(id),
    FOREIGN KEY (document_id) REFERENCES documents(id)
);

CREATE TABLE tags (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    color TEXT,
    description TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE entity_tags (
    entity_type TEXT NOT NULL,
    entity_id TEXT NOT NULL,
    tag_id TEXT NOT NULL,
    created_at TEXT NOT NULL,
    PRIMARY KEY (entity_type, entity_id, tag_id),
    FOREIGN KEY (tag_id) REFERENCES tags(id)
);

-- Indexes for performance
CREATE INDEX idx_tasks_status_priority ON tasks(status, priority);
CREATE INDEX idx_tasks_created_at ON tasks(created_at);
CREATE INDEX idx_tasks_parent_id ON tasks(parent_id);
CREATE INDEX idx_documents_doc_type ON documents(doc_type);
CREATE INDEX idx_task_documents_task_id ON task_documents(task_id);
CREATE INDEX idx_task_documents_document_id ON task_documents(document_id);
CREATE INDEX idx_entity_tags_entity ON entity_tags(entity_type, entity_id);
```

### RocksDB Key-Value Store

High-performance storage for metadata and indexes.

### File System Storage

Storage for large binary files and documents.

## Caching Layer

### Cache Implementation

```rust
pub struct CacheManager {
    /// In-memory cache for frequently accessed data
    memory_cache: LruCache<String, CacheEntry>,
    /// Cache configuration
    config: CacheConfig,
    /// Cache statistics
    stats: CacheStats,
}

#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// Maximum cache size in bytes
    pub max_size: usize,
    /// Time-to-live for cache entries
    pub ttl: Duration,
    /// Cache eviction policy
    pub eviction_policy: EvictionPolicy,
    /// Enable cache compression
    pub compression: bool,
}

#[derive(Debug, Clone)]
pub enum EvictionPolicy {
    /// Least recently used
    Lru,
    /// Least frequently used
    Lfu,
    /// First in, first out
    Fifo,
    /// Time-based expiration
    Ttl,
}
```

## Transaction Management

### Transaction Implementation

```rust
pub struct Transaction {
    /// Transaction ID
    id: String,
    /// Transaction state
    state: TransactionState,
    /// Operations in transaction
    operations: Vec<TransactionOperation>,
    /// Storage engine reference
    storage: Arc<dyn StorageEngine>,
}

#[derive(Debug, Clone)]
pub enum TransactionState {
    /// Transaction is active
    Active,
    /// Transaction is committed
    Committed,
    /// Transaction is rolled back
    RolledBack,
    /// Transaction is aborted
    Aborted,
}

#[derive(Debug, Clone)]
pub enum TransactionOperation {
    /// Create task operation
    CreateTask(Task),
    /// Update task operation
    UpdateTask(Task),
    /// Delete task operation
    DeleteTask(TaskId),
    /// Create document operation
    CreateDocument(Document),
    /// Update document operation
    UpdateDocument(Document),
    /// Delete document operation
    DeleteDocument(DocumentId),
}
```

## Backup and Recovery

### Backup Strategy

```rust
pub struct BackupManager {
    /// Backup configuration
    config: BackupConfig,
    /// Storage engine reference
    storage: Arc<dyn StorageEngine>,
    /// Backup location
    backup_path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct BackupConfig {
    /// Backup frequency
    pub frequency: BackupFrequency,
    /// Retention policy
    pub retention: RetentionPolicy,
    /// Compression settings
    pub compression: bool,
    /// Encryption settings
    pub encryption: Option<EncryptionConfig>,
}

#[derive(Debug, Clone)]
pub enum BackupFrequency {
    /// No automatic backups
    Never,
    /// Daily backups
    Daily,
    /// Weekly backups
    Weekly,
    /// Monthly backups
    Monthly,
    /// Custom interval
    Custom(Duration),
}
```

## Performance Optimization

### Index Management

```rust
pub struct IndexManager {
    /// Index configurations
    indexes: HashMap<String, IndexConfig>,
    /// Storage engine reference
    storage: Arc<dyn StorageEngine>,
}

#[derive(Debug, Clone)]
pub struct IndexConfig {
    /// Index name
    pub name: String,
    /// Indexed columns
    pub columns: Vec<String>,
    /// Index type
    pub index_type: IndexType,
    /// Unique constraint
    pub unique: bool,
    /// Partial index condition
    pub where_clause: Option<String>,
}

#[derive(Debug, Clone)]
pub enum IndexType {
    /// B-tree index
    BTree,
    /// Hash index
    Hash,
    /// Full-text search index
    FullText,
    /// Spatial index
    Spatial,
}
```

## Security and Encryption

### Encryption Implementation

```rust
pub struct EncryptionManager {
    /// Encryption configuration
    config: EncryptionConfig,
    /// Master key
    master_key: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct EncryptionConfig {
    /// Encryption algorithm
    pub algorithm: EncryptionAlgorithm,
    /// Key derivation function
    pub kdf: KeyDerivationFunction,
    /// Key rotation policy
    pub key_rotation: KeyRotationPolicy,
}

#[derive(Debug, Clone)]
pub enum EncryptionAlgorithm {
    /// AES-256-GCM
    Aes256Gcm,
    /// ChaCha20-Poly1305
    ChaCha20Poly1305,
    /// XChaCha20-Poly1305
    XChaCha20Poly1305,
}
```

## Monitoring and Metrics

### Storage Metrics

```rust
#[derive(Debug, Clone)]
pub struct StorageMetrics {
    /// Database size in bytes
    pub database_size: u64,
    /// Number of tasks
    pub task_count: u64,
    /// Number of documents
    pub document_count: u64,
    /// Cache hit rate
    pub cache_hit_rate: f64,
    /// Average query time
    pub avg_query_time: Duration,
    /// Transaction count
    pub transaction_count: u64,
    /// Backup count
    pub backup_count: u64,
    /// Last backup timestamp
    pub last_backup: Option<DateTime<Utc>>,
}
```

This specification provides a comprehensive storage engine architecture that ensures data integrity, performance, and scalability for Edda's AI agent task and document management system.
