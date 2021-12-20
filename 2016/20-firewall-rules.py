with open(
    "20_input.txt",
    "r",
) as f:
    input_blacklist = [
        (int(x[0]), int(x[1])) for x in [x.strip().split("-") for x in f.readlines()]
    ]


def consolidate_blacklist(blacklist: list):
    new_blacklist = list()
    new_blacklist.append((0, 0))
    for b in blacklist:

        idx_match = list()
        for idx in range(len(new_blacklist)):
            cur = new_blacklist[idx]
            if cur[0] <= b[0] <= cur[1] or cur[0] <= b[1] <= cur[1]:
                idx_match.append(idx)
            elif b[0] <= cur[0] <= b[1] or b[0] <= cur[1] <= b[1]:
                idx_match.append(idx)

        minval = b[0]
        maxval = b[1]
        for idx in reversed(idx_match):
            cur = new_blacklist[idx]
            if cur[0] < minval:
                minval = cur[0]
            if cur[1] > maxval:
                maxval = cur[1]
            del new_blacklist[idx]
        new_blacklist.append((minval, maxval))

    return sorted(new_blacklist, key=lambda x: x[0])


def lowest_valued_ip(blacklist: list):
    for i in range(len(blacklist)):
        if blacklist[i][1] + 1 != blacklist[i + 1][0]:
            return blacklist[i][1] + 1


def allowed_ips(blacklist: list):
    allowed = 0
    iprange_start = 0
    iprange_end = 4294967295

    for idx in range(len(blacklist) - 1):
        allowed += blacklist[idx + 1][0] - 1 - blacklist[idx][1]
    allowed += blacklist[0][0] - iprange_start
    allowed += iprange_end - blacklist[-1][1]

    return allowed


blacklist = consolidate_blacklist(input_blacklist)
print(f"Part One: {lowest_valued_ip(blacklist)}")
print(f"Part Two: {allowed_ips(blacklist)}")
