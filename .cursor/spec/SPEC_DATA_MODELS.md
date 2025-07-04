# Edda - Data Models Specification

## Overview

This specification defines the core data structures and storage models for Edda, the AI agent task and document management CLI tool. The data models are designed to be efficient, extensible, and optimized for AI agent workflows.

## Core Data Entities

### Task Entity

The primary unit of work tracking in Edda.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    /// Unique identifier for the task (Taskwarrior: UUID)
    pub id: TaskId,
    /// Sequential identifier for display (Taskwarrior: ID)
    pub display_id: u64,
    /// Primary task description (Taskwarrior: description)
    pub description: String,
    /// Current status (Taskwarrior: status)
    pub status: TaskStatus,
    /// Priority level (Taskwarrior: priority)
    pub priority: Priority,
    /// Due date (Taskwarrior: due)
    pub due_date: Option<DateTime<Utc>>,
    /// Scheduled date (Taskwarrior: scheduled)
    pub scheduled_date: Option<DateTime<Utc>>,
    /// Start date (Taskwarrior: start)
    pub start_date: Option<DateTime<Utc>>,
    /// End date (Taskwarrior: end)
    pub end_date: Option<DateTime<Utc>>,
    /// Entry date (Taskwarrior: entry)
    pub entry_date: DateTime<Utc>,
    /// Modified date (Taskwarrior: modified)
    pub modified_date: DateTime<Utc>,
    /// Associated project (Taskwarrior: project)
    pub project: Option<String>,
    /// Associated tags (Taskwarrior: tags)
    pub tags: Vec<String>,
    /// Task dependencies (Taskwarrior: depends)
    pub depends: Vec<TaskId>,
    /// Recurrence pattern (Taskwarrior: recur)
    pub recur: Option<String>,
    /// Wait date (Taskwarrior: wait)
    pub wait_date: Option<DateTime<Utc>>,
    /// Task annotations/notes (Taskwarrior: annotations)
    pub annotations: Vec<Annotation>,
    /// Custom metadata
    pub metadata: HashMap<String, Value>,
    /// AI agent context
    pub agent_context: Option<AgentContext>,
}
```

#### Task Status Enum

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskStatus {
    /// Task is pending and not yet started (Taskwarrior: pending)
    Pending,
    /// Task is waiting for a specific date (Taskwarrior: waiting)
    Waiting,
    /// Task is completed successfully (Taskwarrior: completed)
    Completed,
    /// Task is deleted (Taskwarrior: deleted)
    Deleted,
}
```

#### Priority Enum

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum Priority {
    /// Low priority (Taskwarrior: L)
    Low = 1,
    /// Medium priority (Taskwarrior: M)
    Medium = 2,
    /// High priority (Taskwarrior: H)
    High = 3,
    /// Numeric priority 0-9 (Taskwarrior: 0-9)
    Numeric(u8),
}
```

### Document Entity

Stores context, notes, and reference materials for tasks.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    /// Unique identifier
    pub id: DocumentId,
    /// Human-readable title
    pub title: String,
    /// Document content
    pub content: DocumentContent,
    /// Document type
    pub doc_type: DocumentType,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last modification timestamp
    pub updated_at: DateTime<Utc>,
    /// Associated tasks
    pub task_ids: Vec<TaskId>,
    /// Document tags
    pub tags: Vec<String>,
    /// Version information
    pub version: Version,
    /// Custom metadata
    pub metadata: HashMap<String, Value>,
    /// AI agent context
    pub agent_context: Option<AgentContext>,
}
```

#### Document Content

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DocumentContent {
    /// Plain text content
    Text(String),
    /// Markdown content
    Markdown(String),
    /// JSON structured data
    Json(Value),
    /// YAML structured data
    Yaml(String),
    /// Binary data with metadata
    Binary {
        data: Vec<u8>,
        mime_type: String,
        filename: Option<String>,
    },
    /// Reference to external file
    FileReference {
        path: PathBuf,
        checksum: Option<String>,
    },
}
```

#### Document Type

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DocumentType {
    /// General note or documentation
    Note,
    /// Reference material
    Reference,
    /// Configuration file
    Config,
    /// Template or boilerplate
    Template,
    /// Log or audit trail
    Log,
    /// Code snippet or script
    Code,
    /// Image or media
    Media,
    /// Custom type
    Custom(String),
}
```

### Agent Context

Tracks AI agent state and context information.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentContext {
    /// Agent identifier
    pub agent_id: String,
    /// Agent type or model
    pub agent_type: String,
    /// Current conversation context
    pub conversation_id: Option<String>,
    /// Session information
    pub session_data: HashMap<String, Value>,
    /// Workflow state
    pub workflow_state: Option<WorkflowState>,
    /// Last interaction timestamp
    pub last_interaction: DateTime<Utc>,
    /// Context metadata
    pub metadata: HashMap<String, Value>,
}
```

### Annotation

Task annotations and notes (Taskwarrior: annotations).

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Annotation {
    /// Annotation entry date
    pub entry: DateTime<Utc>,
    /// Annotation description
    pub description: String,
}
```

### Workflow State

Tracks complex workflow execution state.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowState {
    /// Workflow identifier
    pub workflow_id: String,
    /// Current step
    pub current_step: String,
    /// Step history
    pub step_history: Vec<WorkflowStep>,
    /// Workflow variables
    pub variables: HashMap<String, Value>,
    /// Workflow status
    pub status: WorkflowStatus,
    /// Error information (if failed)
    pub error: Option<WorkflowError>,
}
```

## Identifier Types

### Strongly Typed IDs

```rust
/// Task identifier
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TaskId(pub String);

/// Document identifier
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DocumentId(pub String);

/// Agent identifier
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AgentId(pub String);

/// Workflow identifier
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WorkflowId(pub String);
```

## Query Models

### Query Structure

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Query {
    /// Query type
    pub query_type: QueryType,
    /// Query filters
    pub filters: Vec<Filter>,
    /// Sorting criteria
    pub sort: Vec<SortCriteria>,
    /// Pagination
    pub pagination: Option<Pagination>,
    /// Output format
    pub output_format: OutputFormat,
}
```

#### Query Types

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QueryType {
    /// Find tasks matching criteria
    FindTasks,
    /// Find documents matching criteria
    FindDocuments,
    /// Get task details
    GetTask(TaskId),
    /// Get document details
    GetDocument(DocumentId),
    /// Get task relationships
    GetTaskRelationships(TaskId),
    /// Get document relationships
    GetDocumentRelationships(DocumentId),
    /// Search across all entities
    Search(String),
    /// Get statistics and metrics
    GetStats,
}
```

#### Filter Types

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Filter {
    /// Status filter
    Status(TaskStatus),
    /// Priority filter
    Priority(Priority),
    /// Date range filter
    DateRange {
        field: String,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    },
    /// Tag filter
    HasTag(String),
    /// Text search filter
    TextSearch(String),
    /// Custom metadata filter
    Metadata {
        key: String,
        value: Value,
        operator: FilterOperator,
    },
}
```

## Storage Models

### Database Schema

#### Tasks Table

```sql
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
```

#### Documents Table

```sql
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
```

#### Task-Document Relationships

```sql
CREATE TABLE task_documents (
    task_id TEXT NOT NULL,
    document_id TEXT NOT NULL,
    relationship_type TEXT NOT NULL,
    created_at TEXT NOT NULL,
    PRIMARY KEY (task_id, document_id),
    FOREIGN KEY (task_id) REFERENCES tasks(id),
    FOREIGN KEY (document_id) REFERENCES documents(id)
);
```

#### Tags Table

```sql
CREATE TABLE tags (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    color TEXT,
    description TEXT,
    created_at TEXT NOT NULL
);
```

#### Entity Tags

```sql
CREATE TABLE entity_tags (
    entity_type TEXT NOT NULL,
    entity_id TEXT NOT NULL,
    tag_id TEXT NOT NULL,
    created_at TEXT NOT NULL,
    PRIMARY KEY (entity_type, entity_id, tag_id),
    FOREIGN KEY (tag_id) REFERENCES tags(id)
);
```

### Index Strategy

#### Primary Indexes

- `tasks(id)` - Primary key index
- `documents(id)` - Primary key index
- `tags(name)` - Unique tag names
- `entity_tags(entity_type, entity_id)` - Fast tag lookups

#### Secondary Indexes

- `tasks(status, priority)` - Status and priority queries
- `tasks(created_at)` - Time-based queries
- `tasks(parent_id)` - Hierarchical queries
- `documents(doc_type, created_at)` - Document type queries
- `task_documents(task_id)` - Task document relationships
- `task_documents(document_id)` - Document task relationships

## Data Validation

### Validation Rules

#### Task Validation

- `id` must be non-empty and unique
- `title` must be non-empty and <= 255 characters
- `description` must be <= 10,000 characters
- `status` must be a valid TaskStatus enum value
- `priority` must be a valid Priority enum value
- `created_at` must be <= `updated_at`
- `completed_at` must be >= `created_at` if present
- `due_date` must be in the future if present

#### Document Validation

- `id` must be non-empty and unique
- `title` must be non-empty and <= 255 characters
- `content` must be valid for the specified content type
- `doc_type` must be a valid DocumentType enum value
- `version` must follow semantic versioning format
- `created_at` must be <= `updated_at`

### Constraint Enforcement

```rust
/// Validate task data
pub fn validate_task(task: &Task) -> Result<(), ValidationError> {
    // ID validation
    if task.id.0.is_empty() {
        return Err(ValidationError::InvalidId);
    }

    // Title validation
    if task.title.is_empty() || task.title.len() > 255 {
        return Err(ValidationError::InvalidTitle);
    }

    // Date validation
    if task.created_at > task.updated_at {
        return Err(ValidationError::InvalidDates);
    }

    if let Some(completed_at) = task.completed_at {
        if completed_at < task.created_at {
            return Err(ValidationError::InvalidCompletionDate);
        }
    }

    Ok(())
}
```

## Serialization Formats

### JSON Schema

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "definitions": {
    "Task": {
      "type": "object",
      "properties": {
        "id": { "type": "string" },
        "title": { "type": "string", "maxLength": 255 },
        "description": { "type": "string", "maxLength": 10000 },
        "status": {
          "type": "string",
          "enum": [
            "pending",
            "in_progress",
            "blocked",
            "completed",
            "cancelled",
            "paused"
          ]
        },
        "priority": { "type": "integer", "minimum": 1, "maximum": 5 },
        "created_at": { "type": "string", "format": "date-time" },
        "updated_at": { "type": "string", "format": "date-time" },
        "completed_at": { "type": "string", "format": "date-time" },
        "due_date": { "type": "string", "format": "date-time" },
        "tags": { "type": "array", "items": { "type": "string" } },
        "parent_id": { "type": "string" },
        "children": { "type": "array", "items": { "type": "string" } },
        "documents": { "type": "array", "items": { "type": "string" } },
        "metadata": { "type": "object" },
        "agent_context": { "$ref": "#/definitions/AgentContext" }
      },
      "required": [
        "id",
        "title",
        "status",
        "priority",
        "created_at",
        "updated_at"
      ]
    }
  }
}
```

## Migration Strategy

### Version Management

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaVersion {
    /// Current schema version
    pub version: String,
    /// Migration history
    pub migrations: Vec<Migration>,
    /// Last migration timestamp
    pub last_migration: DateTime<Utc>,
}
```

### Migration Types

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Migration {
    /// Add new table
    CreateTable { name: String, schema: String },
    /// Add new column
    AddColumn { table: String, column: String, definition: String },
    /// Modify existing column
    ModifyColumn { table: String, column: String, definition: String },
    /// Add index
    CreateIndex { name: String, table: String, columns: Vec<String> },
    /// Data transformation
    TransformData { description: String, sql: String },
}
```

## Performance Considerations

### Indexing Strategy

- **Primary indexes** on all ID fields for O(1) lookups
- **Composite indexes** for common query patterns
- **Partial indexes** for filtered queries (e.g., active tasks only)
- **Text indexes** for full-text search capabilities

### Caching Strategy

- **In-memory cache** for frequently accessed tasks and documents
- **Query result cache** for complex queries
- **Metadata cache** for tags and relationships
- **LRU eviction** for memory management

### Storage Optimization

- **Compression** for large text content
- **Binary storage** for media files
- **Deduplication** for identical content
- **Archival strategy** for old/inactive data

## Security Considerations

### Data Protection

- **Encryption at rest** for sensitive data
- **Access control** based on agent context
- **Audit logging** for data modifications
- **Data sanitization** for user inputs

### Privacy Compliance

- **Data retention policies** for automatic cleanup
- **Anonymization** for exported data
- **Consent tracking** for data processing
- **Right to deletion** implementation

## Extensibility

### Plugin Architecture

```rust
/// Plugin trait for extending data models
pub trait DataModelPlugin {
    /// Plugin name
    fn name(&self) -> &str;

    /// Plugin version
    fn version(&self) -> &str;

    /// Initialize plugin
    fn initialize(&self, context: &PluginContext) -> Result<(), PluginError>;

    /// Register custom data types
    fn register_types(&self, registry: &mut TypeRegistry) -> Result<(), PluginError>;

    /// Handle custom queries
    fn handle_query(&self, query: &Query) -> Result<Option<QueryResult>, PluginError>;
}
```

### Custom Data Types

```rust
/// Custom data type registration
pub struct TypeRegistry {
    /// Registered custom types
    types: HashMap<String, Box<dyn CustomType>>,
    /// Type serializers
    serializers: HashMap<String, Box<dyn Serializer>>,
    /// Type validators
    validators: HashMap<String, Box<dyn Validator>>,
}
```

This specification provides a comprehensive foundation for Edda's data models, ensuring efficient storage, querying, and extensibility for AI agent workflows.
