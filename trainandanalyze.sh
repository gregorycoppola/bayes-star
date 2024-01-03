#!/bin/bash

# Step 1: Build the Rust project
cd rust
cargo build
cd ..

# Step 2 and 3: Run the main Rust program and pipe its output to the Python script
./rust/target/debug/bayes-star | python3 python/analyze.py
