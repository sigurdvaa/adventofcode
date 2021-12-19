with open("16-input.txt", "r") as f:
    input_raw = f.read()


def parse_aunts(string: str):
    aunts = []
    for line in string.splitlines():
        split = line.split()
        aunts.append(
            {
                split[2][:-1]: int(split[3][:-1]),
                split[4][:-1]: int(split[5][:-1]),
                split[6][:-1]: int(split[7]),
            }
        )

    return aunts


def find_matching_aunt(aunts: list, mfcsam: dict, outdated: bool = True):
    for i, aunt in enumerate(aunts):
        match = True
        for prop in aunt:
            if outdated:
                if aunt[prop] != mfcsam[prop]:
                    match = False
            else:
                if prop in ["cats", "trees"]:
                    if aunt[prop] <= mfcsam[prop]:
                        match = False
                elif prop in ["pomeranians", "goldfish"]:
                    if aunt[prop] >= mfcsam[prop]:
                        match = False
                else:
                    if aunt[prop] != mfcsam[prop]:
                        match = False
            if not match:
                break

        if match:
            return i + 1

    return -1


mfcsam = {
    "children": 3,
    "cats": 7,
    "samoyeds": 2,
    "pomeranians": 3,
    "akitas": 0,
    "vizslas": 0,
    "goldfish": 5,
    "trees": 3,
    "cars": 2,
    "perfumes": 1,
}
aunts = parse_aunts(input_raw)
print(f"Part One: {find_matching_aunt(aunts, mfcsam)}")
print(f"Part Two: {find_matching_aunt(aunts, mfcsam, False)}")
