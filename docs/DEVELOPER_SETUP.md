# 🛠️ Developer Setup

This document provides detailed instructions for setting up the Edda development environment.

## Prerequisites

- [Rust](https://rustup.rs/) - Rust programming language
- Git

## Building from Source

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

## Available Development Tools

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

## Development Workflow

1. **Setup Environment**: Follow the prerequisites and setup instructions above
2. **Create Feature Branch**: `git checkout -b feature/your-feature-name`
3. **Make Changes**: Implement your feature or fix
4. **Run Tests**: `cargo test` to ensure everything works
5. **Format Code**: `cargo fmt` to ensure consistent formatting
6. **Lint Code**: `cargo clippy` to catch potential issues
7. **Commit**: Your changes are ready for commit
8. **Push**: Push your changes to the remote repository
9. **Create PR**: Submit your pull request

## Debugging

### Enable Debug Logging

```bash
RUST_LOG=debug cargo run -- task list
```

### Run Specific Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture

# Run coverage
cargo tarpaulin
```

## Common Issues

### Build Issues

- Ensure you have the correct Rust version: `rustc --version`
- Clean and rebuild: `cargo clean && cargo build`
- Update dependencies: `cargo update`

### Test Issues

- Run tests in isolation: `cargo test -- --test-threads=1`
- Check for database conflicts: Tests use in-memory SQLite databases
- Verify environment variables: Some tests depend on specific configurations

## IDE Setup

### VS Code

Recommended extensions:

- `rust-analyzer` - Rust language support
- `crates` - Cargo.toml dependency management
- `CodeLLDB` - Debugging support

### Development Commands

```bash
# Build the project
cargo build

# Build in release mode
cargo build --release

# Run the application
cargo run

# Run with specific arguments
cargo run -- task list

# Format code
cargo fmt

# Check code with clippy
cargo clippy

# Run tests
cargo test

# Run tests with coverage
cargo tarpaulin

# Check for security vulnerabilities
cargo audit

# Watch for changes and rebuild
cargo watch -x check -x test
```

## Code Quality

### Formatting

The project uses `rustfmt` for consistent code formatting:

```bash
# Format all code
cargo fmt

# Check formatting without making changes
cargo fmt --check
```

### Linting

The project uses `clippy` for additional linting:

```bash
# Run clippy
cargo clippy

# Run clippy with all warnings as errors
cargo clippy -- -D warnings
```

### Testing

```bash
# Run all tests
cargo test

# Run tests with verbose output
cargo test -- --nocapture

# Run specific test module
cargo test module_name

# Run tests in parallel
cargo test -- --test-threads=4
```
