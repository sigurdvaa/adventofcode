input_raw = "29000000"


def lowest_house_number_with_presents_ten(target: int) -> int:
    target = target // 10
    for house in range(0, target, 20160):
        presents = 0
        for elf in range(house, 0, -2):
            if house % elf == 0:
                presents += elf

        if presents >= target:
            return house

    return -1


def lowest_house_number_with_presents_eleven(target: int) -> int:
    for house in range(0, target // 10, 20160):
        presents = 0
        for elf in range(house, 0, -2):
            if house % elf == 0:
                if elf * 50 >= house:
                    presents += elf
                else:
                    break

        if presents * 11 >= target:
            return house

    return -1


print(f"Part One: {lowest_house_number_with_presents_ten(int(input_raw))}")
print(f"Part Two: {lowest_house_number_with_presents_eleven(int(input_raw))}")
