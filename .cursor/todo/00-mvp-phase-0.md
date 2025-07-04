# MVP Phase 0 - TODO

## Overview

MVP Phase 0 focuses on simple task management and GitHub sync to enable internal dogfooding. When these requirements are met, Edda will be used internally to track development.

## Core MVP Requirements

- [x] **TASK-001**: Project setup and foundation (Rust project, dependencies, basic structure)
- [x] **TASK-002**: Taskwarrior-compatible task management (create, list, modify, done, delete)
- [x] **TASK-003**: Local-first architecture with SQLite storage
- [x] **TASK-004**: Basic client-server sync protocol
- [x] **TASK-005**: Sync with GitHub Issues (bi-directional, minimal viable integration)
  - Complete GitHub integration with bi-directional sync, field mapping, and CLI commands implemented
- [ ] **TASK-006**: Foundation for extensible sync (GitLab, JIRA, etc. to follow)
- [ ] **TASK-007**: Taskwarrior data import/export compatibility
- [x] **TASK-008**: Recursive project config search for `.edda.toml` (search up from CWD, fallback to home config)
  - Edda now searches recursively for `.edda.toml` from the current directory up to the home directory, falling back to the home config if not found.
- [x] **TASK-009**: `edda init` creates a default `.edda.toml` in the current directory if none exists
  - On initialization, Edda will create a project-local config file with sensible defaults if not present.

## Project Setup and Foundation

### TASK-001: Project Setup and Foundation

#### TASK-001-001: Rust Project Initialization

- [x] Initialize new Rust project with `cargo new edda`
- [x] Set up Cargo.toml with project metadata
- [x] Configure Rust edition (2024)
- [x] Set up workspace structure for future components
- [x] Add .gitignore for Rust projects
  - Commit: e584dec

#### TASK-001-002: Core Dependencies

- [x] Add clap for command-line argument parsing
- [x] Add serde for serialization/deserialization
- [x] Add tokio for async runtime
- [x] Add sqlx for database operations
- [x] Add chrono for date/time handling
- [x] Add uuid for unique identifiers
- [x] Add anyhow for error handling
- [x] Add tracing for logging
  - Commit: b0343aa

#### TASK-001-003: Project Structure

- [x] Create src/ directory structure
- [x] Set up module organization (cli, core, storage, sync)
- [x] Create basic error types
- [x] Set up configuration management
- [x] Create basic logging setup
  - Commit: ce05d28

#### TASK-001-004: Basic CLI Framework

- [x] Set up clap command structure
- [x] Implement basic help system
- [x] Add global options (--config, --data-dir, --format, --quiet, --verbose)
- [x] Create placeholder for task subcommands
- [x] Add basic error handling and exit codes
  - Commit: ce05d28

#### TASK-001-005: Configuration System

- [x] Define configuration file format (TOML)
- [x] Implement configuration loading from file
- [x] Add environment variable support
- [x] Create default configuration
- [x] Add configuration validation
  - Commit: ce05d28

#### TASK-001-006: Data Directory Setup

- [x] Create data directory structure
- [x] Set up database initialization
- [x] Create configuration directory
- [x] Set up log directory
- [x] Add data directory permissions handling
  - Commit: f4c8538

#### TASK-001-007: Testing Infrastructure

- [x] Add comprehensive tests for error types
- [x] Add tests for configuration management
- [x] Add tests for storage and database initialization
- [x] Fix Rust 2024 edition compatibility (unsafe env var calls)
- [x] Use in-memory SQLite databases for reliable testing
- [x] All 21 tests passing
  - Commit: 8f2e1a3

## Task Management Implementation

### TASK-002: Taskwarrior-Compatible Task Management

#### TASK-002-001: Core Task Data Model

- [x] Define Task struct with Taskwarrior-compatible fields (id, uuid, description, status, priority, dates, project, tags, annotations)
- [x] Implement TaskStatus enum (pending, completed, deleted, waiting) - matching Taskwarrior
- [x] Implement Priority enum (H, M, L, 0-9) - matching Taskwarrior
- [x] Add Taskwarrior date fields (due, scheduled, start, end, entry, modified)
- [x] Add serialization/deserialization support (serde)
  - Commit: [latest]

#### TASK-002-002: Task Storage Engine

- [x] Implement SQLite database schema for tasks table
- [x] Create TaskStorage trait and SQLite implementation
- [x] Add basic CRUD operations (create, read, update, delete)
- [x] Implement task ID generation (UUID-based)
  - Commit: [latest]

#### TASK-002-003: Task Manager Core

- [x] Implement TaskEngine struct
- [x] Add task validation logic
- [x] Implement task lifecycle management
- [x] Add task relationship support (parent/child tasks)
  - Commit: [latest]

#### TASK-002-004: CLI Task Commands

- [x] Implement `edda task add <description>` command (Taskwarrior: `task add`)
- [x] Implement `edda task list` command with Taskwarrior-compatible filtering
- [x] Implement `edda task <id> info` command (Taskwarrior: `task <id> info`)
- [x] Implement `edda task <id> modify <field> <value>` command (Taskwarrior: `task <id> modify`)
- [x] Implement `edda task <id> done` command (Taskwarrior: `task <id> done`)
- [x] Implement `edda task <id> delete` command (Taskwarrior: `task <id> delete`)
- [x] Implement `edda task <id> start` command (Taskwarrior: `task <id> start`)
- [x] Implement `edda task <id> stop` command (Taskwarrior: `task <id> stop`)
- [x] Implement `edda task <id> annotate <note>` command (Taskwarrior: `task <id> annotate`)
- [x] Implement `edda task <id> +tag` and `edda task <id> -tag` commands
  - Commit: 08a1b02
  - Note: All CLI task commands are implemented and working.

#### TASK-002-005: Task Output Formatting

- [x] Implement text output format for human readability
- [x] Implement JSON output format for automation
- [x] Add basic table formatting for task lists
- [x] Support --format flag for output selection
  - Commit: ed30684
  - Note: Both text and JSON output are supported for task list and get commands.

#### TASK-002-006: Task Validation

- [x] Validate task title (non-empty)
- [x] Validate task status transitions
- [x] Validate priority values
- [x] Add helpful error messages for validation failures
  - Commit: ed30684
  - Note: Validation for title, status transitions, priority, and error messages is enforced and tested.

## Local Storage Implementation

### TASK-003: Local-First Architecture

#### TASK-003-001: SQLite Storage Engine

- [x] Implement SQLite database schema for all data types
- [x] Create StorageEngine trait and SQLite implementation
- [x] Add data versioning and migration support
- [x] Implement efficient indexing for queries
- [x] Add data integrity checks and constraints
  - Commit: cbf2ffa
  - Note: Database schema includes tasks, documents, state tables with constraints, indexes, and migration support.

#### TASK-003-002: Data Persistence

- [x] Implement automatic data persistence
- [x] Add data backup and recovery mechanisms
- [x] Implement data compression for large datasets
- [x] Add data validation and sanitization
  - Commit: 27fb33a
  - Note: Backup, recovery, compression, and data validation are implemented.

#### TASK-003-003: Offline Support

- [x] Implement offline-first architecture
- [x] Add local data caching
- [x] Implement conflict resolution for sync
- [x] Add offline queue for pending operations
  - Commit: 14b0a52
  - Note: Offline-first architecture, local caching, conflict resolution, and offline operation queue are implemented.

## Client-Server Sync Implementation

### TASK-004: Basic Client-Server Protocol

#### TASK-004-001: Sync Protocol Design

- [x] Define client-server communication protocol
- [x] Implement authentication and session management
- [x] Add change tracking and versioning
- [x] Implement conflict detection and resolution

## GitHub Sync Implementation

### TASK-005: GitHub Issues Sync

#### TASK-005-001: GitHub API Integration

- [x] Add GitHub API client dependency (octocrab or similar)
- [x] Implement GitHub authentication (OAuth or personal access token)
- [x] Create GitHub API wrapper for Issues
- [x] Add configuration for GitHub repository and authentication
  - Commit: [latest]

#### TASK-005-002: Sync Provider Interface

- [x] Define SyncProvider trait with standard methods
- [x] Implement GitHubSyncProvider struct
- [x] Add bi-directional sync methods (pull, push)
- [x] Implement conflict resolution strategy
  - Commit: [latest]

#### TASK-005-003: Field Mapping

- [x] Map Edda task title to GitHub issue title
- [x] Map Edda task description to GitHub issue body
- [x] Map Edda task status to GitHub issue state (open/closed)
- [x] Map Edda task priority to GitHub issue labels
- [x] Handle GitHub issue comments (if feasible for MVP)
  - Commit: [latest]

#### TASK-005-004: Sync Commands

- [x] Implement `edda sync github pull` command
- [x] Implement `edda sync github push` command
- [x] Implement `edda sync github status` command
- [x] Add sync configuration commands
  - Commit: [latest]

#### TASK-005-005: Sync Configuration

- [x] Add GitHub configuration to EddaConfig
- [x] Support repository selection
- [x] Support authentication token configuration
- [x] Add sync interval configuration
  - Commit: [latest]

#### TASK-005-006: Error Handling

- [x] Handle GitHub API rate limiting
- [x] Handle authentication failures
- [x] Handle network connectivity issues
- [x] Provide clear error messages for sync failures
  - Commit: [latest]

## Taskwarrior Compatibility

### TASK-007: Taskwarrior Data Import/Export

#### TASK-007-001: Taskwarrior Data Format

- [ ] Implement Taskwarrior data format parser
- [ ] Add Taskwarrior data format serializer
- [ ] Support Taskwarrior configuration file format
- [ ] Implement Taskwarrior data validation

#### TASK-007-002: Import/Export Commands

- [ ] Implement `edda import taskwarrior <file>` command
- [ ] Implement `edda export taskwarrior <file>` command
- [ ] Add Taskwarrior data migration tools
- [ ] Implement Taskwarrior compatibility mode

#### TASK-007-003: Data Migration

- [ ] Create Taskwarrior to Edda data converter
- [ ] Add Edda to Taskwarrior data converter
- [ ] Implement migration validation
- [ ] Add migration rollback capabilities

## Foundation for Extensibility

### TASK-006: Extensible Sync Architecture

#### TASK-006-001: Provider Plugin System

- [ ] Define SyncProvider trait with standard interface
- [ ] Create provider registration system
- [ ] Add provider configuration management
- [ ] Implement provider discovery mechanism

#### TASK-006-002: Configuration Extensibility

- [ ] Design extensible configuration structure for providers
- [ ] Add provider-specific configuration sections
- [ ] Implement configuration validation for providers
- [ ] Add configuration migration support

#### TASK-006-003: Authentication Framework

- [ ] Define authentication interface for providers
- [ ] Implement OAuth support for GitHub
- [ ] Add token-based authentication support
- [ ] Create secure credential storage

## Testing and Quality Assurance

### TASK-008: MVP Testing

#### TASK-008-001: Unit Tests

- [ ] Write unit tests for TaskEngine
- [ ] Write unit tests for GitHubSyncProvider
- [ ] Write unit tests for CLI commands
- [ ] Add test coverage reporting

#### TASK-008-002: Integration Tests

- [x] Test task CRUD operations end-to-end
- [x] Test GitHub sync with mock API
- [x] Test CLI command integration
- [x] Test error handling scenarios
  - Commit: 4f7039b

#### TASK-008-003: Manual Testing

- [ ] Test task management workflow
- [ ] Test GitHub sync with real repository
- [ ] Test CLI usability and error messages
- [ ] Validate output formats

## Documentation

### TASK-009: MVP Documentation

#### TASK-009-001: User Documentation

- [ ] Write basic usage guide for task management
- [ ] Write GitHub sync setup guide
- [ ] Document CLI commands and options
- [ ] Add troubleshooting section

#### TASK-009-002: Developer Documentation

- [ ] Document task management API
- [ ] Document sync provider interface
- [ ] Add code examples and usage patterns
- [ ] Document configuration options

## Deployment and Distribution

### TASK-010: MVP Distribution

#### TASK-010-001: Build System

- [ ] Set up Cargo build configuration
- [ ] Add release build optimization
- [ ] Configure cross-platform compilation
- [ ] Add version management

#### TASK-010-002: Packaging

- [ ] Create binary distribution packages
- [ ] Add installation scripts
- [ ] Configure shell completion generation
- [ ] Add man page generation

## Success Criteria

### Internal Dogfooding Milestone

- [ ] **MILESTONE-001**: Edda project is set up with basic Rust structure and dependencies
- [ ] **MILESTONE-002**: Edda can manage tasks with Taskwarrior-compatible commands
- [ ] **MILESTONE-003**: Edda has local-first architecture with offline support
- [ ] **MILESTONE-004**: Edda can sync with centralized server across devices
- [ ] **MILESTONE-005**: Edda can sync tasks with GitHub Issues
- [ ] **MILESTONE-006**: Edda can import/export Taskwarrior data
- [ ] **MILESTONE-007**: Edda is ready for internal development tracking

### Quality Gates

- [ ] All unit tests pass
- [ ] All integration tests pass
- [ ] Manual testing completed successfully
- [ ] Documentation is complete and accurate
- [ ] Binary distribution works on target platforms

## Notes

- Focus on simplicity and reliability for MVP
- GitHub sync should be production-ready for internal use
- Extensibility should be designed but not fully implemented
- Error handling should be comprehensive and user-friendly
- Performance should be acceptable for internal development use
