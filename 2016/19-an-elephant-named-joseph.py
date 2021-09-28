from collections import deque

input_raw = "3014603"


def elven_ring_next(amount_elves: int):
    elves = deque()
    for i in range(1, amount_elves + 1):
        elves.append((i, 1))

    while len(elves) > 1:
        first = elves.popleft()
        second = elves.popleft()
        elves.append((first[0], first[1] + second[1]))

    return elves[0][0]


def elven_ring_across(amount_elves: int):
    elves_north = deque()
    elves_south = deque()
    for i in range(1, (amount_elves + 1) // 2):
        elves_north.append((i, 1))
    for i in range((amount_elves + 1) // 2, amount_elves + 1):
        elves_south.append((i, 1))

    while len(elves_north) > 0:
        first = elves_north.popleft()
        steal = elves_south.popleft()
        elves_south.append((first[0], first[1] + steal[1]))
        if len(elves_south) > len(elves_north) + 1:
            elves_north.append(elves_south.popleft())

    return elves_south[0][0]


print(f"Part One: {elven_ring_next(int(input_raw))}")
print(f"Part Two: {elven_ring_across(int(input_raw))}")
