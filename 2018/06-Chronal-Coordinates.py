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


input_raw = """1, 1
1, 6
8, 3
3, 4
5, 5
8, 9"""


def noninfinite_index(coords: list) -> list:
    x_coords = [x[0] for x in coords]
    y_coords = [x[1] for x in coords]
    x_max = max(x_coords)
    x_min = min(x_coords)
    y_max = max(y_coords)
    y_min = min(y_coords)

    non_infinite = []
    for i in range(len(coords)):
        if x_min < coords[i][0] < x_max and y_min < coords[i][1] < y_max:
            non_infinite.append(i)

    return non_infinite


def largest_noninfinite_area(coords: str) -> int:
    coords = [
        (int(y[0]), int(y[1])) for y in [x.split(", ") for x in coords.splitlines()]
    ]
    max_coord = 0
    for coord in coords:
        if max(coord) > max_coord:
            max_coord = max(coord)
    max_coord *= 2
    locations = [[-1] * max_coord for x in range(max_coord)]
    non_infinite_index = noninfinite(coords)

    searching = True
    while searching:
        for c in coords:
            pass


print(f"Part One: { largest_noninfinite_area(input_raw) }")
