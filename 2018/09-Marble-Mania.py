input_raw = "435 players; last marble is worth 71184 points"


class Marble:
    def __init__(self, prev: "Marble", nxt: "Marble", data: int):
        self.prev = prev
        self.next = nxt
        self.data = data


class MarbleCircle:
    def __init__(self):
        _a = Marble(None, None, 0)
        _b = Marble(_a, None, 2)
        _c = Marble(_b, _a, 1)
        _a.next = _b
        _a.prev = _c
        _b.next = _c
        self.current = _b

    def __repr__(self):
        curr_marble = self.current
        marbles = []
        for _ in range(len(self)):
            marbles.append(str(curr_marble.data))
            curr_marble = curr_marble.next
        return " <> ".join(marbles)

    def clockwise(self):
        self.current = self.current.next.next

    def counter_clockwise(self):
        for _ in range(7):
            self.current = self.current.prev

    def insert(self, data):
        self.current = Marble(self.current.prev, self.current, data)
        self.current.prev.next = self.current
        self.current.next.prev = self.current

    def pop(self):
        self.current.prev.next = self.current.next
        self.current.next.prev = self.current.prev
        data = self.current.data
        self.current = self.current.next
        return data


def play_marble(players: int, int_marbles: int) -> int:
    marbles: MarbleCircle = MarbleCircle()
    curr_marble: int = 2
    curr_player: int = 2
    player_score: list[int] = [0] * players
    while curr_marble < int_marbles:
        curr_player += 1
        if curr_player > players:
            curr_player = 1

        curr_marble += 1
        if curr_marble % 23 == 0:
            marbles.counter_clockwise()
            player_score[curr_player - 1] += curr_marble + marbles.pop()
        else:
            marbles.clockwise()
            marbles.insert(curr_marble)

    return max(player_score)


players = int(input_raw.split(" ")[0])
marbles = int(input_raw.split(" ")[6])
print(f"Part One: {play_marble(players, marbles)}")
print(f"Part Two: {play_marble(players, marbles*100)}")
