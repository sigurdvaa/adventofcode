class Point:
    def __init__(self, x: int, y: int, vel_x: int, vel_y: int):
        self.x: int = x
        self.y: int = y
        self.vel_x: int = vel_x
        self.vel_y: int = vel_y

    def update(self):
        self.x += self.vel_x
        self.y += self.vel_y

    def reverse(self):
        self.x -= self.vel_x
        self.y -= self.vel_y


def parse_points(string: str) -> list[Point]:
    points: list[Point] = []
    for line in string.splitlines():
        split = line[10:].split(">")
        pos = split[0].split(",")
        vel = split[1][11:].split(",")
        points.append(Point(int(pos[0]), int(pos[1]), int(vel[0]), int(vel[1])))
    return points


def points_xy_edge(points: list[Point]) -> tuple[int, int, int, int]:
    x_min: int = points[0].x
    x_max: int = x_min
    y_min: int = points[0].y
    y_max: int = y_min
    for p in points:
        if p.x < x_min:
            x_min = p.x
        if p.x > x_max:
            x_max = p.x
        if p.y < y_min:
            y_min = p.y
        if p.y > y_max:
            y_max = p.y
    return (x_min, x_max, y_min, y_max)


def points_to_str(points: list[Point]) -> str:
    output: list[str] = []
    x_min, x_max, y_min, y_max = points_xy_edge(points)
    for y in range(y_min, y_max + 1):
        row: list[str] = []
        for x in range(x_min, x_max + 1):
            printed = False
            for p in points:
                if p.x == x and p.y == y:
                    row.append("#")
                    printed = True
                    break
            if not printed:
                row.append(".")
        output.append("".join(row))

    return "\n".join(output)


def search_msg(points: list[Point]) -> tuple[str, int]:
    s: int = 0
    x_min, x_max, y_min, y_max = points_xy_edge(points)
    prev_delta_x: int = x_max - x_min
    prev_delta_y: int = y_max - y_min
    prev_delta: int = prev_delta_x + prev_delta_y

    while True:
        x_min, x_max, y_min, y_max = points_xy_edge(points)
        delta_x: int = x_max - x_min
        delta_y: int = y_max - y_min
        delta: int = delta_x + delta_y

        if delta_x < prev_delta_x:
            prev_delta_x = delta_x
        if delta_y < prev_delta_y:
            prev_delta_y = delta_y
        if delta <= prev_delta:
            prev_delta = delta
        else:
            for p in points:
                p.reverse()
            s -= 1
            return (points_to_str(points), s)

        for p in points:
            p.update()
        s += 1


with open("10_input.txt", "r") as fp:
    input_raw = fp.read()

points = parse_points(input_raw)
msg, s = search_msg(points)
print(f"Part One: \n{msg}")
print(f"Part Two: {s}")
