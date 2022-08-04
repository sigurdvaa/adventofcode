with open("17_input.txt", "r") as fp:
    input_raw = fp.read()

input_test = """x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504"""


def parse_scan(string: str) -> set[tuple[int]]:
    scan: set[tuple[int]] = set()
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


def flow_side(flow, y_limit, clay, reached, step):
    next_flow = (flow[-1][0] + step, flow[-1][1])
    if next_flow in clay or next_flow in reached:
        return len(flow) - 1, 0

    flow.append(next_flow)
    reached.add(next_flow)

    down_flow = (next_flow[0], next_flow[1] + 1)
    if down_flow in clay or down_flow in reached:
        return flow_side(flow, y_limit, clay, reached, step)
    return len(flow) - 1, flow_down([next_flow], y_limit, clay, reached)


def flow_down(flow, y_limit, clay, reached=None):
    if reached == None:
        reached = set()
    next_flow = (flow[-1][0], flow[-1][1] + 1)
    if next_flow[1] > y_limit:
        return len(flow) - 1
    if next_flow in clay or next_flow in reached:
        i = len(flow) - 1
        sides = 0
        while i >= 0:
            left, left_down = flow_side([flow[i]], y_limit, clay, reached, -1)
            right, right_down = flow_side([flow[i]], y_limit, clay, reached, 1)
            sides += left + left_down + right + right_down
            if left_down + right_down != 0:
                break
            i -= 1
        return len(flow) - 1 + sides
    flow.append(next_flow)
    reached.add(next_flow)
    return flow_down(flow, y_limit, clay, reached)


clay: set[tuple[int]] = parse_scan(input_raw)
y_limit: int = max(clay, key=lambda x: x[1])[1]
print(f"Part One: {flow_down([(500, 0)], y_limit, clay)}")

clay: set[tuple[int]] = parse_scan(input_test)
y_limit: int = max(clay, key=lambda x: x[1])[1]
print(f"Test (57): {flow_down([(500, 0)], y_limit, clay)}")

# 957 low
