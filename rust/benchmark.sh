#!/bin/bash

# First, compile the program
cargo build --release

# Array of different entities_per_domain values
entities_values=(10 50 100 150 200 250 300)

# Loop through each value and run the compiled program
for value in "${entities_values[@]}"
do
    echo "Running with entities_per_domain = $value"
    time ./target/release/bayes-star --entities_per_domain=$value
done
