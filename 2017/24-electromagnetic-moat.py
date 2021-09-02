string = '''0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10'''

string2 = '''48/5
25/10
35/49
34/41
35/35
47/35
34/46
47/23
28/8
27/21
40/11
22/50
48/42
38/17
50/33
13/13
22/33
17/29
50/0
20/47
28/0
42/4
46/22
19/35
17/22
33/37
47/7
35/20
8/36
24/34
6/7
7/43
45/37
21/31
37/26
16/5
11/14
7/23
2/23
3/25
20/20
18/20
19/34
25/46
41/24
0/33
3/7
49/38
47/22
44/15
24/21
10/35
6/21
14/50'''

components = {}
for line in string2.splitlines():
    a, b = [int(x) for x in line.split("/")]
    if not a in components:
        components[a] = set()
        components[a].add(b)
    else:
        components[a].add(b)
    if not b in components:
        components[b] = set()
        components[b].add(a)
    else:
        components[b].add(a)

# Fastest with python3
def buildBridges(components):
    bridges = [[(0, 0)]]
    start = 0
    length = 1
    i = 0
    while 1:
        for i in range(start, length):
            bridge = bridges[i]
            lastport = bridge[-1][1]
            pairs = set(bridge)
            for nextport in components[lastport]:
                if not ((lastport, nextport) in pairs or (nextport, lastport) in pairs):
                    bridges += [bridge+[(lastport, nextport)]]
                    length += 1
        if start == length:
            break
        start = i+1
    return bridges

# Fastest with pypy3
#def buildBridges(components):
#    bridges = [[(0, 0)]]
#    start = 0
#    length = 1
#    i = 0
#    while 1:
#        for i in range(start, length):
#            bridge = bridges[i]
#            lastport = bridge[-1][1]
#            for nextport in components[lastport]:
#                if not ((lastport, nextport) in bridge or (nextport, lastport) in bridge):
#                    bridges += [bridge+[(lastport, nextport)]]
#                    length += 1
#        if start == length:
#            break
#        start = i+1
#    return bridges

#def buildBridges(components, bridge = [(0,0)]):
#        lastport = bridge[-1][1]
#        for nextport in components[lastport]:
#            if not ((lastport, nextport) in bridge or (nextport, lastport) in bridge):
#                newbridge = bridge+[(lastport, nextport)]
#                yield newbridge
#                yield from buildBridges(components, newbridge)

high = 0
longesthigh = 0
longest = 0
for b in buildBridges(components):
    
    strength = 0
    for c in b:
        strength += c[0] + c[1]

    if strength > high:
        high = strength

    length = len(b)
    if length > longest:
        longest = length
        longesthigh = strength
    elif length == longest:
        if strength > longesthigh:
            longesthigh = strength

print("Part 1")
print(high)

print("Part 2")
print(longesthigh)
