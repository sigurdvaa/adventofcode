from dataclasses import dataclass, field


@dataclass
class Node:
    int_child: int
    int_meta: int
    children: list
    metadata: list


def parse_nodes(data: list, int_child: int = 1, i: int = 0) -> tuple[list[Node], int]:
    children = []
    for _ in range(int_child):
        node_int_child = data[i]
        node_int_meta = data[i + 1]

        node_children, next_i = parse_nodes(data, node_int_child, i + 2)
        i = next_i + node_int_meta
        node_metadata = data[next_i:i]

        children.append(
            Node(node_int_child, node_int_meta, node_children, node_metadata)
        )

    return (children, i)


def sum_nodes_meta(node: Node) -> int:
    meta = 0
    for c in node.children:
        meta += sum_nodes_meta(c)
    return meta + sum(node.metadata)


def sum_nodes_value(node: Node) -> int:
    if node.int_child == 0:
        return sum(node.metadata)

    value = 0
    for m in node.metadata:
        m -= 1
        if -1 < m < node.int_child:
            value += sum_nodes_value(node.children[m])
    return value


with open("08_input.txt", "r") as fp:
    raw_input = [int(x) for x in fp.read().strip().split(" ")]

rootnode: Node = parse_nodes(raw_input)[0][0]

print(f"Part One: {sum_nodes_meta(rootnode)}")
print(f"Part Two: {sum_nodes_value(rootnode)}")
