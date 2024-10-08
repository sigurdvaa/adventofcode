with open("21_input.txt", "r") as f:
    input_raw = f.read()


def rotate(piece):
    length = len(piece)
    temp = ["" for x in range(length)]
    for x in range(length):
        for y in range(length - 1, -1, -1):
            temp[x] += piece[y][x]
    return temp


def get_pieces(size, piece_size, art):
    pieces = []
    for y in range(0, size, piece_size):
        for x in range(0, size, piece_size):
            piece = []
            for line in art[y : y + piece_size]:
                piece += [line[x : x + piece_size]]
            pieces += [piece]
    return pieces


def pixels_on(iterations, art):
    for i in range(iterations):
        size = len(art)
        if size % 2 == 0:
            pieces = get_pieces(size, 2, art)
            for p in range(len(pieces)):
                pieces[p] = rules["/".join(pieces[p])]

            lines = 3 * (size // 2)
            per_line = lines // 3
            art = ["" for x in range(lines)]
            for p in range(lines // 3):
                for piece in pieces[p * per_line : p * per_line + per_line]:
                    art[p * 3] += piece[0]
                    art[p * 3 + 1] += piece[1]
                    art[p * 3 + 2] += piece[2]

        elif size % 3 == 0:
            pieces = get_pieces(size, 3, art)
            for p in range(len(pieces)):
                pieces[p] = rules["/".join(pieces[p])]

            lines = 4 * (size // 3)
            per_line = lines // 4
            art = ["" for x in range(lines)]
            for p in range(lines // 4):
                for piece in pieces[p * per_line : p * per_line + per_line]:
                    art[p * 4] += piece[0]
                    art[p * 4 + 1] += piece[1]
                    art[p * 4 + 2] += piece[2]
                    art[p * 4 + 3] += piece[3]

    count = 0
    for line in art:
        for c in line:
            if c == "#":
                count += 1
    return count


# rotate and flip each rule and add to rules
rules = {}
for rule in input_raw.splitlines():
    split = rule.split(" => ")
    piece = split[0].split("/")
    newpiece = split[1].split("/")

    # original rule
    rules[split[0]] = newpiece
    # flip
    match = "/".join([line[::-1] for line in piece])
    rules[match] = newpiece

    # rotate 90
    piece = rotate(piece)
    match = "/".join(piece)
    rules[match] = newpiece
    # flip
    match = "/".join([line[::-1] for line in piece])
    rules[match] = newpiece

    # rotate 90
    piece = rotate(piece)
    match = "/".join(piece)
    rules[match] = newpiece
    # flip
    match = "/".join([line[::-1] for line in piece])
    rules[match] = newpiece

    # rotate 90
    piece = rotate(piece)
    match = "/".join(piece)
    rules[match] = newpiece
    # flip
    match = "/".join([line[::-1] for line in piece])
    rules[match] = newpiece

art = [".#.", "..#", "###"]

print("Part 1")
print(pixels_on(5, art[:]))

print("Part 2")
print(pixels_on(18, art[:]))
