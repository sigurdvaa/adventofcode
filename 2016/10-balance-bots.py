with open('10-input.txt', 'r') as f:
    input_instructions = [x.strip() for x in f.readlines()]

class Bot:
    def __init__(self, id):
        self.id = id
        self.chips = []

    def add_chip(self, value):
        self.chips += [value]
        self.chips = sorted(self.chips)

    def give_low(self):
        low = self.chips[0]
        del self.chips[0]
        return low

bots = {}
outputs = {}

def bot_add(botid, value):
    global bots
    if not botid in bots:
        bots[botid] = [value]
    else:
        bots[botid] += [value]
        bots[botid] = sorted(bots[botid])
    print(bots[botid])
    if 61 in bots[botid]:
        print(f"Part One: {botid}")

def output_add(outputid):
    global outputs
    if not outputid in outputs:
        outputs[outputid] = [value]
    else:
        outputs[outputid] += [value]

for ins in input_instructions:
    split = ins.split()
    if split[0] == "value":
        botid = int(split[5])
        value = int(split[1])
        bot_add(botid, value)

for ins in input_instructions:
    split = ins.split()
    if split[0] == "bot":
        giver_id = int(split[1])
        if giver_id in bots and len(bots[giver_id]) > 1:
            low_type = split[5]
            low_id = int(split[6])
            high_type = split[10]
            high_id = int(split[11])

            if low_type == "output":
                output_add(low_id, bots[giver_id][0])
            else:
                bot_add(low_id, bots[giver_id][0])

            if high_type == "output":
                output_add(high_id, bots[giver_id][-1])
            else:
                bot_add(high_id, bots[giver_id][-1])

            del bots[giver_id][-1]
            del bots[giver_id][0]

