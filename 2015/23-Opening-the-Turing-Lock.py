input_raw = """jio a, +18
inc a
tpl a
inc a
tpl a
tpl a
tpl a
inc a
tpl a
inc a
tpl a
inc a
inc a
tpl a
tpl a
tpl a
inc a
jmp +22
tpl a
inc a
tpl a
inc a
inc a
tpl a
inc a
tpl a
inc a
inc a
tpl a
tpl a
inc a
inc a
tpl a
inc a
inc a
tpl a
inc a
inc a
tpl a
jio a, +8
inc b
jie a, +4
tpl a
inc a
jmp +2
hlf a
jmp -7"""


def parse_instructions(string: str) -> list:
    instructions = []
    for line in string.splitlines():
        split = line.strip().split()
        if len(split) == 2:
            if len(split[1]) > 1:
                instructions.append([split[0], int(split[1])])
            else:
                instructions.append([split[0], split[1]])
        elif len(split) == 3:
            instructions.append([split[0], split[1][:-1], int(split[2])])

    return instructions


def run_program(registers: dict, instructions: list) -> int:
    ip = 0
    prog_len = len(instructions)
    while ip < prog_len:
        ins = instructions[ip]
        if ins[0] == "jio":
            if registers[ins[1]] == 1:
                ip += ins[2] - 1
        elif ins[0] == "jie":
            if registers[ins[1]] % 2 == 0:
                ip += ins[2] - 1
        elif ins[0] == "jmp":
            ip += ins[1] - 1
        elif ins[0] == "hlf":
            registers[ins[1]] = registers[ins[1]] // 2
        elif ins[0] == "tpl":
            registers[ins[1]] *= 3
        elif ins[0] == "inc":
            registers[ins[1]] += 1

        ip += 1

    return registers["b"]


instructions = parse_instructions(input_raw)
registers = {"a": 0, "b": 0}
print(f"Part One: {run_program(registers, instructions)}")
registers = {"a": 1, "b": 0}
print(f"Part Two: {run_program(registers, instructions)}")
