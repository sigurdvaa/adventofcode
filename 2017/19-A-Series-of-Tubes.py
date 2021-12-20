from string import ascii_uppercase

with open("19_input.txt", "r") as f:
    input_raw = f.read()

diagram = [[char for char in line] for line in input_raw.splitlines()]

# start at "|" on the first line and move down
x, y = diagram[0].index("|"), 0
xDir, yDir = 0, 1
last = "|"
xMax, yMax = len(diagram[0]) - 1, len(diagram) - 1
validpath = "+|-" + ascii_uppercase
letters = []
steps = 0
while 1:
    steps += 1
    if diagram[y][x] == "+":
        if xDir != 0:
            if y != yMax and diagram[y + 1][x] in validpath:
                yDir, xDir = 1, 0
            elif y != 0 and diagram[y - 1][x] in validpath:
                yDir, xDir = -1, 0
        elif yDir != 0:
            if x != xMax and diagram[y][x + 1] in validpath:
                xDir, yDir = 1, 0
            elif x != 0 and diagram[y][x - 1] in validpath:
                xDir, yDir = -1, 0
    elif diagram[y][x] in ascii_uppercase:
        letters += [diagram[y][x]]

    x, y = x + xDir, y + yDir
    if diagram[y][x] == " ":
        break

print("Part 1\n" + "".join(letters), "\nPart 2\n" + str(steps))
