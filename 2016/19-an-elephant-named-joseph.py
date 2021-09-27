from collections import deque

input_raw = "3014603"


def elven_ring(amount_elves: int):
    elves = deque()
    for i in range(1, amount_elves + 1):
        elves.append((i, 1))

    while len(elves) > 1:
        first = elves.popleft()
        second = elves.popleft()
        elves.append((first[0], first[1] + second[1]))

    return elves[0][0]


def elven_ring2(amount_elves: int):
    elves = deque()
    for i in range(1, amount_elves + 1):
        elves.append((i, 1))

    while len(elves) > 1:
        print(len(elves))
        first = elves.popleft()
        steal = len(elves)//2
        elves.append((first[0], first[1] + elves[steal][1]))
        del(elves[steal])

    return elves[0][0]

print(f"Part One: {elven_ring(int(input_raw))}")
print(f"Part Two: {elven_ring2(int(input_raw))}")
