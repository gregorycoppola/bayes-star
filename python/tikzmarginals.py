import json
import sys

tix_begin = """
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
"""
    
"""
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
"""
tix_end = """
    \end{axis}
\end{tikzpicture}
"""

prop_order = [
    "lonely[sub=test_Jack9]",
    "exciting[sub=test_Jill9]",
    "like[obj=test_Jack9,sub=test_Jill9]",
    "like[obj=test_Jill9,sub=test_Jack9]",
    "date[obj=test_Jill9,sub=test_Jack9]",
]

legend_mapping = {
    "lonely[sub=test_Jack9]": ['red', 'triangle', 'lonely boy'],
    "exciting[sub=test_Jill9]": ['green', 'square', 'exciting girl'],
    "like[obj=test_Jack9,sub=test_Jill9]": ['blue', 'o', 'boy likes girl'],
    "like[obj=test_Jill9,sub=test_Jack9]": ['yellow', 'triangle', 'girl likes boy'],
    "date[obj=test_Jill9,sub=test_Jack9]": ['orange', 'square', 'boy dates girl'],
}

def read_tuple_list_from_file(file_path):
    data = {}
    with open(file_path, 'r') as file:
        for i, line in enumerate(file):
            print(f"time point {i}")
            json_line = json.loads(line)
            for entry in json_line['entries']:
                print(f"entry: {entry}")
                condition, probability = entry
                if not "exist" in condition and not '{' in condition:
                    print(f"\"{condition}\" {probability}")
                    if condition not in data:
                        data[condition] = []
                    data[condition].append(probability)
    last_size = -1
    for key, value in data.items():
        if last_size == -1:
            last_size = len(value)
        else:
            assert(last_size == len(value))
    return data

def tikz_render_one_curve(prop, tuple):
    legend_tuple = legend_mapping[prop]
    color = legend_tuple[0]
    shape = legend_tuple[1]
    legend = legend_tuple[2]
    tikz = f"""
    \addplot[
        color={color},
        mark={shape},
        ]
        coordinates {
        (0,0.2)(1,0.5)(2,0.6)(3,0.8)
        };
        \addlegendentry{{{legend}}}
"""
    return tikz

def tikz_render_curves(data):
    buffer = ''
    for prop in prop_order:
        row = data[prop]
        part = tikz_render_one_curve(prop, row)
        buffer += part

    return part

def create_tikz_preamble(N):
    xtick_values = ', '.join(str(i) for i in range(N))
    tix_preamble = f"""
\\begin{{tikzpicture}}
    \\begin{{axis}}[
        title={{Probability over Time}},
        xlabel={{Time Points}},
        ylabel={{Marginal Probability}},
        xmin=0, xmax={N-1},
        ymin=0, ymax=1,
        xtick={{{xtick_values}}},
        ytick={{0,0.2,0.4,0.6,0.8,1}},
        legend pos=north west,
        ymajorgrids=true,
        grid style=dashed,
    ]
"""
    return tix_preamble

def main():
    if len(sys.argv) < 2:
        print("Usage: python script.py <file_path>")
        sys.exit(1)
    file_path = sys.argv[1]
    out_path = '' # sys.argv[2]
    data = read_tuple_list_from_file(file_path)

    preamble = create_tikz_preamble(5)
    print(preamble)
    curves = tikz_render_curves(data)
    print(curves)

if __name__ == "__main__":
    main()
