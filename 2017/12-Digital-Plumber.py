with open("12_input.txt", "r") as f:
    input_raw = f.read()


groups = {}
for line in input_raw.splitlines():
    gid = int(line.split(" ")[0])
    groups[gid] = []
    for member in line.split(" <-> ")[1].split(", "):
        groups[gid] += [int(member)]


def findMembers(gid, members):
    for m in groups[gid]:
        if m not in members:
            members += [m]
            members = findMembers(m, members)
    for i in groups:
        if gid in groups[i] and i not in members:
            members += [i]
            members = findMembers(i, members)
    return members


print("Part 1")
members = []
members = findMembers(0, members)
print(len(members))

print("Part 2")
intGroups = 0
members = []
for gid in groups:
    if not gid in members:
        intGroups += 1
        members += [gid]
        members = findMembers(gid, members)
print(intGroups)
