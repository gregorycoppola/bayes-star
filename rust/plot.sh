#!/bin/bash

ROOT_DATA=$1
SCENARIO_NAME=$2
TEST_SCENARIO=$3
NUM_ITERATIONS_TO_PLOT=$4

if [ -z "$ROOT_DATA" ] || [ -z "$SCENARIO_NAME" ] || [ -z "$TEST_SCENARIO" ] || [ -z "$NUM_ITERATIONS_TO_PLOT" ]; then
  echo "Error: ROOT_DATA directory, scenario name, and test scenario must be provided."
  echo "usage: ./plot.sh <ROOT_DATA> <SCENARIO_NAME> <TEST_SCENARIO> <NUM_ITERATIONS_TO_PLOT>"
  exit 1
fi

# Compute MARGINAL_OUTPUT_FILE based on SCENARIO_NAME and TEST_SCENARIO
MARGINAL_OUTPUT_FILE="${ROOT_DATA}/${SCENARIO_NAME}_${TEST_SCENARIO}"

# Proceed with the original command using the variables and add the --marginal_output_file argument
RUST_BACKTRACE=1 RUST_LOG=info cargo run --bin plot -- --print_training_loss --entities_per_domain=1024 --test_example=0 --scenario_name=$SCENARIO_NAME --test_scenario=$TEST_SCENARIO --marginal_output_file=$MARGINAL_OUTPUT_FILE

# TODO: replace with javascript
# python3 ../python/plotmarginals.py $MARGINAL_OUTPUT_FILE $NUM_ITERATIONS_TO_PLOT