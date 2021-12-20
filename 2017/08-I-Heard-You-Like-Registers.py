with open("08_input.txt", "r") as f:
    input_raw = f.read()


registers = {}
instructions = input_raw.splitlines()
cases = ["<", ">", "<=", ">=", "=="]
for line in instructions:
    r = line.split(" ")[0]
    registers[r] = 0


def conditionMet(instruction):
    r = instruction[4]
    case = instruction[5]
    value = int(instruction[6])
    if case == "==":
        if registers[r] == value:
            return True
    elif case == "<":
        if registers[r] < value:
            return True
    elif case == ">":
        if registers[r] > value:
            return True
    elif case == "<=":
        if registers[r] <= value:
            return True
    elif case == ">=":
        if registers[r] >= value:
            return True
    elif case == "!=":
        if registers[r] != value:
            return True
    return False


highestInt = 0
for instruction in instructions:
    instruction = instruction.split(" ")

    # If condition met
    if conditionMet(instruction):
        r = instruction[0]
        action = instruction[1]
        value = int(instruction[2])

        # Do action
        if action == "inc":
            registers[r] += value
        elif action == "dec":
            registers[r] -= value

        if registers[r] > highestInt:
            highestInt = registers[r]

maxAfter = 0
for r in registers:
    if registers[r] > maxAfter:
        maxAfter = registers[r]

print("Part 1\nHighest value after: " + str(maxAfter))
print("Part 2\nHighest value during: " + str(highestInt))
