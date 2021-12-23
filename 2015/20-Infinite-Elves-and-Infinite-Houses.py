input_raw = "29000000"


def lowest_house_with_presents(target: int) -> int:
    target = target // 10
    max_houses = target // 2
    houses = [0] * max_houses
    for house in range(1, max_houses, 1):
        for elf in range(house, max_houses, house):
            houses[elf] += house

        if houses[house] >= target:
            return house

    return -1


def lowest_house_with_presents_max_visits(target: int, max_visits: int = 50) -> int:
    max_houses = target // (11 * 2)
    houses = [0] * max_houses
    for house in range(1, max_houses, 1):
        visits = 0
        for elf in range(house, max_houses, house):
            houses[elf] += house
            visits += 1
            if visits == 50:
                break

        if houses[house] * 11 >= target:
            return house

    return -1


print(f"Part One: {lowest_house_with_presents(int(input_raw))}")
print(f"Part Two: {lowest_house_with_presents_max_visits(int(input_raw))}")
