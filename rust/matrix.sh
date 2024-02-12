#!/bin/bash

# Assign variable names for better readability
SCENARIO_NAME=$1
TEST_SCENARIO=$2


for i in {1..12}
do
  ./fullpipe.sh $SCENARIO_NAME $TEST_SCENARIO $i
done