#!/bin/bash

# Assign variable names for better readability
SCENARIO_NAME=$1

# Check if the scenario name is provided
if [ -z "$SCENARIO_NAME" ] ; then
  echo "usage: ./train.sh <SCENARIO_NAME>"
  exit 1
fi

# User has typed the confirmation message or BAYES_STAR_CAN_CLEAR_REDIS is set to 1, proceed with the command.
RUST_BACKTRACE=1 RUST_LOG=info cargo run --bin train -- --print_training_loss --entities_per_domain=4096 --scenario_name=$SCENARIO_NAME
