# CLI Development Learnings

## Overview

This document captures learnings and insights from developing CLI applications in Rust, specifically for the Edda project.

## Key Learnings

### 1. CLI Framework Selection

**Learning**: `clap` is the de facto standard for Rust CLI applications.

**Why clap?**

- Excellent derive macro support
- Comprehensive help generation
- Shell completion support
- Active maintenance and community
- Performance-focused design

**Example Implementation**:

```rust
#[derive(Parser)]
#[command(name = "edda", about = "AI agent task and document manager")]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, default_value = "text")]
    format: OutputFormat,

    #[arg(short, long)]
    quiet: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Manage tasks
    Task(TaskCommand),
    /// Manage documents
    Doc(DocCommand),
    /// Manage state
    State(StateCommand),
}
```

### 2. Output Formatting

**Learning**: Multiple output formats are essential for automation.

**Formats to Support**:

- **Text**: Human-readable tables and lists
- **JSON**: Machine-readable for automation
- **YAML**: Human-readable structured data
- **CSV**: Spreadsheet compatibility

**Implementation Pattern**:

```rust
pub trait OutputFormatter {
    fn format_tasks(&self, tasks: &[Task]) -> Result<String, Error>;
    fn format_documents(&self, docs: &[Document]) -> Result<String, Error>;
    fn format_state(&self, state: &[StateEntry]) -> Result<String, Error>;
}

pub struct TextFormatter;
pub struct JsonFormatter;
pub struct YamlFormatter;
```

### 3. Error Handling

**Learning**: CLI errors must be user-friendly and actionable.

**Error Categories**:

- **User Errors**: Invalid input, missing files
- **System Errors**: Permission denied, disk full
- **Configuration Errors**: Invalid config, missing dependencies

**Best Practices**:

```rust
#[derive(Debug, thiserror::Error)]
pub enum CliError {
    #[error("Task not found: {id}")]
    TaskNotFound { id: u64 },

    #[error("Invalid query syntax: {query}")]
    InvalidQuery { query: String },

    #[error("Permission denied: {path}")]
    PermissionDenied { path: PathBuf },
}

impl CliError {
    pub fn exit_code(&self) -> i32 {
        match self {
            Self::TaskNotFound { .. } => 2,
            Self::InvalidQuery { .. } => 2,
            Self::PermissionDenied { .. } => 3,
        }
    }
}
```

### 4. Configuration Management

**Learning**: Configuration should be layered and flexible.

**Configuration Sources** (in order of precedence):

1. Command line arguments
2. Environment variables
3. Configuration file
4. Default values

**Implementation**:

```rust
pub struct Config {
    pub data_dir: PathBuf,
    pub format: OutputFormat,
    pub quiet: bool,
    pub verbose: bool,
}

impl Config {
    pub fn load() -> Result<Self, Error> {
        let mut config = Self::default();

        // Load from config file
        if let Some(config_file) = Self::find_config_file() {
            config.merge_from_file(config_file)?;
        }

        // Override with environment variables
        config.merge_from_env()?;

        // Override with command line arguments
        config.merge_from_args()?;

        Ok(config)
    }
}
```

### 5. Testing CLI Applications

**Learning**: CLI testing requires special considerations.

**Testing Strategies**:

- **Unit Tests**: Test individual functions
- **Integration Tests**: Test complete commands
- **Snapshot Tests**: Verify output formatting
- **Performance Tests**: Measure command execution time

**Testing Tools**:

```rust
use assert_cmd::Command;
use tempfile::TempDir;

#[test]
fn test_task_add_command() {
    let temp_dir = TempDir::new().unwrap();

    let cmd = Command::cargo_bin("edda")
        .unwrap()
        .args(&["task", "add", "Test task"])
        .env("EDDA_DATA_DIR", temp_dir.path())
        .assert()
        .success();

    // Verify task was created
    let output = Command::cargo_bin("edda")
        .unwrap()
        .args(&["task", "list"])
        .env("EDDA_DATA_DIR", temp_dir.path())
        .output()
        .unwrap();

    assert!(output.stdout.contains("Test task"));
}
```

### 6. Performance Optimization

**Learning**: CLI performance is critical for user experience.

**Performance Considerations**:

- **Startup Time**: Minimize initialization overhead
- **Memory Usage**: Efficient data structures
- **I/O Operations**: Async operations where appropriate
- **Caching**: Cache frequently accessed data

**Optimization Techniques**:

```rust
// Lazy loading of heavy components
pub struct LazyTaskManager {
    inner: OnceCell<TaskManager>,
}

impl LazyTaskManager {
    pub async fn get(&self) -> &TaskManager {
        self.inner.get_or_init(|| TaskManager::new()).await
    }
}

// Efficient output streaming
pub async fn stream_tasks<W: Write>(writer: &mut W, tasks: impl Stream<Item = Task>) {
    for await task in tasks {
        writeln!(writer, "{}", task.format())?;
    }
}
```

### 7. Shell Integration

**Learning**: Shell integration improves user experience significantly.

**Integration Features**:

- **Command Completion**: Tab completion for all commands
- **Shell Aliases**: Common command shortcuts
- **Shell Functions**: Complex command combinations

**Implementation**:

```rust
// Generate completion scripts
#[derive(Parser)]
pub struct CompletionsCommand {
    #[arg(value_enum)]
    shell: clap_complete::Shell,
}

pub fn generate_completions(shell: clap_complete::Shell) -> Result<(), Error> {
    let mut cmd = Cli::command();
    let name = cmd.get_name().to_string();

    clap_complete::generate(shell, &mut cmd, name, &mut std::io::stdout());
    Ok(())
}
```

### 8. Documentation

**Learning**: CLI documentation must be comprehensive and accessible.

**Documentation Types**:

- **Help Text**: Inline command help
- **Man Pages**: Traditional Unix documentation
- **README**: Project overview and examples
- **Examples**: Code examples and use cases

**Help Text Best Practices**:

```rust
#[derive(Parser)]
#[command(
    name = "edda",
    about = "AI agent task and document manager",
    long_about = "Edda is a fast, minimal CLI tool for managing tasks and documents on behalf of AI agents."
)]
pub struct Cli {
    /// Output format for results
    #[arg(short, long, value_enum, default_value_t = OutputFormat::Text)]
    format: OutputFormat,

    /// Suppress non-error output
    #[arg(short, long)]
    quiet: bool,
}
```

### 9. Cross-Platform Compatibility

**Learning**: CLI applications must work consistently across platforms.

**Platform Considerations**:

- **Path Handling**: Use `std::path::PathBuf` for cross-platform paths
- **Line Endings**: Handle different line ending conventions
- **File Permissions**: Respect platform-specific permissions
- **Terminal Colors**: Use `colored` crate for cross-platform colors

**Implementation**:

```rust
use std::path::PathBuf;
use dirs::home_dir;

pub fn get_data_dir() -> PathBuf {
    home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".local")
        .join("share")
        .join("edda")
}
```

### 10. Security Considerations

**Learning**: CLI applications must handle security properly.

**Security Practices**:

- **Input Validation**: Validate all user input
- **File Permissions**: Set appropriate file permissions
- **Sensitive Data**: Handle sensitive data carefully
- **Error Messages**: Don't leak sensitive information

**Implementation**:

```rust
pub fn validate_file_path(path: &str) -> Result<PathBuf, Error> {
    let path = PathBuf::from(path);

    // Prevent directory traversal attacks
    if path.components().any(|c| matches!(c, Component::ParentDir)) {
        return Err(Error::InvalidPath("Directory traversal not allowed"));
    }

    // Ensure path is within allowed directory
    let allowed_dir = get_data_dir();
    if !path.starts_with(&allowed_dir) {
        return Err(Error::InvalidPath("Path outside allowed directory"));
    }

    Ok(path)
}
```

## Common Pitfalls

### 1. Over-Engineering

**Problem**: Adding unnecessary complexity to simple commands.

**Solution**: Start simple and add complexity only when needed.

### 2. Poor Error Messages

**Problem**: Cryptic error messages that don't help users.

**Solution**: Provide clear, actionable error messages with hints.

### 3. Ignoring Performance

**Problem**: Slow commands that frustrate users.

**Solution**: Profile and optimize critical paths.

### 4. Inconsistent Interface

**Problem**: Inconsistent command structure and options.

**Solution**: Follow established CLI patterns and conventions.

## Best Practices Summary

1. **Use clap** for argument parsing
2. **Support multiple output formats** for automation
3. **Provide clear error messages** with actionable hints
4. **Test thoroughly** with real-world scenarios
5. **Optimize for performance** in critical paths
6. **Document comprehensively** with examples
7. **Follow platform conventions** for file paths and permissions
8. **Implement shell integration** for better UX
9. **Handle security properly** with input validation
10. **Keep it simple** and avoid over-engineering

## References

- [clap Documentation](https://docs.rs/clap/)
- [Rust CLI Guidelines](https://rust-cli.github.io/book/)
- [Command Line Interface Guidelines](https://clig.dev/)
- [Unix Philosophy](https://en.wikipedia.org/wiki/Unix_philosophy)
