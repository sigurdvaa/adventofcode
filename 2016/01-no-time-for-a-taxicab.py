input_raw = "R1, L3, R5, R5, R5, L4, R5, R1, R2, L1, L1, R5, R1, L3, L5, L2, R4, L1, R4, R5, L3, R5, L1, R3, L5, R1, L2, R1, L5, L1, R1, R4, R1, L1, L3, R3, R5, L3, R4, L4, R5, L5, L1, L2, R4, R3, R3, L185, R3, R4, L5, L4, R48, R1, R2, L1, R1, L4, L4, R77, R5, L2, R192, R2, R5, L4, L5, L3, R2, L4, R1, L5, R5, R4, R1, R2, L3, R4, R4, L2, L4, L3, R5, R4, L2, L1, L3, R1, R5, R5, R2, L5, L2, L3, L4, R2, R1, L4, L1, R1, R5, R3, R3, R4, L1, L4, R1, L2, R3, L3, L2, L1, L2, L2, L1, L2, R3, R1, L4, R1, L1, L4, R1, L2, L5, R3, L5, L2, L2, L3, R1, L4, R1, R1, R2, L1, L4, L4, R2, R2, R2, R2, R5, R1, L1, L4, L5, R2, R4, L3, L5, R2, R3, L4, L1, R2, R3, R5, L2, L3, R3, R1, R3"
input_steps = [x.strip() for x in input_raw.split(",")]

x = 0
y = 0
direction = 0
been = set()
been_twice = False
been_twice_tuple = tuple()
for step in input_steps:
    turn = step[0]
    blocks = int(step[1:])

    if turn == "R":
        direction += 1
        if direction > 3:
            direction = 0
    elif turn == "L":
        direction -= 1
        if direction < 0:
            direction = 3

    for walk in range(blocks):
        if direction == 0:
            y -= 1
        elif direction == 1:
            x += 1
        elif direction == 2:
            y += 1
        elif direction == 3:
            x -= 1

        if not been_twice:
            if (x, y) not in been:
                been.add((x, y))
            else:
                been_twice = True
                been_twice_tuple = (x, y)

print("Part One: " + str(abs(x) + abs(y)))
print("Part Two: " + str(abs(been_twice_tuple[0]) + abs(been_twice_tuple[1])))
