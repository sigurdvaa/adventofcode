input_raw = """set i 31
set a 1
mul p 17
jgz p p
mul a 2
add i -1
jgz i -2
add a -1
set i 127
set p 618
mul p 8505
mod p a
mul p 129749
add p 12345
mod p a
set b p
mod b 10000
snd b
add i -1
jgz i -9
jgz a 3
rcv b
jgz b -1
set f 0
set i 126
rcv a
rcv b
set p a
mul p -1
add p b
jgz p 4
snd a
set a b
jgz 1 3
snd b
set f 1
add i -1
jgz i -11
snd a
jgz f -16
jgz a -19"""


instructions = input_raw.splitlines()
REGISTERS = dict([(x, 0) for x in "abcdefghijklmnopqrstuvwxyz"])


def prog1(registers):
    def getValue(val):
        try:
            val = int(val)
        except:
            val = registers[val]
        return val

    i = 0
    length = len(instructions)
    played = 0
    while i < length:
        ins = instructions[i].split(" ")
        if ins[0] == "snd":
            played = getValue(ins[1])
        elif ins[0] == "set":
            registers[ins[1]] = getValue(ins[2])
        elif ins[0] == "add":
            registers[ins[1]] += getValue(ins[2])
        elif ins[0] == "mul":
            registers[ins[1]] *= getValue(ins[2])
        elif ins[0] == "mod":
            registers[ins[1]] %= getValue(ins[2])
        elif ins[0] == "rcv":
            if registers[ins[1]] != 0:
                print(played)
                return
        if ins[0] == "jgz":
            if getValue(ins[1]) > 0:
                i += getValue(ins[2])
            else:
                i += 1
        else:
            i += 1


def prog2(registers, send, receive, pid):
    def getValue(val):
        try:
            val = int(val)
        except:
            val = registers[val]
        return val

    global lock
    global waiting
    i = 0
    registers["p"] = pid
    length = len(instructions)
    sent = 0

    while i < length:
        ins = instructions[i].split(" ")
        if ins[0] == "snd":
            with lock:
                send.insert(0, getValue(ins[1]))
                if pid == 1:
                    sent += 1
        elif ins[0] == "set":
            registers[ins[1]] = getValue(ins[2])
        elif ins[0] == "add":
            registers[ins[1]] += getValue(ins[2])
        elif ins[0] == "mul":
            registers[ins[1]] *= getValue(ins[2])
        elif ins[0] == "mod":
            registers[ins[1]] %= getValue(ins[2])
        elif ins[0] == "rcv":
            with lock:
                waiting += 1
            while len(receive) == 0:
                with lock:
                    if waiting == 2 and len(receive) == 0 and len(send) == 0:
                        if pid == 1:
                            print(sent)
                        return
            with lock:
                waiting -= 1
                registers[ins[1]] = receive.pop()

        if ins[0] == "jgz":
            if getValue(ins[1]) > 0:
                i += getValue(ins[2])
            else:
                i += 1
        else:
            i += 1


print("Part 1")
reg = dict(REGISTERS)
prog1(reg)

print("Part 2")
import threading

lock = threading.Lock()
mq0 = []
mq1 = []
waiting = 0
t1 = threading.Thread(target=prog2, args=(dict(REGISTERS), mq1, mq0, 0))
t2 = threading.Thread(target=prog2, args=(dict(REGISTERS), mq0, mq1, 1))
threads = [t1, t2]
for t in threads:
    t.start()
for t in threads:
    t.join()
