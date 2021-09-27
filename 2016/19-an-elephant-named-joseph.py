input_raw = "3014603"
# input_raw = "5"


def next_elf_with_present(elves: list, idx: int, amount_elves: int):
    for i in range(idx + 1, amount_elves):
        if elves[i]:
            return i
    for i in range(idx):
        if elves[i]:
            return i
    return False


def elven_ring(amount_elves: int):
    elves = list()
    for i in range(amount_elves):
        elves.append(1)

    elves_left = amount_elves
    i = 0
    while elves_left > 1:
        print(elves_left)
        if elves[i]:
            next_elf = next_elf_with_present(elves, i, amount_elves)
            elves[i] += elves[next_elf]
            elves[next_elf] = 0
            i = next_elf
            elves_left -= 1
        else:
            i += 1
            if i == amount_elves:
                i = 0

    for i in range(amount_elves):
        if elves[i]:
            return i + 1


print(f"Part One: {elven_ring(int(input_raw))}")
