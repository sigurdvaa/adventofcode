with open("15_input.txt", "r") as fp:
    input_raw = fp.read()


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
    def __init__(self):
        self.area: set[tuple[int, int]] = set()
        self.units: list[Unit] = []

    def attack(self, attacker: Unit) -> bool:
        targets: list[Unit] = []
        for unit in self.units:
            if not unit.dead and unit != attacker and unit.type != attacker.type:
                if unit.x == attacker.x:
                    if unit.y == attacker.y - 1 or unit.y == attacker.y + 1:
                        targets.append(unit)
                if unit.y == attacker.y:
                    if unit.x == attacker.x - 1 or unit.x == attacker.x + 1:
                        targets.append(unit)
        if len(targets):
            targets.sort()
            targets[0].hp -= attacker.dmg
            if targets[0].hp < 1:
                targets[0].dead = True
            return True
        return False

    def next_states(self, state: State) -> State:
        for i in range(-1, 2, 2):
            loc1 = (state.path[-1][0], state.path[-1][1] + i)
            loc2 = (state.path[-1][0] + i, state.path[-1][1])
            for loc in [loc1, loc2]:
                if loc in state.options:
                    next_options = state.options.copy()
                    next_options.remove(loc)
                    next_path = state.path.copy()
                    next_path.append(loc)
                    yield State(next_options, next_path)

    def shortest_path(self, src: Unit, target: tuple[int, int], unit_locs: list[tuple[int, int]]) -> tuple[int, int]:
        options = self.area.copy()
        for loc in unit_locs:
            options.remove(loc)
        state: State = State(options.copy(), [src.loc()])
        states: list[State] = [state]
        while len(states):
            state = states.pop(0)
            for next_state in self.next_states(state):
                if next_state.path[-1] == target:
                    print("-"*40)
                    print("return", src, target, len(next_state.path) - 1, next_state.path[1:])
                    input("pause...")
                    return (len(next_state.path) - 1, next_state.path[1:])
                states.append(next_state)

        return -1, []

    def move(self, mover: Unit) -> bool:
        unit_locs: set[tuple[int, int]] = set()
        for unit in self.units:
            if not unit.dead:
                unit_locs.add((unit.x, unit.y))

        in_range: list[tuple[int, int]] = []
        for unit in self.units:
            if not unit.dead and unit != mover and unit.type != mover.type:
                for i in range(-1, 2, 2):
                    loc1 = (unit.x, unit.y + i)
                    loc2 = (unit.x + i, unit.y)
                    if loc1 in self.area and loc1 not in unit_locs:
                        in_range.append(loc1)
                    if loc2 in self.area and loc2 not in unit_locs:
                        in_range.append(loc2)

        if len(in_range):
            nearest_dist: int = -1
            nearest_path: list[tuple[int, int]]
            for loc in in_range:
                loc_dist, loc_path = self.shortest_path(mover, loc, unit_locs)
                if loc_dist > 0:
                    if nearest_dist == -1 or loc_dist < nearest_dist:
                        nearest_dist = loc_dist
                        nearest_path = loc_path
        else:
            return False

        move_to: tuple[int, int] = self.shortest_path(mover, nearest)
        mover.move(move_to[1])
        return True


def parse_map(area_map: str) -> Battle:
    battle: Battle = Battle()
    lines = area_map.splitlines()
    for y in range(len(lines)):
        for x in range(len(lines[y])):
            if lines[y][x] != "#":
                battle.area.add((x,y))
                if lines[y][x] != ".":
                    battle.units.append(Unit(x, y, lines[y][x]))
    return battle


def combat_score(battle: Battle) -> int:
    #while True:
    battle.units.sort()
    for unit in battle.units:
        if not battle.attack(unit):
            if battle.move(unit):
                battle.attack(unit)
            else:
                return "score"


battle = parse_map(input_raw)
combat_score(battle)
