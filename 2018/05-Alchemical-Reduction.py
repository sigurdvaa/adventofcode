with open("05_input.txt", "r") as f:
    input_raw = f.readline()


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
                i += 1
            elif (
                polymer[i].lower() == polymer[i + 1].lower()
                and polymer[i] != polymer[i + 1]
            ):
                reacting = True
                i += 2
            else:
                reduced_polymer.append(polymer[i])
                i += 1
        polymer = reduced_polymer

    return len(polymer)


print(f"Part One: { polymer_len_after_react(input_raw) }")

# 10709 high
# 10034 low
