from collections import namedtuple

with open("03_input.txt", "r") as f:
    input_raw = [x.strip() for x in f.readlines()]


def parse_claims(strings: list) -> list:
    Claim = namedtuple("Claim", ["id", "loc", "area"])
    Xy = namedtuple("Xy", ["x", "y"])
    claims = []
    for string in strings:
        split = string.split()
        idx = int(split[0][1:])
        location = [int(x) for x in split[2][:-1].split(",")]
        area = [int(x) for x in split[3].split("x")]
        claims.append(Claim(idx, Xy(location[0], location[1]), Xy(area[0], area[1])))
    return claims


def claims_overlap(fabric_size, claims) -> int:
    fabric = [[0] * fabric_size for y in range(fabric_size)]
    for claim in claims:
        for x in range(claim.loc.x, claim.loc.x + claim.area.x):
            for y in range(claim.loc.y, claim.loc.y + claim.area.y):
                fabric[y][x] += 1
    overlaps = 0
    for y in fabric:
        for x in y:
            if x > 1:
                overlaps += 1

    return overlaps


def claims_no_overlap(fabric_size, claims) -> int:
    fabric = [[0] * fabric_size for y in range(fabric_size)]
    for claim in claims:
        for x in range(claim.loc.x, claim.loc.x + claim.area.x):
            for y in range(claim.loc.y, claim.loc.y + claim.area.y):
                fabric[y][x] += claim.id
    for claim in claims:
        overlap = False
        for x in range(claim.loc.x, claim.loc.x + claim.area.x):
            for y in range(claim.loc.y, claim.loc.y + claim.area.y):
                if fabric[y][x] != claim.id:
                    overlap = True
                    break
            if overlap:
                break
        if not overlap:
            return claim.id


fabric_size = 1000
claims = parse_claims(input_raw)
print(f"Part One: { claims_overlap(fabric_size, claims)}")
print(f"Part Two: { claims_no_overlap(fabric_size, claims)}")
