string = '''set b 93
set c b
jnz a 2
jnz 1 5
mul b 100
sub b -100000
set c b
sub c -17000
set f 1
set d 2
set e 2
set g d
mul g e
sub g b
jnz g 2
set f 0
sub e -1
set g e
sub g b
jnz g -8
sub d -1
set g d
sub g b
jnz g -13
jnz f 2
sub h -1
set g b
sub g c
jnz g 2
jnz 1 3
sub b -17
jnz 1 -23'''

'''
Part 2
set a 1                         # as per Part 2 instruction
set b 93                        # b = 93
set c b                         # c = 93
jnz a 2                         # Part 2 starts
jnz 1 5                         / Part 1 starts
    mul b 100                   #
    sub b -100000               # b = (b * 100) + 100000
    set c b                     #
    sub c -17000                # c = b + 17000
set f 1                         # LOOP1, set f = 1
    set d 2                     # LOOP2, set d = 2
        set e 2                 # LOOP3, set e = 2
            set g d             # 
            mul g e             #
            sub g b             # g = (d * e) - b
            jnz g 2             # IF g == 0
                set f 0         # THEN f = 0
            sub e -1            # e += 1
            set g e             # 
            sub g b             # g = e - b
            jnz g -8            # LOOP3 while g != 0
        sub d -1                # d += 1
        set g d                 #
        sub g b                 # g = d - b
        jnz g -13               # LOOP2 while g != 0
        jnz f 2                 # IF f == 0
            sub h -1            # THEN h += 1
        set g b                 # g = b
        sub g c                     # g = g - c
        jnz g 2                     # IF g = 0
            jnz 1 3                 # THEN done
        sub b -17                   # else, b += 17
    jnz 1 -23                   # LOOP1 while True
'''

orig_registers = {}
for c in "abcdefgh":
    orig_registers[c] = 0

instructions = []
for line in string.splitlines():
    split = line.split(" ")
    try:
        split[1] = int(split[1])
    except:
        pass
    try:
        split[2] = int(split[2])
    except:
        pass   
    instructions.append([split[0], split[1], split[2]])

def getValue(value):
    if type(value) == int:
        return value
    else:
        return registers[value]

print("Part 1")
registers = dict(orig_registers)
mulcount = 0
length = len(instructions)
i = 0
while i < length:
    ins = instructions[i]
    if ins[0] == "set":
        registers[ins[1]] = getValue(ins[2])
    elif ins[0] == "sub":
        registers[ins[1]] -= getValue(ins[2])
    elif ins[0] == "mul":
        registers[ins[1]] *= getValue(ins[2])
        mulcount += 1
    if ins[0] == "jnz":
        if getValue(ins[1]) != 0:
            i += getValue(ins[2])
        else:
            i += 1
    else:
        i += 1
print(mulcount)

print("Part 2")
# for range b - c+1 (including c) with step 17. count not primes
b = (93 * 100) + 100000
c = b + 17000
step = 17
h = 0
for i in range(b, c+1, step):
    for s in range(2,int(i**(1/2.0))):
        if i % s == 0:
            h += 1
            break
print(h)
