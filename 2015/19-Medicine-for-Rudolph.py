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


replacements = parse_replacements(input_raw[:-2])
molecule = input_raw[-1].strip()
print(f"Part One: {len(possible_replacements(replacements, molecule))}")
# 506
