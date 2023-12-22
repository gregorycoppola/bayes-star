import sys
import re
import matplotlib.pyplot as plt
import random

# Step 1: Read from stdin and process each line
loss_values = []
for line in sys.stdin:
    print(line.strip())
    match = re.search(r'"loss":\s*([0-9.]+)', line)
    if match:
        # Convert the value to double and add it to the result list
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

