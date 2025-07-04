# Edda - CLI Design Specification

## Overview

The Edda CLI provides a fast, intuitive interface for managing AI agent tasks, documents, and state. The design emphasizes discoverability, consistency, and automation-friendly output formats.

## Architecture Context

This component operates within the Edda architecture as defined in [SPEC_ARCHITECTURE.md](./SPEC_ARCHITECTURE.md):

- **Layer**: CLI Interface Layer
- **Component**: Command Parser, Argument Validator, Output Formatter, Help System, Completion Generator, Error Handler
- **Responsibilities**: Parse and validate command-line arguments, format output for different display modes, provide user-friendly error messages, generate help documentation and shell completions, handle cross-platform CLI differences
- **Dependencies**: Core Engine Layer (for business logic execution), Data Storage Layer (for data persistence), AI Agent Integration Layer (for AI agent interactions)

## Design Principles

### 1. Minimal and Fast

- **Zero-configuration startup**: Works out of the box with sensible defaults
- **Fast command execution**: Sub-100ms response times for basic operations
- **Efficient resource usage**: Minimal memory footprint and CPU usage

### 2. Discoverable and Self-Documenting

- **Comprehensive help system**: Every command provides detailed usage information
- **Progressive disclosure**: Simple commands for common tasks, advanced options for power users
- **Consistent patterns**: Similar command structures across different data types

### 3. Automation-Friendly

- **Structured output**: JSON format for machine consumption
- **Exit codes**: Consistent exit codes for error handling
- **Streaming support**: Real-time data processing capabilities

### 4. Cross-Platform Consistency

- **Platform-agnostic**: Same experience on Windows, macOS, and Linux
- **Native integration**: Respects platform conventions where appropriate
- **Path handling**: Consistent path handling across platforms

## Command Structure

### Top-Level Commands

```
edda [OPTIONS] <COMMAND>
```

**Global Options:**

- `--config <FILE>`: Specify configuration file
- `--data-dir <DIR>`: Override data directory
- `--format <FORMAT>`: Output format (text, json, yaml)
- `--quiet`: Suppress non-error output
- `--verbose`: Enable verbose logging

### Core Commands

#### Task Management

```
edda task [OPTIONS] <SUBCOMMAND>
```

**Subcommands:**

- `add <DESCRIPTION>`: Create a new task (Taskwarrior: `task add`)
- `list [QUERY]`: List tasks with optional filtering (Taskwarrior: `task list`)
- `get <ID>`: Get detailed task information (Taskwarrior: `task <id> info`)
- `modify <ID> <FIELD> <VALUE>`: Update task fields (Taskwarrior: `task <id> modify`)
- `done <ID>`: Mark task as completed (Taskwarrior: `task <id> done`)
- `delete <ID>`: Delete a task (Taskwarrior: `task <id> delete`)
- `start <ID>`: Start time tracking (Taskwarrior: `task <id> start`)
- `stop <ID>`: Stop time tracking (Taskwarrior: `task <id> stop`)
- `annotate <ID> <NOTE>`: Add annotation (Taskwarrior: `task <id> annotate`)
- `denotate <ID> <NOTE>`: Remove annotation (Taskwarrior: `task <id> denotate`)
- `tag <ID> <TAG>`: Add tag (Taskwarrior: `task <id> +tag`)
- `untag <ID> <TAG>`: Remove tag (Taskwarrior: `task <id> -tag`)
- `project <ID> <PROJECT>`: Set project (Taskwarrior: `task <id> modify project:`)
- `priority <ID> <PRIORITY>`: Set priority (Taskwarrior: `task <id> modify priority:`)
- `due <ID> <DATE>`: Set due date (Taskwarrior: `task <id> modify due:`)
- `scheduled <ID> <DATE>`: Set scheduled date (Taskwarrior: `task <id> modify scheduled:`)
- `wait <ID> <DATE>`: Set wait date (Taskwarrior: `task <id> modify wait:`)
- `depends <ID> <DEPENDENCY>`: Add dependency (Taskwarrior: `task <id> modify depends:`)
- `recur <ID> <PATTERN>`: Set recurrence (Taskwarrior: `task <id> modify recur:`)

#### Document Management

```
edda doc [OPTIONS] <SUBCOMMAND>
```

**Subcommands:**

- `add <PATH> [--title <TITLE>]`: Add a document
- `list [QUERY]`: List documents with optional filtering
- `get <ID>`: Get document content and metadata
- `update <ID> <FIELD> <VALUE>`: Update document metadata
- `content <ID>`: Get document content
- `delete <ID>`: Delete a document

#### State Management

```
edda state [OPTIONS] <SUBCOMMAND>
```

**Subcommands:**

- `set <KEY> <VALUE>`: Set a state value
- `get <KEY>`: Get a state value
- `list [PREFIX]`: List state keys with optional prefix
- `delete <KEY>`: Delete a state value
- `clear`: Clear all state

#### Query Engine

```
edda query <QUERY> [OPTIONS]
```

**Features:**

- SQL-like syntax for complex queries
- Support for joins across tasks, documents, and state
- Aggregation functions (count, sum, avg, etc.)
- Filtering and sorting capabilities

#### System Commands

```
edda system [OPTIONS] <SUBCOMMAND>
```

**Subcommands:**

- `init`: Initialize Edda data directory
- `backup`: Create backup of data
- `restore <BACKUP>`: Restore from backup
- `config`: Show/edit configuration
- `status`: Show system status
- `cleanup`: Clean up temporary files

## Output Formats

### Text Format (Default)

Human-readable output with consistent formatting:

```
$ edda task list
ID  Title                    Status    Created     Updated
1   Implement user auth      pending   2024-01-15  2024-01-15
2   Add API endpoints        done      2024-01-10  2024-01-14
3   Write documentation      pending   2024-01-12  2024-01-12
```

### JSON Format

Machine-readable output for automation:

```json
{
  "tasks": [
    {
      "id": 1,
      "title": "Implement user auth",
      "status": "pending",
      "created": "2024-01-15T10:30:00Z",
      "updated": "2024-01-15T10:30:00Z",
      "description": "Add user authentication system",
      "tags": ["auth", "backend"]
    }
  ],
  "meta": {
    "total": 3,
    "pending": 2,
    "completed": 1
  }
}
```

### YAML Format

Human-readable structured output:

```yaml
tasks:
  - id: 1
    title: "Implement user auth"
    status: pending
    created: 2024-01-15T10:30:00Z
    updated: 2024-01-15T10:30:00Z
    description: "Add user authentication system"
    tags: [auth, backend]
meta:
  total: 3
  pending: 2
  completed: 1
```

## Error Handling

> **Note**: Error handling patterns are standardized in [SPEC_ERROR_HANDLING.md](./SPEC_ERROR_HANDLING.md). This section provides CLI-specific error handling details.

### Exit Codes

Standard exit codes for CLI operations (see [SPEC_ERROR_HANDLING.md](./SPEC_ERROR_HANDLING.md) for complete error handling patterns):

- `0`: Success
- `1`: General error
- `2`: Invalid command or arguments
- `3`: File I/O error
- `4`: Database error
- `5`: Configuration error

### Error Messages

Clear, actionable error messages following the standardized error format:

```
Error 3000: Task with ID '123' not found
Hint: Use 'edda task list' to see available tasks

Error 6001: Invalid query syntax
Details: Expected 'SELECT' keyword at position 5

Error 5002: Permission denied writing to data directory
Hint: Check file permissions or use --data-dir to specify different location
```

## Configuration

> **Note**: Configuration patterns are standardized in [SPEC_CONFIGURATION.md](./SPEC_CONFIGURATION.md). This section provides CLI-specific configuration details.

### Configuration File

Default location: `~/.config/edda/config.toml` (see [SPEC_CONFIGURATION.md](./SPEC_CONFIGURATION.md) for complete configuration patterns)

```toml
[app]
name = "edda"
version = "0.1.0"
data_dir = "~/.local/share/edda"
cache_dir = "~/.local/share/edda/cache"
temp_dir = "~/.local/share/edda/temp"
debug = false
verbose = false

[storage]
backend = "sqlite"
database_path = "~/.local/share/edda/data.db"
file_storage_path = "~/.local/share/edda/files"
connection_pool_size = 10

[cli]
default_format = "text"
color = true
progress = true
timeout = 30
max_concurrent = 4
shell_completion = true

[logging]
level = "info"
console = true
format = "detailed"
structured = true

[ai]
enabled = true

[sync]
enabled = false
interval = 300

[security]
encryption_enabled = false
```

### Environment Variables

Standard environment variables (see [SPEC_CONFIGURATION.md](./SPEC_CONFIGURATION.md) for complete list):

- `EDDA_CONFIG`: Configuration file path
- `EDDA_DATA_DIR`: Data directory path
- `EDDA_FORMAT`: Default output format
- `EDDA_QUIET`: Suppress non-error output
- `EDDA_VERBOSE`: Enable verbose logging

## Shell Integration

### Command Completion

Generate completion scripts for common shells:

```bash
# Bash
edda completions bash > ~/.local/share/bash-completion/completions/edda

# Zsh
edda completions zsh > ~/.zsh/completions/_edda

# Fish
edda completions fish > ~/.config/fish/completions/edda.fish
```

### Aliases and Functions

Common shell aliases and functions:

```bash
# Quick task creation
alias ta='edda task add'

# Quick task listing
alias tl='edda task list'

# Quick state setting
alias ss='edda state set'

# Quick state getting
alias sg='edda state get'
```

## Examples

### Basic Task Management

```bash
# Add a new task
edda task add "Implement user authentication"

# List all tasks
edda task list

# Get task details
edda task get 1

# Mark task as complete
edda task complete 1

# Update task description
edda task update 1 description "Add OAuth2 authentication"
```

### Document Management

```bash
# Add a document
edda doc add README.md --title "Project Documentation"

# List documents
edda doc list

# Get document content
edda doc content 1

# Update document metadata
edda doc update 1 tags "docs,important"
```

### State Management

```bash
# Set state values
edda state set current_user "john.doe"
edda state set session_id "abc123"

# Get state values
edda state get current_user

# List all state
edda state list

# Clear all state
edda state clear
```

### Advanced Queries

```bash
# Find all pending tasks
edda query "SELECT * FROM tasks WHERE status = 'pending'"

# Count tasks by status
edda query "SELECT status, COUNT(*) as count FROM tasks GROUP BY status"

# Find documents with specific tags
edda query "SELECT * FROM documents WHERE 'important' IN tags"

# Join tasks and documents
edda query "SELECT t.title, d.title as doc_title FROM tasks t JOIN documents d ON t.doc_id = d.id"
```

### Automation Examples

```bash
# Get task count in JSON format
edda task list --format json | jq '.meta.total'

# Set state from environment variable
edda state set build_id "$BUILD_ID"

# Create backup and upload
edda system backup | gzip > backup.tar.gz

# Monitor task completion
watch -n 5 'edda task list --format json | jq ".meta.completed"'
```

## Performance Considerations

### Startup Time

- Lazy loading of heavy components
- Minimal initialization overhead
- Efficient configuration parsing

### Memory Usage

- Streaming for large datasets
- Efficient data structures
- Memory pooling for repeated operations

### Query Performance

- Indexed database queries
- Query result caching
- Optimized data access patterns

## Accessibility

### Screen Reader Support

- Semantic output formatting
- Clear error messages
- Descriptive help text

### Keyboard Navigation

- Tab completion support
- Keyboard shortcuts for common operations
- Consistent navigation patterns

### High Contrast Support

- Configurable color schemes
- Support for high contrast terminals
- Clear visual hierarchy

## Future Enhancements

### Planned Features

- **Interactive Mode**: TUI interface for complex operations
- **Plugin System**: Extensible command architecture
- **Remote Sync**: Multi-device synchronization
- **Web Interface**: HTTP API for web integration
- **Mobile Support**: Companion mobile applications

### Integration Points

- **Git Hooks**: Automatic task tracking
- **CI/CD Integration**: Build state management
- **IDE Plugins**: Editor integration
- **API Gateway**: REST API for external tools
