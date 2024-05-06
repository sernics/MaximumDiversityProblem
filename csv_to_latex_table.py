import csv

def csv_to_latex(input_file, output_file):
    with open(input_file, 'r') as csvfile:
        csvreader = csv.reader(csvfile)
        
        with open(output_file, 'w') as latexfile:
            latexfile.write("\\begin{table}[h]\n")
            latexfile.write("\\centering\n")
            latexfile.write("\\begin{tabular}{")
            
            # Assuming the first row contains column headers
            header = next(csvreader)
            num_columns = len(header)
            latexfile.write("|" + "c|"*num_columns)
            latexfile.write("}\n")
            latexfile.write("\\hline\n")
            
            # Writing column headers
            latexfile.write(" & ".join(header) + " \\\\\n")
            latexfile.write("\\hline\n")
            
            # Writing data
            for row in csvreader:
                # Todo lo que sea _ lo cambia por \_
                row = [i.replace("_", "\_") for i in row]
                latexfile.write(" & ".join(row) + " \\\\\n")
            
            latexfile.write("\\hline\n")
            latexfile.write("\\end{tabular}\n")
            latexfile.write("\\caption{Caption}\n")
            latexfile.write("\\label{tab:my_table}\n")
            latexfile.write("\\end{table}")

# Usage
input_csv = "results/orden_grasp_bb.csv"
output_latex = input_csv.replace(".csv", ".tex")
csv_to_latex(input_csv, output_latex)
