import sys
import matplotlib.pyplot as plt
import re

# Step 1: Read from stdin
lines = sys.stdin.readlines()

# Step 2: Parse the data
loss_values = []
for line in lines:
    match = re.search(r"loss:\s*([0-9.]+)", line)
    if match:
        loss_values.append(float(match.group(1)))

# Step 3: Generate the graph
plt.plot(loss_values)
plt.title('Loss over Time')
plt.xlabel('Iteration')
plt.ylabel('Loss')

# Step 4: Save the graph as an image
plt.savefig('loss_graph.png')  # Or 'loss_graph.jpg' for JPG format
plt.show()

