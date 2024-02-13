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
    file_path = sys.argv[1]
    round_number = sys.argv[2]
    max_lines = int(sys.argv[3])
    
    # Generate a dynamic output file path based on the input file name
    base_name = os.path.basename(file_path)
    name_without_ext = os.path.splitext(base_name)[0]
    out_path = f"./output/{name_without_ext}_plot_{max_lines}_round_{round_number}.png"  # Modify this line as needed

    data = read_and_process_file(file_path, max_lines)
    plot_data(data, out_path)

    print(f"Plot saved to {out_path}")

if __name__ == "__main__":
    main()