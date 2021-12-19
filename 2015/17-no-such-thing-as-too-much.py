input_raw = """50
44
11
49
42
46
18
32
26
40
21
7
18
43
10
47
36
24
22
40"""


def all_combinations(items: list):
    combinations = []
    for item in items:
        new_combinations = combinations.copy()
        for c in combinations:
            new_combinations.append(c + [item])
        new_combinations.append([item])
        combinations = new_combinations

    return combinations


def num_combinations_fit(combinations: list, liters: int):
    num = 0
    for combination in combinations:
        if sum(combination) == liters:
            num += 1

    return num


def num_combinations_fit_min(combinations: list, liters: int):
    num = 0
    min_size = -1
    for combination in combinations:
        if sum(combination) == liters:
            if min_size == -1:
                min_size = len(combination)
                num = 1
            else:
                if min_size > len(combination):
                    min_size = len(combination)
                    num = 0
                elif min_size == len(combination):
                    num += 1

    return num


liters = 150
containers = [int(x) for x in input_raw.splitlines()]
combinations = all_combinations(containers)
print(f"Part One: {num_combinations_fit(combinations, liters)}")
print(f"Part Two: {num_combinations_fit_min(combinations, liters)}")
