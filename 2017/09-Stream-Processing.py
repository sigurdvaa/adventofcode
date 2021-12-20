with open("09_input.txt", "r") as f:
    input_raw = f.read()


def parse(string, part=1):
    garbage = False
    level = 0
    score = 0
    crap = 0

    c = 0
    while c < len(string):
        if garbage:
            if string[c] == "!":
                c += 1
            elif string[c] == ">":
                garbage = False
            else:
                crap += 1
        elif string[c] == "<":
            garbage = True
        elif string[c] == "{":
            level += 1
        elif string[c] == "}":
            score += level
            level -= 1
        c += 1

    if part == 1:
        return score
    else:
        return crap


print("Part 1")
print(parse(input_raw, 1))
print("Part 2")
print(parse(input_raw, 2))
