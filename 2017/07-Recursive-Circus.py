with open("07_input.txt", "r") as f:
    input_raw = f.read()


weights = {}
disks = {}
for line in input_raw.splitlines():
    line_split = line.split(" ")
    weights[line_split[0]] = int(line_split[1][1:-1])

    if "->" in line:
        disks[line_split[0]] = []
        for child in line.split(" -> ")[1].split(", "):
            disks[line_split[0]] += [child]

print("Part 1")
for prog in weights:
    isChild = False
    for disk in disks:
        if prog in disks[disk]:
            isChild = True
    if not isChild:
        bottom = prog
        break
print(bottom)

print("Part 2")


def diskWeight(base):
    weight = 0
    if base in disks:
        for child in disks[base]:
            if child in disks:
                weight += diskWeight(child)
            else:
                weight += weights[child]
        return weight + weights[base]
    else:
        # not a disk, toplevel node
        return weights[base]


# Start from bottom and follow the odd one up
# Assuming only on per disk is wrong
def findWrongWeight(name):
    childWeights = {}
    for child in disks[name]:
        childWeights[child] = diskWeight(child)
    for child1 in childWeights:
        match = 0
        for child2 in childWeights:
            if childWeights[child1] == childWeights[child2]:
                match += 1
        # Only one has this value = false weight
        if match == 1:
            return findWrongWeight(child1)
    return name


wrong = findWrongWeight(bottom)
for disk in disks:
    if wrong in disks[disk]:
        weight1 = 0
        weight2 = 0
        for child in disks[disk]:
            if child == wrong:
                weight1 = diskWeight(child)
            else:
                weight2 = diskWeight(child)
        diff = weight2 - weight1
        falseWeight = weights[wrong]
        print(wrong + " should weigh " + str(falseWeight + diff))
        break
