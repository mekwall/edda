# Edda - Architecture Specification

## Overview

Edda follows a **modular CLI architecture** designed for performance, extensibility, and AI agent integration. The architecture emphasizes separation of concerns, efficient data management, and automation-friendly interfaces.

## Master Architecture Reference

> **Note**: This is the definitive architecture reference for Edda. All other specifications should reference this architecture instead of defining their own diagrams. This ensures consistency and eliminates redundancy across all specification documents.

### Architecture Overview

Edda follows a **modular CLI architecture** with clear separation of concerns across four primary layers:

```
┌───────────────────────────────────────────────────────────────────────────────┐
│                              Edda Architecture                                │
├───────────────────────────────────────────────────────────────────────────────┤
│                             CLI Interface Layer                               │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌─────────────────────────┐  │
│  │   Command   │ │   Argument  │ │   Output    │ │   Help &                │  │
│  │   Parser    │ │   Validator │ │   Formatter │ │   Completion            │  │
│  └─────────────┘ └─────────────┘ └─────────────┘ └─────────────────────────┘  │
├───────────────────────────────────────────────────────────────────────────────┤
│                             Core Engine Layer                                 │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌─────────────────────────┐  │
│  │   Task      │ │   Document  │ │   Query     │ │   Cache &               │  │
│  │   Manager   │ │   Manager   │ │   Engine    │ │   Event System          │  │
│  └─────────────┘ └─────────────┘ └─────────────┘ └─────────────────────────┘  │
├───────────────────────────────────────────────────────────────────────────────┤
│                            Data Storage Layer                                 │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌─────────────────────────┐  │
│  │   SQLite    │ │   RocksDB   │ │   File      │ │   Backup &              │  │
│  │   Database  │ │   Store     │ │   System    │ │   Sync Engine           │  │
│  └─────────────┘ └─────────────┘ └─────────────┘ └─────────────────────────┘  │
├───────────────────────────────────────────────────────────────────────────────┤
│                           AI Agent Integration                                │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌─────────────────────────┐  │
│  │   Agent     │ │   Context   │ │   Workflow  │ │   Communication         │  │
│  │   Interface │ │   Manager   │ │   Manager   │ │   Manager               │  │
│  └─────────────┘ └─────────────┘ └─────────────┘ └─────────────────────────┘  │
└───────────────────────────────────────────────────────────────────────────────┘
```

### Layer Responsibilities

#### 1. CLI Interface Layer

The top layer handles user interaction and command processing:

**Components:**

- **Command Parser**: Uses clap for argument parsing and validation
- **Argument Validator**: Ensures input data meets requirements
- **Output Formatter**: Handles text, JSON, and YAML output formats
- **Help System**: Generates comprehensive help documentation
- **Completion Generator**: Creates shell completion scripts
- **Error Handler**: Provides clear, actionable error messages

**Responsibilities:**

- Parse and validate command-line arguments
- Format output for different display modes
- Provide user-friendly error messages
- Generate help documentation and shell completions
- Handle cross-platform CLI differences

#### 2. Core Engine Layer

The middle layer contains the business logic and data processing:

**Components:**

- **Task Manager**: Handles task CRUD operations and lifecycle
- **Document Manager**: Manages document storage and metadata
- **State Manager**: Handles key-value state storage
- **Query Engine**: Processes SQL-like queries across data types
- **Cache Manager**: Optimizes performance with intelligent caching
- **Event System**: Handles event-driven automation and hooks

**Responsibilities:**

- Execute business logic for all operations
- Manage data relationships and constraints
- Provide query and search capabilities
- Handle caching and performance optimization
- Manage event-driven workflows
- Ensure data consistency and integrity

#### 3. Data Storage Layer

The bottom layer manages data persistence and synchronization:

**Components:**

- **SQLite Database**: Primary storage for structured data
- **RocksDB Store**: High-performance key-value storage
- **File System**: Document content and configuration storage
- **Backup Manager**: Automated backup and restore functionality
- **Sync Engine**: Remote synchronization capabilities
- **Plugin System**: Extensible sync provider framework
- **Migration System**: Database schema evolution

**Responsibilities:**

- Provide reliable data persistence
- Handle data synchronization across devices
- Manage backup and recovery operations
- Ensure data integrity and consistency
- Support schema evolution and migrations
- Optimize storage performance
- Provide extensible plugin system for external integrations

#### 4. AI Agent Integration Layer

The integration layer provides interfaces for AI agent interaction:

**Components:**

- **Agent Interface**: Manages AI agent connections and authentication
- **Context Manager**: Handles agent context and state management
- **Workflow Manager**: Orchestrates AI agent workflows
- **Communication Manager**: Manages real-time communication channels

**Responsibilities:**

- Provide standardized AI agent interfaces
- Manage agent context and state
- Orchestrate complex AI workflows
- Handle real-time communication
- Ensure secure agent interactions
- Support multiple AI agent types

### Architecture Reference Guidelines

> **Important**: All other specifications should reference this master architecture instead of defining their own diagrams. Use the following format:

**For Component-Specific Specs:**

```markdown
## Architecture Context

This component operates within the Edda architecture as defined in [SPEC_ARCHITECTURE.md](./SPEC_ARCHITECTURE.md):

- **Layer**: [Layer Name] (e.g., "Core Engine Layer")
- **Component**: [Component Name] (e.g., "Task Manager")
- **Responsibilities**: [Brief description of component responsibilities]
- **Dependencies**: [List of dependencies on other components]
```

**For Cross-Component Specs:**

```markdown
## Architecture Integration

This specification defines interactions between components within the Edda architecture as defined in [SPEC_ARCHITECTURE.md](./SPEC_ARCHITECTURE.md). The components involved are:

- [Component A] in [Layer A]
- [Component B] in [Layer B]
- [Component C] in [Layer C]
```

### Architecture Evolution

When adding new components or modifying existing ones:

1. **Update this master architecture reference first**
2. **Update component responsibilities and dependencies**
3. **Update the master diagram if needed**
4. **Reference the updated architecture from other specs**

## Data Models

> **Note**: All data model definitions are centralized in [SPEC_DATA_MODELS.md](./SPEC_DATA_MODELS.md) to ensure consistency and avoid redundancy. The models shown below are simplified references - see the data models specification for complete definitions.

### Task Model

```rust
// Task model definition is standardized in SPEC_DATA_MODELS.md
// Reference: See SPEC_DATA_MODELS.md for complete and up-to-date Task definition
```

// TaskStatus and Priority definitions are standardized in SPEC_DATA_MODELS.md
// Reference: See SPEC_DATA_MODELS.md for complete and up-to-date definitions

````

### Document Model

```rust
// Document model definitions are standardized in SPEC_DATA_MODELS.md
// Reference: See SPEC_DATA_MODELS.md for complete and up-to-date Document definitions
````

### State Model

```rust
// State model definition is standardized in SPEC_DATA_MODELS.md
// Reference: See SPEC_DATA_MODELS.md for complete and up-to-date StateEntry definition
```

## Command Architecture

### Command Structure

Each command follows a consistent pattern:

```rust
#[derive(Parser)]
pub struct TaskCommand {
    #[command(subcommand)]
    command: TaskSubcommand,
}

#[derive(Subcommand)]
pub enum TaskSubcommand {
    /// Add a new task
    Add(AddTaskCommand),
    /// List tasks
    List(ListTasksCommand),
    /// Get task details
    Get(GetTaskCommand),
    /// Update task
    Update(UpdateTaskCommand),
    /// Complete task
    Complete(CompleteTaskCommand),
    /// Delete task
    Delete(DeleteTaskCommand),
}
```

### Command Execution Flow

```
1. Parse Arguments → 2. Validate Input → 3. Execute Command → 4. Format Output → 5. Handle Errors
```

**Example Flow:**

```rust
pub async fn execute_task_add(cmd: AddTaskCommand) -> Result<(), Error> {
    // 1. Validate input
    let task = validate_task_input(&cmd)?;

    // 2. Execute business logic
    let created_task = task_manager.create_task(task).await?;

    // 3. Format output
    let output = format_task_output(created_task, cmd.format)?;

    // 4. Write to stdout
    println!("{}", output);

    Ok(())
}
```

## Storage Architecture

### Database Schema

```sql
-- Tasks table
CREATE TABLE tasks (
    id INTEGER PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL DEFAULT 'pending',
    priority TEXT NOT NULL DEFAULT 'medium',
    tags TEXT, -- JSON array
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    completed_at TEXT,
    due_date TEXT,
    assigned_to TEXT,
    parent_id INTEGER,
    metadata TEXT -- JSON object
);

-- Documents table
CREATE TABLE documents (
    id INTEGER PRIMARY KEY,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    content_type TEXT NOT NULL DEFAULT 'text/plain',
    file_path TEXT,
    tags TEXT, -- JSON array
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    version INTEGER NOT NULL DEFAULT 1,
    metadata TEXT -- JSON object
);

-- State table
CREATE TABLE state (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    expires_at TEXT,
    metadata TEXT -- JSON object
);

-- Indexes for performance
CREATE INDEX idx_tasks_status ON tasks(status);
CREATE INDEX idx_tasks_priority ON tasks(priority);
CREATE INDEX idx_tasks_tags ON tasks(tags);
CREATE INDEX idx_documents_tags ON documents(tags);
CREATE INDEX idx_state_expires ON state(expires_at);
```

### Storage Strategy

**Primary Storage (SQLite):**

- Structured data (tasks, documents, state)
- ACID compliance for data integrity
- Efficient querying with indexes
- Transaction support for complex operations

**Secondary Storage (RocksDB):**

- High-performance key-value storage
- Caching layer for frequently accessed data
- Temporary data and session storage
- Large binary data storage

**File System:**

- Document content files
- Configuration files
- Backup archives
- Log files

## Query Engine

### Query Language

Edda implements a SQL-like query language for complex data retrieval:

```sql
-- Basic task queries
SELECT * FROM tasks WHERE status = 'pending'
SELECT title, priority FROM tasks WHERE priority = 'high'

-- Aggregation queries
SELECT status, COUNT(*) as count FROM tasks GROUP BY status
SELECT AVG(priority_score) FROM tasks WHERE status = 'completed'

-- Join queries
SELECT t.title, d.title as doc_title
FROM tasks t
JOIN documents d ON t.doc_id = d.id

-- Complex filtering
SELECT * FROM tasks
WHERE 'urgent' IN tags
AND created_at > '2024-01-01'
ORDER BY priority DESC, created_at ASC
```

### Query Processing

```
1. Parse Query → 2. Validate Syntax → 3. Optimize Query → 4. Execute → 5. Format Results
```

**Query Optimization:**

- Index usage optimization
- Query plan generation
- Result caching
- Streaming for large results

## Event System

### Event Types

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Event {
    TaskCreated { task_id: u64, task: Task },
    TaskUpdated { task_id: u64, changes: HashMap<String, Value> },
    TaskCompleted { task_id: u64, task: Task },
    DocumentAdded { doc_id: u64, document: Document },
    StateChanged { key: String, value: Value },
    QueryExecuted { query: String, result_count: usize },
    SystemBackup { backup_path: PathBuf },
    ErrorOccurred { error: String, context: String },
}
```

### Event Handlers

```rust
pub trait EventHandler: Send + Sync {
    async fn handle_event(&self, event: Event) -> Result<(), Error>;
}

// Example handlers
pub struct LoggingHandler;
pub struct WebhookHandler;
pub struct NotificationHandler;
pub struct AutomationHandler;
```

## Configuration Management

### Configuration Structure

```toml
[core]
data_dir = "~/.local/share/edda"
format = "text"
quiet = false
verbose = false

[storage]
type = "sqlite"
path = "~/.local/share/edda/data.db"
backup_interval = "24h"
max_backups = 10

[query]
default_limit = 100
enable_streaming = true
cache_results = true
cache_ttl = "1h"

[output]
colors = true
pager = "less"
table_style = "compact"
date_format = "%Y-%m-%d %H:%M:%S"

[ai_integration]
enable_hooks = true
webhook_url = ""
api_key = ""
```

### Configuration Loading

```rust
pub struct Config {
    pub core: CoreConfig,
    pub storage: StorageConfig,
    pub query: QueryConfig,
    pub output: OutputConfig,
    pub ai_integration: AiIntegrationConfig,
}

impl Config {
    pub fn load() -> Result<Self, Error> {
        // 1. Load default config
        let mut config = Self::default();

        // 2. Load from config file
        if let Some(config_file) = Self::find_config_file() {
            config.merge_from_file(config_file)?;
        }

        // 3. Override with environment variables
        config.merge_from_env()?;

        // 4. Override with command line arguments
        config.merge_from_args()?;

        Ok(config)
    }
}
```

## Error Handling

### Error Types

```rust
#[derive(Debug, thiserror::Error)]
pub enum EddaError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Task not found: {id}")]
    TaskNotFound { id: u64 },

    #[error("Document not found: {id}")]
    DocumentNotFound { id: u64 },

    #[error("Query error: {0}")]
    Query(String),
}
```

### Error Handling Strategy

1. **Graceful Degradation**: Continue operation when possible
2. **Clear Error Messages**: Provide actionable error information
3. **Error Recovery**: Automatic retry for transient errors
4. **Error Logging**: Comprehensive error tracking
5. **User Feedback**: Clear indication of what went wrong

## Performance Optimization

### Caching Strategy

```rust
pub struct CacheManager {
    memory_cache: LruCache<String, Value>,
    disk_cache: RocksDB,
    query_cache: LruCache<String, QueryResult>,
}

impl CacheManager {
    pub async fn get_or_set<T>(&mut self, key: &str, f: impl FnOnce() -> T) -> T {
        if let Some(value) = self.memory_cache.get(key) {
            return value.clone();
        }

        let value = f();
        self.memory_cache.put(key.to_string(), value.clone());
        value
    }
}
```

### Database Optimization

- **Indexes**: Strategic indexing for common queries
- **Connection Pooling**: Efficient database connection management
- **Query Optimization**: Query plan analysis and optimization
- **Batch Operations**: Bulk operations for better performance

### Memory Management

- **Streaming**: Process large datasets without loading into memory
- **Memory Pooling**: Reuse memory allocations
- **Garbage Collection**: Efficient memory cleanup
- **Memory Monitoring**: Track memory usage and optimize

## Security Considerations

### Data Protection

- **Encryption**: Sensitive data encryption at rest
- **Access Control**: File system permissions and database access control
- **Input Validation**: Comprehensive input sanitization
- **Output Encoding**: Prevent injection attacks in output

### Privacy

- **Data Minimization**: Only collect necessary data
- **Local Storage**: Default to local storage only
- **Data Retention**: Configurable data retention policies
- **Audit Logging**: Track data access and modifications

## Testing Strategy

> **Note**: Testing strategies and standards are centralized in [SPEC_DEVELOPMENT.md](./SPEC_DEVELOPMENT.md). This section provides architecture-specific testing context and references the standardized testing approaches.

### Architecture Testing Context

The Edda architecture requires testing across all four layers:

- **CLI Interface Layer**: Test command parsing, argument validation, and output formatting
- **Core Engine Layer**: Test business logic, data processing, and workflow management
- **Data Storage Layer**: Test data persistence, synchronization, and integrity
- **AI Agent Integration Layer**: Test agent communication, context management, and workflow orchestration

### Testing Integration Points

- **Cross-Layer Integration**: Test interactions between architectural layers
- **External System Integration**: Test sync providers, AI agents, and external APIs
- **Performance Testing**: Test system performance under various load conditions
- **Security Testing**: Test data protection, access control, and vulnerability assessment

### Testing Standards Reference

For detailed testing standards, patterns, and examples, see the **Standardized Testing Strategies** section in [SPEC_DEVELOPMENT.md](./SPEC_DEVELOPMENT.md), which includes:

- Unit testing standards and patterns
- Integration testing approaches
- Performance testing benchmarks
- Security testing requirements
- Documentation testing standards
- Test configuration and reporting standards

## Deployment Architecture

### Single Binary Deployment

```
edda/
├── edda.exe (Windows)
├── edda (Linux/macOS)
└── config/
    └── config.toml
```

### Data Directory Structure

```
~/.local/share/edda/
├── data.db (SQLite database)
├── documents/ (Document files)
├── backups/ (Backup archives)
├── cache/ (Temporary cache)
└── logs/ (Application logs)
```

### Configuration Locations

- **System Config**: `/etc/edda/config.toml`
- **User Config**: `~/.config/edda.toml`
- **Project Config**: `./.edda.toml`
- **Environment Variables**: `EDDA_*` variables

## Future Architecture Considerations

### Plugin System

```rust
pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn commands(&self) -> Vec<Box<dyn Command>>;
    fn hooks(&self) -> Vec<Box<dyn Hook>>;
}
```

### API Layer

```rust
pub struct ApiServer {
    router: Router,
    task_service: TaskService,
    document_service: DocumentService,
    state_service: StateService,
}

impl ApiServer {
    pub async fn start(&self, addr: SocketAddr) -> Result<(), Error> {
        // HTTP API server implementation
    }
}
```

### Distributed Architecture

- **Multi-Node Support**: Distributed data storage
- **Replication**: Data replication across nodes
- **Load Balancing**: Request distribution
- **Service Discovery**: Automatic service discovery

## MVP/Phase 0 Architecture Context

- The Sync Engine must support GitHub as a first-class provider for task sync in the MVP.
- The architecture must allow for pluggable sync providers (GitLab, JIRA, etc.) in future phases.
- See Sync Engine spec for provider interface and extensibility details.

> When Edda can manage tasks and sync with GitHub, it will be used internally for development tracking.
