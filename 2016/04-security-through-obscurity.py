rooms = []
with open('04-input.txt','r') as f:
    for line in f.readlines():
        parts = line.strip().split("-")
        room = {
            "enc_name": "".join(parts[:-1]),
            "id": int(parts[-1].split("[")[0]),
            "checksum": parts[-1].split("[")[1][:-1],
            "real": False
        }
        rooms += [room]

for i in range(len(rooms)):
    count_chars = dict()
    for c in rooms[i]["enc_name"]:
        if c in count_chars.keys():
            count_chars[c] += 1
        else:
            count_chars[c] = 1
   
    # need better sort, alphabetic if same count
    sorted_count_chars = dict(sorted(count_chars.items(), key=lambda x:x[1], reverse=True))
    if "".join(sorted_count_chars.keys())[:5] == rooms[i]["checksum"]:
        rooms[i]["real"] = True

sum_real_id = 0
for room in rooms:
    if room["real"]:
        sum_real_id += room["id"]

print(f"Part One: {sum_real_id}")
