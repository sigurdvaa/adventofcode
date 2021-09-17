from copy import deepcopy

input_raw = """The first floor contains a promethium generator and a promethium-compatible microchip.
The second floor contains a cobalt generator, a curium generator, a ruthenium generator, and a plutonium generator.
The third floor contains a cobalt-compatible microchip, a curium-compatible microchip, a ruthenium-compatible microchip, and a plutonium-compatible microchip.
The fourth floor contains nothing relevant."""

init_state = [
    set(["E","PrG", "PrM"]),
    set(["CoG", "CuG", "RuG", "PlG"]),
    set(["CoM", "CuM", "RuM", "PlM"]),
    set(["E"]),
]

Qinit_state = [
    set(["E", "HyM", "LiM"]),
    set(["HyG"]),
    set(["LiG"]),
    set(),
]

queue = [deepcopy(init_state)]
seen_states = [deepcopy(init_state)]

def get_next_states(state):
    next_states = []
    # find and remove elevator
    e = -1
    for f in range(4):
        for item in state[f]:
            if item == "E":
                e = f
                break
        if e != -1:
            break
    state[e].remove("E")

    # items on elevator floor can be moved
    items = list(state[e])
    directions = []
    if e != 3:
        directions += [1]
    if e != 0:
        directions += [-1]

    items_len = len(items)
    # for each possible direction
    for d in directions:    
        # new state after moving 1 item
        for i_1 in range(items_len):
            new_state_1 = deepcopy(state)
            new_state_1[e].remove(items[i_1])
            new_state_1[e+d].add("E")
            new_state_1[e+d].add(items[i_1])
            next_states += [deepcopy(new_state_1)]
            # new state after moving 2 items
            for i_2 in range(i_1+1, items_len):
                new_state_2 = deepcopy(new_state_1)
                new_state_2[e].remove(items[i_2])
                new_state_2[e+d].add(items[i_2])
                next_states += [deepcopy(new_state_2)]
    return next_states

def get_unseen_states(states):
    global seen_states
    i = 0
    while i < len(states):
        seen = False
        for seen_state in seen_states:
            match = 0
            for f in range(4):
                if states[i][f] == seen_state[f]:
                    match += 1
            if match == 4:
                seen = True
                del states[i]
                break
        if not seen:
            seen_states += [deepcopy(states[i])]
            i += 1
    return states

def valid_state(state):
    for floor in state:
        microchips = []
        generators = []
        for item in floor:
            if item[-1] == "M":
                microchips += [item]
            elif item[-1] == "G":
                generators += [item]
        if len(generators) > 0:
            for m in microchips:
                shielded = False
                for g in generators:
                    if m[:2] == g[:2]:
                        shielded = True
                        break
                if not shielded:
                    return False
    return True

def get_valid_states(states):
    i = 0
    while i < len(states):
        if not valid_state(states[i]):
            del states[i]
        else:
            i += 1
    return states

i = 0
searching = True
while searching:
    i += 1
    new_queue = []
    print(f"step: {i}, queue: {len(queue)}, seen states: {len(seen_states)}")
    for state in queue:
        next_possible_states = get_next_states(state)
        next_valid_states = get_valid_states(next_possible_states)
        next_unseen_states = get_unseen_states(next_valid_states)
        new_queue += next_unseen_states
    for state in new_queue:
        searching = False
        for f in range(3):
            if len(state[f]) > 0:
                searching = True
                break
        if not searching:
            break
    queue = new_queue

print(f"Part One: {i}")

