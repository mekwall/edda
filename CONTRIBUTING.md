# 🤝 Contributing

We welcome contributions! This document provides guidelines and information for contributing to Edda.

## Development Workflow

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Add tests for new functionality
5. Run the test suite (`cargo test`)
6. Commit your changes (`git commit -m 'feat: add amazing feature'`)
7. Push to the branch (`git push origin feature/amazing-feature`)
8. Open a Pull Request

## Code Quality Standards

### Pre-commit Checks

Before committing, ensure your code meets our quality standards:

- **Formatting**: `cargo fmt` - Ensures code formatting
- **Linting**: `cargo clippy -- -D warnings` - Enforces linting standards
- **Testing**: `cargo test` - Runs the test suite
- **Security**: `cargo audit` - Checks for security vulnerabilities

### Manual Quality Checks

```bash
# Format code
cargo fmt

# Check formatting without making changes
cargo fmt --check

# Run linter
cargo clippy

# Run tests
cargo test

# Check for security vulnerabilities
cargo audit
```

## Development Setup

For detailed development setup instructions, see [docs/DEVELOPER_SETUP.md](docs/DEVELOPER_SETUP.md).

### Prerequisites

- [Rust](https://rustup.rs/) - Rust programming language
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

### Available Development Tools

The project uses several Rust development tools that are installed via cargo:

- **cargo-watch**: Hot reloading during development
- **cargo-audit**: Security vulnerability scanning
- **cargo-tarpaulin**: Code coverage analysis

Install them with: `cargo install cargo-watch cargo-audit cargo-tarpaulin`

## Project Structure

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
├── scripts/            # Installation scripts
└── .cursor/            # Development automation
```

## Code Style Guidelines

- Follow Rust conventions and best practices
- Use meaningful variable and function names
- Add comprehensive documentation for public APIs
- Write tests for new functionality
- Ensure all tests pass before submitting PRs

## Commit Message Format

We follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

- `feat:` for new features
- `fix:` for bug fixes
- `docs:` for documentation changes
- `style:` for formatting changes
- `refactor:` for code refactoring
- `test:` for adding or updating tests
- `chore:` for maintenance tasks

## Pull Request Guidelines

- Provide a clear description of the changes
- Include tests for new functionality
- Ensure all CI checks pass
- Update documentation if needed
- Follow the existing code style

## Getting Help

- **Issues**: [GitHub Issues](https://github.com/mekwall/edda/issues)
- **Discussions**: [GitHub Discussions](https://github.com/mekwall/edda/discussions)
- **Documentation**: [Wiki](https://github.com/mekwall/edda/wiki)

Thank you for contributing to Edda! 🚀
