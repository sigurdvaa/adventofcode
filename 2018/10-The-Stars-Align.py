with open("10_input.txt", "r") as fp:
    input_raw = fp.read()


input_raw = """position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>"""


class Point:
    def __init__(self, x: int, y: int, vel_x: int, vel_y: int):
        self.x: int = x
        self.y: int = y
        self.vel_x: int = vel_x
        self.vel_y: int = vel_y

    def __repr__(self):
        return f"({self.x},{self.y}) <{self.vel_x},{self.vel_y}>"

    def update(self):
        self.x += self.vel_x
        self.y += self.vel_y


def parse_points(string: str) -> list[Point]:
    points: list[Point] = []
    for line in string.splitlines():
        split = line[10:].split(">")
        pos = split[0].split(",")
        vel = split[1][11:].split(",")
        points.append(Point(int(pos[0]), int(pos[1]), int(vel[0]), int(vel[1])))
    return points


def search_message(points: list[Point]):
    s: int = 0
    while True:
        for p in points:
            p.update()

        for p1 in range(len(points)):
            for p2 in range(p1 + 1, len(points)):
                

        s += 1


points = parse_points(input_raw)
search_message(points)

