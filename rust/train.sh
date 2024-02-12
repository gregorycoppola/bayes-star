#!/bin/bash

# Assign variable names for better readability
SCENARIO_NAME=$1

# Check if both variables are provided
if [ -z "$SCENARIO_NAME" ] ; then
  echo "usage: ./train <scenario_name>"
  exit 1
fi

# Warning message to the user.
echo "WARNING: This software will clear your Redis database on localhost."
echo "If you have an existing Redis database that you do not want to clear, please STOP now."
echo "By running this software, you agree that you are doing so at your own risk."
echo "Coppola.ai cannot be held responsible for any loss or damage caused by the use of this software."
echo "Please type 'I understand clear redis' to confirm that you understand the risks and wish to continue, or anything else to quit."

# Wait for user confirmation.
read -p "Type your response here: " user_confirmation

# Check user input.
if [ "$user_confirmation" == "I understand clear redis" ]; then
    # User has typed the confirmation message, proceed with the command.
    RUST_BACKTRACE=1 RUST_LOG=info cargo run --bin train -- --print_training_loss --entities_per_domain=1096 --scenario_name=$SCENARIO_NAME
else
    # User did not type the confirmation message, exit the script.
    echo "User did not confirm. Exiting the script to prevent potential data loss."
    exit 1
fi
