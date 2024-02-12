import json
import sys
import os

tikz_end = """
    \end{axis}
\end{tikzpicture}
"""

prop_order = [
    "alpha0[sub=test_Jack0]",
    "alpha1[sub=test_Jack0]",
    "alpha2[sub=test_Jack0]",
    "alpha3[sub=test_Jack0]",
    "alpha4[sub=test_Jack0]",
    "alpha5[sub=test_Jack0]",
    "alpha6[sub=test_Jack0]",
    "alpha7[sub=test_Jack0]",
    "alpha8[sub=test_Jack0]",
    "alpha9[sub=test_Jack0]",
    "alpha10[sub=test_Jack0]",
]

legend_mapping = {
    "alpha0[sub=test_Jack0]": ['red', 'triangle', 'alpha0'],
    "alpha1[sub=test_Jack0]": ['green', 'triangle', 'alpha1'],
    "alpha2[sub=test_Jack0]": ['blue', 'triangle', 'alpha2'],
    "alpha3[sub=test_Jack0]": ['yellow', 'triangle', 'alpha3'],
    "alpha4[sub=test_Jack0]": ['orange', 'triangle', 'alpha4'],
    "alpha5[sub=test_Jack0]": ['purple', 'triangle', 'alpha5'],
    "alpha6[sub=test_Jack0]": ['black', 'triangle', 'alpha6'],
    "alpha7[sub=test_Jack0]": ['red', 'square', 'alpha7'],
    "alpha8[sub=test_Jack0]": ['green', 'square', 'alpha8'],
    "alpha9[sub=test_Jack0]": ['blue', 'square', 'alpha9'],
    "alpha10[sub=test_Jack0]": ['yellow', 'square', 'alpha10'],
}

def read_and_process_file(file_path, max_lines):
    data = {}
    with open(file_path, 'r') as file:
        for i, line in enumerate(file):
            if max_lines is not None and i >= max_lines:
                break  # Stop reading if max_lines is reached
            json_line = json.loads(line)
            for entry in json_line['entries']:
                condition, probability = entry
                if not "exist" in condition:
                    if condition not in data:
                        data[condition] = []
                    data[condition].append(probability)
    return data

def tikz_render_one_curve(prop, probabilities):
    legend_tuple = legend_mapping[prop]  # Default to black and circle if not found
    color, shape, legend = legend_tuple
    data_string = format_probability_vector(probabilities)
    tikz = f"""
    \\addplot[
        color={color},
        mark={shape},
        ]
        coordinates {{
        {data_string}
        }};
    \\addlegendentry{{{legend}}}
"""
    return tikz

def format_probability_vector(probabilities):
    # Create a list of formatted strings "(index, probability)" for each probability
    formatted_pairs = [f"({index},{prob})" for index, prob in enumerate(probabilities)]

    # Join all the formatted strings into a single string
    result = ''.join(formatted_pairs)
    return result

def tikz_render_curves(data):
    buffer = ""
    # print(f"prop_order {prop_order}")
    for prop in prop_order:
        row = data[prop]
        part = tikz_render_one_curve(prop, row)
        # print(f"part: {part}")
        buffer = buffer + part
    return buffer

def create_tikz_preamble(N):
    xtick_values = ', '.join(str(i) for i in range(N))
    tix_preamble = f"""
\\begin{{tikzpicture}}
    \\begin{{axis}}[
        xlabel={{Iteration}},
        ylabel={{Marginal}},
        xmin=0, xmax={N-1},
        ymin=0, ymax=1,
        xtick={{{xtick_values}}},
        ytick={{0,0.2,0.4,0.6,0.8,1}},
        legend pos=south east,
        ymajorgrids=true,
        grid style=dashed,
    ]
"""
    return tix_preamble

def get_tuple_size(data):
    for key, value in data.items():
        return len(value)
    assert(False)

def main():
    if len(sys.argv) < 2:
        print("Usage: python script.py <file_path> [max_lines]")
        sys.exit(1)
    file_path = sys.argv[1]
    max_lines = int(sys.argv[2])
    base_name = os.path.basename(file_path)
    name_without_ext = os.path.splitext(base_name)[0]
    out_path = f"./tikzoutput/{name_without_ext}_max_{max_lines}.tex"  # Modify this line as needed

    data = read_and_process_file(file_path, max_lines)
    # plot_data(data, out_path)

    tuple_size = get_tuple_size(data)
    preamble = create_tikz_preamble(tuple_size)
    print(preamble)
    curves = tikz_render_curves(data)
    print(curves)

    total_tikz = '\n'.join([preamble, curves, tikz_end])
    out_file = open(out_path, 'w')
    out_file.write(total_tikz)

if __name__ == "__main__":
    main()
