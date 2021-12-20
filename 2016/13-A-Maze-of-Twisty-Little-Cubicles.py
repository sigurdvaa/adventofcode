from collections import deque

input_raw = "1362"


def unseen(seen, state):
    loc = (state[1], state[2])
    if loc in seen:
        return False
    else:
        seen.add(loc)
        return True


def next_states(queue, state):
    state[0] += 1
    state[1] += 1
    queue.append(state[:])
    state[1] -= 2
    queue.append(state[:])
    state[1] += 1
    state[2] += 1
    queue.append(state[:])
    state[2] -= 2
    queue.append(state[:])


def valid(state, fav):
    x = state[1]
    y = state[2]
    if x < 0 or y < 0:
        return False
    loc_value = x * x + 3 * x + 2 * x * y + y + y * y + fav
    on_bits = str(bin(loc_value))[2:].count("1")
    if on_bits % 2 == 0:
        return True
    else:
        return False


def solved(state, end):
    if state[1] == end[0]:
        if state[2] == end[1]:
            return True
    return False


def bfs(init, fav, end: tuple = (0, 0), seen_at_step: int = -1):
    queue = deque()
    queue.append(init)
    seen = set()

    while queue:
        state = queue.popleft()
        if seen_at_step > -1:
            if state[0] > seen_at_step:
                return len(seen)
        if valid(state, fav) and unseen(seen, state):
            if solved(state, end):
                return state[0]
            next_states(queue, state)

    return "No solution"


init_state = [0, 1, 1]
print(f"Part One: {bfs(init_state, int(input_raw), end = (31, 39))}")
init_state = [0, 1, 1]
print(f"Part Two: {bfs(init_state, int(input_raw), end = (-1, -1), seen_at_step = 50)}")
