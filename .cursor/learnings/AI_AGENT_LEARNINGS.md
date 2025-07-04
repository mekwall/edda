# AI Agent Integration Learnings

## Overview

This document captures learnings and insights about integrating with AI agents and automation systems, specifically for the Edda project.

## Key Learnings

### 1. Structured Data for AI Consumption

**Learning**: AI agents need structured, machine-readable data formats.

**Data Formats**:

- **JSON**: Primary format for AI consumption
- **YAML**: Human-readable structured data
- **CSV**: Tabular data for analysis
- **GraphQL**: Queryable data interface

**Implementation Pattern**:

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: u64,
    pub title: String,
    pub description: Option<String>,
    pub status: TaskStatus,
    pub priority: Priority,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub metadata: HashMap<String, Value>,
}

impl Task {
    pub fn to_json(&self) -> Result<String, Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn to_yaml(&self) -> Result<String, Error> {
        serde_yaml::to_string(self)
    }
}
```

### 2. Query Language Design

**Learning**: AI agents need powerful query capabilities for data exploration.

**Query Features**:

- **SQL-like Syntax**: Familiar query language
- **Full-text Search**: Content-based searching
- **Aggregation Functions**: Statistical analysis
- **Joins**: Cross-entity relationships

**Implementation**:

```rust
pub struct QueryEngine {
    db: Database,
    parser: QueryParser,
    executor: QueryExecutor,
}

impl QueryEngine {
    pub async fn execute(&self, query: &str) -> Result<QueryResult, Error> {
        let parsed = self.parser.parse(query)?;
        let optimized = self.optimizer.optimize(parsed)?;
        self.executor.execute(optimized).await
    }
}

// Example queries
// SELECT * FROM tasks WHERE status = 'pending' AND priority = 'high'
// SELECT COUNT(*) as count, status FROM tasks GROUP BY status
// SELECT t.title, d.content FROM tasks t JOIN documents d ON t.doc_id = d.id
```

### 3. Event-Driven Architecture

**Learning**: AI agents benefit from real-time event notifications.

**Event Types**:

- **Data Changes**: CRUD operations
- **State Transitions**: Status changes
- **System Events**: Backups, errors, warnings
- **User Actions**: Command executions

**Implementation**:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Event {
    TaskCreated { task_id: u64, task: Task },
    TaskUpdated { task_id: u64, changes: HashMap<String, Value> },
    TaskCompleted { task_id: u64, task: Task },
    DocumentAdded { doc_id: u64, document: Document },
    StateChanged { key: String, value: Value },
    SystemBackup { backup_path: PathBuf },
    ErrorOccurred { error: String, context: String },
}

pub trait EventHandler: Send + Sync {
    async fn handle_event(&self, event: Event) -> Result<(), Error>;
}

pub struct WebhookHandler {
    url: String,
    client: reqwest::Client,
}

impl EventHandler for WebhookHandler {
    async fn handle_event(&self, event: Event) -> Result<(), Error> {
        let payload = serde_json::to_string(&event)?;
        self.client.post(&self.url)
            .json(&event)
            .send()
            .await?;
        Ok(())
    }
}
```

### 4. Automation Hooks

**Learning**: AI agents need hooks for custom automation logic.

**Hook Types**:

- **Pre-hooks**: Validation and preparation
- **Post-hooks**: Cleanup and notifications
- **Error-hooks**: Error handling and recovery
- **Scheduled-hooks**: Time-based automation

**Implementation**:

```rust
pub trait Hook: Send + Sync {
    fn name(&self) -> &str;
    async fn execute(&self, context: HookContext) -> Result<(), Error>;
}

pub struct HookContext {
    pub command: String,
    pub args: Vec<String>,
    pub data: HashMap<String, Value>,
    pub timestamp: DateTime<Utc>,
}

pub struct HookManager {
    hooks: HashMap<String, Vec<Box<dyn Hook>>>,
}

impl HookManager {
    pub async fn execute_hooks(&self, event: &str, context: HookContext) -> Result<(), Error> {
        if let Some(hooks) = self.hooks.get(event) {
            for hook in hooks {
                hook.execute(context.clone()).await?;
            }
        }
        Ok(())
    }
}
```

### 5. State Management

**Learning**: AI agents need persistent state for context and continuity.

**State Types**:

- **Session State**: Temporary data for current session
- **Persistent State**: Long-term data storage
- **Configuration State**: Settings and preferences
- **Cache State**: Performance optimization

**Implementation**:

```rust
pub struct StateManager {
    db: Database,
    cache: LruCache<String, Value>,
}

impl StateManager {
    pub async fn set(&mut self, key: &str, value: Value) -> Result<(), Error> {
        // Update cache
        self.cache.put(key.to_string(), value.clone());

        // Persist to database
        self.db.execute(
            "INSERT OR REPLACE INTO state (key, value, updated_at) VALUES (?, ?, ?)",
            (key, serde_json::to_string(&value)?, Utc::now()),
        ).await?;

        Ok(())
    }

    pub async fn get(&self, key: &str) -> Result<Option<Value>, Error> {
        // Check cache first
        if let Some(value) = self.cache.get(key) {
            return Ok(Some(value.clone()));
        }

        // Query database
        let row = self.db.query_row(
            "SELECT value FROM state WHERE key = ?",
            (key,),
            |row| row.get(0),
        ).await?;

        let value: Value = serde_json::from_str(&row)?;
        Ok(Some(value))
    }
}
```

### 6. API Integration

**Learning**: AI agents need programmatic access to data and functionality.

**API Design**:

- **RESTful Endpoints**: Standard HTTP API
- **GraphQL Interface**: Flexible querying
- **WebSocket Events**: Real-time updates
- **gRPC Services**: High-performance RPC

**Implementation**:

```rust
use axum::{Router, Json, extract::State};
use serde::{Deserialize, Serialize};

pub struct ApiServer {
    task_service: TaskService,
    document_service: DocumentService,
    state_service: StateService,
}

impl ApiServer {
    pub fn router(self) -> Router {
        Router::new()
            .route("/tasks", get(list_tasks).post(create_task))
            .route("/tasks/:id", get(get_task).put(update_task).delete(delete_task))
            .route("/documents", get(list_documents).post(create_document))
            .route("/state/:key", get(get_state).put(set_state))
            .route("/query", post(execute_query))
            .with_state(self)
    }
}

async fn list_tasks(
    State(state): State<ApiServer>,
    Query(params): Query<ListParams>,
) -> Result<Json<Vec<Task>>, Error> {
    let tasks = state.task_service.list_tasks(params).await?;
    Ok(Json(tasks))
}

async fn execute_query(
    State(state): State<ApiServer>,
    Json(query): Json<QueryRequest>,
) -> Result<Json<QueryResult>, Error> {
    let result = state.query_engine.execute(&query.sql).await?;
    Ok(Json(result))
}
```

### 7. Data Validation

**Learning**: AI agents need robust data validation to prevent errors.

**Validation Strategies**:

- **Schema Validation**: JSON schema validation
- **Type Checking**: Runtime type validation
- **Business Rules**: Domain-specific validation
- **Sanitization**: Input cleaning and normalization

**Implementation**:

```rust
use validator::{Validate, ValidationError};

#[derive(Debug, Validate)]
pub struct CreateTaskRequest {
    #[validate(length(min = 1, max = 200))]
    pub title: String,

    #[validate(length(max = 1000))]
    pub description: Option<String>,

    #[validate(custom = "validate_priority")]
    pub priority: Priority,

    #[validate(custom = "validate_tags")]
    pub tags: Vec<String>,
}

fn validate_priority(priority: &Priority) -> Result<(), ValidationError> {
    match priority {
        Priority::Low | Priority::Medium | Priority::High | Priority::Critical => Ok(()),
        _ => Err(ValidationError::new("invalid_priority")),
    }
}

fn validate_tags(tags: &[String]) -> Result<(), ValidationError> {
    for tag in tags {
        if tag.len() > 50 {
            return Err(ValidationError::new("tag_too_long"));
        }
        if !tag.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
            return Err(ValidationError::new("invalid_tag_format"));
        }
    }
    Ok(())
}
```

### 8. Error Handling for AI

**Learning**: AI agents need detailed error information for debugging and recovery.

**Error Information**:

- **Error Codes**: Machine-readable error codes
- **Error Context**: Relevant data and state
- **Recovery Suggestions**: Possible solutions
- **Error Classification**: Error categories and severity

**Implementation**:

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
    pub context: HashMap<String, Value>,
    pub suggestions: Vec<String>,
    pub severity: ErrorSeverity,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ErrorSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

impl From<EddaError> for ErrorResponse {
    fn from(error: EddaError) -> Self {
        match error {
            EddaError::TaskNotFound { id } => ErrorResponse {
                code: "TASK_NOT_FOUND".to_string(),
                message: format!("Task with ID {} not found", id),
                context: HashMap::from([
                    ("task_id".to_string(), Value::Number(id.into())),
                ]),
                suggestions: vec![
                    "Use 'edda task list' to see available tasks".to_string(),
                    "Check if the task ID is correct".to_string(),
                ],
                severity: ErrorSeverity::Warning,
            },
            EddaError::InvalidQuery { query } => ErrorResponse {
                code: "INVALID_QUERY".to_string(),
                message: format!("Invalid query syntax: {}", query),
                context: HashMap::from([
                    ("query".to_string(), Value::String(query)),
                ]),
                suggestions: vec![
                    "Check query syntax".to_string(),
                    "Use 'edda query --help' for examples".to_string(),
                ],
                severity: ErrorSeverity::Error,
            },
            // ... other error types
        }
    }
}
```

### 9. Performance Optimization

**Learning**: AI agents need fast response times for interactive workflows.

**Optimization Strategies**:

- **Caching**: Cache frequently accessed data
- **Indexing**: Database indexes for fast queries
- **Streaming**: Stream large datasets
- **Parallelization**: Concurrent operations

**Implementation**:

```rust
pub struct OptimizedQueryEngine {
    db: Database,
    cache: LruCache<String, QueryResult>,
    query_planner: QueryPlanner,
}

impl OptimizedQueryEngine {
    pub async fn execute(&mut self, query: &str) -> Result<QueryResult, Error> {
        // Check cache first
        if let Some(cached) = self.cache.get(query) {
            return Ok(cached.clone());
        }

        // Parse and optimize query
        let parsed = self.parser.parse(query)?;
        let optimized = self.query_planner.optimize(parsed)?;

        // Execute query
        let result = self.executor.execute(optimized).await?;

        // Cache result
        self.cache.put(query.to_string(), result.clone());

        Ok(result)
    }
}
```

### 10. Security for AI Integration

**Learning**: AI agent integration requires careful security considerations.

**Security Measures**:

- **Authentication**: API key or token-based auth
- **Authorization**: Role-based access control
- **Input Validation**: Prevent injection attacks
- **Rate Limiting**: Prevent abuse
- **Audit Logging**: Track all operations

**Implementation**:

```rust
pub struct SecurityManager {
    api_keys: HashSet<String>,
    rate_limiter: RateLimiter,
    audit_logger: AuditLogger,
}

impl SecurityManager {
    pub fn authenticate(&self, api_key: &str) -> Result<(), Error> {
        if !self.api_keys.contains(api_key) {
            return Err(Error::Unauthorized);
        }
        Ok(())
    }

    pub fn check_rate_limit(&mut self, client_id: &str) -> Result<(), Error> {
        if !self.rate_limiter.check(client_id) {
            return Err(Error::RateLimitExceeded);
        }
        Ok(())
    }

    pub async fn log_operation(&self, operation: &str, client_id: &str, details: &Value) {
        self.audit_logger.log(operation, client_id, details).await;
    }
}
```

## Common Patterns

### 1. Command Pattern

**Use Case**: AI agents need to execute commands and track their results.

```rust
pub trait Command {
    fn name(&self) -> &str;
    async fn execute(&self, args: &[String]) -> Result<CommandResult, Error>;
}

pub struct CommandResult {
    pub success: bool,
    pub output: String,
    pub error: Option<String>,
    pub duration: Duration,
    pub timestamp: DateTime<Utc>,
}
```

### 2. Observer Pattern

**Use Case**: AI agents need to be notified of system events.

```rust
pub trait Observer: Send + Sync {
    async fn on_event(&self, event: &Event) -> Result<(), Error>;
}

pub struct EventBus {
    observers: Vec<Box<dyn Observer>>,
}

impl EventBus {
    pub async fn publish(&self, event: Event) -> Result<(), Error> {
        for observer in &self.observers {
            observer.on_event(&event).await?;
        }
        Ok(())
    }
}
```

### 3. Strategy Pattern

**Use Case**: AI agents need different output formats and processing strategies.

```rust
pub trait OutputStrategy {
    fn format(&self, data: &Value) -> Result<String, Error>;
}

pub struct JsonStrategy;
pub struct YamlStrategy;
pub struct CsvStrategy;

impl OutputStrategy for JsonStrategy {
    fn format(&self, data: &Value) -> Result<String, Error> {
        Ok(serde_json::to_string_pretty(data)?)
    }
}
```

## Best Practices Summary

1. **Provide structured data** in machine-readable formats
2. **Implement powerful query capabilities** for data exploration
3. **Use event-driven architecture** for real-time updates
4. **Create automation hooks** for custom logic
5. **Maintain persistent state** for context and continuity
6. **Design RESTful APIs** for programmatic access
7. **Validate all inputs** to prevent errors
8. **Provide detailed error information** for debugging
9. **Optimize for performance** with caching and indexing
10. **Implement security measures** for safe integration

## References

- [OpenAI API Documentation](https://platform.openai.com/docs)
- [Anthropic Claude API](https://docs.anthropic.com/)
- [LangChain Framework](https://python.langchain.com/)
- [REST API Design Guidelines](https://restfulapi.net/)
- [GraphQL Specification](https://graphql.org/learn/)
