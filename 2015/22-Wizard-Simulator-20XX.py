input_raw = """Boss:
Hit Points: 71
Damage: 10

Player:
Hit Points: 50
Mana Points: 500

Spells:
Magic Missile: cost 53, damage 4
Drain: cost 73, damage 2, heal 2
Shield: cost 113, armor 7, lasts 7
Poison: cost 173, damage 3, lasts 6
Recharge: cost 229, mana 101, lasts 5"""


def next_spells(queue: list, state: dict, spells: list) -> bool:
    afford_spell = False
    for spell in spells:
        if spell["cost"] <= state["player_mana"]:
            affort_spell = True
            is_active = False
            for active_spell in state["active_spells"]:
                if spell["name"] == active_spell["name"]:
                    if active_spell["lasts"] > 1:
                        is_active = True
            if not is_active:
                next_state = state.copy()
                next_state["active_spells"] = []
                for active_spell in state["active_spells"]:
                    next_state["active_spells"].append(active_spell.copy())
                next_state["current_spell"] = spell.copy()
                next_state["player_mana"] -= spell["cost"]
                next_state["total_cost"] += spell["cost"]
                queue.append(next_state)

    return afford_spell


def update_state(state: dict, wounded: bool) -> None:
    # player turn
    if wounded:
        state["player_hp"] -= 1
        if state["player_hp"] < 1:
            return None

    for i in range(len(state["active_spells"]) - 1, -1, -1):
        active_spell = state["active_spells"][i]
        if "dmg" in active_spell:
            state["boss_hp"] -= active_spell["dmg"]
        if "mana" in active_spell:
            state["player_mana"] += active_spell["mana"]
        active_spell["lasts"] -= 1
        if active_spell["lasts"] < 1:
            del state["active_spells"][i]

    spell = state["current_spell"]
    if "lasts" in spell:
        state["active_spells"].append(spell)
    else:
        if "dmg" in spell:
            state["boss_hp"] -= spell["dmg"]
        if "heal" in spell:
            state["player_hp"] += spell["heal"]

    # boss turn
    if state["boss_hp"] > 0:
        player_armor = 0
        for i in range(len(state["active_spells"]) - 1, -1, -1):
            active_spell = state["active_spells"][i]
            if "dmg" in active_spell:
                state["boss_hp"] -= active_spell["dmg"]
            if "mana" in active_spell:
                state["player_mana"] += active_spell["mana"]
            if "armor" in active_spell:
                player_armor = active_spell["armor"]
            active_spell["lasts"] -= 1
            if active_spell["lasts"] < 1:
                del state["active_spells"][i]

    if state["boss_hp"] > 0:
        boss_dmg = state["boss_dmg"] - player_armor
        boss_dmg = boss_dmg if boss_dmg > 0 else 1
        state["player_hp"] -= boss_dmg


def search_least_mana(
    boss: tuple, player: tuple, spells: dict, wounded: bool = False
) -> int:
    from collections import deque

    init_state = {
        "boss_hp": boss[0],
        "boss_dmg": boss[1],
        "player_hp": player[0],
        "player_mana": player[1],
        "active_spells": [],
        "current_spell": {},
        "total_cost": 0,
    }
    queue = deque()
    next_spells(queue, init_state, spells)
    wins = []
    losses = []
    while len(queue):
        state = queue.popleft()
        update_state(state, wounded)
        if state["player_hp"] < 1:
            losses.append(state["total_cost"])
        elif state["boss_hp"] < 1:
            wins.append(state["total_cost"])
        elif not next_spells(queue, state, spells):
            losses.append(state["total_cost"])

    if len(wins):
        wins.sort()
        return wins[0]
    else:
        return -1


boss = (71, 10)
player = (50, 500)
# 226
# boss = (13, 8)
# 641
# boss = (14, 8)
# player = (10, 250)
spells = [
    {"name": "Magic Missile", "cost": 53, "dmg": 4},
    {"name": "Drain", "cost": 73, "dmg": 2, "heal": 2},
    {"name": "Shield", "cost": 113, "armor": 7, "lasts": 6},
    {"name": "Poison", "cost": 173, "dmg": 3, "lasts": 6},
    {"name": "Recharge", "cost": 229, "mana": 101, "lasts": 5},
]
print(f"Part One: {search_least_mana(boss, player, spells)}")
print(f"Part Two: {search_least_mana(boss, player, spells, True)}")
