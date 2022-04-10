input_raw = """1
3
5
11
13
17
19
23
29
31
37
41
43
47
53
59
67
71
73
79
83
89
97
101
103
107
109
113"""


def product(numbers: list) -> int:
    product = 1
    for num in numbers:
        product *= num
    return product


def smallest_groups(packages: list, int_groups: int) -> list:
    group_weight = sum(packages) // int_groups
    groups = set([(x,) for x in packages if x <= group_weight])
    while True:
        next_groups = set()
        for group in groups:
            current_weight = sum(group)
            if current_weight == group_weight:
                return set([x for x in groups if sum(x) == group_weight])
            else:
                for weight in packages:
                    if weight not in group:
                        if current_weight + weight <= group_weight:
                            new_group = tuple(sorted(group + (weight,)))
                            next_groups.add(new_group)
        groups = next_groups


def qe_smallest_group(packages: list, int_groups: int = 3) -> int:
    groups = smallest_groups(packages, int_groups)
    groups = sorted(groups, key=lambda x: (product(x)))
    groups = sorted(groups, key=len)
    return product(groups[0])


packages = [int(x) for x in input_raw.splitlines()]
print(f"Part One: { qe_smallest_group(packages) }")
print(f"Part Two: { qe_smallest_group(packages, 4) }")
