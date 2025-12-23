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
git clone https://github.com/JadenB9/mode.git

echo "[INSTALL] Running installation..."
cd mode
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
echo "Reload your shell and run 'mode':"
echo ""
echo "  source ~/.bashrc"
echo "  mode"
echo ""
echo "Or open a new terminal and type: mode"
echo ""
