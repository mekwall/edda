#!/bin/bash

# This script sets up the Edda development environment

set -e

echo "🚀 Setting up Edda development environment..."

# Check if Rust is installed
if ! command -v rustc &> /dev/null; then
    echo "📦 Installing Rust..."

    # Detect OS and install Rust
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        echo "   Installing Rust on Linux..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        echo "   Installing Rust on macOS..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    else
        echo "   Installing Rust using default method..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    fi

    # Add Rust to PATH for current session
    source "$HOME/.cargo/env"
    echo "   Added Rust to PATH for current session"
else
    echo "✅ Rust is already installed"
fi

# Verify Rust installation
if command -v rustc &> /dev/null; then
    echo "✅ Rust installed successfully"
    rustc --version
else
    echo "⚠️  Rust installed but not in PATH. Please restart your terminal."
    exit 1
fi

# Install development tools
echo "📦 Installing development tools..."
cargo install cargo-watch cargo-audit cargo-tarpaulin

# Build the project
echo "🔨 Building the project..."
cargo build

# Run tests
echo "🧪 Running tests..."
cargo test

echo "✅ Setup complete!"
echo ""
echo "🎉 Edda is ready for development!"
echo ""
echo "📋 Next steps:"
echo "  • Run 'cargo build' to build the project"
echo "  • Run 'cargo test' to run tests"
echo "  • Run 'cargo run' to start the application"
echo "  • Run 'cargo fmt' to format code"
echo "  • Run 'cargo clippy' to lint code"
