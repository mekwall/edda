# MVP Phase 0 - TODO

## Overview

MVP Phase 0 focuses on simple task management and GitHub sync to enable internal dogfooding. When these requirements are met, Edda will be used internally to track development.

## Core MVP Requirements

- [ ] **TASK-001**: Project setup and foundation (Rust project, dependencies, basic structure)
- [ ] **TASK-002**: Taskwarrior-compatible task management (create, list, modify, done, delete)
- [ ] **TASK-003**: Local-first architecture with SQLite storage
- [ ] **TASK-004**: Basic client-server sync protocol
- [ ] **TASK-005**: Sync with GitHub Issues (bi-directional, minimal viable integration)
- [ ] **TASK-006**: Foundation for extensible sync (GitLab, JIRA, etc. to follow)
- [ ] **TASK-007**: Taskwarrior data import/export compatibility

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

- [ ] Implement `edda task add <description>` command (Taskwarrior: `task add`)
- [ ] Implement `edda task list` command with Taskwarrior-compatible filtering
- [ ] Implement `edda task <id> info` command (Taskwarrior: `task <id> info`)
- [ ] Implement `edda task <id> modify <field> <value>` command (Taskwarrior: `task <id> modify`)
- [ ] Implement `edda task <id> done` command (Taskwarrior: `task <id> done`)
- [ ] Implement `edda task <id> delete` command (Taskwarrior: `task <id> delete`)
- [x] Implement `edda task <id> start` command (Taskwarrior: `task <id> start`)
- [x] Implement `edda task <id> stop` command (Taskwarrior: `task <id> stop`)
- [x] Implement `edda task <id> annotate <note>` command (Taskwarrior: `task <id> annotate`)
- [x] Implement `edda task <id> +tag` and `edda task <id> -tag` commands
  - Commit: <pending-commit-hash>

#### TASK-002-005: Task Output Formatting

- [ ] Implement text output format for human readability
- [ ] Implement JSON output format for automation
- [ ] Add basic table formatting for task lists
- [ ] Support --format flag for output selection

#### TASK-002-006: Task Validation

- [ ] Validate task title (non-empty)
- [ ] Validate task status transitions
- [ ] Validate priority values
- [ ] Add helpful error messages for validation failures

## Local Storage Implementation

### TASK-003: Local-First Architecture

#### TASK-003-001: SQLite Storage Engine

- [ ] Implement SQLite database schema for all data types
- [ ] Create StorageEngine trait and SQLite implementation
- [ ] Add data versioning and migration support
- [ ] Implement efficient indexing for queries
- [ ] Add data integrity checks and constraints

#### TASK-003-002: Data Persistence

- [ ] Implement automatic data persistence
- [ ] Add data backup and recovery mechanisms
- [ ] Implement data compression for large datasets
- [ ] Add data validation and sanitization

#### TASK-003-003: Offline Support

- [ ] Ensure all operations work offline
- [ ] Implement change tracking for offline operations
- [ ] Add conflict detection for offline changes
- [ ] Implement eventual consistency model

## Client-Server Sync Implementation

### TASK-004: Basic Client-Server Protocol

#### TASK-004-001: Sync Protocol Design

- [ ] Define client-server communication protocol
- [ ] Implement authentication and session management
- [ ] Add change tracking and versioning
- [ ] Implement conflict detection and resolution

#### TASK-004-002: Client Sync Engine

- [ ] Implement client-side sync manager
- [ ] Add background sync capabilities
- [ ] Implement change batching and optimization
- [ ] Add sync status monitoring

#### TASK-004-003: Server Sync Engine

- [ ] Implement server-side sync coordinator
- [ ] Add user management and authentication
- [ ] Implement multi-device support
- [ ] Add real-time sync via WebSockets

## GitHub Sync Implementation

### TASK-005: GitHub Issues Sync

#### TASK-005-001: GitHub API Integration

- [ ] Add GitHub API client dependency (octocrab or similar)
- [ ] Implement GitHub authentication (OAuth or personal access token)
- [ ] Create GitHub API wrapper for Issues
- [ ] Add configuration for GitHub repository and authentication

#### TASK-005-002: Sync Provider Interface

- [ ] Define SyncProvider trait with standard methods
- [ ] Implement GitHubSyncProvider struct
- [ ] Add bi-directional sync methods (pull, push)
- [ ] Implement conflict resolution strategy

#### TASK-005-003: Field Mapping

- [ ] Map Edda task title to GitHub issue title
- [ ] Map Edda task description to GitHub issue body
- [ ] Map Edda task status to GitHub issue state (open/closed)
- [ ] Map Edda task priority to GitHub issue labels
- [ ] Handle GitHub issue comments (if feasible for MVP)

#### TASK-005-004: Sync Commands

- [ ] Implement `edda sync github pull` command
- [ ] Implement `edda sync github push` command
- [ ] Implement `edda sync github status` command
- [ ] Add sync configuration commands

#### TASK-005-005: Sync Configuration

- [ ] Add GitHub configuration to EddaConfig
- [ ] Support repository selection
- [ ] Support authentication token configuration
- [ ] Add sync interval configuration

#### TASK-005-006: Error Handling

- [ ] Handle GitHub API rate limiting
- [ ] Handle authentication failures
- [ ] Handle network connectivity issues
- [ ] Provide clear error messages for sync failures

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
