import sys
import re
import matplotlib.pyplot as plt
import random

# Step 1: Read from stdin and process each line
loss_values = []
for line in sys.stdin:
    # Step 2: Randomly decide whether to print the line
    if random.randint(1, 100) == 1:
        print(line, end='')

    # Check if the line contains 'loss:'
    if "loss:" in line:
        # Parse the loss value
        match = re.search(r"loss:\s*([0-9.]+)", line)
        if match:
            loss_values.append(float(match.group(1)))

# Step 3: Generate the graph only if there are loss values
if loss_values:
    plt.plot(loss_values)
    plt.title('Loss over Time')
    plt.xlabel('Iteration')
    plt.ylabel('Loss')

    # Step 4: Save the graph as an image
    plt.savefig('loss_graph.png')  # Or 'loss_graph.jpg' for JPG format
    plt.show()
else:
    print("No loss data found.")

