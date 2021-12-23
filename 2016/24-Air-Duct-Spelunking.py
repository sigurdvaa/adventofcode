input_raw = """#########################################################################################################################################################################################
#...#...............#.#...#.#.......#.....#.....#.....#...............#.....#.#.....#.....#.......#.........#.#.....#.....#.#.#...#...........#.................#.#...#.....#.....#...#.#
#.###.#.###.###.###.#.###.#.###.#####.###.#.#.#.#.#.#.#.#.###.#.#####.#.#.#.#.#.#.###.###.#.#.#.#.###.###.#.#.#.#.###.###.#.#.#.#.#####.#####.#.#.###########.###.#.#.#.###.#.###.#.#.#.#
#.#...#.....#.....#.#.#...#.#.....#.#.#...#.....#.#.#.....#.............#...#.....#...#.#.........#.#.....#.......#...#.#.#.#.#.........#.....#.#.#...#...#...#...#...#.....#.....#...#.#
#.#.###.#.#.#.#.###.#.#.###.#.#####.#.#.#.#.#####.#.#.#.###.#.#.#.#.###.#.#.###.#.#.#.#.###.###.#.#.###.#.#.#.###.###.#.#.#.#.###########.#.###.#.###.#.#.#.#.#.#.#.#.###.#.#.#.###.#.#.#
#...........#.#.#...#.#.....#.....#...........#...........#.....#.............#.......#.#...............#.....#.......#.....#...............#1....#.......#.#.....#.......#...#.....#...#
#.###.#.#.#.#.#.#.#.#########.#.#.#.#.#.###.#.#.#####.#.#.#.###.#.###.#########.#######.#.#.#.#.#############.#####.#.###.#.#.#.#.#####.#.#######.#####.#.#.#.#.#.#####.#.#.#.#.###.#.###
#...#.....#.....#.#.#.#.................#.#.#...#.......#...#...#.#...........#.#...#.......#.#.#...#...#.......#...#.#.#.#.#.....#.....#.....#.......#...#...#...#.......#...#.....#...#
#.#.#####.#.#####.#.#.#.#.###.###.#.#.###.#.#.#.#.#######.#.#.#.#.###.#.###.#####.#.#.#.#.#.#.#.#.#.#.#.#.#.#.#.#######.#.#####.#.#.#.#######.#.#.#.#.#.#######.#.###.#.#.#.###.###.#.#.#
#...#.....#.......#.#...#.#...#...........#...#...#.#.....#...#.......#.......#.#.#.#...#.#.....#.#...#...#.....#.........#...#.........#.#.....#.....#...#.#.#...#...#.....#.#3#.#.....#
#.#.###.#.#.###.#######.#.#.#.#.###.###.#####.#.#.#.#.#.#.#.#.#.#####.#.###.###.###.#.#.#.#.#.#.#.###.#.#######.###.#.###.#.#.#.###.#.#.#.#.#.#.#.#.###.###.#.#.###.#.#.###.#.#.#.###.#.#
#.......#.#...#.#.#.....#...#...#0..#.#.....#.#.....#.#.#...#...#.....#.....#.#.....#...#.#...#.#...#.#.#...#...#...#...#.#.#.............#.#...#.......#...#...#...#.#.#.....#...#.#...#
#####.#####.#.###.#.#.#.#.#.#.#.###.#.#.#.#.#.#.###.#.###.#.#.#.#.#.#.###.###.###.#.#####.#####.#.#.#.#.#.#.#.#.#.#.#.###.#.###.###########.#.#.#########.#.#.#.#.###.#.###.#.###.#.#.#.#
#...#...#.......#.#.#...#.......#...#...#.#...#.#...#.....#.....#.....#.#.#.........#.#.......#...#.#...#.#.........#...#.........#...#...#.#.....#.....#...#.#.......#.#.....#.........#
#.#.###.#.#####.###.###.#.#.###.###.#.###.#.#.#.#.#.#.#####.#.###.#.#.#.###.###.#####.#.#.#.#.#.###.###.#.#########.#.###.#.#.#.#.#.#.#.#.#.#####.###.#.#.#.#.#.#.#.#.#####.#.###.#.###.#
#.....#.#.#...#.#...#.#.....#.#...........#.....#...#.#...#.#.....#.#.#.....#.......#...#.#.#.......#.......#.........#.........#.#.#.#...#...#.#...#...#.......#.....#.....#.......#...#
###.#.#.#####.#.#.#.#.#.#.#.#.#.#.#.#.#.#######.###.#.#####.#####.###.#.#.#.###.#.#.###.#.#.#.#.#####.#.###.#.#.#.#########.#.###.#.#.#.#.#####.###.#.#.#.###.#####.#.#.#.#.#.#.#.#.#.###
#.#...#.#.......#.#.#...#.......#.#.....#.#.........#...#.....#.................#.........#...#...#.#...#.....#...#...#.#...#...#...#.....#...#.......#.......#.....#...#...#...#...#.#.#
#.#.#####.#.#####.###.#.###.#.###.#.#.#.#.#######.#.###.###.#.#.#########.#.###.#########.###.###.#.###.#.#####.#.###.#.#.#.#.#.#.#.#.###.#.#.#.#########.#####.#.#.###.#.#.#.#.#####.#.#
#.............#2#.#.....#.#.#...#...#.#...........#.........#...#.#.......#.#.........#.#.#.....#.#.......#...#.....#...#.#.#.......#...#.#.......#.......#...#.........#...#...#.....#.#
#.#.###.#.#.#.#.#.#.#.###.#.###.#.#####.###.###.#.#.#.#.#.#.#####.#.#.#.#.###.#.#####.#.#.#.#.#####.#.#####.###.###.#.#.#.#.#.#.#.#.#.#.#.#.###.#.###.#####.#######.#.#.#####.###.###.#.#
#...#...#...#...#.#.#.#...#.#.........#.#.#...............#.#...#...#.....#...#.#...#...#...........#.#.....#.....#...#.#.#...#...#.#.....#.....#...........#...#.....#.....#.#6#.....#.#
#####.#.#####.###.#.#.#.#.#.###.###.#.###.#.#########.#.###.#.#.#.###.###.#####.#####.###.###.#.#.#.###.#.#.#.#.###.#.###.#.#.#.#.###.#.#.#.###.#.#.#.#.#.###.#.#####.#.###.#.#.#.###.#.#
#...#...#.....#.....#...#...#...#.....#.....................#...#.....#.........#.....#.#.#...#.#.........#.....................#...#...#.....#...#...#.....#...#...#.....#.#.......#...#
#.###.#.#.#.#.###.#.###.#.#####.#.###.#.#####.#.#######.#.#.#.###.#.#.#.#.#.#.#.#.#.#.#.#.#.#.###.#####.###.#####.#######.###.#.#.#######.###.#.###.###.#.#.###.#.#.#.#####.#.#.#######.#
#.#...#.........#...#.....#.......#.....#.#.....#.#...#.#...#.#...#.........#.....#.......#.......#.......#.......#...#...#.......#.....#.#...........#.........#.#.#...#.#.#.....#...#.#
#.#.#.#.#.#.#.#.#.#.#########.#####.#.###.#.#####.#.###.#.#.#.#.#########.#.#.#.#.###.###.#####.###.###.#.#.#.#####.#.#.#########.###.#.###.###.###.#.#######.#.###.###.#.###.###.#.###.#
#.....#.....#...#.#.........#...#.....#...#.............#.....#.#.........#...#.......#.................#...#.........#...#...........#.........#.....#.......#.....#.....#.#.#.#...#.#.#
#.###.#.###.#.#.#.#.#.#.#.###.###.#.#######.#.###.#####.#.###.#.#.#.#.#.#.#.#.#######.###.###.#######.#.###.###.#####.#.#.#.#.#.#.#.###.#.###.###.###.#.###.#.###.###.#####.#.#.###.#.###
#.....#...#.#.....#.#...#.....#.....#.#...#...#...........#.#.#...........#.#.#.....#.....#...#...#...#...#.....#.......#.......#.#.........#.....#.....#.#...#.......#.........#.......#
#.#.#.###.#.#.#.###.###.#.###.#####.#.#.#.#.#.#######.###.#.#.#.#####.###.#.#.###.###.#.#.#.#.#.###.#####.###.###.#######.###.#.#.#.#.#.#.#####.#########.#.#.#.#.#.###.#######.###.#.#.#
#.......#.#.#.....#.....#.#...#.....#...#.#.....#...#.#...#.....#.......#...#.#.#...#.....#.......#.......#...#.#.......#...#...#.#.#...#.......#.....#.#...#.#.#...#...#.......#.......#
#.###.###.#.#.###.#.#.#.#.#.#.#.#.#.###.###.#.#.###.#.###.###.#.#.#.#.#.#####.#.#.#.#.#.#########.#.###.#.#.#.#.#.#.#.#.#.#####.###.###.#.#.#.#.#.#.#.#.#.#.###.#.###.#.#####.#######.#.#
#.....#.#5......#.......#...#.#.#...#.#.#.#.....#.....#.#.....#.......#.#...#...#.....#.....#...........#...#.......#.#...#.........#.........#.........#...........#.#.#.....#7....#.#.#
#.#.#.#.###.#####.#####.#.###.#.#.#.#.###.#.#.#.#####.#.#.#.#.#.###.#.###.###.###.#.#.###.#.#.#.#.#######.#.#.#####.#.#.###.#.#####.#.#.#.#.#.#.#.#####.#.#.#.#.#####.#.#.#.#.#####.#.#.#
#...........#.#...#.........#...#.#.#.#.......#.#.....#...#...#.#.....#...#...#.......#.................................#...#...#.#.....#.....#...#...#.#...#...#.#.......#.........#.#.#
###.###.#.###.#.###.#.#.#.#.#.#.#.#.#.#.###.#.#.#.#.#.#.###.###.###.###.#.###.#.###.#####.#.#.###.###.#.#####.#.#.#.#####.#.###.#.#.#.#.###.#.###.###.#.#######.#.#.#.#.###.###.#.#.#.#.#
#.......#.#.........#...#.#.#.................#.......#...#.....#...............#...#.#...#...#.......#.#.#.......#.......#...............#4#.......#.....#...#...#.#...#.......#.....#.#
#########################################################################################################################################################################################"""


def bfs_solved(locations: dict):
    for xy in locations:
        if not locations[xy]:
            return False
    return True


def bfs_unseen(seen: set, state: list):
    generalized = [str(state[1][xy]) for xy in state[1]]
    generalized.append(str(state[2]))
    state_generalized = "".join(generalized)
    if not state_generalized in seen:
        seen.add(state_generalized)
        return True
    return False


def bfs_next_states(queue: list, validpos: set, state: list):
    steps = state[0] + 1
    next_pos = {
        (state[2][0] + 1, state[2][1]),
        (state[2][0] - 1, state[2][1]),
        (state[2][0], state[2][1] + 1),
        (state[2][0], state[2][1] - 1),
    }
    for xy in next_pos:
        if xy in validpos:
            locations = state[1].copy()
            if xy in locations:
                locations[xy] = 1
            queue.append([steps, locations, xy])


def bfs_spelunking(maze: list, return_to_start: bool = False):
    from collections import deque

    queue = deque()
    seen = set()

    start = (0, 0)
    validpos = set()
    locations = dict()
    for y in range(len(maze)):
        for x in range(len(maze[y])):
            if maze[y][x].isdigit():
                validpos.add((x, y))
                if maze[y][x] == "0":
                    start = (x, y)
                else:
                    locations[(x, y)] = 0
            elif maze[y][x] == ".":
                validpos.add((x, y))

    queue.append([0, locations, start])
    while len(queue):
        state = queue.popleft()
        if bfs_solved(state[1]):
            if not return_to_start:
                return state[0]
            else:
                if state[2] == start:
                    return state[0]
        if bfs_unseen(seen, state):
            bfs_next_states(queue, validpos, state)

    return -1


maze = input_raw.splitlines()
print(f"Part One: {bfs_spelunking(maze)}")
print(f"Part Two: {bfs_spelunking(maze, True)}")