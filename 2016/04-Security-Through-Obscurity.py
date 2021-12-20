import string

rooms = []
with open("04_input.txt", "r") as f:
    for line in f.readlines():
        parts = line.strip().split("-")
        room = {
            "enc_name": "".join(parts[:-1]),
            "dec_name": "",
            "id": int(parts[-1].split("[")[0]),
            "checksum": parts[-1].split("[")[1][:-1],
            "real": False,
        }
        rooms += [room]

for i in range(len(rooms)):
    count_chars = []
    for c in rooms[i]["enc_name"]:
        count_chars_len = len(count_chars)
        for s in range(count_chars_len):
            if c == count_chars[s][0]:
                count_chars[s] = (c, count_chars[s][1] + 1)
                break
        else:
            count_chars += [(c, 1)]

    char_i = 0
    count_chars_len = len(count_chars)
    while char_i < count_chars_len:
        for s in range(count_chars_len - 1, char_i, -1):
            if count_chars[char_i][1] < count_chars[s][1] or (
                count_chars[char_i][1] == count_chars[s][1]
                and count_chars[char_i][0] > count_chars[s][0]
            ):
                count_chars.insert(s + 1, count_chars[char_i])
                del count_chars[char_i]
                char_i = -1
                break
        char_i += 1

    if "".join([x[0] for x in count_chars[:5]]) == rooms[i]["checksum"]:
        rooms[i]["real"] = True

sum_real_id = 0
for room in rooms:
    if room["real"]:
        sum_real_id += room["id"]

print(f"Part One: {sum_real_id}")

alphabet = string.ascii_lowercase
for r in range(len(rooms)):
    if rooms[r]["real"]:
        dec_name = []
        for char in rooms[r]["enc_name"]:
            i = alphabet.index(char)
            dec_name += [alphabet[(i + rooms[r]["id"]) % len(alphabet)]]
        rooms[r]["dec_name"] = "".join(dec_name)

for room in rooms:
    if room["real"]:
        if "northpole" in room["dec_name"]:
            print(f"Part Two: {room['id']}")
