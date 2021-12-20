with open("18_input.txt", "r") as f:
    input_raw = f.read()


def num_neighbors_on(grid: list, y: int, x: int):
    num = 0
    size = len(grid)

    if y > 0:
        if x > 0:
            if grid[y - 1][x - 1] == "#":
                num += 1
        if grid[y - 1][x] == "#":
            num += 1
        if x < size - 1:
            if grid[y - 1][x + 1] == "#":
                num += 1

    if x > 0:
        if grid[y][x - 1] == "#":
            num += 1
    if x < size - 1:
        if grid[y][x + 1] == "#":
            num += 1

    if y < size - 1:
        if x > 0:
            if grid[y + 1][x - 1] == "#":
                num += 1
        if grid[y + 1][x] == "#":
            num += 1
        if x < size - 1:
            if grid[y + 1][x + 1] == "#":
                num += 1

    return num


def game_of_life(start_grid: str, steps: int, stuck: bool = False):
    grid = [list(x) for x in start_grid.splitlines()]
    size = len(grid)
    if stuck:
        grid[0][0] = "#"
        grid[0][size - 1] = "#"
        grid[size - 1][0] = "#"
        grid[size - 1][size - 1] = "#"

    for _ in range(steps):
        next_grid = [["."] * size for _ in range(size)]
        for y in range(size):
            for x in range(size):
                neighbors_on = num_neighbors_on(grid, y, x)
                if grid[y][x] == "#":
                    if neighbors_on in [2, 3]:
                        next_grid[y][x] = "#"
                else:
                    if neighbors_on == 3:
                        next_grid[y][x] = "#"
        if stuck:
            next_grid[0][0] = "#"
            next_grid[0][size - 1] = "#"
            next_grid[size - 1][0] = "#"
            next_grid[size - 1][size - 1] = "#"

        grid = next_grid

    lights_on = 0
    for y in range(size):
        for x in range(size):
            if grid[y][x] == "#":
                lights_on += 1

    return lights_on


print(f"Part One: {game_of_life(input_raw, 100)}")
print(f"Part Two: {game_of_life(input_raw, 100, True)}")
