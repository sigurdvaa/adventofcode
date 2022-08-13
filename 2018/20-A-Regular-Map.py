with open("20_input.txt", "r") as fp:
    input_raw = fp.read().strip()


def expand_longest_path(pattern: str) -> int:
    pattern = pattern[1:-1]
    exp_end: int = pattern.find(")")
    while exp_end != -1:
        exp_start = pattern[:exp_end].rfind("(")
        split = pattern[exp_start + 1:exp_end].split("|")
        split.sort(key=len)
        replace: str = ""
        if split[0] != "":
            replace = split[-1]
        pattern = pattern[:exp_start] + replace + pattern[exp_end + 1:]
        exp_end = pattern.find(")")
    return len(pattern)


def walk_paths(pattern: str, min_distance: int):
    Point = tuple[int, int]
    pattern = pattern[1:-1]
    head: Point = (0, 0)
    heads: list[Point] = []
    rooms: set[Point] = set([head])
    doors: set[Point] = set()

    traveled_head: int = 0
    traveled: list[int] = []
    rooms_min_dist: set[Point] = set()

    i: int = 0
    while i < len(pattern):
        if pattern[i] in "NSWE":
            if pattern[i] == "N":
                door = (head[0], head[1] - 1)
                room = (head[0], head[1] - 2)
            elif pattern[i] == "S":
                door = (head[0], head[1] + 1)
                room = (head[0], head[1] + 2)
            elif pattern[i] == "W":
                door = (head[0] - 1, head[1])
                room = (head[0] - 2, head[1])
            elif pattern[i] == "E":
                door = (head[0] + 1, head[1])
                room = (head[0] + 2, head[1])
            doors.add(door)
            rooms.add(room)
            head = room
            traveled_head += 1
            if traveled_head >= min_distance:
                rooms_min_dist.add(room)
        elif pattern[i] == "(":
            heads.append(head)
            traveled.append(traveled_head)
        elif pattern[i] == ")":
            heads.pop()
            traveled.pop()
        elif pattern[i] == "|":
            head = heads[-1]
            traveled_head = traveled[-1]
        else:
            assert False, "unreachable"
        i += 1
    return len(rooms_min_dist)


print(f"Part One: {expand_longest_path(input_raw)}")
print(f"Part Two: {walk_paths(input_raw, 1000)}")
