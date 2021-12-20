from collections import deque

with open("22_input.txt", "r") as f:
    input_raw = f.readlines()[2:]


nodes = dict()
for line in input_raw:
    split = line.split()
    xy = split[0].split("-")
    nodes[(int(xy[1][1:]), int(xy[2][1:]))] = {
        "size": int(split[1][:-1]),
        "used": int(split[2][:-1]),
        "free": int(split[3][:-1]),
        "perc": int(split[4][:-1]),
    }


def valid_nodes(nodes):
    valid = list()
    xmax = ymax = 0
    for xy in nodes:
        if xy[0] > xmax:
            xmax = xy[0]
        if xy[1] > ymax:
            ymax = xy[1]
    xmax += 1
    ymax += 1
    for x1 in range(xmax):
        for y1 in range(ymax):
            for x2 in range(xmax):
                for y2 in range(ymax):
                    if (x1, y1) == (x2, y2):
                        continue
                    if nodes[(x1, y1)]["used"] > 0:
                        if nodes[(x2, y2)]["free"] >= nodes[(x1, y1)]["used"]:
                            valid.append((x1, y1))
    return valid


def pretty_print(nodes: dict, valid: list, ygoal: int):
    xmax = ymax = 0
    for xy in nodes:
        if xy[0] > xmax:
            xmax = xy[0]
        if xy[1] > ymax:
            ymax = xy[1]
    access = (0, 0)
    goal = (xmax, ygoal)
    xmax += 1
    ymax += 1
    for y in range(ymax):
        for x in range(xmax):
            if nodes[(x, y)]["used"] == 0:
                print(" _ ", end="")
            elif not (x, y) in valid:
                print(" # ", end="")
            elif (x, y) == goal:
                print(" G ", end="")
            elif (x, y) == access:
                print("(.)", end="")
            else:
                print(" . ", end="")
        print()


def next_states(queue: deque, valid: list, state: tuple):
    """
    Check if goal can be moved to empty, if not, move empty to valid node
    """
    for x in range(-1, 2, 2):
        if state[1][0] + x == state[2][0] and state[1][1] == state[2][1]:
            queue.append((state[0] + 1, state[2], state[1]))
        empty = (state[2][0] + x, state[2][1])
        if empty in valid:
            if not empty == state[1]:
                queue.append((state[0] + 1, state[1], empty))
    for y in range(-1, 2, 2):
        if state[1][0] == state[2][0] and state[1][1] + y == state[2][1]:
            queue.append((state[0] + 1, state[2], state[1]))
        empty = (state[2][0], state[2][1] + y)
        if empty in valid:
            if not empty == state[1]:
                queue.append((state[0] + 1, state[1], empty))


def next_states_optimized(queue: deque, valid: list, state: tuple):
    """
    Check if goal can be moved to empty, if not, move empty to valid node.
    Looking at the input with pretty print we can see that there's no oversized
    nodes on y=0, meaning we only need to explore the space where goal moves towards x=0.
    """
    if state[1][0] - 1 == state[2][0] and state[1][1] == state[2][1]:
        queue.append((state[0] + 1, state[2], state[1]))
    for x in range(-1, 2, 2):
        empty = (state[2][0] + x, state[2][1])
        if empty in valid:
            if not empty == state[1]:
                queue.append((state[0] + 1, state[1], empty))
    for y in range(-1, 2, 2):
        empty = (state[2][0], state[2][1] + y)
        if empty in valid:
            if not empty == state[1]:
                queue.append((state[0] + 1, state[1], empty))


def unseen(seen: set, state):
    state = (state[1], state[2])
    if state in seen:
        return False
    seen.add(state)
    return True


def access_goaldata(nodes: dict, valid: list, ygoal: int):
    """
    BFS to find least steps to get goal data to access point.
    We know there's only one empty node, which can be used to move data,
    and only one node with the goal data, meaning the state we need to track
    only needs to contain coords for empty node and goal node.
    Only valid nodes can move data to empty node.
    """
    queue = deque()
    seen = set()
    access = (0, 0)
    xmax = 0
    for xy in nodes:
        if xy[0] > xmax:
            xmax = xy[0]
        if nodes[xy]["used"] == 0:
            empty = xy
    goal = (xmax, ygoal)
    queue.append((0, goal, empty))

    while len(queue):
        state = queue.popleft()
        if unseen(seen, state):
            if state[1] == access:
                return state[0]
            next_states_optimized(queue, valid, state)

    return -1


valid = valid_nodes(nodes)
print(f"Part One: {len(valid)}")
print(f"Part Two (bfs): {access_goaldata(nodes, valid, 0)}")
print(f"Part Two (manual): 45 + (30 * 5) = 195")
pretty_print(nodes, valid, 0)
