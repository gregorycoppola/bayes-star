#!/bin/bash

# Check if a file name is provided as an argument
if [ $# -eq 0 ]; then
    echo "Usage: $0 <filename>"
    exit 1
fi

# Filename is the first argument
filename=$1

# Use grep to print lines that contain non-whitespace characters
grep -v '^[[:space:]]*$' "$filename"
