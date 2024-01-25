import json
import matplotlib.pyplot as plt
import sys

def read_and_process_file(file_path, out_path):
    # out_file = open(out_path, 'w')
    data = {}
    with open(file_path, 'r') as file:
        for i, line in enumerate(file):
            print(f"time point {i}")
            json_line = json.loads(line)
            for entry in json_line['entries']:
                condition, probability = entry
                if not "exist" in condition:
                    print(f"{condition} {probability}")
                    if condition not in data:
                        data[condition] = []
                    data[condition].append(probability)
    return data

def plot_data(data):
    plt.figure(figsize=(10, 6))
    for condition, probabilities in data.items():
        plt.plot(probabilities, label=condition)

    plt.xlabel('Timepoint')
    plt.ylabel('Probability')
    plt.title('Probability of Conditions Over Time')
    plt.legend()
    plt.show()

def main():
    if len(sys.argv) < 2:
        print("Usage: python script.py <file_path>")
        sys.exit(1)
    file_path = sys.argv[1]
    out_path = '' # sys.argv[2]
    data = read_and_process_file(file_path, out_path)
    plot_data(data)

if __name__ == "__main__":
    main()
