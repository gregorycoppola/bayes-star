#!/bin/bash

# Assign variable names for better readability
SCENARIO_NAME=$1
TEST_SCENARIO=$2

# Check if both variables are provided
if [ -z "$SCENARIO_NAME" ] || [ -z "$TEST_SCENARIO" ]; then
  echo "Error: Both scenario name and test scenario must be provided."
  exit 1
fi

# Compute MARGINAL_OUTPUT_FILE based on SCENARIO_NAME and TEST_SCENARIO
MARGINAL_OUTPUT_FILE="data/${SCENARIO_NAME}_${TEST_SCENARIO}"

# Proceed with the original command using the variables and add the --marginal_output_file argument
RUST_BACKTRACE=1 RUST_LOG=info cargo run --bin plot -- --print_training_loss --entities_per_domain=1024 --test_example=0 --scenario_name=$SCENARIO_NAME --test_scenario=$TEST_SCENARIO --marginal_output_file=$MARGINAL_OUTPUT_FILE
