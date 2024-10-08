from typing import Iterator, Union

with open("15_input.txt", "r") as fp:
    input_raw = fp.read()


class Unit:
    def __init__(self, x: int, y: int, typ: str, dmg: int = 3):
        self.hp: int = 200
        self.dmg: int = dmg
        self.x: int = x
        self.y: int = y
        self.type: str = typ
        self.dead: bool = False

    def __repr__(self) -> str:
        return f"{self.type}:({self.x},{self.y}){self.hp}"

    def __lt__(self, other) -> bool:
        if self.y < other.y:
            return True
        if self.y == other.y and self.x < other.x:
            return True
        return False


class State:
    def __init__(self, options, path):
        self.options: set[tuple[int, int]] = options
        self.path: list[tuple[int, int]] = path

    def __lt__(self, other) -> bool:
        if self.path[-1][1] < other.path[-1][1]:
            return True
        if (
            self.path[-1][1] == other.path[-1][1]
            and self.path[-1][0] < other.path[-1][0]
        ):
            return True
        return False


class Battle:
    def __init__(self, x_max: int, y_max: int):
        self.unit_locs: set[tuple[int, int]] = set()
        self.area: set[tuple[int, int]] = set()
        self.units: list[Unit] = []
        self.x_max = x_max
        self.y_max = y_max

    def print_map(self):
        print("-" * 5, "Battle map", "-" * 5)
        for y in range(self.y_max):
            for x in range(self.x_max):
                curr_loc = (x, y)
                if curr_loc in self.unit_locs:
                    printed = False
                    for u in self.units:
                        if not u.dead and curr_loc == (u.x, u.y):
                            printed = True
                            print(u.type, end="")
                    if not printed:
                        print(".", end="")
                elif curr_loc in self.area:
                    print(".", end="")
                else:
                    print("#", end="")
            print()

    def add_unit(self, unit: Unit):
        self.units.append(unit)
        self.unit_locs.add((unit.x, unit.y))

    def attack(self, attacker: Unit) -> bool:
        targets: list[Unit] = []
        for unit in self.units:
            if not unit.dead and unit != attacker and unit.type != attacker.type:
                if unit.x == attacker.x:
                    if unit.y == attacker.y - 1 or unit.y == attacker.y + 1:
                        targets.append(unit)
                elif unit.y == attacker.y:
                    if unit.x == attacker.x - 1 or unit.x == attacker.x + 1:
                        targets.append(unit)
        if len(targets):
            targets.sort(reverse=True)
            target: Unit = targets[-1]
            for t in targets:
                if t.hp <= target.hp:
                    target = t
            target.hp -= attacker.dmg
            if target.hp < 1:
                target.dead = True
                self.unit_locs.remove((target.x, target.y))
            return True
        return False

    def next_states(self, state: State) -> Iterator[State]:
        next_locs = [
            (state.path[-1][0], state.path[-1][1] - 1),
            (state.path[-1][0] - 1, state.path[-1][1]),
            (state.path[-1][0] + 1, state.path[-1][1]),
            (state.path[-1][0], state.path[-1][1] + 1),
        ]
        for loc in next_locs:
            if loc in state.options:
                state.options.remove(loc)
                next_path = state.path.copy()
                next_path.append(loc)
                yield State(state.options, next_path)

    def shortest_path(
        self, src: Unit, targets: set[tuple[int, int]]
    ) -> Union[tuple[int, int], tuple]:

        options = self.area.copy()
        for loc in self.unit_locs:
            options.remove(loc)
        state: State = State(options, [(src.x, src.y)])
        states: list[State] = [state]

        reachable: list[State] = []
        while len(states):
            state = states.pop(0)
            if len(reachable):
                if len(state.path) > len(reachable[0].path):
                    break
            if state.path[-1] in targets:
                reachable.append(state)
            else:
                for next_state in self.next_states(state):
                    states.append(next_state)

        if len(reachable):
            reachable.sort()
            return reachable[0].path[1]

        return tuple()

    def move(self, mover: Unit) -> bool:
        targets: set[tuple[int, int]] = set()
        for unit in self.units:
            if not unit.dead and unit != mover and unit.type != mover.type:
                locs = [
                    (unit.x, unit.y - 1),
                    (unit.x - 1, unit.y),
                    (unit.x + 1, unit.y),
                    (unit.x, unit.y + 1),
                ]
                for loc in locs:
                    if loc in self.area and loc not in self.unit_locs:
                        targets.add(loc)

        if len(targets):
            shortest_path = self.shortest_path(mover, targets)
            if len(shortest_path):
                self.unit_locs.remove((mover.x, mover.y))
                mover.x = shortest_path[0]
                mover.y = shortest_path[1]
                self.unit_locs.add((mover.x, mover.y))
                return True
        return False


def parse_map(area_map: str, elf_dmg: int = 3) -> Battle:
    lines = area_map.splitlines()
    battle: Battle = Battle(len(lines[0]), len(lines))
    for y in range(len(lines)):
        for x in range(len(lines[y])):
            if lines[y][x] != "#":
                battle.area.add((x, y))
                if lines[y][x] != ".":
                    if lines[y][x] == "E":
                        battle.add_unit(Unit(x, y, lines[y][x], elf_dmg))
                    else:
                        battle.add_unit(Unit(x, y, lines[y][x]))
    return battle


def combat_score(battle_map: str, elf_dmg: int = 3, no_elf_death: bool = False) -> int:
    battle: Battle = parse_map(battle_map, elf_dmg)
    rounds: int = 0
    while True:
        battle.units.sort()
        for unit in battle.units:
            if not unit.dead and not battle.attack(unit):
                if battle.move(unit):
                    battle.attack(unit)
                else:
                    target_remaining: bool = False
                    for other_unit in battle.units:
                        if not other_unit.dead and other_unit.type != unit.type:
                            target_remaining = True
                            break
                    if not target_remaining:
                        return sum(u.hp for u in battle.units if not u.dead) * rounds
            if no_elf_death:
                for other_unit in battle.units:
                    if other_unit.dead and other_unit.type == "E":
                        return 0
        rounds += 1


def elves_no_loss(battle_map: str) -> int:
    elf_dmg: int = 4
    while elf_dmg < 201:
        score = combat_score(battle_map, elf_dmg, no_elf_death=True)
        if score > 0:
            return score
        elf_dmg += 1
    return 0


print(f"Part One: {combat_score(input_raw)}")
print(f"Part Two: {elves_no_loss(input_raw)}")
