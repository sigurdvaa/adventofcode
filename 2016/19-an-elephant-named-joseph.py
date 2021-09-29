from collections import deque

input_raw = "3014603"


def elven_ring_next(amount_elves: int):
    elves = deque()
    for i in range(1, amount_elves + 1):
        elves.append(i)

    while len(elves) > 1:
        elves.append(elves.popleft())
        elves.popleft()

    return elves[0]


def elven_ring_across(amount_elves: int):
    elves_right = deque()
    elves_left = deque()
    for i in range(1, (amount_elves + 1) // 2):
        elves_right.append(i)
    for i in range((amount_elves + 1) // 2, amount_elves + 1):
        elves_left.append(i)
    
    right_len = len(elves_right)
    left_len = len(elves_left)

    while right_len > 0:
        elves_left.append(elves_right.popleft())
        elves_left.popleft()

        if left_len > right_len:
            elves_right.append(elves_left.popleft())
            left_len -= 1
        else:
            right_len -= 1

    return elves_left[0]


print(f"Part One: {elven_ring_next(int(input_raw))}")
print(f"Part Two: {elven_ring_across(int(input_raw))}")
