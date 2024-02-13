import json
import matplotlib.pyplot as plt
import sys
import os

def read_and_process_file(file_path, max_lines=None):
    data = {}
    with open(file_path, 'r') as file:
        for i, line in enumerate(file):
            if max_lines is not None and i >= max_lines:
                break  # Stop reading if max_lines is reached
            print(f"time point {i}")
            json_line = json.loads(line)
            for entry in json_line['entries']:
                condition, probability = entry
                if not "exist" in condition:
                    print(f"\"{condition}\" {probability}")
                    if condition not in data:
                        data[condition] = []
                    data[condition].append(probability)
    return data

def plot_data(data, out_file_path):
    plt.figure(figsize=(10, 6))
    for condition, probabilities in data.items():
        plt.plot(probabilities, label=condition)
    plt.xlabel('Timepoint')
    plt.ylabel('Probability')
    plt.title('Probability of Conditions Over Time')
    plt.legend()
    plt.savefig(out_file_path)  # Save the plot to a file

def main():
    if len(sys.argv) < 2:
        print("Usage: python script.py <file_path> [max_lines]")
        sys.exit(1)
    input_path = sys.argv[1]
    max_lines = int(sys.argv[2])
    out_path = f"{input_path}_plot_{max_lines}.png"
    data = read_and_process_file(input_path, max_lines)
    plot_data(data, out_path)
    print(f"Plot saved to {out_path}")

if __name__ == "__main__":
    main()