with open("02_input.txt", "r") as f:
    input_raw = [x.strip() for x in f.readlines()]


def exactly_n_letter(string: str, n: int) -> bool:
    chars = {}
    for c in string:
        if c not in chars:
            chars[c] = 1
        else:
            chars[c] += 1
    for c in chars:
        if chars[c] == n:
            return True
    return False


def rudimentary_checksum(ids: list) -> int:
    twice = 0
    triple = 0
    for idx in ids:
        if exactly_n_letter(idx, 2):
            twice += 1
        if exactly_n_letter(idx, 3):
            triple += 1
    return twice * triple


def id_differ_by_one(ids: list) -> str:
    ids_len = len(ids)
    id_len = len(ids[0])

    for id_1 in range(ids_len):
        for id_2 in range(id_1 + 1, ids_len):

            diff = 0
            for c in range(id_len):
                if ids[id_1][c] != ids[id_2][c]:
                    diff += 1

            if diff == 1:
                same_c = []
                for c in range(id_len):
                    if ids[id_1][c] == ids[id_2][c]:
                        same_c.append(ids[id_1][c])
                return "".join(same_c)

    return None


print(f"Part One: { rudimentary_checksum(input_raw) }")
print(f"Part Two: { id_differ_by_one(input_raw) }")
