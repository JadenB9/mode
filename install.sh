#!/usr/bin/env bash
set -euo pipefail

# Save the original directory
ORIGINAL_DIR="$(pwd)"

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Change to the MODE project directory
cd "$SCRIPT_DIR"

echo "╔════════════════════════════════════════╗"
echo "║   MODE - Terminal Utility Manager     ║"
echo "║           Installation Script          ║"
echo "╚════════════════════════════════════════╝"
echo ""

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Install build dependencies if needed
echo "[CHECK] Checking for build dependencies..."
if ! command_exists cc && ! command_exists gcc && ! command_exists clang; then
    echo "[INSTALL] Installing build dependencies..."
    if command_exists apt-get; then
        sudo apt-get update && sudo apt-get install -y build-essential
    elif command_exists yum; then
        sudo yum groupinstall -y "Development Tools"
    elif command_exists dnf; then
        sudo dnf groupinstall -y "Development Tools"
    elif command_exists pacman; then
        sudo pacman -S --noconfirm base-devel
    else
        echo "[WARNING] Could not detect package manager. Please install build tools manually (gcc/clang)."
    fi
fi
echo "[OK] Build dependencies available"

# Check for Rust toolchain and install if needed
if ! command_exists cargo; then
    echo "[INSTALL] Rust toolchain not found. Installing Rust..."
    echo ""
    echo "This will download and install Rust from https://rustup.rs/"
    echo "Press Enter to continue or Ctrl+C to cancel..."
    read -r

    # Install Rust
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

    # Source cargo env
    if [ -f "$HOME/.cargo/env" ]; then
        source "$HOME/.cargo/env"
    fi

    echo ""
    echo "[OK] Rust installed successfully"
else
    echo "[OK] Rust toolchain found"
fi

# Verify cargo is now available
if ! command_exists cargo; then
    echo "[ERROR] Cargo still not found after installation. Please install Rust manually:"
    echo "        https://rustup.rs/"
    exit 1
fi

# Build release binary
echo "[BUILD] Building MODE in release mode..."
cargo build --release

if [ $? -ne 0 ]; then
    echo "[ERROR] Build failed"
    echo "Please check the error messages above and try again"
    exit 1
fi

echo "[OK] Build successful"

# Determine install location
INSTALL_DIR="${HOME}/.local/bin"
mkdir -p "$INSTALL_DIR"

# Copy binary
echo "[INSTALL] Installing MODE to $INSTALL_DIR..."
cp target/release/mode "$INSTALL_DIR/mode"
chmod +x "$INSTALL_DIR/mode"

echo "[OK] MODE installed to $INSTALL_DIR/mode"

# Add to PATH if not already there
PATH_ADDED=false
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo "[SETUP] Adding $INSTALL_DIR to PATH..."

    # Detect shell and add to appropriate RC file
    CURRENT_SHELL=$(basename "$SHELL")
    if [ "$CURRENT_SHELL" = "bash" ]; then
        RC_FILE="$HOME/.bashrc"
    elif [ "$CURRENT_SHELL" = "zsh" ]; then
        RC_FILE="$HOME/.zshrc"
    else
        RC_FILE="$HOME/.profile"
    fi

    # Add PATH export if not already present
    if ! grep -q "export PATH=\"\$HOME/.local/bin:\$PATH\"" "$RC_FILE" 2>/dev/null; then
        echo "" >> "$RC_FILE"
        echo "# Added by MODE installer" >> "$RC_FILE"
        echo "export PATH=\"\$HOME/.local/bin:\$PATH\"" >> "$RC_FILE"
        PATH_ADDED=true
        echo "[OK] Added $INSTALL_DIR to PATH in $RC_FILE"
    fi

    # Add to current session PATH
    export PATH="$HOME/.local/bin:$PATH"
else
    echo "[OK] $INSTALL_DIR is already in your PATH"
fi

# Setup shell integration
echo "[SETUP] Setting up shell integration..."

# Detect the user's shell
CURRENT_SHELL=$(basename "$SHELL")
RC_FILE=""
WRAPPER_FILE=""

if [ "$CURRENT_SHELL" = "bash" ]; then
    RC_FILE="$HOME/.bashrc"
    WRAPPER_FILE="$(pwd)/mode-wrapper.sh"
elif [ "$CURRENT_SHELL" = "zsh" ]; then
    RC_FILE="$HOME/.zshrc"
    WRAPPER_FILE="$(pwd)/mode-wrapper.zsh"
else
    echo "[WARNING] Unknown shell: $CURRENT_SHELL (expected bash or zsh)"
    echo "[SKIP] Shell integration not configured automatically"
fi

# Install shell integration if we detected the shell
if [ -n "$RC_FILE" ] && [ -f "$WRAPPER_FILE" ]; then
    # Check if shell integration already exists
    if grep -q "# Mode shell integration - auto-reload" "$RC_FILE" 2>/dev/null; then
        echo "[OK] Shell integration already configured in $RC_FILE"
    else
        # Add the wrapper function to the RC file
        echo "" >> "$RC_FILE"
        echo "# Mode shell integration - auto-reload after creating aliases/bookmarks" >> "$RC_FILE"
        cat "$WRAPPER_FILE" >> "$RC_FILE"
        echo "[OK] Shell integration added to $RC_FILE"
    fi
fi

# Return to original directory
cd "$ORIGINAL_DIR"

# Export variables for quick-install.sh to use
export MODE_INSTALL_COMPLETE=1
export MODE_RC_FILE="${RC_FILE:-$HOME/.bashrc}"

# Only show completion message if run directly (not from quick-install.sh)
if [ -z "$QUICK_INSTALL_MODE" ]; then
    echo ""
    echo "╔════════════════════════════════════════╗"
    echo "║    Installation Complete!              ║"
    echo "╚════════════════════════════════════════╝"
    echo ""
    echo "MODE has been successfully installed!"
    echo ""

    if [ "$PATH_ADDED" = true ] || [ -n "$WRAPPER_FILE" ]; then
        echo "To start using MODE, run:"
        echo ""
        echo "  source $RC_FILE && mode"
        echo ""
    else
        echo "MODE is ready! Run: mode"
    fi
    echo ""
fi
