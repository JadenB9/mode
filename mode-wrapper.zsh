#!/bin/zsh
# Mode wrapper function for automatic shell sourcing
# This file is sourced by install.sh into your ~/.zshrc

mode() {
    local cmd_file="$HOME/.mode_exit_cmd"

    # Remove old command file
    rm -f "$cmd_file"

    # Run mode using the installed binary (finds it in PATH)
    command mode "$@"
    local exit_code=$?

    # Check if mode wrote an exit command
    if [ -f "$cmd_file" ]; then
        local exit_cmd
        exit_cmd=$(cat "$cmd_file")
        rm -f "$cmd_file"

        if [ -n "$exit_cmd" ]; then
            eval "$exit_cmd"
            echo "✓ Shell configuration reloaded"
        fi
    fi

    return $exit_code
}
