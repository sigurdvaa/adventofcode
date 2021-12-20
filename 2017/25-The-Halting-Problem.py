with open("25_input.txt", "r") as f:
    input_raw = f.read()


tape = ["0"]
tapelength = 1
cursor = 0
state = "A"
direction = 0
states = {}
steps = 0
i = 0

lines = input_raw.splitlines()
state = lines[0][-2]
steps = int(lines[1].split(" ")[-2])

for i in range(3, len(lines), 10):
    name = lines[i][-2]
    # rule0if = lines[i+1][-2]
    rule0write = lines[i + 2][-2]
    rule0direction = lines[i + 3].split(" ")[-1][:-1]
    if rule0direction == "right":
        rule0direction = 1
    else:
        rule0direction = -1
    rule0nextstate = lines[i + 4][-2]
    # rule1if = lines[i+5][-2]
    rule1write = lines[i + 6][-2]
    rule1direction = lines[i + 7].split(" ")[-1][:-1]
    if rule1direction == "right":
        rule1direction = 1
    else:
        rule1direction = -1
    rule1nextstate = lines[i + 8][-2]
    states[name] = {
        "0": {
            "write": rule0write,
            "direction": rule0direction,
            "nextstate": rule0nextstate,
        },
        "1": {
            "write": rule1write,
            "direction": rule1direction,
            "nextstate": rule1nextstate,
        },
    }

for i in range(steps):

    cursor += direction
    if cursor == tapelength:
        tapelength += 1
        tape += ["0"]
    elif cursor == -1:
        cursor = 0
        tape[0:0] = "0"
        tapelength += 1

    rule = states[state][tape[cursor]]
    direction = rule["direction"]
    tape[cursor] = rule["write"]
    state = rule["nextstate"]

checksum = 0
for c in tape:
    if c == "1":
        checksum += 1
print("Part 1")
print(checksum)
