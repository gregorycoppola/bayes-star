#!/bin/bash

# Assign variable names for better readability
SCENARIO_NAME=$1
TEST_SCENARIO=$2


for i in {1..12}
do
  MARGINAL_OUTPUT_FILE="../rust/data/${SCENARIO_NAME}_${TEST_SCENARIO}_${i}"
  python3 plotmarginals.py $SCENARIO_NAME $TEST_SCENARIO $i
done