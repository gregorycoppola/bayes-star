#!/bin/bash

# Run cargo check
cargo check

# Check the exit status of the last command (cargo check)
if [ $? -eq 0 ]; then
    # If cargo check was successful, run cargo check for tests
    cargo check --tests
else
    # If cargo check failed, exit the script
    echo "cargo check failed, aborting script."
    exit 1
fi

