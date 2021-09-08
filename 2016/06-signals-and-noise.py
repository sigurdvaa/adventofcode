with open('06-input.txt','r') as f:
    input_lines = [x.strip() for x in f.readlines()]

most_message = ["" for x in range(8)]
less_message = ["" for x in range(8)]
for i in range(8):
    chars = {}
    for line in input_lines:
        if line[i] in chars:
            chars[line[i]] += 1
        else:
            chars[line[i]] = 1

    most_message[i] = list(chars.keys())[0]
    less_message[i] = list(chars.keys())[0]
    for c in chars:
        if chars[c] > chars[most_message[i]]:
            most_message[i] = c
        if chars[c] < chars[less_message[i]]:
            less_message[i] = c

print("Part One: " + "".join(most_message))
print("Part Two: " + "".join(less_message))

