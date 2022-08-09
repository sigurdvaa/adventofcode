from collections import namedtuple

Subgrid: tuple[int, int, int, int] = namedtuple("Subgrid", ["x", "y", "size", "power"])


def calc_power_level(x: int, y: int, serial: int) -> int:
    power: int = 0
    rackid: int = x + 10
    power += rackid * y
    power += serial
    power *= rackid
    if power // 100:
        power = int(str(power)[-3])
    else:
        power = 0
    power -= 5

    return power


def get_coord_power(serial: int, grid: int, xindexed: bool = False) -> list[list[int]]:
    power: list[list[int]] = [[0] * grid for _ in range(grid)]
    if not xindexed:
        for y in range(grid):
            for x in range(grid):
                power[y][x] = calc_power_level(x + 1, y + 1, serial)
    else:
        for x in range(grid):
            for y in range(grid):
                power[x][y] = calc_power_level(x + 1, y + 1, serial)
    return power


def largest_subgrid_power(
    serial: int, subgrid: int = 0, grid: int = 300
) -> Subgrid:
    coord_power_yindexed = get_coord_power(serial, grid)
    coord_power_xindexed = get_coord_power(serial, grid, xindexed=True)
    subgrid_power = [[0] * grid for _ in range(grid)]

    max_subgrid = Subgrid(0, 0, 0, 0)

    sub_end: int = grid
    if subgrid > 0:
        sub_end = subgrid

    for sub in range(sub_end):
        for y in range(grid - sub):
            for x in range(grid - sub):
                subgrid_power[y][x] += sum(
                    coord_power_yindexed[y + sub][x : x + sub + 1]
                )
                subgrid_power[y][x] += sum(coord_power_xindexed[x + sub][y : y + sub])
                if subgrid_power[y][x] > max_subgrid.power:
                    max_subgrid = Subgrid(x + 1, y + 1, sub + 1, subgrid_power[y][x])

    return max_subgrid


input_raw = 8868

largest_3x3 = largest_subgrid_power(input_raw, 3)
print(f"Part One: {largest_3x3.x},{largest_3x3.y}")

largest_power = largest_subgrid_power(input_raw)
print(f"Part Two: {largest_power.x},{largest_power.y},{largest_power.size}")
