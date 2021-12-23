with open("21_input.txt", "r") as f:
    input_raw = f.read()


def parse_shop(strings: list) -> dict:
    shop = {}
    category = ""
    for line in strings:
        if line == "":
            category = ""
        elif category == "":
            split = line.strip().split()
            category = split[0][:-1]
            shop[category] = []
        else:
            split = line.strip().split()
            if len(split) == 4:
                shop[category].append((int(split[1]), int(split[2]), int(split[3])))
            else:
                shop[category].append((int(split[2]), int(split[3]), int(split[4])))

    return shop


def parse_stats(strings: list) -> list:
    stats = []
    for line in strings:
        split = line.strip().split()
        stats.append(int(split[-1]))

    return tuple(stats)


def win_fight(me: tuple, boss: tuple) -> bool:
    me_dmg = me[1] - boss[2]
    if me_dmg < 1:
        me_dmg = 1
    me_steps_win = -(-boss[0] // me_dmg)

    boss_dmg = boss[1] - me[2]
    if boss_dmg < 1:
        boss_dmg = 1
    boss_steps_win = -(-me[0] // boss_dmg)

    if me_steps_win <= boss_steps_win:
        return True

    return False


def get_shop_options(shop: dict) -> list:
    options = []
    for item in shop["Weapons"]:
        options.append([item])

    next_options = options.copy()
    for option in options:
        for item in shop["Armor"]:
            next_options.append(option + [item])
    options = next_options

    next_options = options.copy()
    for option in options:
        for i, item1 in enumerate(shop["Rings"]):
            next_options.append(option + [item1])
            if i < len(shop["Rings"]):
                for item2 in shop["Rings"][i + 1 :]:
                    next_options.append(option + [item1, item2])
    options = next_options

    return options


def cost_of_fight(boss: tuple, me: tuple, shop_options: list, win: bool = True) -> int:
    costs_win = []
    costs_lose = []
    for option in shop_options:
        cost = dmg = armor = 0
        for item in option:
            cost += item[0]
            dmg += item[1]
            armor += item[2]

        if win_fight((me[0], dmg, armor), boss):
            costs_win.append(cost)
        else:
            costs_lose.append(cost)

    if win:
        costs_win.sort()
        return costs_win[0]
    else:
        costs_lose.sort()
        return costs_lose[-1]


shop = parse_shop(input_raw.splitlines()[:-10])
boss = parse_stats(input_raw.splitlines()[23:26])
me = parse_stats(input_raw.splitlines()[28:])
shop_options = get_shop_options(shop)
print(f"Part One: {cost_of_fight(boss, me, shop_options)}")
print(f"Part Two: {cost_of_fight(boss, me, shop_options, False)}")
