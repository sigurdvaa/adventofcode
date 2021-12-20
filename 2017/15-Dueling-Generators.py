input_raw = """Generator A starts with 699
Generator B starts with 124"""

generators = {}
for line in input_raw.splitlines():
    split = line.split(" ")
    generators[split[1]] = int(split[4])
factora = 16807
factorb = 48271
mod = 2147483647

print("Part 1")
pairs = 40000000
gena = generators["A"]
genb = generators["B"]
matches = 0
for i in range(pairs):
    gena = (gena * factora) % mod
    genb = (genb * factorb) % mod
    # compare 16 last bits = n & 65535 (2**16 - 1), this is faster than bin(genx)[-16:]
    if gena & 65535 == genb & 65535:
        matches += 1
print(matches)

print("Part 2")
pairs = 5000000
gena = generators["A"]
genb = generators["B"]
matches = 0
for i in range(pairs):
    gena = (gena * factora) % mod
    # & 3 instead of % 4, slightly faster
    while gena & 3 != 0:
        gena = (gena * factora) % mod
    genb = (genb * factorb) % mod
    # & 7 instead of % 8, slighty faster
    while genb & 7 != 0:
        genb = (genb * factorb) % mod
    # compare 16 last bits = n & 65535 (2**16 - 1), this is faster than bin(genx)[-16:]
    if gena & 65535 == genb & 65535:
        matches += 1
print(matches)
