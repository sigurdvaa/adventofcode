with open("12_input.txt", "r") as fp:
    input_raw = [x.strip() for x in fp.readlines()]


def parse_combinations(strings: list[str]) -> dict[str, str]:
    combinations = {}
    for line in strings:
        split = line.split(" => ")
        combinations[split[0]] = split[1]
    return combinations


def pots_gens(state: str, combinations: dict[str, str], target_gen: int = 20) -> int:
    states: dict[str, str] = {}
    zero_index: int = 0
    while state[:3] != "...":
        state = "." + state
        zero_index += 1
    while state[-3:] != "...":
        state = state + "."

    gen: int = 0
    while gen < target_gen:
        if state in states:
            next_state, zero_index_diff = states[state]
            if state == next_state:
                zero_index -= zero_index_diff * (target_gen - gen)
                break
            state = next_state
            zero_index -= zero_index_diff
        else:
            prev_zero_index = zero_index
            next_state: list[str] = [".", "."]

            for i in range(2, len(state) - 2):
                next_state.append(combinations[state[i - 2 : i + 3]])

            while next_state[:3] != [".", ".", "."]:
                next_state.insert(0, ".")
                zero_index += 1
            while next_state[3] != "#":
                next_state.pop(0)
                zero_index -= 1
            while next_state[-3:] != [".", ".", "."]:
                next_state.append(".")
            while next_state[-4] != "#":
                next_state.pop()

            states[state] = ("".join(next_state), prev_zero_index - zero_index)
            state = states[state][0]

        gen += 1

    sum_pots: int = 0
    for i in range(len(state)):
        if state[i] == "#":
            sum_pots += i - zero_index
    return sum_pots


init_state = input_raw[0][15:]
combinations = parse_combinations(input_raw[2:])
print(f"Part One: {pots_gens(init_state, combinations)}")
print(f"Part Two: {pots_gens(init_state, combinations, 50_000_000_000)}")
