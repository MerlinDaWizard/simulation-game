# A quick and dirty script to convert copy pasted tables from github into json formatted structs
# Mainly used for https://github.com/catppuccin/catppuccin
file_in = open("input.txt",'r').read()
rows = file_in.split("\n")
name = input("Name of theme: ").strip()
output = open(f"{name}.json",'w')

output.write(f"{{\n\t\"name\": \"{name}\",\n")

for i, row in enumerate(rows):
    cells = row.strip().split("\t") # Tab
    print(cells)
    print(cells[2][4:-1])
    rgb = cells[2][4:-1].split(',') # Strip rgb(...), it works
    print(rgb)
    a = 255
    output.write(f"\t\"{cells[0].lower()}\": [{rgb[0].strip()}, {rgb[1].strip()}, {rgb[2].strip()}, {a}]")
    if i != len(rows)-1:
        output.write(',')
    output.write('\n')
output.write("}")
output.close()
