from dataclasses import dataclass


@dataclass
class Pos:
    x: int
    y: int
    z: int

    def distance(self, other: object) -> int:
        if not isinstance(other, Pos):
            raise NotImplementedError
        x = abs(self.x - other.x)
        y = abs(self.y - other.y)
        z = abs(self.z - other.z)
        return x + y + z


@dataclass
class Bot:
    p: Pos
    r: int

    def __lt__(self, other: object) -> bool:
        if not isinstance(other, Bot):
            raise NotImplementedError
        return self.r < other.r


def parse_bots(lines: list[str]) -> list[Bot]:
    bots = []
    for line in lines:
        split = line.split(" ")
        pos = split[0][5:-2].split(",")
        bots.append(
            Bot(
                p=Pos(
                    x=int(pos[0]),
                    y=int(pos[1]),
                    z=int(pos[2]),
                ),
                r=int(split[1][2:]),
            ),
        )
    return bots


def bots_in_range(bots: list[Bot], source: Bot) -> int:
    in_range = 0
    for bot in bots:
        if bot.p.distance(source.p) <= source.r:
            in_range += 1
    return in_range


def best_range(bots: list[Bot]) -> int:
    in_range = {}
    for b in bots:
        num = bots_in_range(bots, b)
        if num in in_range:
            in_range[num].append(b)
        else:
            in_range[num] = [b]
    n_max = max(in_range)
    print(in_range)
    bot = min(in_range[n_max], key=lambda x: Pos(0, 0, 0).distance(x.p))
    return Pos(0, 0, 0).distance(bot.p) - bot.r


# test
example1 = [
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
bots = parse_bots(example1)
assert len(bots) == 9
assert bots_in_range(bots, max(bots)) == 7
example2 = [
    "pos=<10,12,12>, r=2",
    "pos=<12,14,12>, r=2",
    "pos=<16,12,12>, r=4",
    "pos=<14,14,14>, r=6",
    "pos=<50,50,50>, r=200",
    "pos=<10,10,10>, r=5",
]
bots = parse_bots(example2)
print(best_range(bots))
assert best_range(bots) == 36


# answer
with open("23_input.txt") as fp:
    input_raw = [x.strip() for x in fp.readlines()]
bots = parse_bots(input_raw)
print(f"Part One: {bots_in_range(bots, max(bots))}")
# print(f"Part Two: {best_range(bots)}")
