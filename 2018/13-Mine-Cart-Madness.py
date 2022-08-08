with open("13_input.txt", "r") as fp:
    input_raw = fp.read()


class Cart:
    dir_str: list[str] = ["^", ">", "v", "<"]
    dir_track: list[str] = ["|", "-", "-", "|"]
    dir_int: list[tuple[int, int]] = [(0, -1), (1, 0), (0, 1), (-1, 0)]

    def __init__(self, x: int, y: int, d: int):
        self.x: int = x
        self.y: int = y
        self.d: int = d
        self.i: int = -1
        self.crashed: bool = False

    def __repr__(self):
        return f"`{self.dir_str[self.d]}` ({self.x}, {self.y})"

    def __lt__(self, other):
        if self.y < other.y:
            return True
        if self.y == other.y and self.x < other.x:
            return True
        return False

    def move(self, tracks: list[list[str]]):
        self.x += Cart.dir_int[self.d][0]
        self.y += Cart.dir_int[self.d][1]
        self.turn(tracks)

    def turn(self, tracks: list[list[str]]):
        if tracks[self.y][self.x] == "+":
            self.d += self.i
            self.i += 1
            if self.i > 1:
                self.i = -1
        elif tracks[self.y][self.x] == "\\":
            if Cart.dir_int[self.d][0] == 0:
                self.d -= 1
            else:
                self.d += 1
        elif tracks[self.y][self.x] == "/":
            if Cart.dir_int[self.d][0] == 0:
                self.d += 1
            else:
                self.d -= 1
        self.d %= len(Cart.dir_str)


def parse_tracks(string_map: str) -> list[list[str]]:
    tracks: list[list[str]] = []
    for line in string_map.splitlines():
        tracks.append(list(line))
    return tracks


def find_and_replace_carts(tracks: list[list[str]]) -> list[Cart]:
    carts: list[Cart] = []
    for y in range(len(tracks)):
        for x in range(len(tracks[y])):
            if tracks[y][x] in Cart.dir_str:
                dir_index: int = Cart.dir_str.index(tracks[y][x])
                carts.append(Cart(x, y, dir_index))
                tracks[y][x] = Cart.dir_track[dir_index]
    return carts


def find_first_crash(tracks: list[list[str]], carts: list[Cart]) -> tuple[int, int]:
    while True:
        carts.sort()
        for cart in carts:
            cart.move(tracks)
            for other in carts:
                if cart != other:
                    if cart.x == other.x and cart.y == other.y:
                        return (cart.x, cart.y)


def find_last_cart(tracks: list[list[str]], carts: list[Cart]) -> tuple[int, int]:
    crashed_carts: int = 0
    while True:
        carts.sort()
        for i in range(len(carts)):
            cart = carts[i]
            if not cart.crashed:
                cart.move(tracks)
                for o in range(len(carts)):
                    other = carts[o]
                    if not other.crashed:
                        if cart != other:
                            if cart.x == other.x and cart.y == other.y:
                                carts[i].crashed = True
                                carts[o].crashed = True
                                crashed_carts += 2
                                break
        if len(carts) - crashed_carts == 1:
            for cart in carts:
                if not cart.crashed:
                    return (cart.x, cart.y)


tracks = parse_tracks(input_raw)
carts = find_and_replace_carts(tracks)
first_crach = find_first_crash(tracks, carts)
print(f"Part One: {first_crach[0]},{first_crach[1]}")

tracks = parse_tracks(input_raw)
carts = find_and_replace_carts(tracks)
last_cart = find_last_cart(tracks, carts)
print(f"Part Two: {last_cart[0]},{last_cart[1]}")
