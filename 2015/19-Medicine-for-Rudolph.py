with open("19_input.txt", "r") as f:
    input_raw = f.readlines()


def parse_replacements(strings: list):
    replacements = {}
    for line in strings:
        split = line.strip().split(" => ")
        if split[0] in replacements:
            replacements[split[0]].append(split[1])
        else:
            replacements[split[0]] = [split[1]]

    return replacements


def possible_replacements(replacements: dict, molecule: str):
    possible = set()
    for key in replacements:
        size = len(key)
        for i in range(len(molecule) + 1 - size):
            if molecule[i : i + size] == key:
                for r in replacements[key]:
                    possible.add(molecule[:i] + r + molecule[i + size :])

    return possible


def steps_to_molecule(replacements: dict, molecule: str):
    steps = {}
    token = []
    last = len(molecule) - 1
    for i, v in enumerate(molecule):
        token.append(v)
        if i == last or molecule[i + 1].isupper():
            curr_token = "".join(token)
            token = []
            if curr_token in steps:
                steps[curr_token] += 1
            else:
                steps[curr_token] = 1

    num_steps = -1
    for k in steps:
        if k in replacements:
            num_steps += steps[k]
        elif k == "Y":
            num_steps -= steps[k]

    return num_steps


replacements = parse_replacements(input_raw[:-2])
molecule = input_raw[-1].strip()
print(f"Part One: {len(possible_replacements(replacements, molecule))}")
print(f"Part Two: {steps_to_molecule(replacements, molecule)}")
