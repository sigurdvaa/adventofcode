from dataclasses import dataclass
from enum import Enum
from functools import cache


class Erosion(Enum):
    ROCKY: int = 0
    WET: int = 1
    NARROW: int = 2


@dataclass
class Point:
    x: int
    y: int

    def __hash__(self) -> int:
        return hash(repr(self))


@cache
def geologic_index(point: Point, target: Point, depth: int) -> int:
    if (point.x, point.y) == (0, 0) or point == target:
        return 0
    if point.y == 0:
        return point.x * 16807
    if point.x == 0:
        return point.y * 48271

    idx1 = geologic_index(Point(point.x - 1, point.y), target, depth)
    idx2 = geologic_index(Point(point.x, point.y - 1), target, depth)
    return erosion_level(idx1, depth) * erosion_level(idx2, depth)


def erosion_level(idx: int, depth: int) -> int:
    return (idx + depth) % 20183


def region_type(point: Point, target: Point, depth: int) -> Erosion:
    idx = geologic_index(point, target, depth)
    lvl = erosion_level(idx, depth)
    return Erosion(lvl % 3)


def area_risk(depth: int, target: Point) -> int:
    risk = 0
    for x in range(target.x + 1):
        for y in range(target.y + 1):
            risk += region_type(Point(x, y), target, depth).value
    return risk


# test
depth = 510
target = Point(10, 10)
assert region_type(Point(0, 0), target, depth) == Erosion.ROCKY
assert region_type(Point(1, 0), target, depth) == Erosion.WET
assert region_type(Point(0, 1), target, depth) == Erosion.ROCKY
assert region_type(Point(1, 1), target, depth) == Erosion.NARROW
assert region_type(Point(10, 10), target, depth) == Erosion.ROCKY
assert area_risk(depth, target) == 114

depth = 8112
target = Point(13, 743)
print(f"Part One: {area_risk(depth, target)}")
