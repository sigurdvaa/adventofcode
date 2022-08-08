with open("17_input.txt", "r") as fp:
    input_raw = fp.read()


Point = tuple[int, int]
Points = set[Point]
Stream = list[Point]


def parse_scan(string: str) -> Points:
    scan: Points = set()
    for line in string.splitlines():
        split: list[str] = line.split(", ")
        c_start: int = int(split[0][2:])
        c_range: list[int] = list(map(int, split[1][2:].split("..")))
        x_indexed: bool = True if (split[0][0] == "x") else False
        for i in range(c_range[0], c_range[1] + 1):
            if x_indexed:
                scan.add((c_start, i))
            else:
                scan.add((i, c_start))
    return scan


def flow_side(
    stream: Stream,
    y_max: int,
    clay: Points,
    flow: Points,
    pool: Points,
    step: int,
) -> tuple[Stream, bool]:
    next_side: Point = (stream[-1][0] + step, stream[-1][1])
    while next_side not in clay:
        next_down: Point = (next_side[0], next_side[1] + 1)
        stream.append(next_side)
        if next_down not in clay and next_down not in pool:
            return stream, flow_down([next_side], y_max, clay, flow, pool)
        next_side = (next_side[0] + step, next_side[1])
    return (stream, True)


def flow_down(
    stream: Stream,
    y_max: int,
    clay: Points,
    flow: Points,
    pool: Points,
) -> bool:
    next_down: Point = (stream[-1][0], stream[-1][1] + 1)
    while next_down not in clay and next_down not in pool:
        if next_down[1] > y_max:
            return False
        if next_down in flow:
            return False
        stream.append(next_down)
        flow.add(next_down)
        next_down = (stream[-1][0], stream[-1][1] + 1)
    i: int = len(stream) - 1
    while i >= 0:
        l_stream, l_backflow = flow_side([stream[i]], y_max, clay, flow, pool, -1)
        r_stream, r_backflow = flow_side([stream[i]], y_max, clay, flow, pool, 1)
        if l_backflow and r_backflow:
            if stream[i] in flow:
                flow.remove(stream[i])
            pool.update(l_stream)
            pool.update(r_stream)
            i -= 1
        else:
            flow.update(l_stream)
            flow.update(r_stream)
            break
    if i < 0:
        return True
    else:
        return False


def print_flow(clay: Points, flow: Points, pool: Points):
    x_min: int = min(clay, key=lambda x: x[0])[0] - 1
    x_max: int = max(clay, key=lambda x: x[0])[0] + 2
    y_max: int = max(clay, key=lambda y: y[1])[1] + 2
    for y in range(y_max):
        for x in range(x_min, x_max):
            xy = (x, y)
            if xy in clay:
                print("\033[97m#\033[00m", end="")
            elif xy in pool:
                print("\033[96m~\033[00m", end="")
            elif xy in flow:
                print("\033[96m|\033[00m", end="")
            else:
                print("\033[93m.\033[00m", end="")
        print()


def water_flow(scan: str, show_flow=False):
    flow: Points = set()
    pool: Points = set()
    clay: Points = parse_scan(scan)
    y_max: int = max(clay, key=lambda x: x[1])[1]
    y_min: int = min(clay, key=lambda x: x[1])[1]
    stream: Stream = [(500, y_min - 1)]
    flow_down(stream, y_max, clay, flow, pool)
    if show_flow:
        print_flow(clay, flow, pool)
    return (len(flow), len(pool))


flow, pool = water_flow(input_raw)
print(f"Part One: {flow + pool}")
print(f"Part Two: {pool}")
