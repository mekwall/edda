# Edda - Error Handling Specification

## Overview

This specification defines the standardized error handling patterns, error types, and error codes for Edda. All components should reference this specification for consistent error handling across the application.

## Error Handling Principles

### 1. Consistent Error Types

All errors should follow a consistent structure:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EddaError {
    /// Error code for programmatic handling
    pub code: ErrorCode,
    /// Human-readable error message
    pub message: String,
    /// Detailed error context
    pub details: Option<String>,
    /// Error source/component
    pub source: ErrorSource,
    /// Timestamp when error occurred
    pub timestamp: DateTime<Utc>,
    /// Additional error metadata
    pub metadata: HashMap<String, Value>,
}
```

### 2. Standardized Error Codes

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ErrorCode {
    // General errors (1000-1999)
    GeneralError = 1000,
    InvalidArgument = 1001,
    NotFound = 1002,
    AlreadyExists = 1003,
    PermissionDenied = 1004,
    Timeout = 1005,
    NetworkError = 1006,

    // CLI errors (2000-2999)
    InvalidCommand = 2000,
    InvalidOption = 2001,
    MissingArgument = 2002,
    InvalidFormat = 2003,

    // Task errors (3000-3999)
    TaskNotFound = 3000,
    TaskAlreadyExists = 3001,
    InvalidTaskData = 3002,
    TaskHasChildren = 3003,
    InvalidTaskStatus = 3004,
    InvalidTaskPriority = 3005,

    // Document errors (4000-4999)
    DocumentNotFound = 4000,
    DocumentAlreadyExists = 4001,
    InvalidDocumentData = 4002,
    DocumentContentError = 4003,
    InvalidDocumentType = 4004,

    // Storage errors (5000-5999)
    StorageError = 5000,
    DatabaseError = 5001,
    FileSystemError = 5002,
    BackupError = 5003,
    MigrationError = 5004,

    // Query errors (6000-6999)
    QueryParseError = 6000,
    QueryExecutionError = 6001,
    InvalidQuery = 6002,
    QueryTimeout = 6003,

    // Sync errors (7000-7999)
    SyncError = 7000,
    ConflictError = 7001,
    NetworkSyncError = 7002,
    VersionConflict = 7003,

    // AI Integration errors (8000-8999)
    AgentError = 8000,
    ContextError = 8001,
    WorkflowError = 8002,
    CommunicationError = 8003,

    // Configuration errors (9000-9999)
    ConfigError = 9000,
    InvalidConfig = 9001,
    MissingConfig = 9002,
    ConfigParseError = 9003,
}
```

### 3. Error Sources

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ErrorSource {
    CLI,
    TaskManager,
    DocumentManager,
    StorageEngine,
    QueryEngine,
    SyncEngine,
    AIIntegration,
    Configuration,
    System,
}
```

## Error Handling Patterns

### 1. Result Type Pattern

All functions should return `Result<T, EddaError>`:

```rust
pub fn create_task(&self, task_data: &TaskData) -> Result<Task, EddaError> {
    // Validate input
    if task_data.title.is_empty() {
        return Err(EddaError {
            code: ErrorCode::InvalidTaskData,
            message: "Task title cannot be empty".to_string(),
            details: Some("Title field is required".to_string()),
            source: ErrorSource::TaskManager,
            timestamp: Utc::now(),
            metadata: HashMap::new(),
        });
    }

    // Implementation...
    Ok(task)
}
```

### 2. Error Propagation

Use `?` operator for error propagation:

```rust
pub fn process_task(&self, task_id: &TaskId) -> Result<Task, EddaError> {
    let task = self.storage.get_task(task_id)?;
    let processed = self.validator.validate_task(&task)?;
    Ok(processed)
}
```

### 3. Error Context

Add context to errors when propagating:

```rust
pub fn update_task(&self, task_id: &TaskId, updates: &TaskUpdates) -> Result<Task, EddaError> {
    let task = self.storage.get_task(task_id)
        .map_err(|e| EddaError {
            code: ErrorCode::TaskNotFound,
            message: format!("Failed to retrieve task {}", task_id),
            details: Some(e.message.clone()),
            source: ErrorSource::TaskManager,
            timestamp: Utc::now(),
            metadata: HashMap::new(),
        })?;

    // Implementation...
    Ok(updated_task)
}
```

## CLI Error Handling

### 1. Exit Codes

Standard exit codes for CLI operations:

```rust
pub const EXIT_SUCCESS: i32 = 0;
pub const EXIT_GENERAL_ERROR: i32 = 1;
pub const EXIT_INVALID_COMMAND: i32 = 2;
pub const EXIT_FILE_IO_ERROR: i32 = 3;
pub const EXIT_DATABASE_ERROR: i32 = 4;
pub const EXIT_CONFIG_ERROR: i32 = 5;
```

### 2. Error Output Format

```rust
pub fn format_error(error: &EddaError, format: OutputFormat) -> String {
    match format {
        OutputFormat::Text => format_text_error(error),
        OutputFormat::Json => format_json_error(error),
        OutputFormat::Yaml => format_yaml_error(error),
    }
}

fn format_text_error(error: &EddaError) -> String {
    format!(
        "Error {}: {}\n{}\n",
        error.code as i32,
        error.message,
        error.details.as_deref().unwrap_or("")
    )
}
```

## Error Recovery Strategies

### 1. Retry Logic

```rust
pub fn with_retry<T, F>(operation: F, max_retries: u32) -> Result<T, EddaError>
where
    F: Fn() -> Result<T, EddaError>,
{
    let mut last_error = None;

    for attempt in 0..=max_retries {
        match operation() {
            Ok(result) => return Ok(result),
            Err(error) => {
                last_error = Some(error);
                if attempt < max_retries {
                    std::thread::sleep(Duration::from_millis(100 * (attempt + 1) as u64));
                }
            }
        }
    }

    Err(last_error.unwrap())
}
```

### 2. Fallback Strategies

```rust
pub fn with_fallback<T, F, G>(primary: F, fallback: G) -> Result<T, EddaError>
where
    F: Fn() -> Result<T, EddaError>,
    G: Fn() -> Result<T, EddaError>,
{
    primary().or_else(|_| fallback())
}
```

## Error Logging

### 1. Structured Logging

```rust
pub fn log_error(error: &EddaError) {
    log::error!(
        "Error occurred: code={}, source={:?}, message={}",
        error.code as i32,
        error.source,
        error.message
    );

    if let Some(details) = &error.details {
        log::debug!("Error details: {}", details);
    }
}
```

### 2. Error Metrics

```rust
pub fn record_error_metric(error: &EddaError) {
    metrics::counter!("edda_errors_total", 1,
        "code" => error.code as i32,
        "source" => format!("{:?}", error.source)
    );
}
```

## Error Handling Best Practices

### 1. Always Provide Context

```rust
// Good
return Err(EddaError {
    code: ErrorCode::TaskNotFound,
    message: format!("Task with ID '{}' not found", task_id),
    details: Some("Use 'edda task list' to see available tasks".to_string()),
    source: ErrorSource::TaskManager,
    timestamp: Utc::now(),
    metadata: HashMap::new(),
});

// Bad
return Err(EddaError {
    code: ErrorCode::NotFound,
    message: "Not found".to_string(),
    details: None,
    source: ErrorSource::TaskManager,
    timestamp: Utc::now(),
    metadata: HashMap::new(),
});
```

### 2. Use Specific Error Codes

```rust
// Good - specific error code
ErrorCode::TaskNotFound

// Bad - generic error code
ErrorCode::GeneralError
```

### 3. Include Actionable Details

```rust
// Good - provides actionable information
details: Some("Use 'edda task list' to see available tasks".to_string()),

// Bad - no actionable information
details: None,
```

## Error Handling in Tests

### 1. Error Assertions

```rust
#[test]
fn test_invalid_task_creation() {
    let result = create_task(&invalid_data);
    assert!(result.is_err());

    let error = result.unwrap_err();
    assert_eq!(error.code, ErrorCode::InvalidTaskData);
    assert!(error.message.contains("title"));
}
```

### 2. Error Context Testing

```rust
#[test]
fn test_error_context() {
    let result = update_task(&nonexistent_id, &updates);
    let error = result.unwrap_err();

    assert_eq!(error.source, ErrorSource::TaskManager);
    assert!(error.details.is_some());
}
```

## Integration with Other Specs

This error handling specification should be referenced by:

- `SPEC_CLI_DESIGN.md` - For CLI error handling patterns
- `SPEC_TASK_MANAGEMENT.md` - For task-specific error handling
- `SPEC_DOCUMENT_MANAGEMENT.md` - For document-specific error handling
- `SPEC_STORAGE_ENGINE.md` - For storage error handling
- `SPEC_QUERY_ENGINE.md` - For query error handling
- `SPEC_SYNC_ENGINE.md` - For sync error handling
- `SPEC_AI_INTEGRATION.md` - For AI integration error handling

All other specifications should reference this document instead of defining their own error handling patterns.
