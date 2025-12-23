#!/usr/bin/env bash
# MODE Quick Installer
# Usage: curl -sSL https://raw.githubusercontent.com/JadenB9/mode/main/quick-install.sh | bash

set -euo pipefail

echo "╔════════════════════════════════════════╗"
echo "║   MODE - Quick Installer              ║"
echo "╚════════════════════════════════════════╝"
echo ""

# Create temporary directory
TEMP_DIR=$(mktemp -d)
cd "$TEMP_DIR"

echo "[DOWNLOAD] Cloning MODE repository..."
git clone https://github.com/JadenB9/mode.git 2>&1 | grep -v "Cloning into" || true

echo "[INSTALL] Running installation..."
cd mode

# Set flag so install.sh knows it's being called from quick-install
export QUICK_INSTALL_MODE=1

# Run the installer
bash ./install.sh

# Cleanup
echo ""
echo "[CLEANUP] Removing temporary files..."
cd ~
rm -rf "$TEMP_DIR"

echo ""
echo "╔════════════════════════════════════════╗"
echo "║   Installation Complete!               ║"
echo "╚════════════════════════════════════════╝"
echo ""

# Detect shell RC file
if [ -n "${MODE_RC_FILE:-}" ]; then
    RC_FILE="$MODE_RC_FILE"
else
    CURRENT_SHELL=$(basename "$SHELL")
    if [ "$CURRENT_SHELL" = "bash" ]; then
        RC_FILE="$HOME/.bashrc"
    elif [ "$CURRENT_SHELL" = "zsh" ]; then
        RC_FILE="$HOME/.zshrc"
    else
        RC_FILE="$HOME/.bashrc"
    fi
fi

# Auto-reload shell config and launch mode
echo "Launching MODE..."
echo ""

# Start a new shell that sources the RC file and runs mode
if [ -f "$RC_FILE" ]; then
    exec $SHELL -c "source '$RC_FILE' && mode"
else
    exec mode
fi
