with open("03-input.txt", "r") as f:
    input_raw = f.read().strip()


def houses_visited(directions: str, robot: bool = False):
    xy = xy_prev = (0, 0)
    visited = {xy: 1}
    for d in directions:
        if d == "^":
            xy = (xy[0], xy[1] - 1)
        elif d == "v":
            xy = (xy[0], xy[1] + 1)
        elif d == "<":
            xy = (xy[0] - 1, xy[1])
        elif d == ">":
            xy = (xy[0] + 1, xy[1])
        else:
            raise ValueError(f"Unknown direction: {d}")

        if not xy in visited:
            visited[xy] = 1

        if robot:
            xy, xy_prev = xy_prev, xy

    return len(visited)


print(f"Part One: {houses_visited(input_raw)}")
print(f"Part Two: {houses_visited(input_raw, True)}")
