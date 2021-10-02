with open("21-input.txt", "r") as f:
    scramble_instructions = f.readlines()


def move_pos(string: list, x: int, y: int):
    temp = string[x]
    del string[x]
    string.insert(y, temp)
    return string


def reverse_pos(string: list, x: int, y: int):
    y += 1
    new_string = string[:x]
    new_string += reversed(string[x:y])
    new_string += string[y:]
    return new_string


def rotate_left(string: list, steps: int):
    strlen = len(string)
    steps = steps % strlen
    new_string = string[steps:]
    new_string += string[:steps]
    return new_string


def rotate_right(string: list, steps: int):
    strlen = len(string)
    steps = steps % strlen
    steps = strlen - steps
    new_string = string[steps:]
    new_string += string[:steps]
    return new_string


def rotate_left_letter(string: list, a: str):
    for i in range(len(string)):
        new_string = string[i:]
        new_string += string[:i]
        if rotate_right_letter(new_string, a) == string:
            break
    return new_string


def rotate_right_letter(string: list, a: str):
    steps = string.index(a)
    if steps > 3:
        steps += 1
    steps += 1
    strlen = len(string)
    steps = steps % strlen
    steps = strlen - steps
    new_string = string[steps:]
    new_string += string[:steps]
    return new_string


def swap_letters(string: list, a: str, b: str):
    idx_a = string.index(a)
    idx_b = string.index(b)
    string[idx_a] = b
    string[idx_b] = a
    return string


def swap_pos(string: list, x: int, y: int):
    temp = string[x]
    string[x] = string[y]
    string[y] = temp
    return string


def scramble(instructions: list, string: str):
    string = list(string)
    for ins in instructions:
        ins = ins.split()
        if ins[0] == "move":
            string = move_pos(string, int(ins[2]), int(ins[5]))

        elif ins[0] == "reverse":
            string = reverse_pos(string, int(ins[2]), int(ins[4]))

        elif ins[0] == "rotate":
            if ins[1] == "left":
                string = rotate_left(string, int(ins[2]))
            if ins[1] == "right":
                string = rotate_right(string, int(ins[2]))
            if ins[1] == "based":
                string = rotate_right_letter(string, ins[6])

        elif ins[0] == "swap":
            if ins[1] == "letter":
                string = swap_letters(string, ins[2], ins[5])
            elif ins[1] == "position":
                string = swap_pos(string, int(ins[2]), int(ins[5]))

    return "".join(string)


def scramble_reverse(instructions: list, string: str):
    string = list(string)
    for ins in reversed(instructions):
        ins = ins.split()
        if ins[0] == "move":
            string = move_pos(string, int(ins[5]), int(ins[2]))

        elif ins[0] == "reverse":
            string = reverse_pos(string, int(ins[2]), int(ins[4]))

        elif ins[0] == "rotate":
            if ins[1] == "left":
                string = rotate_right(string, int(ins[2]))
            if ins[1] == "right":
                string = rotate_left(string, int(ins[2]))
            if ins[1] == "based":
                string = rotate_left_letter(string, ins[6])

        elif ins[0] == "swap":
            if ins[1] == "letter":
                string = swap_letters(string, ins[5], ins[2])
            elif ins[1] == "position":
                string = swap_pos(string, int(ins[5]), int(ins[2]))

    return "".join(string)


print(f"Part One: {scramble(scramble_instructions, 'abcdefgh')}")
print(f"Part Two: {scramble_reverse(scramble_instructions, 'fbgdceah')}")
