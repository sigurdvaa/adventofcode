with open("05_input.txt", "r") as f:
    input_raw = f.read()


print("Part 1")
jumps = [int(x) for x in input_raw.splitlines()]
length = len(jumps)
i = 0
steps = 0
while i < length:
    steps += 1
    jump = jumps[i]
    if jump != 0:
        jumps[i] += 1
        i += jump
    else:
        jumps[i] = 2
        steps += 1
        i += 1
print(steps)

print("Part 2")
jumps = [int(x) for x in input_raw.splitlines()]
length = len(jumps)
i = 0
steps = 0
while i < length:
    steps += 1
    jump = jumps[i]
    if jump < 3:
        if jump != 0:
            jumps[i] += 1
            i += jump
        else:
            jumps[i] = 2
            steps += 1
            i += 1
    else:
        jumps[i] -= 1
        i += jump
print(steps)
