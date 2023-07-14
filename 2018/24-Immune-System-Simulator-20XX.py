from __future__ import annotations

from copy import copy
from dataclasses import dataclass
from typing import Optional


@dataclass
class Group:
    units: int
    hp: int
    dmg: int
    dmg_type: str
    init: int
    weak: list[str]
    immune: list[str]
    target: Optional[Group] = None
    targeted: bool = False

    @property
    def effective(self) -> int:
        return self.units * self.dmg


def parse_weakness_and_immune(line: str) -> tuple[list[str], list[str]]:
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
        weak, immune = parse_weakness_and_immune(line[attr_start + 1 : attr_end])
    else:
        weak, immune = [], []

    return Group(units, hp, dmg, dmg_type, init, weak, immune)


def parse_all_groups(lines: list[str]) -> tuple[list[Group], list[Group]]:
    immune_start = lines.index("Immune System:")
    infection_start = lines.index("Infection:")
    immune_system = []
    infection = []

    for i in range(immune_start + 1, infection_start - 1):
        immune_system.append(parse_group(lines[i]))
    for i in range(infection_start + 1, len(lines)):
        infection.append(parse_group(lines[i]))

    return immune_system, infection


def del_dead(groups: list[Group]) -> None:
    for i in range(len(groups) - 1, -1, -1):
        if groups[i].units < 1:
            del groups[i]


def target_selection(attackers: list[Group], defenders: list[Group]) -> None:
    attackers.sort(key=lambda x: (x.effective, x.init), reverse=True)
    defenders.sort(key=lambda x: (x.effective, x.init), reverse=True)

    for g in defenders:
        g.targeted = False

    for att_g in attackers:
        att_g.target = None

        targets = [x for x in defenders if not x.targeted and att_g.dmg_type in x.weak]
        targets.extend(
            [x for x in defenders if not x.targeted and att_g.dmg_type not in x.immune],
        )
        if targets:
            att_g.target = targets[0]
            targets[0].targeted = True


def attacking(immune_system: list[Group], infection: list[Group]) -> None:
    all_groups = immune_system + infection
    all_groups.sort(key=lambda x: x.init, reverse=True)
    for g in all_groups:
        if not g.target or g.units < 1:
            continue
        dmg_mul = 1
        if g.dmg_type in g.target.weak:
            dmg_mul = 2
        g.target.units -= (g.effective * dmg_mul) // g.target.hp

    del_dead(immune_system)
    del_dead(infection)


def fight_loop(immune_system: list[Group], infection: list[Group]) -> tuple[int, int]:
    units = sum(x.units for x in immune_system), sum(x.units for x in infection)
    while len(immune_system) and len(infection):
        target_selection(immune_system, infection)
        target_selection(infection, immune_system)
        attacking(immune_system, infection)
        new_units = sum(x.units for x in immune_system), sum(x.units for x in infection)
        if new_units == units:
            return units
        units = new_units
    return units


def boost_immune_system(immune_system: list[Group], infection: list[Group]) -> int:
    units = fight_loop(
        [copy(x) for x in immune_system],
        [copy(x) for x in infection],
    )
    while units[1]:
        for g in immune_system:
            g.dmg += 1
        units = fight_loop(
            [copy(x) for x in immune_system],
            [copy(x) for x in infection],
        )

    return units[0]


# test
with open("24_test.txt") as fp:
    input_test = [x.strip() for x in fp.readlines()]
immune_system, infection = parse_all_groups(input_test)
assert len(immune_system) == 2
assert len(infection) == 2
assert sum(fight_loop(immune_system, infection)) == 5216
immune_system, infection = parse_all_groups(input_test)
assert boost_immune_system(immune_system, infection) == 51


# answer
with open("24_input.txt") as fp:
    input_raw = [x.strip() for x in fp.readlines()]
immune_system, infection = parse_all_groups(input_raw)
print(f"Part One: {sum(fight_loop(immune_system, infection))}")
immune_system, infection = parse_all_groups(input_raw)
print(f"Part Two: {boost_immune_system(immune_system, infection)}")
