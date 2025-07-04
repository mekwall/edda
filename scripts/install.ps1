# This script sets up the Edda development environment

param(
    [switch]$SkipRustInstall
)

Write-Host "🚀 Setting up Edda development environment..." -ForegroundColor Green

# Check if Rust is installed
try {
    $null = Get-Command rustc -ErrorAction Stop
    Write-Host "✅ Rust is already installed" -ForegroundColor Green
    rustc --version
} catch {
    if ($SkipRustInstall) {
        Write-Host "❌ Rust is required but not installed. Please install Rust manually:" -ForegroundColor Red
        Write-Host "   Visit: https://rustup.rs/" -ForegroundColor Yellow
        exit 1
    }

    Write-Host "📦 Installing Rust..." -ForegroundColor Yellow

    # Try different installation methods
    $rustInstalled = $false

    # Method 1: Try winget
    try {
        Write-Host "   Trying winget..." -ForegroundColor Gray
        winget install Rustlang.Rust --accept-source-agreements --accept-package-agreements
        $rustInstalled = $true
    } catch {
        Write-Host "   winget failed, trying alternative method..." -ForegroundColor Gray
    }

    # Method 2: Try scoop
    if (-not $rustInstalled) {
        try {
            Write-Host "   Trying scoop..." -ForegroundColor Gray
            scoop install rust
            $rustInstalled = $true
        } catch {
            Write-Host "   scoop failed, trying curl method..." -ForegroundColor Gray
        }
    }

    # Method 3: Try curl (fallback)
    if (-not $rustInstalled) {
        try {
            Write-Host "   Trying curl method..." -ForegroundColor Gray
            Invoke-WebRequest -Uri "https://win.rustup.rs/" -OutFile "rustup-init.exe"
            Start-Process -FilePath ".\rustup-init.exe" -ArgumentList "-y" -Wait
            Remove-Item "rustup-init.exe" -Force
            $rustInstalled = $true
        } catch {
            Write-Host "   curl method failed." -ForegroundColor Gray
        }
    }

    if (-not $rustInstalled) {
        Write-Host "❌ Failed to install Rust automatically. Please install manually:" -ForegroundColor Red
        Write-Host "   Visit: https://rustup.rs/" -ForegroundColor Yellow
        Write-Host "   Or run: winget install Rustlang.Rust" -ForegroundColor Yellow
        Write-Host "   Or run: scoop install rust" -ForegroundColor Yellow
        exit 1
    }
}

# Verify Rust installation
try {
    $null = Get-Command rustc -ErrorAction Stop
    Write-Host "✅ Rust installed successfully" -ForegroundColor Green
} catch {
    Write-Host "⚠️  Rust installed but not in PATH. Please restart your terminal." -ForegroundColor Yellow
    exit 1
}

# Install development tools
Write-Host "📦 Installing development tools..." -ForegroundColor Yellow
cargo install cargo-watch cargo-audit cargo-tarpaulin

# Build the project
Write-Host "🔨 Building the project..." -ForegroundColor Yellow
cargo build

# Run tests
Write-Host "🧪 Running tests..." -ForegroundColor Yellow
cargo test

Write-Host "✅ Setup complete!" -ForegroundColor Green
Write-Host ""
Write-Host "🎉 Edda is ready for development!" -ForegroundColor Green
Write-Host ""
Write-Host "📋 Next steps:" -ForegroundColor White
Write-Host "  • Run 'cargo build' to build the project" -ForegroundColor White
Write-Host "  • Run 'cargo test' to run tests" -ForegroundColor White
Write-Host "  • Run 'cargo run' to start the application" -ForegroundColor White
Write-Host "  • Run 'cargo fmt' to format code" -ForegroundColor White
Write-Host "  • Run 'cargo clippy' to lint code" -ForegroundColor White
