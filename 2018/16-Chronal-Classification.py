with open("16_input.txt", "r") as fp:
    input_raw = fp.read()


class Opcode:
    ops: list["Opcode"] = []

    def __init__(self, name: str, op: "lambda"):
        self.name: str = name
        self.op: "lambda" = op
        self.ids: set[int] = set()

    def __lt__(self, other) -> bool:
        return self.get_id() < other.get_id()

    def __repr__(self) -> str:
        return f"{self.name}: {self.ids}"

    def exec(self, regs: list[int], ins: list[int]) -> list[int]:
        regs[ins[3]] = self.op(regs, ins)
        return regs

    def add_id(self, idx: int):
        self.ids.add(idx)

    def remove_id(self, idx: int) -> bool:
        if idx in self.ids:
            self.ids.remove(idx)
            return True
        return False

    def get_id(self) -> int:
        if len(self.ids) == 1:
            for first in self.ids:
                return first
        return -1


class Sample:
    def __init__(self, before: list[int], ins: list[int], after: list[int]):
        self.before: list[int] = before
        self.ins: list[int] = ins
        self.after: list[int] = after

    def __repr__(self) -> str:
        return f"ins: {self.ins}, before: {self.before}, after: {self.after}"


def parse_input(string: str) -> list[Sample]:
    samples: list[Sample] = []
    test_prog: list[tuple[int]] = []
    lines: list[str] = string.splitlines()
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

    i += 2
    while i < len(lines):
        ins = tuple(map(int, lines[i].split()))
        test_prog.append(ins)
        i += 1

    return (samples, test_prog)


def samples_matching_opcodes(
    ops: list[Opcode], samples: list[Sample], threshold: int = 3
) -> int:
    matches: int = 0
    for sample in samples:
        after_match: int = 0
        for op in ops:
            if sample.after == op.exec(sample.before.copy(), sample.ins):
                after_match += 1
                op.add_id(sample.ins[0])
        if after_match >= threshold:
            matches += 1
            continue
    return matches


def find_opcode_id(ops: list[Opcode]):
    change: bool = True
    while change:
        change = False
        for op in ops:
            if op.get_id() != -1:
                for other in ops:
                    if op != other:
                        if other.remove_id(op.get_id()):
                            change = True
    ops.sort()


def run_test(ops: list[Opcode], ins: list[tuple[int]]) -> int:
    find_opcode_id(ops)
    regs = [0, 0, 0, 0]
    for i in ins:
        ops[i[0]].exec(regs, i)
    return regs[0]


Opcode.ops.extend(
    [
        Opcode("addr", lambda regs, ins: regs[ins[1]] + regs[ins[2]]),
        Opcode("addi", lambda regs, ins: regs[ins[1]] + ins[2]),
        Opcode("mulr", lambda regs, ins: regs[ins[1]] * regs[ins[2]]),
        Opcode("muli", lambda regs, ins: regs[ins[1]] * ins[2]),
        Opcode("bani", lambda regs, ins: regs[ins[1]] & regs[ins[2]]),
        Opcode("banr", lambda regs, ins: regs[ins[1]] & ins[2]),
        Opcode("bori", lambda regs, ins: regs[ins[1]] | regs[ins[2]]),
        Opcode("borr", lambda regs, ins: regs[ins[1]] | ins[2]),
        Opcode("setr", lambda regs, ins: regs[ins[1]]),
        Opcode("seti", lambda regs, ins: ins[1]),
        Opcode("gtir", lambda regs, ins: 1 if (ins[1] > regs[ins[2]]) else 0),
        Opcode("gtri", lambda regs, ins: 1 if (regs[ins[1]] > ins[2]) else 0),
        Opcode("gtrr", lambda regs, ins: 1 if (regs[ins[1]] > regs[ins[2]]) else 0),
        Opcode("eqir", lambda regs, ins: 1 if (ins[1] == regs[ins[2]]) else 0),
        Opcode("eqri", lambda regs, ins: 1 if (regs[ins[1]] == ins[2]) else 0),
        Opcode("eqrr", lambda regs, ins: 1 if (regs[ins[1]] == regs[ins[2]]) else 0),
    ]
)
samples, test_prog = parse_input(input_raw)
print(f"Part One: {samples_matching_opcodes(Opcode.ops, samples)}")
print(f"Part Two: {run_test(Opcode.ops, test_prog)}")
