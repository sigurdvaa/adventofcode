with open("12_input.txt", "r") as fp:
    input_raw = [x.strip() for x in fp.readlines()]


def parse_combos(strings: list[str]) -> dict[str, str]:
    combos = {}
    for line in strings:
        split = line.split(" => ")
        combos[split[0]] = split[1]
    return combos


def pot_gens(state: str, combos: dict[str, str], target_gen: int) -> int:
    states: dict[str, tuple[str, int]] = {}
    zero_index: int = 0
    while state[:3] != "...":
        state = "." + state
        zero_index += 1
    while state[-3:] != "...":
        state = state + "."

    gen: int = 0
    while gen < target_gen:
        if state in states:
            seen_state, zero_index_diff = states[state]
            if state == seen_state:
                zero_index += zero_index_diff * (target_gen - gen)
                break
            state = seen_state
            zero_index += zero_index_diff
        else:
            prev_zero_index = zero_index
            next_state: list[str] = [".", "."]

            for i in range(2, len(state) - 2):
                next_state.append(combos[state[i - 2 : i + 3]])

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

            states[state] = ("".join(next_state), zero_index - prev_zero_index)
            state = states[state][0]

        gen += 1

    sum_pots: int = 0
    for i in range(len(state)):
        if state[i] == "#":
            sum_pots += i - zero_index
    return sum_pots


init_state = input_raw[0][15:]
combos = parse_combos(input_raw[2:])
print(f"Part One: {pot_gens(init_state, combos, 20)}")
print(f"Part Two: {pot_gens(init_state, combos, 50_000_000_000)}")
