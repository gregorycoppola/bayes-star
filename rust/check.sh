#!/bin/bash

# Run cargo check
cargo check

# Check the exit status of the last command (cargo check)
if [ $? -eq 0 ]; then
    # Check if the first argument is "test"
    if [ "$1" == "test" ]; then
        # If cargo check was successful and first argument is "test", run cargo check for tests
        cargo check --tests
    else
        echo "Skipping test checks, as the first argument is not 'test'."
    fi
else
    # If cargo check failed, exit the script
    echo "cargo check failed, aborting script."
    exit 1
fi

