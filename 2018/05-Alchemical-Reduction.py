with open("05_input.txt", "r") as f:
    input_raw = f.readline().strip()


def polymer_len_after_react(polymer: str) -> int:
    polymer = list(polymer)

    reacting = True
    while reacting:
        reacting = False
        reduced_polymer = []
        i = 0
        while i < len(polymer):
            if i == len(polymer) - 1:
                reduced_polymer.append(polymer[i])
            elif (
                polymer[i].lower() == polymer[i + 1].lower()
                and polymer[i] != polymer[i + 1]
            ):
                reacting = True
                i += 1
            else:
                reduced_polymer.append(polymer[i])
            i += 1

        polymer = reduced_polymer

    return len(polymer)


def polymer_len_after_react2(polymer: str) -> int:
    from collections import namedtuple
    Unit = namedtuple("Unit", ["type", "polarity", "reacted"])
    polymer = [Unit(x.lower(), 0 if ord(x) > 96 else 1, False) for x in polymer]

    reacting = True
    while reacting:
        reacting = False
        i = 0
        prev_i = -1
        while i < len(polymer):
            if polymer[i].reacted:
                pass
            elif prev_i == -1:
                prev_i = i
            if polymer[i].type == polymer[prev_i].type and polymer[i].polarity != polymer[prev_i].polarity:
                reacting = True
                polymer[i] = Unit(polymer[i].type, polymer[i].polarity, True)
                polymer[prev_i] = Unit(polymer[prev_i].type, polymer[prev_i].polarity, True)
                prev_i = -1
            i += 1

    polymer_len = 0
    for u in polymer:
        if u.reacted == 0:
            polymer_len += 1

    return polymer_len


def polymer_len_after_best_react(polymer: str) -> int:
    units = set()
    for u in polymer:
        units.add(u.lower())

    min_polymer_len = len(polymer)
    for u in units:
        new_polymer = polymer.translate({ord(u): None, ord(u.upper()): None})
        polymer_len = polymer_len_after_react2(new_polymer)
        if polymer_len < min_polymer_len:
            min_polymer_len = polymer_len

    return min_polymer_len


print(f"Part One: { polymer_len_after_react2(input_raw) }")
#print(f"Part Two: { polymer_len_after_best_react(input_raw) }")
