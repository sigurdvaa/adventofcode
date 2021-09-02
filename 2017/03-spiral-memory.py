size = 549
center = 274
total = 549*549
find = 277678

print("Part 1")
step = 1
x = center
y = center
xDir = 1
yDir = -1
i = 2
array = [[0 for x in range(size)] for y in range(size)] 
array[x][y] = 1
done = False
while 1:
    for s in range(step):
        x += xDir
        if i == find:
            xDiff = abs(center - x)
            yDiff = abs(center - y)
            print(xDiff + yDiff)
            done = True
            break
        array[x][y] = i
        i += 1
    if done:
        break

    for s in range(step):
        y += yDir
        if i == find:
            xDiff = abs(center - x)
            yDiff = abs(center - y)
            print(xDiff + yDiff)
            done = True
            break
        array[x][y] = i
        i += 1
    if done:
        break

    step += 1
    xDir = xDir * -1
    yDir = yDir * -1

print("Part 2")
step = 1
total = 549*549
x = center
y = center
xDir = 1
yDir = -1
array = [[0 for x in range(size)] for y in range(size)] 
array[x][y] = 1
i = 2
done = False
def sumAdjacent(x, y):
    return array[x+1][y] + array[x+1][y-1] + array[x][y-1] + array[x-1][y-1] + array[x-1][y] + array[x-1][y+1] + array[x][y+1] + array[x+1][y+1]

while 1:
    for s in range(step):
        x += xDir
        value = sumAdjacent(x,y)
        if value > find:
            print(value)
            done = True
            break
        array[x][y] = value
        i += 1
    if done:
        break

    for s in range(step):
        y += yDir
        value = sumAdjacent(x,y)
        if value > find:
            print(value)
            done = True
            break
        array[x][y] = value
        i += 1
    if done:
        break

    step += 1
    xDir = xDir * -1
    yDir = yDir * -1
