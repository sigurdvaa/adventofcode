with open("22-input.txt", "r") as f:
    input_raw = f.readlines()[2:]

nodes = list()
for line in input_raw:
    split = line.split()
    xy = split[0].split("-")
    node = {
        "xy": (int(xy[1][1:]), int(xy[2][1:])),
        "size": int(split[1][:-1]),
        "used": int(split[2][:-1]),
        "free": int(split[3][:-1]),
        "perc": int(split[4][:-1]),
    }
    nodes.append(node)


def valid_nodes(nodes):
    valid = list()
    nodes_len = len(nodes)
    for i in range(nodes_len - 1):
        for j in range(i + 1, nodes_len):
            if nodes[i]["used"] > 0:
                if nodes[j]["free"] >= nodes[i]["used"]:
                    valid.append((nodes[i], nodes[j]))
            if nodes[j]["used"] > 0:
                if nodes[i]["free"] >= nodes[j]["used"]:
                    valid.append((nodes[j], nodes[i]))
    return valid


print(f"Part One: {len(valid_nodes(nodes))}")
