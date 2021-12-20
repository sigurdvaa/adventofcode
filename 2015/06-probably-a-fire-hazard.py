with open("06_input.txt", "r") as f:
    input_raw = [x.strip().split() for x in f.readlines()]


def run_lights(instructions: list, xsize: int = 1000, ysize: int = 1000):
    lights = [[0] * xsize for _ in range(ysize)]
    for ins in instructions:
        if ins[0] == "turn":
            xy_start = tuple(map(int, ins[2].split(",")))
            xy_end = tuple(map(int, ins[4].split(",")))
            if ins[1] == "on":
                state = 1
            else:
                state = 0
            for y in range(xy_start[1], xy_end[1] + 1):
                for x in range(xy_start[0], xy_end[0] + 1):
                    lights[y][x] = state
        elif ins[0] == "toggle":
            xy_start = tuple(map(int, ins[1].split(",")))
            xy_end = tuple(map(int, ins[3].split(",")))
            for y in range(xy_start[1], xy_end[1] + 1):
                for x in range(xy_start[0], xy_end[0] + 1):
                    if lights[y][x]:
                        lights[y][x] = 0
                    else:
                        lights[y][x] = 1

    return lights


def count_lit_lights(instructions):
    lights = run_lights(instructions)
    count = 0
    for y in range(len(lights)):
        count += sum(lights[y])

    return count


def run_lights_part2(instructions: list, xsize: int = 1000, ysize: int = 1000):
    lights = [[0] * xsize for _ in range(ysize)]
    for ins in instructions:
        if ins[0] == "turn":
            xy_start = tuple(map(int, ins[2].split(",")))
            xy_end = tuple(map(int, ins[4].split(",")))
            if ins[1] == "on":
                state = 1
            else:
                state = -1
            for y in range(xy_start[1], xy_end[1] + 1):
                for x in range(xy_start[0], xy_end[0] + 1):
                    lights[y][x] += state
                    if lights[y][x] < 0:
                        lights[y][x] = 0
        elif ins[0] == "toggle":
            xy_start = tuple(map(int, ins[1].split(",")))
            xy_end = tuple(map(int, ins[3].split(",")))
            for y in range(xy_start[1], xy_end[1] + 1):
                for x in range(xy_start[0], xy_end[0] + 1):
                    lights[y][x] += 2

    return lights


def count_brightness(instructions):
    lights = run_lights_part2(instructions)
    brightness = 0
    for y in range(len(lights)):
        brightness += sum(lights[y])

    return brightness


print(f"Part One: {count_lit_lights(input_raw)}")
print(f"Part Two: {count_brightness(input_raw)}")
