#!/usr/bin/env bash
set -euo pipefail

echo "╔════════════════════════════════════════╗"
echo "║   MODE - Terminal Utility Manager     ║"
echo "║           Installation Script          ║"
echo "╚════════════════════════════════════════╝"
echo ""

# Check for Rust toolchain
if ! command -v cargo &> /dev/null; then
    echo "❌ Error: Rust toolchain not found"
    echo ""
    echo "Please install Rust from: https://rustup.rs/"
    echo "Run: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

echo "✓ Rust toolchain found"

# Build release binary
echo "🔨 Building MODE in release mode..."
cargo build --release

if [ $? -ne 0 ]; then
    echo "❌ Build failed"
    exit 1
fi

echo "✓ Build successful"

# Determine install location
INSTALL_DIR="${HOME}/.local/bin"
mkdir -p "$INSTALL_DIR"

# Copy binary
echo "📦 Installing MODE to $INSTALL_DIR..."
cp target/release/mode "$INSTALL_DIR/mode"
chmod +x "$INSTALL_DIR/mode"

echo "✓ MODE installed to $INSTALL_DIR/mode"

# Check if in PATH
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo ""
    echo "⚠️  WARNING: $INSTALL_DIR is not in your PATH"
    echo ""
    echo "Add this to your ~/.bashrc or ~/.zshrc:"
    echo "    export PATH=\"\$HOME/.local/bin:\$PATH\""
    echo ""
    echo "Then reload your shell or run:"
    echo "    source ~/.bashrc  # or source ~/.zshrc"
else
    echo "✓ $INSTALL_DIR is in your PATH"
fi

echo ""
echo "╔════════════════════════════════════════╗"
echo "║    Installation Complete! 🎉           ║"
echo "╚════════════════════════════════════════╝"
echo ""
echo "Run 'mode' to start the application"
echo ""
