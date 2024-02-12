import json
import sys
import matplotlib.pyplot as plt
import os
import numpy as np  # For calculating variance

def read_and_process_file(file_path, max_lines=None):
    data = {}
    with open(file_path, 'r') as file:
        for i, line in enumerate(file):
            if max_lines is not None and i >= max_lines:
                break
            json_line = json.loads(line)
            for entry in json_line['entries']:
                condition, probability = entry
                if "exist" not in condition:
                    if condition not in data:
                        data[condition] = []
                    data[condition].append(probability)
    return data

def aggregate_data(base_path, scenario_name, test_scenario, max_lines=None):
    aggregated_data = {}
    for round_number in range(1, 13):  # Loop through rounds 1 to 12
        file_path = f"{base_path}/{scenario_name}_{test_scenario}_{round_number}"
        round_data = read_and_process_file(file_path, max_lines)
        
        # Aggregate data from this round into the overall dataset
        for condition, probabilities in round_data.items():
            if condition not in aggregated_data:
                aggregated_data[condition] = []
            # Append the list of probabilities as a new vector for this round
            aggregated_data[condition].append(probabilities)
    
    return aggregated_data

def analyze_convergence(aggregated_data):
    convergence_indices = {}
    for condition, rounds_data in aggregated_data.items():
        earliest_rounds = []
        for round_data in rounds_data:
            ultimate_value = round_data[-1]  # Assuming ultimate value is the last one in the sequence
            try:
                earliest_index = round_data.index(ultimate_value)
            except ValueError:
                earliest_index = len(round_data)  # If ultimate value not found, consider it the last index
            earliest_rounds.append(earliest_index)
        
        # Check if all elements in earliest_rounds are the same
        if len(set(earliest_rounds)) != 1:
            raise ValueError(f"Convergence iteration changes between rounds for condition: {condition}")
        
        # Calculate average and variance for earliest rounds
        # Note: In this specific implementation, since all values in earliest_rounds are the same,
        # the variance will always be 0, and the average will be equal to any of the elements.
        average_index = np.mean(earliest_rounds)
        variance_index = np.var(earliest_rounds)
        convergence_indices[condition] = (average_index, variance_index)
    return convergence_indices


def plot_convergence_statistics(convergence_data):
    conditions = list(convergence_data.keys())
    averages = [data[0] for data in convergence_data.values()]
    variances = [data[1] for data in convergence_data.values()]
    
    plt.figure(figsize=(10, 6))
    plt.bar(conditions, averages, yerr=np.sqrt(variances), capsize=5)  # Using standard deviation as error bar
    plt.xlabel('Condition')
    plt.ylabel('Average Earliest Convergence Round')
    plt.title('Convergence Analysis')
    plt.xticks(rotation=45, ha="right")
    plt.tight_layout()
    # plt.show()

def sanitize_key(key):
    mapping = {
        "lonely[sub=test_Jack0]": "Jack lonely",
        "exciting[sub=test_Jill0]": "Jill exciting",
        "like[obj=test_Jack0,sub=test_Jill0]": "Jill likes Jack",
        "like[obj=test_Jill0,sub=test_Jack0]": "Jack likes Jill",
        "date[obj=test_Jill0,sub=test_Jack0]": "Jack dates Jill"
    }
    # Handle complex or combined conditions if any, or fallback to a default transformation
    return mapping.get(key, key.replace("[", " ").replace("]", " ").replace("_", " ").replace(",", " and ").capitalize())

def tikz_plot_convergence_statistics(convergence_data, out_file_path):
    with open(out_file_path, 'w') as tikz_file:
        # Adjustments for symbolic x coords and legend entries
        sanitized_keys = [sanitize_key(key) for key in convergence_data.keys()]
        
        tikz_file.write("\\begin{tikzpicture}\n")
        tikz_file.write("\\begin{axis}[\n")
        tikz_file.write("ybar,\n")
        tikz_file.write("enlargelimits=0.15,\n")
        tikz_file.write("legend style={at={(0.5,-0.15)},anchor=north,legend columns=-1},\n")
        tikz_file.write("ylabel={Average Earliest Convergence Round},\n")
        tikz_file.write("xlabel={Condition},\n")
        tikz_file.write("symbolic x coords={" + ",".join(sanitized_keys) + "},\n")
        tikz_file.write("xtick=data,\n")
        tikz_file.write("nodes near coords align={vertical},\n")
        tikz_file.write("x tick label style={rotate=45,anchor=east},\n")
        tikz_file.write("]\n")

        for key, (average, variance) in convergence_data.items():
            sanitized_key = sanitize_key(key)
            tikz_file.write(f"\\addplot+[error bars/.cd, y dir=both, y explicit]\n")
            tikz_file.write(f"coordinates {{{{{sanitized_key}, {average}}} +- (0,{np.sqrt(variance)})}};\n")
        
        tikz_file.write("\\legend{" + ",".join(sanitized_keys) + "}\n")
        tikz_file.write("\\end{axis}\n")
        tikz_file.write("\\end{tikzpicture}\n")



def main():
    if len(sys.argv) < 3:
        print("Usage: python aggregate.py <base_path> <scenario_name> <test_scenario> [max_lines]")
        sys.exit(1)
    
    base_path = sys.argv[1]
    scenario_name = sys.argv[2]
    test_scenario = sys.argv[3]
    max_lines = int(sys.argv[4]) if len(sys.argv) > 4 else None
    
    # aggregated_data = aggregate_data(base_path, scenario_name, test_scenario, max_lines)
    aggregated_data = aggregate_data(base_path, scenario_name, test_scenario, max_lines)
    convergence_data = analyze_convergence(aggregated_data)
    # plot_convergence_statistics(convergence_data)
    tikz_file_path = "./tikzoutput/convergence_analysis.tex"  # Specify your desired output path
    tikz_plot_convergence_statistics(convergence_data, tikz_file_path)
    
    # Here, you might want to do something with the aggregated_data,
    # like saving it to a file or processing it further.
    print("Aggregated data:", aggregated_data)

if __name__ == "__main__":
    main()
