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

    def attack(self, battle: "Battle") -> bool:
        targets: list[Unit] = []
        for unit in battle.units:
            if not unit.dead and unit != self:
                if unit.x == self.x:
                    if unit.y == self.y - 1 or unit.y == self.y + 1:
                        targets.append(unit)
                if unit.y == self.y:
                    if unit.x == self.x - 1 or unit.x == self.x + 1:
                        targets.append(unit)
        if len(targets):
            targets.sort()
            targets[0].hp -= self.dmg
            if targets[0].hp < 1:
                targets[0].dead = True
            return True
        return False

    def loc(self) -> tuple[int, int]:
        return (self.x, self.y)


class Battle:
    def __init__(self):
        self.units: list[Unit] = []
        self.area: tuple[int, int] = set()


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
    battle.units.sort()
    for unit in battle.units:
        if not unit.attack(battle):
            # find moves
            # if moves, move and attack
            # else, return score


battle = parse_map(input_raw)
combat_score(battle)
