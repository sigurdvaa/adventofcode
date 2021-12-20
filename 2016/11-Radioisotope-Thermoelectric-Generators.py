from collections import deque

input_raw = """The first floor contains a promethium generator and a promethium-compatible microchip.
The second floor contains a cobalt generator, a curium generator, a ruthenium generator, and a plutonium generator.
The third floor contains a cobalt-compatible microchip, a curium-compatible microchip, a ruthenium-compatible microchip, and a plutonium-compatible microchip.
The fourth floor contains nothing relevant."""

# items and which floor they're on (state)
test_items = ["step", "elevator", "e", "hyg", "hym", "lig", "lim"]
test_init_state = [0, 0, 1, 0, 2, 0]
part1_items = [
    "step",
    "elevator",
    "cog",
    "com",
    "cug",
    "cum",
    "plg",
    "plm",
    "prg",
    "prm",
    "rug",
    "rum",
]
part1_init_state = [0, 0, 1, 2, 1, 2, 1, 2, 0, 0, 1, 2]
part2_items = [
    "step",
    "elevator",
    "cog",
    "com",
    "cug",
    "cum",
    "plg",
    "plm",
    "prg",
    "prm",
    "rug",
    "rum",
    "elg",
    "elm",
    "dig",
    "dim",
]
part2_init_state = [0, 0, 1, 2, 1, 2, 1, 2, 0, 0, 1, 2, 0, 0, 0, 0]


def unseen_generalized_state(seen_states, state):
    g_per_f = [0, 0, 0, 0]
    m_per_f = [0, 0, 0, 0]
    state_len = len(state)
    for i in range(2, state_len, 2):
        g_per_f[state[i]] += 1
    for i in range(3, state_len, 2):
        m_per_f[state[i]] += 1
    state_arr = [state[1]] + g_per_f + m_per_f
    state_str = "".join(map(str, state_arr))
    if state_str in seen_states:
        return False
    else:
        seen_states.add(state_str)
        return True


def unseen_state(seen_states, state):
    state_str = "".join(map(str, state))
    if state_str in seen_states:
        return False
    else:
        seen_states.add(state_str)
        return True


def valid_state(state):
    state_len = len(state)
    for m in range(3, state_len, 2):
        # unshielded microchip
        if state[m] != state[m - 1]:
            for g in range(2, state_len, 2):
                if state[m] == state[g]:
                    return False
    return True


def next_states(queue, state):
    state_len = len(state)
    directions = []
    e = state[1]
    if e != 3:
        directions += [1]
    if e != 0:
        for i in range(2, state_len):
            if state[i] < e:
                directions += [-1]
                break
    for d in directions:
        for i_1 in range(2, state_len):
            if state[i_1] == e:
                state_1 = state[:]
                state_1[0] += 1
                state_1[1] += d
                state_1[i_1] += d
                queue.append(state_1[:])
                for i_2 in range(i_1 + 1, state_len):
                    if state[i_2] == e:
                        state_2 = state_1[:]
                        state_2[i_2] += d
                        queue.append(state_2[:])


def solved(state):
    for i in range(2, len(state)):
        if state[i] < 3:
            return False
    return True


def bfs(init_state):
    queue = deque()
    queue.append(init_state)
    seen_states = set()

    while queue:
        state = queue.popleft()
        if valid_state(state):
            if unseen_generalized_state(seen_states, state):
                if solved(state):
                    return state[0]
                next_states(queue, state)
    return "No solution"


print(f"Part One: {bfs(part1_init_state)}")
print(f"Part Two: {bfs(part2_init_state)}")
