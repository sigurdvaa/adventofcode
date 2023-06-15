from __future__ import annotations

from dataclasses import dataclass
from enum import Enum
from functools import cache
from typing import Iterator, Optional


class Erosion(Enum):
    ROCKY: int = 0
    WET: int = 1
    NARROW: int = 2


class Equipment(Enum):
    NONE: int = 0
    CLIMBING_GEAR: int = 1
    TORCH: int = 2


@dataclass
class Point:
    x: int
    y: int
    time: int = 0
    equipped: Equipment = Equipment.TORCH
    r_type: Erosion = Erosion.ROCKY

    def __hash__(self) -> int:
        return hash((self.x, self.y, self.equipped))

    def __eq__(self, other: object) -> bool:
        if not isinstance(other, Point):
            return NotImplemented
        return (self.x, self.y, self.equipped) == (other.x, other.y, other.equipped)

    def __lt__(self, other: object) -> bool:
        if not isinstance(other, Point):
            return NotImplemented
        return self.time < other.time

    def __gt__(self, other: object) -> bool:
        if not isinstance(other, Point):
            return NotImplemented
        return self.time > other.time

    def copy(self) -> Point:
        return Point(self.x, self.y, self.time, self.equipped, self.r_type)

    def next_points(self, target: Point, depth: int) -> Iterator[Point]:
        points = []
        for i in range(-1, 2, 2):
            new = self.copy()
            new.x += i
            points.append(new)
            new = self.copy()
            new.y += i
            points.append(new)

        for new in points:
            if new.x > -1 and new.y > -1:
                new.r_type = region_type(new, target, depth)
                for item in Equipment:
                    out = new.copy()
                    out.equipped = item
                    if item != self.equipped:
                        out.time += 8
                    else:
                        out.time += 1
                    if out.valid_equipment(self):
                        yield out

        for item in Equipment:
            out = self.copy()
            out.equipped = item
            if item != self.equipped:
                out.time += 7
            if out.valid_equipment(self):
                yield out

    def valid_equipment(self, prev: Point) -> bool:
        if self.equipped == Equipment.NONE:
            return self.r_type != Erosion.ROCKY and prev.r_type != Erosion.ROCKY
        if self.equipped == Equipment.TORCH:
            return self.r_type != Erosion.WET and prev.r_type != Erosion.WET
        if self.equipped == Equipment.CLIMBING_GEAR:
            return self.r_type != Erosion.NARROW and prev.r_type != Erosion.NARROW
        return False


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


class MinHeap:
    def __init__(self) -> None:
        self.data: list[Point] = []

    @property
    def length(self) -> int:
        return len(self.data)

    def heapify_up(self, idx: int) -> None:
        if idx == 0:
            return

        p_idx = self.parent(idx)
        parent = self.data[p_idx]
        child = self.data[idx]

        if parent > child:
            self.data[p_idx] = child
            self.data[idx] = parent
            self.heapify_up(p_idx)

    def heapify_down(self, idx: int) -> None:
        parent = self.data[idx]
        l_idx = self.left_child(idx)
        r_idx = self.right_child(idx)

        if l_idx >= self.length:
            return

        left = self.data[l_idx]
        if r_idx >= self.length:
            if parent > left:
                self.data[idx] = left
                self.data[l_idx] = parent
            return

        right = self.data[r_idx]
        if left > right and parent > right:
            self.data[idx] = right
            self.data[r_idx] = parent
            self.heapify_down(r_idx)
        elif parent > left:
            self.data[idx] = left
            self.data[l_idx] = parent
            self.heapify_down(l_idx)

    def insert(self, value: Point) -> None:
        self.data.append(value)
        self.heapify_up(self.length - 1)

    def delete(self) -> Optional[Point]:
        if self.length == 0:
            return None

        p = self.data[0]
        if self.length == 1:
            self.data = []
            return p

        self.data[0] = self.data[-1]
        self.data.pop()
        self.heapify_down(0)
        return p

    def parent(self, idx: int) -> int:
        return (idx - 1) // 2

    def left_child(self, idx: int) -> int:
        return idx * 2 + 1

    def right_child(self, idx: int) -> int:
        return idx * 2 + 2


def fastest_time(depth: int, target: Point) -> int:
    seen: set[Point] = set()
    minheap = MinHeap()
    minheap.insert(Point(0, 0))
    while minheap.length:
        curr = minheap.delete()
        if curr is None:
            return 0

        if curr == target:
            return curr.time

        for new in curr.next_points(target, depth):
            if new in seen:
                continue
            minheap.insert(new)
            seen.add(new)

    return 0


# test
depth = 510
target = Point(10, 10, equipped=Equipment.TORCH)
assert region_type(Point(0, 0), target, depth) == Erosion.ROCKY
assert region_type(Point(1, 0), target, depth) == Erosion.WET
assert region_type(Point(0, 1), target, depth) == Erosion.ROCKY
assert region_type(Point(1, 1), target, depth) == Erosion.NARROW
assert region_type(Point(10, 10), target, depth) == Erosion.ROCKY
assert area_risk(depth, target) == 114
print(fastest_time(depth, target))
# assert fastest_time(depth, target) == 45


depth = 8112
target = Point(13, 743)
print(f"Part One: {area_risk(depth, target)}")
# print(f"Part Two: {fastest_time(depth, target)}")

# 1064 high
# 1038 high
