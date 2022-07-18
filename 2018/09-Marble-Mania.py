input_raw = "435 players; last marble is worth 71184 points"


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
print(f"Part Two: {play_marble(players, marbles*2)}")
#print(f"Part Two: {play_marble(players, marbles*100)}")
