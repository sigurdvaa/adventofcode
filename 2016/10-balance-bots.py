with open("10_input.txt", "r") as f:
    input_instructions = [x.strip() for x in f.readlines()]

bots = {}
inputs = []
outputs = {}
for ins in input_instructions:
    split = ins.split()
    if split[0] == "value":
        botid = int(split[5])
        value = int(split[1])
        inputs += [
            {
                "botid": botid,
                "value": value,
            }
        ]

    if split[0] == "bot":
        bot_id = int(split[1])

        low_type = split[5]
        low_id = int(split[6])
        high_type = split[10]
        high_id = int(split[11])

        bots[bot_id] = {
            "low_type": low_type,
            "low_id": low_id,
            "high_type": high_type,
            "high_id": high_id,
            "chips": [],
        }

for value in inputs:
    bots[value["botid"]]["chips"] += [value["value"]]

moving_chips = True
while moving_chips:
    moving_chips = False
    for bot in bots:
        if len(bots[bot]["chips"]) > 1:
            if 17 in bots[bot]["chips"]:
                if 61 in bots[bot]["chips"]:
                    print(f"Part One: {bot}")

            if bots[bot]["low_type"] == "bot":
                bots[bots[bot]["low_id"]]["chips"] += [bots[bot]["chips"][0]]
                bots[bots[bot]["low_id"]]["chips"] = sorted(
                    bots[bots[bot]["low_id"]]["chips"]
                )
            else:
                if bots[bot]["low_id"] in outputs:
                    outputs[bots[bot]["low_id"]] += [bots[bot]["chips"][0]]
                else:
                    outputs[bots[bot]["low_id"]] = [bots[bot]["chips"][0]]

            if bots[bot]["high_type"] == "bot":
                bots[bots[bot]["high_id"]]["chips"] += [bots[bot]["chips"][-1]]
                bots[bots[bot]["high_id"]]["chips"] = sorted(
                    bots[bots[bot]["high_id"]]["chips"]
                )
            else:
                if bots[bot]["high_id"] in outputs:
                    outputs[bots[bot]["high_id"]] += [bots[bot]["chips"][-1]]
                else:
                    outputs[bots[bot]["high_id"]] = [bots[bot]["chips"][-1]]

            del bots[bot]["chips"][-1]
            del bots[bot]["chips"][0]
            moving_chips = True

product = 1
for o in range(3):
    for i in outputs[o]:
        product *= i
print(f"Part Two: {product}")
