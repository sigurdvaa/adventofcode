from dataclasses import dataclass


@dataclass
class Bot:
    x: int
    y: int
    z: int
    r: int

    def __lt__(self, other: object) -> bool:
        if not isinstance(other, Bot):
            raise NotImplementedError
        return self.r < other.r

    def distance(self, other: object) -> int:
        if not isinstance(other, Bot):
            raise NotImplementedError
        x = abs(self.x - other.x)
        y = abs(self.y - other.y)
        z = abs(self.z - other.z)
        return x + y + z


def parse_bots(lines: list[str]) -> list[Bot]:
    bots = []
    for line in lines:
        split = line.split(" ")
        pos = split[0][5:-2].split(",")
        bots.append(
            Bot(
                x=int(pos[0]),
                y=int(pos[1]),
                z=int(pos[2]),
                r=int(split[1][2:]),
            ),
        )
    return bots


def bots_in_range(bots: list[Bot], source: Bot) -> int:
    in_range = 0
    for bot in bots:
        if bot.distance(source) <= source.r:
            in_range += 1
    return in_range


# test
input_example = [
    "pos=<0,0,0>, r=4",
    "pos=<1,0,0>, r=1",
    "pos=<4,0,0>, r=3",
    "pos=<0,2,0>, r=1",
    "pos=<0,5,0>, r=3",
    "pos=<0,0,3>, r=1",
    "pos=<1,1,1>, r=1",
    "pos=<1,1,2>, r=1",
    "pos=<1,3,1>, r=1",
]
bots = parse_bots(input_example)
assert len(bots) == 9
assert bots_in_range(bots, max(bots)) == 7

with open("23_input.txt") as fp:
    input_raw = [x.strip() for x in fp.readlines()]
bots = parse_bots(input_raw)
print(f"Part One: {bots_in_range(bots, max(bots))}")
