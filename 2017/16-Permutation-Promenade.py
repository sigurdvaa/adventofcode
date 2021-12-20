with open("16_input.txt", "r") as f:
    input_raw = f.read().strip()


# progs = list("abcde")
progs = list("abcdefghijklmnop")
moves = []
for move in input_raw.split(","):
    if move[0] == "s":
        size = int(move[1:])
        moves += [[0, size]]
    elif move[0] == "x":
        a, b = [int(x) for x in move[1:].split("/")]
        moves += [[1, a, b]]
    elif move[0] == "p":
        d1, d2 = move[1:].split("/")
        moves += [[2, d1, d2]]


def dance(iterations, progs):
    memoization = []
    for i in range(iterations):
        if progs in memoization:
            return "".join(memoization[iterations % i])
        memoization += [progs[:]]

        for move in moves:
            if move[0] == 0:  # s
                size = move[1]
                progs = progs[-size:] + progs[:-size]
            elif move[0] == 1:  # x
                a, b = move[1], move[2]
                progs[a], progs[b] = progs[b], progs[a]
            elif move[0] == 2:  # p
                a, b = progs.index(move[1]), progs.index(move[2])
                progs[a], progs[b] = progs[b], progs[a]

    return "".join(progs)


print("Part 1")
print(dance(1, progs[:]))

print("Part 2")
print(dance(1000000000, progs[:]))
