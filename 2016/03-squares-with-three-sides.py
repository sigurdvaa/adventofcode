triangles = []
with open("03-input.txt", "r") as f:
    for line in f.readlines():
        triangles += [[int(x) for x in line.strip().split()]]

possible = 0
for triangle in triangles:
    if triangle[0] + triangle[1] > triangle[2]:
        if triangle[1] + triangle[2] > triangle[0]:
            if triangle[0] + triangle[2] > triangle[1]:
                possible += 1

print(f"Part One: {possible}")

triangles_row = []
triangles_column = []
with open("03-input.txt", "r") as f:
    for line in f.readlines():
        triangles_row += [[int(x) for x in line.strip().split()]]

x = 0
for y in range(0, len(triangles_row), 3):
    triangle = [triangles_row[y][x], triangles_row[y + 1][x], triangles_row[y + 2][x]]
    triangles_column += [triangle]
x = 1
for y in range(0, len(triangles_row), 3):
    triangle = [triangles_row[y][x], triangles_row[y + 1][x], triangles_row[y + 2][x]]
    triangles_column += [triangle]
x = 2
for y in range(0, len(triangles_row), 3):
    triangle = [triangles_row[y][x], triangles_row[y + 1][x], triangles_row[y + 2][x]]
    triangles_column += [triangle]

possible = 0
for triangle in triangles_column:
    if triangle[0] + triangle[1] > triangle[2]:
        if triangle[1] + triangle[2] > triangle[0]:
            if triangle[0] + triangle[2] > triangle[1]:
                possible += 1
print(f"Part Two: {possible}")
