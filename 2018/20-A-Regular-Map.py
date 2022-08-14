with open("20_input.txt", "r") as fp:
    input_raw = fp.read().strip()


def expand_longest_path(pattern: str) -> int:
    pattern = pattern[1:-1]
    exp_end: int = pattern.find(")")
    while exp_end != -1:
        exp_start = pattern[:exp_end].rfind("(")
        split = pattern[exp_start + 1 : exp_end].split("|")
        split.sort(key=len)
        replace: str = ""
        if split[0] != "":
            replace = split[-1]
        pattern = pattern[:exp_start] + replace + pattern[exp_end + 1 :]
        exp_end = pattern.find(")")
    return len(pattern)


def rooms_over_min_dist(pattern: str, min_dist: int):
    Loc = tuple[int, int]
    heads: list[tuple[int, Loc]] = []
    loc: Loc = (0, 0)
    rooms: set[Loc] = set([loc])
    rooms_min_dist: int = 0
    dist: int = 0
    i: int = 0
    for c in pattern:
        if c in "NSWE":
            if c == "N":
                next_loc = (loc[0], loc[1] - 2)
            elif c == "S":
                next_loc = (loc[0], loc[1] + 2)
            elif c == "W":
                next_loc = (loc[0] - 2, loc[1])
            elif c == "E":
                next_loc = (loc[0] + 2, loc[1])
            loc = next_loc
            if loc not in rooms:
                rooms.add(loc)
                dist += 1
                if dist >= min_dist:
                    rooms_min_dist += 1
        elif c == "(":
            heads.append((dist, loc))
        elif c == ")":
            heads.pop()
        elif c == "|":
            dist, loc = heads[-1]
    return rooms_min_dist


print(f"Part One: {expand_longest_path(input_raw)}")
print(f"Part Two: {rooms_over_min_dist(input_raw, 1000)}")
