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


def next_spells(queue: list, state: list, spells: dict) -> None:
    for spell in spells:
        if spell not in state[3]:
            if spells[spell]["cost"] <= state[2]:
                queue.append(
                    [
                        state[0],
                        state[1],
                        state[2] - spells[spell]["cost"],
                        state[3].copy(),
                        spell,
                        state[5] + spells[spell]["cost"],
                    ]
                )


def boss_turn(state: list, damage: int, spells: dict) -> None:
    player_armor = 0
    for active_spell in state[3]:
        if "armor" in spells[active_spell]:
            player_armor += spells[active_spell]["armor"]
    boss_dmg = damage - player_armor
    boss_dmg = boss_dmg if boss_dmg > 0 else 1
    state[1] -= boss_dmg


def player_turn(state: list, spells: dict) -> None:
    spell = spells[state[4]]
    if "lasts" in spell:
        state[3][state[4]] = spell["lasts"]
    else:
        if "dmg" in spell:
            state[0] -= spell["dmg"]
        if "heal" in spell:
            state[1] += spell["heal"]


def update_active_spells(state: list, spells: dict) -> None:
    still_active = {}
    for active_spell in state[3]:
        if "dmg" in spells[active_spell]:
            state[0] -= spells[active_spell]["dmg"]
        if "mana" in spells[active_spell]:
            state[2] += spells[active_spell]["mana"]
        if state[3][active_spell] > 1:
            still_active[active_spell] = state[3][active_spell] - 1
    state[3] = still_active


def search_least_mana(
    boss: tuple, player: tuple, spells: dict, wounded: bool = False
) -> int:
    from collections import deque

    queue = deque()
    init_state = [boss[0], player[0], player[1], {}, "", 0]
    next_spells(queue, init_state, spells)
    least_mana = -1
    while len(queue):
        state = queue.popleft()

        if wounded:
            state[1] -= 1
            if state[1] < 1:
                continue

        player_turn(state, spells)
        update_active_spells(state, spells)
        if state[0] < 1:
            if least_mana == -1 or least_mana > state[5]:
                least_mana = state[5]
            continue

        boss_turn(state, boss[1], spells)
        if state[1] < 1:
            continue

        update_active_spells(state, spells)
        if state[0] < 1:
            if least_mana == -1 or least_mana > state[5]:
                least_mana = state[5]
            continue

        if least_mana != -1:
            if state[5] < least_mana:
                next_spells(queue, state, spells)
        else:
            next_spells(queue, state, spells)

    return least_mana


boss = (71, 10)
player = (50, 500)
spells = {
    "Magic Missile": {"cost": 53, "dmg": 4},
    "Drain": {"cost": 73, "dmg": 2, "heal": 2},
    "Shield": {"cost": 113, "armor": 7, "lasts": 6},
    "Poison": {"cost": 173, "dmg": 3, "lasts": 6},
    "Recharge": {"cost": 229, "mana": 101, "lasts": 5},
}
print(f"Part One: {search_least_mana(boss, player, spells)}")
print(f"Part Two: {search_least_mana(boss, player, spells, True)}")
