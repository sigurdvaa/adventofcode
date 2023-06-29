from dataclasses import dataclass


@dataclass
class Group:
    units: int
    hp: int
    dmg: int
    dmg_type: str
    init: int
    weak: list[str]
    immune: list[str]


def parse_attr(line: str) -> tuple[list[str], list[str]]:
    weak = []
    immune = []
    split = line.split("; ")
    for attr_type in split:
        if attr_type.startswith("weak to"):
            weak.extend(attr_type[8:].split(", "))
        elif attr_type.startswith("immune to"):
            immune.extend(attr_type[10:].split(", "))
    return weak, immune


def parse_group(line: str) -> Group:
    split = line.split()

    units = int(split[0])
    hp = int(split[4])
    dmg = int(split[-6])
    dmg_type = split[-5]
    init = int(split[-1])

    if "(" in line:
        attr_start = line.index("(")
        attr_end = line.index(")")
        weak, immune = parse_attr(line[attr_start + 1 : attr_end])
    else:
        weak, immune = [], []

    return Group(units, hp, dmg, dmg_type, init, weak, immune)


def parse_groups(lines: list[str]) -> tuple[list[Group], list[Group]]:
    immune_start = lines.index("Immune System:")
    infection_start = lines.index("Infection:")
    immune_system = []
    infection = []

    for i in range(immune_start + 1, infection_start - 1):
        immune_system.append(parse_group(lines[i]))
    for i in range(infection_start + 1, len(lines)):
        infection.append(parse_group(lines[i]))

    return immune_system, infection


# test
with open("24_test.txt") as fp:
    input_test = [x.strip() for x in fp.readlines()]
immune_system, infection = parse_groups(input_test)
assert len(immune_system) == 2
assert len(infection) == 2


# answer
with open("24_input.txt") as fp:
    input_raw = [x.strip() for x in fp.readlines()]
immune_system, infection = parse_groups(input_raw)
