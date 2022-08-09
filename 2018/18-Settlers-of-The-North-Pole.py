with open("18_input.txt", "r") as fp:
    input_raw = fp.read()


def count_adjacent(area: list[str], x: int, y: int) -> tuple[int, int]:
    trees: int = 0
    lumberyards: int = 0
    x_min: int = x - 1
    x_max: int = x + 2
    y_min: int = y - 1
    y_max: int = y + 2

    if x_min < 0:
        x_min = 0
    if x_max > len(area[0]):
        x_max = len(area[0])
    if y_min < 0:
        y_min = 0
    if y_max > len(area):
        y_max = len(area)

    if area[y][x] == "|":
        trees -= 1
    elif area[y][x] == "#":
        lumberyards -= 1

    for sy in range(y_min, y_max):
        for sx in range(x_min, x_max):
            if area[sy][sx] == "|":
                trees += 1
            elif area[sy][sx] == "#":
                lumberyards += 1
    return (trees, lumberyards)


def total_resource_value(area: list[str]) -> int:
    trees: int = 0
    lumberyards: int = 0
    for row in area:
        for c in row:
            if c == "|":
                trees += 1
            elif c == "#":
                lumberyards += 1
    return trees * lumberyards


def game_of_trees(area: list[str], target: int, print_states: bool = False) -> int:
    state: list[str] = area.copy()
    seen_states: dict[str, list[str]] = {}
    for i in range(target):
        next_state: list[str] = []
        state_str: str = "".join(state)

        if state_str in seen_states:
            loop_size: int = 1
            next_state_str = "".join(seen_states[state_str])
            while next_state_str != state_str:
                next_state_str = "".join(seen_states[next_state_str])
                loop_size += 1

            s: int = 1
            target_index = (target - i - 1) % loop_size
            next_state_str = "".join(seen_states[state_str])
            while s < target_index:
                next_state_str = "".join(seen_states[next_state_str])
                s += 1
            return total_resource_value(seen_states[next_state_str])

        for y in range(len(state)):
            row: str = ""
            for x in range(len(state[y])):
                trees, lumberyards = count_adjacent(state, x, y)
                if state[y][x] == ".":
                    if trees >= 3:
                        row += "|"
                    else:
                        row += "."
                elif state[y][x] == "|":
                    if lumberyards >= 3:
                        row += "#"
                    else:
                        row += "|"
                elif state[y][x] == "#":
                    if lumberyards >= 1 and trees >= 1:
                        row += "#"
                    else:
                        row += "."
            next_state.append(row)

        seen_states[state_str] = next_state
        state = next_state

        if print_states:
            print("-----", i, "-----")
            for row in state:
                print(row)
            input("pause...")

    return total_resource_value(state)


area: list[str] = input_raw.splitlines()
print(f"Test One: {game_of_trees(area, 10)}")
print(f"Test Two: {game_of_trees(area, 1_000_000_000)}")
