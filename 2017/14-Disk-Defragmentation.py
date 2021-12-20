input_raw = "oundnydw"


def knothash(string):
    # ascii lengths
    lengths = [ord(x) for x in string] + [17, 31, 73, 47, 23]
    listlen = 256
    numbers = [x for x in range(listlen)]

    pos = 0
    skip = 0
    for i in range(64):
        for length in lengths:

            if length == 1:
                pos += length + skip
                skip += 1
                if pos > listlen:
                    pos = pos % listlen
                continue

            if pos + length > listlen:
                overflow = (pos + length) - listlen
                reverse = numbers[overflow - 1 :: -1]
                reverse += numbers[: pos - 1 : -1]
                for i in range(listlen - pos):
                    numbers[pos + i] = reverse[i]
                for i in range(overflow):
                    numbers[i] = reverse[listlen - pos + i]
            else:
                reverse = numbers[pos + length - 1 :: -1]
                for i in range(length):
                    numbers[pos + i] = reverse[i]

            pos += length + skip
            skip += 1
            if pos > listlen:
                pos = pos % listlen

    # sparse to dense
    densehash = []
    for i in range(16):
        xor = 0
        block = numbers[i * 16 : (i + 1) * 16]
        for b in block:
            xor = xor ^ b
        densehash += [xor]
    # 2hex
    hexstring = [format(n, "02x") for n in densehash]
    return "".join(hexstring)


def hextobin(string):
    result = []
    for c in string:
        binary = bin(int(c, 16))[2:]
        while len(binary) < 4:
            binary = "0" + binary
        result += [binary]
    return "".join(result)


def fillRegion(x, y, region):
    disk[x][y] = region
    if x == 0:
        if disk[x + 1][y] == 1:
            fillRegion(x + 1, y, region)
    elif x == 127:
        if disk[x - 1][y] == 1:
            fillRegion(x - 1, y, region)
    else:
        if disk[x + 1][y] == 1:
            fillRegion(x + 1, y, region)
        if disk[x - 1][y] == 1:
            fillRegion(x - 1, y, region)
    if y == 0:
        if disk[x][y + 1] == 1:
            fillRegion(x, y + 1, region)
    elif y == 127:
        if disk[x][y - 1] == 1:
            fillRegion(x, y - 1, region)
    else:
        if disk[x][y + 1] == 1:
            fillRegion(x, y + 1, region)
        if disk[x][y - 1] == 1:
            fillRegion(x, y - 1, region)


used = 0
disk = []
for i in range(128):
    binary = hextobin(knothash(input_raw + "-" + str(i)))
    disk += [[int(x) for x in binary]]
    used += sum([1 for n in binary if n == "1"])

print("Part 1\n" + str(used))

print("Part 2")
regions = 0
for x in range(128):
    for y in range(128):
        if disk[x][y] == 1:
            # New region, set region values to regions+1
            regions += 1
            fillRegion(x, y, regions + 1)
print(regions)
