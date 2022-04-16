with open("06_input.txt", "r") as f:
    input_raw = [x.strip() for x in f.readlines()]


def finite_coord(coords: list, xy: tuple) -> bool:
    finite_x_pos = False
    finite_x_neg = False
    finite_y_pos = False
    finite_y_neg = False
    for c in coords:
        x_diff = abs(xy[0] - c[0])
        y_diff = abs(xy[1] - c[1])

        # x axis
        if x_diff >= y_diff:
            if xy[0] < c[0]:
                finite_x_pos = True
            if xy[0] > c[0]:
                finite_x_neg = True

        # y axis
        if x_diff <= y_diff:
            if xy[1] < c[1]:
                finite_y_pos = True
            if xy[1] > c[1]:
                finite_y_neg = True

    if finite_x_neg and finite_x_pos and finite_y_neg and finite_y_pos:
        return True

    return False


def largest_finite_area(coords: str) -> int:
    areas = []
    finite_areas = 0
    for i, xy in enumerate(coords):
        finite = finite_coord(coords, xy)
        if finite:
            finite_areas += 1
        area = {
            "finite": finite,
            "visited": 0,
            "next": set([(xy[0], xy[1])]),
        }
        areas.append(area)

    seen_coords = dict()
    step = 0
    while finite_areas > 0:
        for i, area in enumerate(areas):
            if len(area["next"]) > 0:
                new_coords = set()
                for next_coord in area["next"]:
                    if next_coord not in seen_coords:
                        area["visited"] += 1
                        seen_coords[next_coord] = (step, i)
                        new_coords.add((next_coord[0] + 1, next_coord[1]))
                        new_coords.add((next_coord[0] - 1, next_coord[1]))
                        new_coords.add((next_coord[0], next_coord[1] + 1))
                        new_coords.add((next_coord[0], next_coord[1] - 1))
                    else:
                        if seen_coords[next_coord][0] == step:
                            areas[seen_coords[next_coord][1]]["visited"] -= 1
                            seen_coords[next_coord] = (-1, -1)

                area["next"] = new_coords
                if area["finite"] and len(area["next"]) == 0:
                    finite_areas -= 1

        step += 1

    max_size = 0
    for area in areas:
        if area["finite"] and area["visited"] > max_size:
            max_size = area["visited"]
    return max_size


def nearest_region_size(coords: list, max_distance: int = 10000) -> int:
    region_size = 0
    x_max = max([xy[0] for xy in coords])
    y_max = max([xy[1] for xy in coords])
    for x in range(x_max):
        for y in range(y_max):
            total_distance = 0
            for coord in coords:
                total_distance += abs(x - coord[0]) + abs(y - coord[1])
            if total_distance < max_distance:
                region_size += 1

    return region_size


coords = [(int(y[0]), int(y[1])) for y in [x.split(", ") for x in input_raw]]
print(f"Part One: { largest_finite_area(coords) }")
print(f"Part Two: { nearest_region_size(coords) }")
