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


def bots_in_range_of_bot(bots: list[Bot], source: Bot) -> int:
    in_range = []
    for bot in bots:
        if bot.p.distance(source.p) <= source.r:
            in_range.append(bot)
    return in_range


def bots_in_range_of_pos(bots: list[Bot], pos: Pos) -> int:
    in_range = []
    for bot in bots:
        if bot.p.distance(pos) <= bot.r:
            in_range.append(bot)
    return in_range


def best_range(bots: list[Bot]) -> int:
    s = Pos(0, 0, 0)
    # biggest group
    group = {}
    for b in bots:
        in_range = bots_in_range_of_bot(bots, b)
        if len(in_range) in group:
            group[len(in_range)].append(in_range)
        else:
            group[len(in_range)] = [in_range]
    ds = {}
    for g in group[max(group)]:
        # bot with smallest range, coord must be within
        smallest_r = min(g, key=lambda x: x.r).r
        print(smallest_r)
        smallest = [x for x in g if x.r == smallest_r]
        for b in smallest:
            seen = set()
            # for x, y, z, check bot.p.distance in group,
            for x in range(abs(b.p.x - b.r), abs(b.p.x + b.r + 1)):
                for y in range(abs(b.p.y - b.r), abs(b.p.y + b.r + 1)):
                    for z in range(abs(b.p.z - b.r), abs(b.p.z + b.r + 1)):
                        # if d < b.r, in range
                        p = Pos(x, y, z)
                        d = s.distance(p)
                        if d in seen:
                            continue
                        in_range = len(bots_in_range_of_pos(bots, p))
                        if in_range in ds:
                            if d < ds[in_range]:
                                ds[in_range] = d
                        else:
                            ds[in_range] = d
    return ds[max(ds)]


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
assert len(bots_in_range_of_bot(bots, max(bots))) == 7
example2 = [
    "pos=<10,12,12>, r=2",
    "pos=<12,14,12>, r=2",
    "pos=<16,12,12>, r=4",
    "pos=<14,14,14>, r=6",
    "pos=<50,50,50>, r=200",
    "pos=<10,10,10>, r=5",
]
bots = parse_bots(example2)
assert best_range(bots) == 36


# answer
with open("23_input.txt") as fp:
    input_raw = [x.strip() for x in fp.readlines()]
bots = parse_bots(input_raw)
print(f"Part One: {len(bots_in_range_of_bot(bots, max(bots)))}")
print(f"Part Two: {best_range(bots)}")
