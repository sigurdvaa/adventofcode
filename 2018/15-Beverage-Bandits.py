with open("15_input.txt", "r") as fp:
    input_raw = fp.read()


input_raw1 = """#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######"""

input_raw2 = """#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######"""

input_raw3 = """#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######"""

input_raw4 = """#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########"""

class Unit:
    def __init__(self, x: int, y: int, typ: str):
        self.hp: int = 200
        self.dmg: int = 3
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

    def loc(self) -> tuple[int, int]:
        return (self.x, self.y)


class State:
    def __init__(self, options, path):
        self.options: set[tuple[int, int]] = options
        self.path: list[tuple[int, int]] = path


class Battle:
    def __init__(self, x_max: int, y_max: int):
        self.area: set[tuple[int, int]] = set()
        self.units: list[Unit] = []
        self.x_max = x_max
        self.y_max = y_max

    def print_map(self):
        print("-"*5, "Battle map", "-"*5)
        unit_locs = set()
        for u in self.units:
            unit_locs.add((u.x, u.y))
        for y in range(self.y_max):
            for x in range(self.x_max):
                curr_loc = (x, y)
                if curr_loc in unit_locs:
                    printed = False
                    for u in self.units:
                        if not u.dead and curr_loc == u.loc():
                            printed = True
                            print(u.type, end="")
                    if not printed:
                        print(".", end="")
                elif curr_loc in self.area:
                    print(".", end="")
                else:
                    print("#", end="")
            print()

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
            return True
        return False

    def next_states(self, state: State) -> State:
        loc1 = (state.path[-1][0], state.path[-1][1] - 1)
        loc2 = (state.path[-1][0] - 1, state.path[-1][1])
        loc3 = (state.path[-1][0] + 1, state.path[-1][1])
        loc4 = (state.path[-1][0], state.path[-1][1] + 1)
        for loc in [loc1, loc2, loc3, loc4]:
            if loc in state.options:
                next_options = state.options.copy()
                next_options.remove(loc)
                next_path = state.path.copy()
                next_path.append(loc)
                yield State(next_options, next_path)

    def shortest_path(self, src: Unit, targets: set[tuple[int, int]], unit_locs: set[tuple[int, int]]) -> tuple[int, int]:
        options = self.area.copy()
        for loc in unit_locs:
            options.remove(loc)
        state: State = State(options, [src.loc()])
        states: list[State] = [state]
        while len(states):
            state = states.pop(0)
            for next_state in self.next_states(state):
                if next_state.path[-1] in targets:
                    return next_state.path[1:]
                states.append(next_state)

        return []

    def move(self, mover: Unit) -> bool:
        unit_locs: set[tuple[int, int]] = set()
        for unit in self.units:
            if not unit.dead:
                unit_locs.add((unit.x, unit.y))

        in_range: set[tuple[int, int]] = set()
        for unit in self.units:
            if not unit.dead and unit != mover and unit.type != mover.type:
                for i in range(-1, 2, 2):
                    loc1 = (unit.x, unit.y + i)
                    loc2 = (unit.x + i, unit.y)
                    if loc1 in self.area and loc1 not in unit_locs:
                        in_range.add(loc1)
                    if loc2 in self.area and loc2 not in unit_locs:
                        in_range.add(loc2)

        if len(in_range):
            shortest_path = self.shortest_path(mover, in_range, unit_locs)
            if len(shortest_path):
                mover.x = shortest_path[0][0]
                mover.y = shortest_path[0][1]
                return True
        return False


def parse_map(area_map: str) -> Battle:
    lines = area_map.splitlines()
    battle: Battle = Battle(len(lines[0]), len(lines))
    for y in range(len(lines)):
        for x in range(len(lines[y])):
            if lines[y][x] != "#":
                battle.area.add((x,y))
                if lines[y][x] != ".":
                    battle.units.append(Unit(x, y, lines[y][x]))
    return battle


def combat_score(battle: Battle) -> int:
    rounds: int = 0
    while True:
        battle.units.sort()
        for unit in battle.units:
            if not unit.dead:
                if not battle.attack(unit):
                    if battle.move(unit):
                        battle.attack(unit)
                    else:
                        target_remaining: bool = False
                        for other_unit in battle.units:
                            if not other_unit.dead and other_unit.type != unit.type:
                                target_remaining = True
                                break
                        if not target_remaining:
                            score: int = 0
                            for u in battle.units:
                                if not u.dead:
                                    score += u.hp
                            return score * rounds
        rounds += 1


#battle = parse_map(input_raw)
#print(f"Part One: {combat_score(battle)}")
battle = parse_map(input_raw1)
print(f"27730: {combat_score(battle)}")
battle = parse_map(input_raw2)
print(f"36334: {combat_score(battle)}")
battle = parse_map(input_raw3)
print(f"39514: {combat_score(battle)}")
#battle = parse_map(input_raw4)
#print(f"18740: {combat_score(battle)}")
