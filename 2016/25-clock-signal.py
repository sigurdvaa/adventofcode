input_raw = """cpy a d
cpy 15 c
cpy 170 b
inc d
dec b
jnz b -2
dec c
jnz c -5
cpy d a
jnz 0 0
cpy a b
cpy 0 a
cpy 2 c
jnz b 2
jnz 1 6
dec b
dec c
jnz c -4
inc a
jnz 1 -7
cpy 2 b
jnz c 2
jnz 1 4
dec b
dec c
jnz 1 -4
jnz 0 0
out b
jnz a -19
jnz 1 -21"""
ins = [x.split() for x in input_raw.splitlines()]


def run_prog(regs: dict, ins: list, outputlen: int):
    output = list()
    ip = 0
    iend = len(ins)
    while ip < iend:
        #print(ip, regs)
        if ins[ip][0] == "cpy":
            if ins[ip][1] in regs:
                regs[ins[ip][2]] = regs[ins[ip][1]]
            else:
                regs[ins[ip][2]] = int(ins[ip][1])
        elif ins[ip][0] == "inc":
            regs[ins[ip][1]] += 1
        elif ins[ip][0] == "dec":
            regs[ins[ip][1]] -= 1
        elif ins[ip][0] == "out":
            output.append(regs["b"])
            if len(output) == outputlen:
                return "".join(map(str, output))
        elif ins[ip][0] == "jnz":
            if ins[ip][1] in regs:
                jnz = int(regs[ins[ip][1]])
            else:
                jnz = int(ins[ip][1])
            if jnz != 0:
                ip += int(ins[ip][2]) - 1
        ip += 1
    return regs["a"]


i = 0
while True:
    regs = {"a": i, "b": 0, "c": 0, "d": 0}
    output = run_prog(regs, ins, 8)
    if output == "01"*4:
        print(f"Part One: {i}")
        break
    i += 1
