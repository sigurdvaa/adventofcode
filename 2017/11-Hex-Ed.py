import math


with open("11_input.txt", "r") as f:
    input_raw = f.read()


def stepsBack(x, y):
    x = abs(x)
    y = abs(y)
    steps = 0
    while x > 0 or y > 0:
        if x == 0:
            y -= 1
        elif y == 0:
            x -= 1
        else:
            x -= 1
            y -= 0.5
        steps += 1

    return steps


x = 0
y = 0
maxSteps = 0
highx = 0
highy = 0
for d in input_raw.split(","):

    if d == "n":
        y -= 1
    elif d == "ne":
        x += 1
        y -= 0.5
    elif d == "se":
        x += 1
        y += 0.5
    elif d == "s":
        y += 1
    elif d == "sw":
        x -= 1
        y += 0.5
    elif d == "nw":
        x -= 1
        y -= 0.5

    if abs(x) > highx:
        highx = abs(x)
        steps = stepsBack(x, y)
        if steps > maxSteps:
            maxSteps = steps
    elif abs(y) > highy:
        highy = abs(x)
        steps = stepsBack(x, y)
        if steps > maxSteps:
            maxSteps = steps

print("Part 1")
print(stepsBack(x, y))

print("Part 2")
print(maxSteps)
