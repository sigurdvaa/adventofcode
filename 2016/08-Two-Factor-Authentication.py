with open("08_input.txt", "r") as f:
    input_instructions = [x.strip() for x in f.readlines()]

display = [["." for x in range(50)] for y in range(6)]


def turnon_rect(wide: int, tall: int):
    for y in range(tall):
        for x in range(wide):
            display[y][x] = "#"


def rotate_column(column: int, by: int):
    for i in range(by):
        temp_pixel = display[-1][column]
        for p in range(len(display) - 1, -1, -1):
            display[p][column] = display[p - 1][column]
        display[0][column] = temp_pixel


def rotate_row(row: int, by: int):
    for i in range(by):
        display[row].insert(0, display[row][-1])
        del display[row][-1]


for ins in input_instructions:
    split = ins.split()
    if split[0] == "rect":
        ab = split[1].split("x")
        turnon_rect(int(ab[0]), int(ab[1]))

    elif split[1] == "column":
        x = int(split[2].split("=")[1])
        by = int(split[4])
        rotate_column(x, by)

    elif split[1] == "row":
        y = int(split[2].split("=")[1])
        by = int(split[4])
        rotate_row(y, by)


int_lit = 0
for row in display:
    for pixel in row:
        if pixel == "#":
            int_lit += 1

print(f"Part One: {int_lit}")

print("Part Two:")
for row in display:
    print("".join(row))
