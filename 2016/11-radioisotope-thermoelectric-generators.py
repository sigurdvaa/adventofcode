from copy import deepcopy

input_raw = """The first floor contains a promethium generator and a promethium-compatible microchip.
The second floor contains a cobalt generator, a curium generator, a ruthenium generator, and a plutonium generator.
The third floor contains a cobalt-compatible microchip, a curium-compatible microchip, a ruthenium-compatible microchip, and a plutonium-compatible microchip.
The fourth floor contains nothing relevant."""

floors2 = [
    {"E","PrG", "PrM"},
    {"CoG", "CuG", "RuG", "PlG"},
    {"CoM", "CuM", "RuM", "PlM"},
    {"E"},
]

init_state = [
    {"E","HM", "LM"},
    {"HG"},
    {"LG"},
    {},
]

queue = []
seen_states = [deepcopy(init_state)]

def get_next_states(state):
    next_states = []

    # find elevator
    e = -1
    for f in range(4):
        for item in state[f]:
            if item == "E":
                e = f
                break
        if e != -1:
            break

    # items on elevator floor can be moved
    items = state[e].copy()
    items.remove("E")
    directions = []
    if e != 3:
        directions += [1]
    if e != 0:
        directions += [-1]

    # for each possible direction
    for d in directions:
        
        # new state after moving 1 item
        for item_1 in items:
            new_state = deepcopy(state)
            new_state[e].remove("E")
            new_state[e].remove(item_1)
            new_state[e+d].add("E")
            new_state[e+d].add(item_1)
            next_states += [deepcopy(new_state)]

            # new state after moving 2 items
            for item_2 in items:
                if item_1 != item_2:
                    new_state = deepcopy(state)
                    new_state[e].remove("E")
                    new_state[e].remove(item_1)
                    new_state[e].remove(item_2)
                    new_state[e+d].add("E")
                    new_state[e+d].add(item_1)
                    new_state[e+d].add(item_2)
                    next_states += [deepcopy(new_state)]
    return next_states

def get_unseen_states(states):
    global seen_states
    for state in states:
        seen = False
        for seen_state in seen_states:
            match = 0
            for i in range(4):
                if state[i] == seen_state[i]:
                    match += 1
            if match == 4:
                seen = True
                states.remove(state)
                break
        if not seen:
            seen_states += [deepcopy(states[s])]
    return states

def get_valid_states(states):
    pass

next_possible_states = get_next_states(init_state)
next_unseen_states = get_unseen_states(next_possible_states)
next_valid_state = []

