from typing import Callable

Ins = tuple[str, int, int, int]

with open("21_input.txt", "r") as fp:
    input_raw: str = fp.read()


def parse_prog(string: str) -> tuple[int, list[Ins]]:
    lines: list[str] = string.splitlines()
    ipx: int = int(lines[0].split()[-1])
    ins: list[Ins] = []
    for line in lines[1:]:
        split = line.split()
        ins.append((split[0], int(split[1]), int(split[2]), int(split[3])))
    return (ipx, ins)


def run_prog(
    opcode: dict[str, Callable],
    regs: list[int],
    ipx: int,
    prog: list[Ins],
) -> int:
    prog_len = len(prog)
    seen = []
    while regs[ipx] < prog_len:
        ins = prog[regs[ipx]]
        regs[ins[3]] = opcode[ins[0]](regs, ins)
        regs[ipx] += 1
        if ins[0] == "eqrr":
            if not regs[ins[1]] in seen:
                seen.append(regs[ins[1]])
            else:
                return seen[0], seen[-1]
    return regs[0]


opcode: dict[str, Callable] = {
    "addr": lambda regs, ins: regs[ins[1]] + regs[ins[2]],
    "addi": lambda regs, ins: regs[ins[1]] + ins[2],
    "mulr": lambda regs, ins: regs[ins[1]] * regs[ins[2]],
    "muli": lambda regs, ins: regs[ins[1]] * ins[2],
    "banr": lambda regs, ins: regs[ins[1]] & regs[ins[2]],
    "bani": lambda regs, ins: regs[ins[1]] & ins[2],
    "borr": lambda regs, ins: regs[ins[1]] | regs[ins[2]],
    "bori": lambda regs, ins: regs[ins[1]] | ins[2],
    "setr": lambda regs, ins: regs[ins[1]],
    "seti": lambda regs, ins: ins[1],
    "gtir": lambda regs, ins: 1 if (ins[1] > regs[ins[2]]) else 0,
    "gtri": lambda regs, ins: 1 if (regs[ins[1]] > ins[2]) else 0,
    "gtrr": lambda regs, ins: 1 if (regs[ins[1]] > regs[ins[2]]) else 0,
    "eqir": lambda regs, ins: 1 if (ins[1] == regs[ins[2]]) else 0,
    "eqri": lambda regs, ins: 1 if (regs[ins[1]] == ins[2]) else 0,
    "eqrr": lambda regs, ins: 1 if (regs[ins[1]] == regs[ins[2]]) else 0,
}
ip, prog = parse_prog(input_raw)

regs: list[int] = [0, 0, 0, 0, 0, 0]
vals = run_prog(opcode, regs, ip, prog)
print(f"Part One: {vals[0]}")
print(f"Part Two: {vals[1]}")
