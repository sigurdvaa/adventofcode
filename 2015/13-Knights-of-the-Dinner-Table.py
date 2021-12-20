with open("13_input.txt", "r") as f:
    input_raw = [x.strip() for x in f.readlines()]


def all_seatings(names: list):
    if len(names) == 1:
        return [names]
    else:
        seatings = []
        for name in names:
            next_names = names.copy()
            next_names.remove(name)
            subseats = all_seatings(next_names)
            for subseat in subseats:
                seatings.append([name] + subseat)

        return seatings


def get_names(seating_preferences: list):
    names = []
    for line in seating_preferences:
        split = line.split()
        if not split[0] in names:
            names.append(split[0])

    return names


def get_relevant_seatings(names: list):
    seatings = []
    subseatings = all_seatings(names[1:])
    for subseat in subseatings:
        seatings.append([names[0]] + subseat)

    return seatings


def parse_preferences(string: str):
    preferences = {}
    for line in string:
        split = line.split()
        person_a = split[0]
        person_b = split[10][:-1]
        influence = split[2]
        units = int(split[3])

        if not person_a in preferences:
            preferences[person_a] = {}

        if influence == "gain":
            preferences[person_a][person_b] = units
        else:
            preferences[person_a][person_b] = -units

    return preferences


def seating_happiness(seating: list, preferences: dict):
    happiness = 0
    for i in range(1, len(seating) - 1):
        happiness += preferences[seating[i]][seating[i + 1]]
        happiness += preferences[seating[i]][seating[i - 1]]

    happiness += preferences[seating[0]][seating[1]]
    happiness += preferences[seating[0]][seating[-1]]

    happiness += preferences[seating[-1]][seating[0]]
    happiness += preferences[seating[-1]][seating[-2]]

    return happiness


def optimal_happiness(seatings: list, preferences: dict):
    optimal = 0
    for seating in seatings:
        happiness = seating_happiness(seating, preferences)
        if happiness > optimal:
            optimal = happiness

    return optimal


names = get_names(input_raw)
seatings = get_relevant_seatings(names)
preferences = parse_preferences(input_raw)
print(f"Part One: {optimal_happiness(seatings, preferences)}")


names.append("Myself")
seatings = get_relevant_seatings(names)
preferences["Myself"] = {}
for name in preferences:
    preferences["Myself"][name] = 0
    preferences[name]["Myself"] = 0
print(f"Part Two: {optimal_happiness(seatings, preferences)}")
