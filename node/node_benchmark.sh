#!/bin/bash

# Array of different TOTAL_MEMBERS_EACH_CLASS values
members_values=(10 50 100 150 200)

# Loop through each value and run the Node.js script
for value in "${members_values[@]}"
do
    echo "Running with TOTAL_MEMBERS_EACH_CLASS = $value"
    time node specs/dating1.js $value
done

