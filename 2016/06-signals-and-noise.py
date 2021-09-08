with open('06-input.txt','r') as f:
    input_lines = [x.strip() for x in f.readlines()]

message = ["" for x in range(8)]
for i in range(8):
    chars = {}
    for line in input_lines:
        if line[i] in chars:
            chars[line[i]] += 1
        else:
            chars[line[i]] = 1

    message[i] = list(chars.keys())[0]
    for c in chars:
        if chars[c] > chars[message[i]]:
            message[i] = c

print("Part One: " + "".join(message))

