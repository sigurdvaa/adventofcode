string = "3, 4, 1, 5"
string2 = "230,1,2,221,97,252,168,169,57,99,0,254,181,255,235,167"

def knothash(numbers, lengths, pos = 0, skip = 0):
    listlen = len(numbers)
    for length in lengths:
            
        if length == 1:
            pos += length + skip
            skip += 1
            if pos > listlen:
                pos = pos % listlen
            continue

        if pos + length > listlen:
            overflow = (pos + length) - listlen
            reverse = numbers[overflow-1::-1]
            reverse += numbers[:pos-1:-1]
            for i in range(listlen-pos):
                numbers[pos+i] = reverse[i]
            for i in range(overflow):
                numbers[i] = reverse[listlen-pos+i]
        else:
            reverse = numbers[pos+length-1::-1]
            for i in range(length):
                numbers[pos+i] = reverse[i]

        pos += length + skip
        skip += 1
        if pos > listlen:
            pos = pos % listlen

    return (numbers, pos, skip)

print("Part 1")
lengths = [int(x) for x in string2.split(",")]
numbers = [x for x in range(256)]
numbers, pos, skip = knothash(numbers, lengths)
print(numbers[0]*numbers[1])

print("Part 2")
# ascii lengths
end = [17, 31, 73, 47, 23]
lengths = [ord(x) for x in string2] + end
numbers = [x for x in range(256)]
pos = 0
skip = 0
for i in range(64):
    numbers, pos, skip = knothash(numbers, lengths, pos, skip)
# sparse to dense
densehash = []
for i in range(16):
    xor = 0
    block = numbers[i*16:(i+1)*16]
    for b in block:
        xor = xor ^ b
    densehash += [xor]
# int2hex
hexstring = []
for n in densehash:
    hexstring += [format(n,"02x")]
print("".join(hexstring))
