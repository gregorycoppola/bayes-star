import numpy as np
import matplotlib.pyplot as plt
import sys
import json

def calculate_average_and_variance(data):
    """Calculate average and variance for each iteration of each condition."""
    averages_variances = {}
    for condition, rounds_data in data.items():
        iteration_averages = []
        iteration_variances = []
        # Determine the number of iterations (assuming all rounds have the same length)
        num_iterations = len(rounds_data[0])
        for i in range(num_iterations):
            iteration_values = [round_data[i] for round_data in rounds_data if i < len(round_data)]
            iteration_average = np.mean(iteration_values)
            iteration_variance = np.var(iteration_values)
            iteration_averages.append(iteration_average)
            iteration_variances.append(iteration_variance)
        averages_variances[condition] = (iteration_averages, iteration_variances)
    return averages_variances

def tikz_plot_data(averages_variances, out_file_path):
    with open(out_file_path, 'w') as tikz_file:
        tikz_file.write("\\begin{tikzpicture}\n")
        tikz_file.write("\\begin{axis}[\n")
        tikz_file.write("xlabel={Iteration},\n")
        tikz_file.write("ylabel={Average Probability},\n")
        tikz_file.write("title={Average Probability and Variance by Iteration},\n")
        tikz_file.write("legend pos=north west,\n")
        tikz_file.write("ymajorgrids=true,\n")
        tikz_file.write("grid style=dashed,\n]\n")

        for condition, (averages, variances) in averages_variances.items():
            iterations = list(range(len(averages)))
            errors = np.sqrt(variances)
            data_points = ""
            for i, avg in enumerate(averages):
                data_points += f"({iterations[i]},{avg}) +- (0,{errors[i]}) "
            tikz_file.write(f"\\addplot+[error bars/.cd, y dir=both, y explicit]\n")
            tikz_file.write(f"coordinates {{\n")
            tikz_file.write(data_points.strip())
            tikz_file.write("\n};\n")
            tikz_file.write(f"\\addlegendentry{{{condition}}}\n")

        tikz_file.write("\\end{axis}\n")
        tikz_file.write("\\end{tikzpicture}\n")
def plot_data(averages_variances):
    """Plot the line graph with averages and variances for each condition."""
    plt.figure(figsize=(10, 6))
    for condition, (averages, variances) in averages_variances.items():
        iterations = list(range(len(averages)))
        plt.errorbar(iterations, averages, yerr=np.sqrt(variances), label=condition)
    
    plt.xlabel('Iteration')
    plt.ylabel('Average Probability')
    plt.title('Average Probability and Variance by Iteration')
    plt.legend()
    plt.show()

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
    averages_variances = calculate_average_and_variance(aggregated_data)
    # plot_data(averages_variances)
    tikz_file_path = f"./tikzoutput/{scenario_name}_{test_scenario}_plot.tex"
    tikz_plot_data(averages_variances, tikz_file_path)
    
    # Here, you might want to do something with the aggregated_data,
    # like saving it to a file or processing it further.
    print("Aggregated data:", aggregated_data)

if __name__ == "__main__":
    main()
