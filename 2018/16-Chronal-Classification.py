with open("16_input.txt", "r") as fp:
    input_raw = fp.read()


class Ins:
    def __init__(self, op: int, a: int, b: int, c: int):
        self.op: int = op
        self.a: int = a
        self.b: int = b
        self.c: int = c


class Sample:
    def __init__(self, before: tuple[int], ins: tuple[int], after: tuple[int]):
        self.before: tuple[int] = before
        self.ins: tuple[int] = ins
        self.after: tuple[int] = after

    def __repr__(self) -> str:
        return f"ins: {self.ins}, before: {self.before}, after: {self.after}"


def parse_samples(string: str) -> list[Sample]:
    samples: list[Sample] = []
    lines = string.splitlines()
    i: int = 0
    while lines[i].startswith("Before: "):
        samples.append(
            Sample(
                tuple(map(int, lines[i][9:-1].split(", "))),
                tuple(map(int, lines[i + 1].split())),
                tuple(map(int, lines[i + 2][9:-1].split(", "))),
            )
        )
        i += 4

    return samples


samples = parse_samples(input_raw)
