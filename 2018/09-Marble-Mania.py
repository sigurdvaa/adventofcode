input_raw = "435 players; last marble is worth 71184 points"


class Marble:
    def __init__(self, prev: "Marble", nxt: "Marble", data: int):
        self.prev = prev
        self.next = nxt
        self.data = data

    def __repr__(self):
        return str(self.data)


class MarbleGame:
    def __init__(self, players: int, marbles: int):
        self._len = 3
        self._player = (2 % players) + 1
        self._marbles = marbles
        _a = Marble(None, None, 0)
        _b = Marble(_a, None, 2)
        _c = Marble(_b, _a, 1)
        _a.next = _b
        _a.prev = _c
        _b.next = _c
        self._head = _a

    def __repr__(self):
        curr_marble = self._head
        marbles = []
        for _ in range(len(self)):
            marbles.append(str(curr_marble))
            curr_marble = curr_marble.next
        return " <> ".join(marbles)

    def __len__(self):
        return self._len

    def react(self, unit):
        self._len -= 2
        if unit.prev == None:
            self.head = unit.next.next
            self.head.prev = None
            return self.head
        elif unit.next.next == None:
            unit.prev.next = None
            return unit.prev
        else:
            unit.prev.next = unit.next.next
            unit.next.next.prev = unit.prev
            return unit.prev


def play_marble(players: int, int_marbles: int) -> int:
    marbles: list[int] = [0, 2, 1]
    curr_pos: int = 1
    curr_player: int = 2
    curr_marble: int = 2
    player_score: list[int] = [0] * players
    while curr_marble < int_marbles:
        curr_pos += 2
        if curr_pos > len(marbles):
            curr_pos -= len(marbles)

        curr_player += 1
        if curr_player > players:
            curr_player = 1

        curr_marble += 1
        if curr_marble % 23 == 0:
            remove_pos = curr_pos - 9
            if remove_pos < 0:
                remove_pos += len(marbles)

            player_score[curr_player - 1] += curr_marble + marbles.pop(remove_pos)
            curr_marble += 1

            curr_pos = remove_pos
            curr_pos += 2
            if curr_pos > len(marbles):
                curr_pos -= len(marbles)

            curr_player += 1
            if curr_player > players:
                curr_player = 1

        marbles.insert(curr_pos, curr_marble)

    return max(player_score)


players = int(input_raw.split(" ")[0])
marbles = int(input_raw.split(" ")[6])
print(f"Part One: {play_marble(players, marbles)}")
#print(f"Part Two: {play_marble(players, marbles*2)}")
#print(f"Part Two: {play_marble(players, marbles*100)}")
