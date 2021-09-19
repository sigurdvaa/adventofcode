input_raw = """The first floor contains a promethium generator and a promethium-compatible microchip.
The second floor contains a cobalt generator, a curium generator, a ruthenium generator, and a plutonium generator.
The third floor contains a cobalt-compatible microchip, a curium-compatible microchip, a ruthenium-compatible microchip, and a plutonium-compatible microchip.
The fourth floor contains nothing relevant."""

# items and which floor they're on (state)
test_items = ["e", "hyg", "hym", "lig", "lim"]
test_init_state = [0, 1, 0, 2, 0]
part1_items = ["e", "cog", "com", "cug", "cum", "plg", "plm", "prg", "prm", "rug", "rum"]
part1_init_state = [0, 1, 2, 1, 2, 1, 2, 0, 0, 1, 2]
part2_items = ["e", "cog", "com", "cug", "cum", "plg", "plm", "prg", "prm", "rug", "rum", "elg", "elm", "dig", "dim"]
part2_init_state = [0, 1, 2, 1, 2, 1, 2, 0, 0, 1, 2, 0, 0, 0, 0]

def unseen_state(seen_states, state):
    #state_str = "".join([str(x) for x in state])
    state_str = "".join(map(str, state))
    if state_str in seen_states:
        return False
    else:
        seen_states.add(state_str)
        return True

def valid_state(state):
    state_len = len(state)
    for m in range(2, state_len, 2):
        # unshielded microchip
        if state[m] != state[m-1]:
            for g in range(1, state_len, 2):
                if state[m] == state[g]:
                    return False
    return True

def next_states(state):
    new_states = []
    directions = []
    e = state[0]
    if e != 3:
        directions += [1]
    if e != 0:
        directions += [-1]

    state_len = len(state)
    for d in directions:

        for i_1 in range(1, state_len):
            if state[i_1] == e:
                state_1 = state.copy()
                state_1[0] += d
                state_1[i_1] += d
                new_states += [state_1.copy()]

                for i_2 in range(i_1 + 1, state_len):
                    if state[i_2] == e:
                        state_2 = state_1.copy()
                        state_2[i_2] += d
                        new_states += [state_2.copy()]
    return new_states

def bfs(init_state):
    queue = [init_state]
    seen_states = {"".join([str(x) for x in init_state])}

    step = 0
    solved = False
    while not solved:
        step += 1
        new_queue = []
        print(f"step: {step}, queue: {len(queue)}, seen: {len(seen_states)}")
        for state in queue:
            new_states = next_states(state)
            for i in range(len(new_states) - 1, -1, -1):
                if not unseen_state(seen_states, new_states[i]):
                    del new_states[i] 
                elif not valid_state(new_states[i]):
                    del new_states[i]
            new_queue += new_states
        queue = new_queue
        for state in queue:
            solved = True
            for i in state:
                if i < 3:
                    solved = False
            if solved:
             break

    return step

print(f"Part One: {bfs(part1_init_state)}")
print(f"Part Two: {bfs(part2_init_state)}")

