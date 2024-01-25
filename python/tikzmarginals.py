import json
import matplotlib.pyplot as plt
import sys

"""
\begin{tikzpicture}
    \begin{axis}[
        title={Probability over Time},
        xlabel={Time Points},
        ylabel={Marginal Probability},
        xmin=0, xmax=3,
        ymin=0, ymax=1,
        xtick={0,1,2,3},
        ytick={0,0.2,0.4,0.6,0.8,1},
        legend pos=north west,
        ymajorgrids=true,
        grid style=dashed,
    ]
    
    % Curve 1
    \addplot[
        color=blue,
        mark=square,
        ]
        coordinates {
        (0,0.2)(1,0.5)(2,0.6)(3,0.8)
        };
        \addlegendentry{Condition A}
    
    % Curve 2
    \addplot[
        color=red,
        mark=triangle,
        ]
        coordinates {
        (0,0.1)(1,0.2)(2,0.3)(3,0.4)
        };
        \addlegendentry{Condition B}
    
    % Curve 3
    \addplot[
        color=green,
        mark=o,
        ]
        coordinates {
        (0,0.3)(1,0.4)(2,0.5)(3,0.7)
        };
        \addlegendentry{Condition C}
    
    \end{axis}
    \end{tikzpicture}
"""

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
                    print(f"\"{condition}\" {probability}")
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
    # plt.show()

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
