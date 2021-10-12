with open("02-input.txt", "r") as f:
    input_sizes = [
        list(map(int, x)) for x in [x.strip().split("x") for x in f.readlines()]
    ]


def total_wrapping(sizes: list):
    total = 0
    for size in sizes:
        side_a = size[0] * size[1]
        side_b = size[0] * size[2]
        side_c = size[1] * size[2]
        area = side_a + side_b + side_c
        area *= 2

        if side_a <= side_b and side_a <= side_c:
            area += side_a
        elif side_b <= side_a and side_b <= side_c:
            area += side_b
        else:
            area += side_c

        total += area

    return total


def total_ribbon(sizes: list):
    total = 0
    for size in sizes:
        size.sort()
        side_a = size[0] * 2
        side_b = size[1] * 2
        volume = size[0] * size[1] * size[2]

        total += side_a + side_b + volume

    return total


print(f"Part One: {total_wrapping(input_sizes)}")
print(f"Part Two: {total_ribbon(input_sizes)}")
