with open(
    "20-input.txt",
    "r",
) as f:
    input_blacklist = [
        (int(x[0]), int(x[1])) for x in [x.strip().split("-") for x in f.readlines()]
    ]


def consolidate_blacklist(blacklist: list):
    new_blacklist = list()
    new_blacklist.append((0, 0))

    

def lowest_valued_ip(blacklist: list):
    iprange_start = 0
    iprange_end = 4294967295
    blacklist = consolidate_blacklist(blacklist)
    blacklist.sort(key=lambda x: x[0])
    print(blacklist[0:8])
    for i in range(len(blacklist)):
        if blacklist[i][1]+1 != blacklist[i+1][0]:
            return blacklist[i][1]+1

print(f"Part One: {lowest_valued_ip(input_blacklist)}")
