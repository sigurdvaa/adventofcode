with open("17_input.txt", "r") as fp:
    input_raw = fp.read()

input_raw = """x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504"""


def parse_scan(string: str) -> set[tuple[int]]:
    scan: set[tuple[int]] = set()
    for line in string.splitlines():
        split: list[str] = line.split(", ")
        c_start: int = int(split[0][2:])
        c_range: list[int] = list(map(int, split[1][2:].split("..")))
        x_indexed: bool = True if (split[0][0] == "x") else False
        for i in range(c_range[0], c_range[1] + 1):
            if x_indexed:
                scan.add((c_start, i))
            else:
                scan.add((i, c_start))

    return scan


print(parse_scan(input_raw))
