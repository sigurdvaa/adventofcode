with open("01_input.txt", "r") as f:
    input_raw = f.readline().strip()


def target_floor(instructions: str):
    floor = 0
    for i in instructions:
        if i == "(":
            floor += 1
        else:
            floor -= 1
    return floor


def first_basement(instructions: str):
    floor = 0
    for i in range(len(instructions)):
        if instructions[i] == "(":
            floor += 1
        else:
            floor -= 1
        if floor == -1:
            return i + 1
    return -1


print(f"Part One: {target_floor(input_raw)}")
print(f"Part Two: {first_basement(input_raw)}")
