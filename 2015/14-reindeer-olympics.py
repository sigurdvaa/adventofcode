input_raw = """Vixen can fly 19 km/s for 7 seconds, but then must rest for 124 seconds.
Rudolph can fly 3 km/s for 15 seconds, but then must rest for 28 seconds.
Donner can fly 19 km/s for 9 seconds, but then must rest for 164 seconds.
Blitzen can fly 19 km/s for 9 seconds, but then must rest for 158 seconds.
Comet can fly 13 km/s for 7 seconds, but then must rest for 82 seconds.
Cupid can fly 25 km/s for 6 seconds, but then must rest for 145 seconds.
Dasher can fly 14 km/s for 3 seconds, but then must rest for 38 seconds.
Dancer can fly 3 km/s for 16 seconds, but then must rest for 37 seconds.
Prancer can fly 25 km/s for 6 seconds, but then must rest for 143 seconds."""


def reindeer_distance(reindeer: list, goal: str):
    for i in range(len(reindeer)):
        full_turns = goal // reindeer[i]["rotation_time"]
        leftover = goal % reindeer[i]["rotation_time"]

        reindeer[i]["distance"] = full_turns * (
            reindeer[i]["fly_speed"] * reindeer[i]["fly_time"]
        )
        if leftover > reindeer[i]["fly_time"]:
            reindeer[i]["distance"] += (
                reindeer[i]["fly_speed"] * reindeer[i]["fly_time"]
            )
        else:
            reindeer[i]["distance"] += reindeer[i]["fly_speed"] * leftover


def reindeer_winner_distance(reindeers: list, goal: int):
    reindeer_distance(reindeers, goal)

    winner = 0
    for item in reindeers:
        if item["distance"] > winner:
            winner = item["distance"]

    return winner


def reindeer_points(reindeers: list, goal: int):
    leading = 0
    for i in range(goal):
        for r in range(len(reindeers)):
            place_in_rot = i % reindeers[r]["rotation_time"]
            if place_in_rot < reindeers[r]["fly_time"]:
                reindeers[r]["distance"] += reindeers[r]["fly_speed"]
                if leading < reindeers[r]["distance"]:
                    leading = reindeers[r]["distance"]

        for r in range(len(reindeers)):
            if leading == reindeers[r]["distance"]:
                reindeers[r]["points"] += 1


def reindeer_winner_points(reindeers: list, goal: int):
    reindeer_points(reindeers, goal)

    winner = 0
    for item in reindeers:
        if item["points"] > winner:
            winner = item["points"]

    return winner


def parse_reindeers(string: str):
    reindeers = []
    for line in string.splitlines():
        split = line.split()
        reindeers.append(
            {
                "distance": 0,
                "points": 0,
                "fly_speed": int(split[3]),
                "fly_time": int(split[6]),
                "rest_time": int(split[13]),
                "rotation_time": int(split[6]) + int(split[13]),
            }
        )

    return reindeers


reindeers = parse_reindeers(input_raw)
print(f"Part One: {reindeer_winner_distance(reindeers, 2503)}")

reindeers = parse_reindeers(input_raw)
print(f"Part Two: {reindeer_winner_points(reindeers, 2503)}")
