input_raw = """Tristram to AlphaCentauri = 34
Tristram to Snowdin = 100
Tristram to Tambi = 63
Tristram to Faerun = 108
Tristram to Norrath = 111
Tristram to Straylight = 89
Tristram to Arbre = 132
AlphaCentauri to Snowdin = 4
AlphaCentauri to Tambi = 79
AlphaCentauri to Faerun = 44
AlphaCentauri to Norrath = 147
AlphaCentauri to Straylight = 133
AlphaCentauri to Arbre = 74
Snowdin to Tambi = 105
Snowdin to Faerun = 95
Snowdin to Norrath = 48
Snowdin to Straylight = 88
Snowdin to Arbre = 7
Tambi to Faerun = 68
Tambi to Norrath = 134
Tambi to Straylight = 107
Tambi to Arbre = 40
Faerun to Norrath = 11
Faerun to Straylight = 66
Faerun to Arbre = 144
Norrath to Straylight = 115
Norrath to Arbre = 135
Straylight to Arbre = 127"""


def parse_locations(distances):
    locations = dict()
    for line in distances.splitlines():
        split = line.split()
        loc_1 = split[0]
        loc_2 = split[2]
        distance = int(split[4])

        if not loc_1 in locations:
            locations[loc_1] = {"visited": False, "to": dict()}
        if not loc_2 in locations:
            locations[loc_2] = {"visited": False, "to": dict()}

        locations[loc_1]["to"][loc_2] = distance
        locations[loc_2]["to"][loc_1] = distance

    return locations


def shortest_path_length(locations):
    loc_len = len(locations)
    paths = [{locs: ["Tristram"], distance: 0}]
    for loc in locations:
        pass


locations = parse_locations(input_raw)
print(f"Part One: {shortest_path_length(locations)}")
