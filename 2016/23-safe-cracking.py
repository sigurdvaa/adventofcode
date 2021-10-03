input_raw = """cpy a b
dec b
cpy a d
cpy 0 a
cpy b c
inc a
dec c
jnz c -2
dec d
jnz d -5
dec b
cpy b c
cpy c d
dec d
inc c
jnz d -2
tgl c
cpy -16 c
jnz 1 c
cpy 73 c
jnz 79 d
inc a
inc d
jnz d -2
inc c
jnz c -5"""


def run_prog(regs: dict, ins: list, multiply: bool = False):
    ip = 0
    iend = len(ins)
    while ip < iend:
        # print(ip, regs)
        if ins[ip][0] == "cpy":
            if ins[ip][1] in regs:
                regs[ins[ip][2]] = regs[ins[ip][1]]
            elif ins[ip][2] in regs:
                regs[ins[ip][2]] = int(ins[ip][1])
        elif ins[ip][0] == "inc":
            if ins[ip][1] in regs:
                if multiply and ip == 5:
                    """
                    Looking at print(ip, regs) you can see that which ip's are repeated in the
                    long loop. This optimization does a = b * d; c = 0, d = 0, ip + 4.
                    """
                    regs[ins[ip][1]] = regs["b"] * regs["d"]
                    regs["c"] = 0
                    regs["d"] = 0
                    ip += 4
                else:
                    regs[ins[ip][1]] += 1
        elif ins[ip][0] == "dec":
            if ins[ip][1] in regs:
                regs[ins[ip][1]] -= 1
        elif ins[ip][0] == "jnz":
            if ins[ip][1] in regs:
                jnz = int(regs[ins[ip][1]])
            else:
                jnz = int(ins[ip][1])
            if jnz != 0:
                if ins[ip][2] in regs:
                    ip += int(regs[ins[ip][2]]) - 1
                else:
                    ip += int(ins[ip][2]) - 1
        elif ins[ip][0] == "tgl":
            idx = ip + int(regs[ins[ip][1]])
            if idx < iend:
                # one-argument
                if len(ins[idx]) == 2:
                    if ins[idx][0] == "inc":
                        ins[idx][0] = "dec"
                    else:
                        ins[idx][0] = "inc"
                # two-argument
                elif len(ins[idx]) == 3:
                    if ins[idx][0] == "jnz":
                        ins[idx][0] = "cpy"
                    else:
                        ins[idx][0] = "jnz"
        ip += 1
    return regs["a"]


ins = [x.split() for x in input_raw.splitlines()]
regs = {"a": 7, "b": 0, "c": 0, "d": 0}
print(f"Part One: {run_prog(regs, ins)}")

ins = [x.split() for x in input_raw.splitlines()]
regs = {"a": 12, "b": 0, "c": 0, "d": 0}
print(f"Part Two: {run_prog(regs, ins, True)}")
