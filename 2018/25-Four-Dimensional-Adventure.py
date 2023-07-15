from __future__ import annotations

from dataclasses import dataclass, field


@dataclass
class Point:
    x: int
    y: int
    z: int
    t: int
    in_range: list[Point] = field(default_factory=lambda: [])
    _hash: int = 0

    def __hash__(self) -> int:
        if self._hash == 0:
            self._hash = hash((self.x, self.y, self.z, self.t))
        return self._hash

    def distance(self, point: Point) -> int:
        return (
            abs(self.x - point.x)
            + abs(self.y - point.y)
            + abs(self.z - point.z)
            + abs(self.t - point.t)
        )


def find_in_range(points: list[Point], dst: int = 3) -> None:
    for p1 in points:
        for p2 in points:
            if p1 == p2:
                continue
            if p1.distance(p2) <= dst:
                p1.in_range.append(p2)


def constellation(point: Point) -> frozenset[Point]:
    const = {point}
    searching = True
    while searching:
        searching = False
        new = set()
        for p1 in const:
            for p2 in p1.in_range:
                if p2 not in const:
                    new.add(p2)
                    searching = True
        const.update(new)
    return frozenset(const)


def num_constellation(points: list[Point]) -> int:
    find_in_range(points)
    constellations: set[frozenset[Point]] = set()
    for p in points:
        const = constellation(p)
        if const not in constellations:
            constellations.add(const)
    return len(constellations)


# test
input_tests = [
    (
        2,
        [
            Point(0, 0, 0, 0),
            Point(3, 0, 0, 0),
            Point(0, 3, 0, 0),
            Point(0, 0, 3, 0),
            Point(0, 0, 0, 3),
            Point(0, 0, 0, 6),
            Point(9, 0, 0, 0),
            Point(12, 0, 0, 0),
        ],
    ),
    (
        4,
        [
            Point(-1, 2, 2, 0),
            Point(0, 0, 2, -2),
            Point(0, 0, 0, -2),
            Point(-1, 2, 0, 0),
            Point(-2, -2, -2, 2),
            Point(3, 0, 2, -1),
            Point(-1, 3, 2, 2),
            Point(-1, 0, -1, 0),
            Point(0, 2, 1, -2),
            Point(3, 0, 0, 0),
        ],
    ),
    (
        3,
        [
            Point(1, -1, 0, 1),
            Point(2, 0, -1, 0),
            Point(3, 2, -1, 0),
            Point(0, 0, 3, 1),
            Point(0, 0, -1, -1),
            Point(2, 3, -2, 0),
            Point(-2, 2, 0, 0),
            Point(2, -2, 0, -1),
            Point(1, -1, 0, -1),
            Point(3, 2, 0, 2),
        ],
    ),
    (
        8,
        [
            Point(1, -1, -1, -2),
            Point(-2, -2, 0, 1),
            Point(0, 2, 1, 3),
            Point(-2, 3, -2, 1),
            Point(0, 2, 3, -2),
            Point(-1, -1, 1, -2),
            Point(0, -2, -1, 0),
            Point(-2, 2, 3, -1),
            Point(1, 2, 2, 0),
            Point(-1, -2, 0, -2),
        ],
    ),
]
for t in input_tests:
    assert num_constellation(t[1]) == t[0]


# answer
with open("25_input.txt") as fp:
    input_raw = [x.strip() for x in fp.readlines()]
points = [
    Point(int(s[0]), int(s[1]), int(s[2]), int(s[3]))
    for s in [x.split(",") for x in input_raw]
]
print(f"Part One: {num_constellation(points)}")
