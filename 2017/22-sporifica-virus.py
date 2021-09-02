string = '''..#
#..
...'''

string2 = '''.......##.#..####.#....##
..###....###.####..##.##.
#..####.#....#.#....##...
.#....#.#.#....#######...
.###..###.#########....##
##...#####..#####.###.#..
.#..##.###.#.#....######.
.#.##.#..####..#.##.....#
#.#..###..##..#......##.#
##.###.##.#.#...##.#.##..
##...#.######.#..##.#...#
....#.####..#..###.##..##
...#....#.###.#.#..#.....
..###.#.#....#.....#.####
.#....##..##.#.#...#.#.#.
...##.#.####.###.##...#.#
##.#.####.#######.##..##.
.##...#......####..####.#
#..###.#.###.##.#.#.##..#
#..###.#.#.#.#.#....#.#.#
####.#..##..#.#..#..#.###
##.....#..#.#.#..#.####..
#####.....###.........#..
##...#...####..#####...##
.....##.#....##...#.....#'''

orig_grid = {}
lines = string2.splitlines()
length = len(lines)
for y in range(length):
    for x in range(len(lines[y])):
        if lines[y][x] == "#":
            orig_grid[x,y] = lines[y][x]

print("Part 1")
grid = dict(orig_grid)
x = length // 2
y = x
xv = 0
yv = -1
infected = 0
bursts = 10000
for i in range(bursts):
    cord = x,y
    node = grid.get(cord, ".")
    if node == "." :
        grid[cord] = "#"
        infected += 1
        # Turn left
        if xv != 0:
            yv = xv * -1
            xv = 0
        else:
            xv = yv
            yv = 0
    elif node == "#":
        grid[cord] = "."
        # Turn right
        if xv != 0:
            yv = xv
            xv = 0
        else:
            xv = yv * -1
            yv = 0
    x += xv
    y += yv
print(infected)

print("Part 2")
grid = dict(orig_grid)
x = length // 2
y = x
xv = 0
yv = -1
infected = 0
bursts = 10000000
for i in range(bursts):
    cord = (x,y)
    node = grid.get(cord, ".")
    if node == ".":
        grid[cord] = "W"
        # Turn left
        if xv != 0:
            yv = xv * -1
            xv = 0
        else:
            xv = yv
            yv = 0
    elif node == "W":
        grid[cord] = "#"
        infected += 1
    elif node == "#":
        grid[cord] = "F"
        # Turn right
        if xv != 0:
            yv = xv
            xv = 0
        else:
            xv = yv * -1
            yv = 0
    elif node == "F":
        grid[cord] = "."
        xv *= -1
        yv *= -1
    x += xv
    y += yv
print(infected)