input_raw = """124, 262
182, 343
79, 341
44, 244
212, 64
42, 240
225, 195
192, 325
192, 318
42, 235
276, 196
181, 262
199, 151
166, 214
49, 81
202, 239
130, 167
166, 87
197, 53
341, 346
235, 241
99, 278
163, 184
85, 152
349, 334
175, 308
147, 51
251, 93
163, 123
151, 219
162, 107
71, 58
249, 293
223, 119
46, 176
214, 140
80, 156
265, 153
92, 359
103, 186
242, 104
272, 202
292, 93
304, 55
115, 357
43, 182
184, 282
352, 228
267, 147
248, 271"""


_input_raw = """1, 1
1, 6
8, 3
3, 4
5, 5
8, 9"""


def finite_indexes(coords: list) -> list:
    finite = []
    for i1 in range(len(coords)):
        finite_x_pos = False
        finite_x_neg = False
        finite_y_pos = False
        finite_y_neg = False
        for i2 in range(len(coords)):
            x_diff = abs(coords[i1][0] - coords[i2][0])
            y_diff = abs(coords[i1][1] - coords[i2][1])

            # x axis
            if x_diff >= y_diff:
                if coords[i1][0] < coords[i2][0]:
                    finite_x_pos = True
                if coords[i1][0] > coords[i2][0]:
                    finite_x_neg = True

            # y axis
            if x_diff <= y_diff:
                if coords[i1][1] < coords[i2][1]:
                    finite_y_pos = True
                if coords[i1][1] > coords[i2][1]:
                    finite_y_neg = True

        if finite_x_neg and finite_x_pos and finite_y_neg and finite_y_pos:
            finite.append(i1)

    return finite


def largest_finite_area(coords: str) -> int:
    coords = [
        (int(y[0]), int(y[1])) for y in [x.split(", ") for x in coords.splitlines()]
    ]
    finite = finite_indexes(coords)

    areas = []
    for i, xy in enumerate(coords):
        area = {
            "finite": True if i in finite else False,
            "visited": 0,
            "next": set([(xy[0], xy[1])]),
        }
        areas.append(area)

    seen_coords = dict()
    step = 0
    expanding = True
    while expanding:
        expanding = False
        for i, area in enumerate(areas):
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
            if area["finite"] and len(area["next"]):
                expanding = True

        step += 1

    max_size = 0
    for area in areas:
        if area["finite"] and area["visited"] > max_size:
            max_size = area["visited"]
    return max_size


print(f"Part One: { largest_finite_area(input_raw) }")
