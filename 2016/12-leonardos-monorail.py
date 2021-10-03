input_raw = """cpy 1 a
cpy 1 b
cpy 26 d
jnz c 2
jnz 1 5
cpy 7 c
inc d
dec c
jnz c -2
cpy a c
inc a
dec b
jnz b -2
cpy c b
dec d
jnz d -6
cpy 19 c
cpy 14 d
inc a
dec d
jnz d -2
dec c
jnz c -5"""

ins = [x.split() for x in input_raw.splitlines()]


def run_prog(regs, ins):
    ip = 0
    iend = len(ins)
    while ip < iend:
        if ins[ip][0] == "cpy":
            if ins[ip][1] in regs:
                regs[ins[ip][2]] = regs[ins[ip][1]]
            else:
                regs[ins[ip][2]] = int(ins[ip][1])
        elif ins[ip][0] == "inc":
            regs[ins[ip][1]] += 1
        elif ins[ip][0] == "dec":
            regs[ins[ip][1]] -= 1
        elif ins[ip][0] == "jnz":
            if ins[ip][1] in regs:
                jnz = int(regs[ins[ip][1]])
            else:
                jnz = int(ins[ip][1])
            if jnz != 0:
                ip += int(ins[ip][2]) - 1
        ip += 1
    return regs["a"]


regs = {"a": 0, "b": 0, "c": 0, "d": 0}
print(f"Part One: {run_prog(regs, ins)}")

regs = {"a": 0, "b": 0, "c": 1, "d": 0}
print(f"Part Two: {run_prog(regs, ins)}")
