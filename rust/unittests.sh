#!/bin/bash

# Check if an argument (test name) is provided
if [ "$#" -eq 1 ]; then
    # Run only the specified test
    cargo test $1 -- --test-threads=1 --nocapture
else
    # Run all tests
    cargo test -- --test-threads=1 --nocapture
fi

