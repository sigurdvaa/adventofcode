with open("16_input.txt", "r") as fp:
    input_raw = fp.read()


opcode = {
    "addr": lambda regs,ins: regs[ins[1]] + regs[ins[2]],
    "addi": lambda regs,ins: regs[ins[1]] + ins[2],
    "mulr": lambda regs,ins: regs[ins[1]] * regs[ins[2]],
    "muli": lambda regs,ins: regs[ins[1]] * ins[2],
    "bani": lambda regs,ins: regs[ins[1]] & regs[ins[2]],
    "banr": lambda regs,ins: regs[ins[1]] & ins[2],
    "bori": lambda regs,ins: regs[ins[1]] | regs[ins[2]],
    "borr": lambda regs,ins: regs[ins[1]] | ins[2],
    "setr": lambda regs,ins: regs[ins[1]],
    "seti": lambda regs,ins: ins[1],
    "gtir": lambda regs,ins: 1 if (ins[1] > regs[ins[2]]) else 0,
    "gtri": lambda regs,ins: 1 if (regs[ins[1]] > ins[2]) else 0,
    "gtrr": lambda regs,ins: 1 if (regs[ins[1]] > regs[ins[2]]) else 0,
    "eqir": lambda regs,ins: 1 if (ins[1] == regs[ins[2]]) else 0,
    "eqri": lambda regs,ins: 1 if (regs[ins[1]] == ins[2]) else 0,
    "eqrr": lambda regs,ins: 1 if (regs[ins[1]] == regs[ins[2]]) else 0,
}


class Opcode:
    def __init__(self, name: str, op: "lambda"):
        self.name: str = name
        self.op = op

    def exec(self, regs: list[int], ins: list[int]) -> list[int]:
        regs[ins[3]] = self.op(regs, ins)
        return regs


opcodes: list[Opcode] = [
    Opcode("addr", lambda regs,ins: regs[ins[1]] + regs[ins[2]]),
    Opcode("addi", lambda regs,ins: regs[ins[1]] + ins[2]),
]


class Sample:
    def __init__(self, before: list[int], ins: list[int], after: list[int]):
        self.before: list[int] = before
        self.ins: list[int] = ins
        self.after: list[int] = after

    def __repr__(self) -> str:
        return f"ins: {self.ins}, before: {self.before}, after: {self.after}"


def parse_samples(string: str) -> list[Sample]:
    samples: list[Sample] = []
    lines = string.splitlines()
    i: int = 0
    while lines[i].startswith("Before: "):
        samples.append(
            Sample(
                list(map(int, lines[i][9:-1].split(", "))),
                list(map(int, lines[i + 1].split())),
                list(map(int, lines[i + 2][9:-1].split(", "))),
            )
        )
        i += 4

    return samples


def samples_matching_opcodes(opcode: dict[str, "lambda"], samples: list[Sample], threshold: int = 3) -> int:
    matches: int = 0
    for sample in samples:
        after_match: int = 0
        for op in opcode.values():
            before = sample.before.copy()
            before[sample.ins[3]] = op(sample.before.copy(), sample.ins)
            if before == sample.after:
                after_match += 1
        if after_match >= threshold:
            matches += 1
            continue
    return matches

samples = parse_samples(input_raw)
print(f"Part One: {samples_matching_opcodes(opcode, samples)}")
