input_raw = """.......##.#..####.#....##
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
.....##.#....##...#.....#"""

orig_grid = {}
lines = input_raw.splitlines()
length = len(lines)
for x, r in enumerate(lines):
    for y, c in enumerate(r):
        if c == "#":
            orig_grid[y - x * 1j] = "#"  # up = decrease in x; right = increase in y

print("Part 1")
grid = dict(orig_grid)
h = length // 2
dirc = 0 + 1j  # up
posc = h - h * 1j
infected = 0
bursts = 10000
for i in range(bursts):
    node = grid.get(posc, ".")
    if node == ".":
        grid[posc] = "#"
        infected += 1
        # Turn left
        dirc *= 1j
    elif node == "#":
        grid[posc] = "."
        # Turn right
        dirc *= -1j
    posc += dirc
print(infected)

print("Part 2")
grid = dict(orig_grid)
h = length // 2
dirc = 0 + 1j  # up
posc = h - h * 1j
infected = 0
bursts = 10000000
for i in range(bursts):
    node = grid.get(posc, ".")
    if node == ".":
        grid[posc] = "W"
        # Turn left
        dirc *= 1j
    elif node == "W":
        grid[posc] = "#"
        infected += 1
    elif node == "#":
        grid[posc] = "F"
        # Turn right
        dirc *= -1j
    elif node == "F":
        grid[posc] = "."
        dirc *= -1
    posc += dirc
print(infected)
