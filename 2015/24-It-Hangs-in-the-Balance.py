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


def all_arrangements(packages: list, int_groups: int = 3) -> list:
    arrangements = []
    group_weight = sum(packages) // int_groups
    groups = [[x] for x in packages if x <= group_weight]
    find_groups = True
    while find_groups:
        find_groups = False
        next_groups = []
        for group in groups:
            current = sum(group)
            if current == group_weight:
                next_groups.append(group)
            else:
                for weight in packages:
                    if current + weight <= group_weight:
                        if weight not in group:
                            find_groups = True
                            next_groups.append(group.copy() + [weight])
        groups = next_groups
    for p in sorted(groups, key=len):
        print(p)

    return arrangements


packages = [int(x) for x in input_raw.splitlines()]
packages = [1, 2, 3, 4, 5, 7, 8, 9, 10, 11]
all_arrangements(packages)
