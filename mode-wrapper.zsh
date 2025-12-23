#!/bin/zsh
# Mode wrapper function for automatic shell sourcing
# Add this to your ~/.zshrc to enable seamless alias/bookmark integration

mode() {
    # Run the actual mode binary and capture output
    local mode_output
    mode_output=$("${0:A:h}/target/debug/mode" "$@")
    local exit_code=$?

    # Check if there's an exit command to execute
    if echo "$mode_output" | grep -q "^MODE_EXIT_CMD:"; then
        local exit_cmd
        exit_cmd=$(echo "$mode_output" | grep "^MODE_EXIT_CMD:" | sed 's/^MODE_EXIT_CMD://')

        # Execute the exit command (typically a source command)
        eval "$exit_cmd"
        echo "✓ Shell configuration reloaded"
    fi

    return $exit_code
}
