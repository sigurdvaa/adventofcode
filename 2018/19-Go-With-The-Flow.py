from typing import Callable

Ins = tuple[str, int, int, int]

with open("19_input.txt", "r") as fp:
    input_raw = fp.read()


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
    target_num_only: bool = False,
) -> int:
    prog_len = len(prog)
    while regs[ipx] < prog_len:
        if target_num_only and regs[ipx] == 1:
            return max(regs)
        else:
            ins = prog[regs[ipx]]
            regs[ins[3]] = opcode[ins[0]](regs, ins)
        regs[ipx] += 1
    return regs[0]


def sum_all_factors(num: int) -> int:
    sum_factors: int = 0
    s = int(num ** 0.5)
    for i in range(1, s + 1):
        if num % i == 0:
            sum_factors += i + num // i
    return sum_factors


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
print(f"Part One: {run_prog(opcode, regs, ip, prog)}")
regs = [1, 0, 0, 0, 0, 0]
num = run_prog(opcode, regs, ip, prog, target_num_only=True)
print(f"Part Two: {sum_all_factors(num)}")
