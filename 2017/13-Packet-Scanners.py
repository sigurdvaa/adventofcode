input_raw = """0: 5
1: 2
2: 3
4: 4
6: 6
8: 4
10: 8
12: 6
14: 6
16: 14
18: 6
20: 8
22: 8
24: 10
26: 8
28: 8
30: 10
32: 8
34: 12
36: 9
38: 20
40: 12
42: 12
44: 12
46: 12
48: 12
50: 12
52: 12
54: 12
56: 14
58: 14
60: 14
62: 20
64: 14
66: 14
70: 14
72: 14
74: 14
76: 14
78: 14
80: 12
90: 30
92: 17
94: 18"""


scanners = {}
for line in input_raw.splitlines():
    depth, scanrange = [int(x) for x in line.split(": ")]
    scanners[depth] = scanrange


print("Part 1")
severity = 0
for i in scanners:
    if i / (scanners[i] - 1) % 2 == 0:
        severity += i * scanners[i]
print(severity)

print("Part 2")
skip = []
for delay in range(10000000):
    fail = False
    for i in scanners:
        if (i + delay) / (scanners[i] - 1) % 2 == 0:
            fail = True
            break
    if not fail:
        print(delay)
        break
