#!/usr/bin/env bash
set -euo pipefail

echo "╔════════════════════════════════════════╗"
echo "║   MODE - Terminal Utility Manager     ║"
echo "║          Uninstall Script              ║"
echo "╚════════════════════════════════════════╝"
echo ""

INSTALL_DIR="${HOME}/.local/bin"
MODE_BINARY="$INSTALL_DIR/mode"

if [ -f "$MODE_BINARY" ]; then
    echo "🗑️  Removing MODE from $MODE_BINARY..."
    rm -f "$MODE_BINARY"
    echo "✓ MODE uninstalled successfully"
else
    echo "ℹ️  MODE is not installed at $MODE_BINARY"
fi

echo ""
echo "Uninstallation complete."
echo ""
