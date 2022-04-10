with open("01_input.txt", "r") as f:
    input_raw = f.readlines()


def new_frequency(changes: list) -> int:
    frequency = 0
    for change in changes:
        frequency += change
    return frequency


def new_frequency_twice(changes: list) -> int:
    frequency = 0
    seen_frequency = set()
    seen_frequency.add(frequency)
    while True:
        for change in changes:
            frequency += change
            if frequency in seen_frequency:
                return frequency
            seen_frequency.add(frequency)


frequency_changes = [int(x) for x in input_raw]
print(f"Part One: { new_frequency(frequency_changes) }")
print(f"Part Two: { new_frequency_twice(frequency_changes) }")
