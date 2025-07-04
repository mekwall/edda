# Edda 🗡️

> **AI agent-native CLI for structured task and document management**
>
> **Built by AI, for AI.**

[![Rust](https://img.shields.io/badge/rust-2024-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://github.com/mekwall/edda/workflows/CI/badge.svg)](https://github.com/mekwall/edda/actions)
[![Crates.io](https://img.shields.io/crates/v/edda)](https://crates.io/crates/edda)

**Edda** is a fast, minimal CLI tool built in Rust for managing tasks and documents with advanced AI agent integration capabilities. Inspired by the Norse sagas, it acts as an operational memory—structured, queryable, and built for automation.

Whether you're tracking goals, maintaining agent context, managing dynamic state, or coordinating multiple AI agents, Edda brings order and clarity to your agents' world.

## ✨ Features

### 🎯 **MVP Features** (Current)

- **📋 Task Management**: Taskwarrior-compatible task operations

  - Create, list, modify, and complete tasks
  - Rich metadata support (priority, due dates, tags, projects)
  - Time tracking and annotations
  - Task relationships and dependencies

- **🗄️ Local-First Storage**: SQLite-based persistent storage

  - Offline-first architecture
  - Automatic data persistence and backup
  - Efficient indexing and querying
  - Data integrity and validation

- **🔄 GitHub Sync**: Bi-directional synchronization with GitHub Issues

  - Seamless integration with existing GitHub workflows
  - Conflict detection and resolution
  - Real-time sync capabilities

- **🔍 Query Engine**: Powerful data retrieval and filtering

  - SQL-like syntax for complex queries
  - Cross-entity joins and aggregations
  - Multiple output formats (text, JSON, YAML)

- **⚙️ System Management**: Comprehensive system administration
  - Configuration management
  - Backup and restore operations
  - System status monitoring
  - Data cleanup and maintenance

### 🚀 **Future Plans**

- **🤖 AI Agent Integration**: Native AI agent communication protocols
- **🔐 Multi-Agent Coordination**: Team management and consensus building
- **🛡️ Safety Framework**: Runtime safety enforcement and information flow control
- **📚 Document Management**: Rich document storage with versioning and metadata
- **🌐 Plugin Ecosystem**: Extensible sync with GitLab, JIRA, and other platforms
- **📊 Advanced Analytics**: Task analytics and performance insights
- **🔗 API Layer**: RESTful API for programmatic access
- **🌍 Multi-Platform Sync**: Cross-device synchronization and collaboration

## 🚀 Quick Start

### Installation

```bash
# From source (recommended)
git clone https://github.com/mekwall/edda.git
cd edda
cargo install --path .

# Or via cargo (when published)
cargo install edda
```

### First Steps

```bash
# Initialize Edda
edda system init

# Create your first task
edda task add "Implement user authentication system"

# List all tasks
edda task list

# Get detailed task information
edda task 1 info

# Mark task as complete
edda task 1 done
```

### GitHub Integration

```bash
# Configure GitHub sync
edda sync github configure

# Sync with GitHub Issues
edda sync github pull
edda sync github push
```

## 📖 Usage Examples

### Task Management

```bash
# Create tasks with metadata
edda task add "Write API documentation" --priority high --project docs
edda task add "Fix login bug" --due 2024-02-01 --tag bug

# List tasks with filtering
edda task list --status pending
edda task list --project docs
edda task list --tag bug

# Modify tasks
edda task 1 modify priority low
edda task 1 modify due 2024-02-15

# Time tracking
edda task 1 start
# ... work on task ...
edda task 1 stop

# Add annotations
edda task 1 annotate "Found related issue in auth module"
```

### Query Engine

```bash
# Simple queries
edda query "SELECT * FROM tasks WHERE status = 'pending'"

# Complex queries with joins
edda query "SELECT t.title, d.content FROM tasks t JOIN documents d ON t.id = d.task_id"

# Aggregations
edda query "SELECT project, COUNT(*) as count FROM tasks GROUP BY project"

# Output in different formats
edda query "SELECT * FROM tasks" --format json
edda query "SELECT * FROM tasks" --format yaml
```

### System Management

```bash
# Check system status
edda system status

# Create backup
edda system backup

# Restore from backup
edda system restore backup-2024-01-15

# View configuration
edda system config
```

## 🏗️ Architecture

Edda follows a modular CLI architecture with clear separation of concerns:

```
┌───────────────────────────────────────────────────────────┐
│                    CLI Interface Layer                    │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────────────┐  │
│  │   Commands  │ │   Arguments │ │   Output Formatting │  │
│  │   & Flags   │ │   & Options │ │   & Display         │  │
│  └─────────────┘ └─────────────┘ └─────────────────────┘  │
└───────────────────────────────────────────────────────────┘
┌───────────────────────────────────────────────────────────┐
│                  Core Engine Layer                        │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────────────┐  │
│  │   Memory    │ │   Query     │ │   State Management  │  │
│  │   Manager   │ │   Engine    │ │   & Persistence     │  │
│  └─────────────┘ └─────────────┘ └─────────────────────┘  │
└───────────────────────────────────────────────────────────┘
┌───────────────────────────────────────────────────────────┐
│                  Data Storage Layer                       │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────────────┐  │
│  │   Local     │ │   Remote    │ │   Cache & Index     │  │
│  │   Storage   │ │   Sync      │ │   Management        │  │
│  └─────────────┘ └─────────────┘ └─────────────────────┘  │
└───────────────────────────────────────────────────────────┘
```

## 🛠️ Development

### Prerequisites

- Rust 1.75+ (2024 edition)
- SQLite development headers
- Git

### Building from Source

```bash
# Clone the repository
git clone https://github.com/mekwall/edda.git
cd edda

# Build the project
cargo build

# Run tests
cargo test

# Run with debug logging
RUST_LOG=debug cargo run -- task list
```

### Project Structure

```
edda/
├── src/
│   ├── cli.rs          # Command-line interface
│   ├── core/           # Core business logic
│   ├── storage/        # Data persistence layer
│   ├── sync/           # Synchronization engine
│   └── main.rs         # Application entry point
├── tests/              # Integration tests
├── docs/               # Documentation
└── .cursor/            # Development automation
```

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Workflow

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Add tests for new functionality
5. Run the test suite (`cargo test`)
6. Commit your changes (`git commit -m 'feat: add amazing feature'`)
7. Push to the branch (`git push origin feature/amazing-feature`)
8. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **Taskwarrior**: For the excellent task management concepts and compatibility
- **Rust Community**: For the amazing ecosystem and tooling
- **AI Agent Community**: For inspiration and feedback on agent-native design

## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/mekwall/edda/issues)
- **Discussions**: [GitHub Discussions](https://github.com/mekwall/edda/discussions)
- **Documentation**: [Wiki](https://github.com/mekwall/edda/wiki)

---

**Built with ❤️ by [Marcus Ekwall](https://github.com/mekwall) and his AI minions**

_Inspired by the Norse sagas, where Edda was the source of wisdom and knowledge._
