input_raw = """Tristram to AlphaCentauri = 34
Tristram to Snowdin = 100
Tristram to Tambi = 63
Tristram to Faerun = 108
Tristram to Norrath = 111
Tristram to Straylight = 89
Tristram to Arbre = 132
AlphaCentauri to Snowdin = 4
AlphaCentauri to Tambi = 79
AlphaCentauri to Faerun = 44
AlphaCentauri to Norrath = 147
AlphaCentauri to Straylight = 133
AlphaCentauri to Arbre = 74
Snowdin to Tambi = 105
Snowdin to Faerun = 95
Snowdin to Norrath = 48
Snowdin to Straylight = 88
Snowdin to Arbre = 7
Tambi to Faerun = 68
Tambi to Norrath = 134
Tambi to Straylight = 107
Tambi to Arbre = 40
Faerun to Norrath = 11
Faerun to Straylight = 66
Faerun to Arbre = 144
Norrath to Straylight = 115
Norrath to Arbre = 135
Straylight to Arbre = 127"""


def parse_distances(input_distances):
    distances = dict()
    for line in input_distances.splitlines():
        split = line.split()
        loc_1 = split[0]
        loc_2 = split[2]
        distance = int(split[4])

        if not loc_1 in distances:
            distances[loc_1] = dict()
        if not loc_2 in distances:
            distances[loc_2] = dict()

        distances[loc_1][loc_2] = distance
        distances[loc_2][loc_1] = distance

    return distances


def bfs_next_states(queue, state, locations, distances):
    if state[-1] != -1:
        curr_loc = locations[state[-1]]
        for next_loc in distances[curr_loc]:
            idx_next_loc = locations.index(next_loc)
            if state[idx_next_loc] == 0:
                next_state = state.copy()
                next_state[idx_next_loc] = 1
                next_state[-1] = idx_next_loc
                next_state[-2] += distances[locations[state[-1]]][next_loc]
                queue.append(next_state)
    else:
        for next_loc in distances:
            idx_next_loc = locations.index(next_loc)
            if state[idx_next_loc] == 0:
                next_state = state.copy()
                next_state[idx_next_loc] = 1
                next_state[-1] = idx_next_loc
                queue.append(next_state)


def bfs_seen_state(seen_states, state):
    if str(state) in seen_states:
        return True
    else:
        seen_states.add(str(state))
        return False


def bfs_solved(state):
    for item in state[:-2]:
        if item < 1:
            return False
    return True


def bfs_paths(locations: list, distances: dict, init_state: list):
    from collections import deque

    queue = deque()
    queue.append(init_state)
    seen_states = set()
    solved = list()

    while queue:
        state = queue.popleft()
        if not bfs_seen_state(seen_states, state):
            if bfs_solved(state):
                solved.append(state[-2])
            else:
                bfs_next_states(queue, state, locations, distances)

    solved = sorted(solved)
    return solved[0], solved[-1]


distances = parse_distances(input_raw)
locations = list(distances)
init_state = [0] * len(locations) + [0, -1]
shortest, longest = bfs_paths(locations, distances, init_state)
print(f"Part One: {shortest}")
print(f"Part Two: {longest}")
