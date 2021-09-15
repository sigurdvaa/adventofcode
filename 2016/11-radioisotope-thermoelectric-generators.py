input_raw = """The first floor contains a promethium generator and a promethium-compatible microchip.
The second floor contains a cobalt generator, a curium generator, a ruthenium generator, and a plutonium generator.
The third floor contains a cobalt-compatible microchip, a curium-compatible microchip, a ruthenium-compatible microchip, and a plutonium-compatible microchip.
The fourth floor contains nothing relevant."""

floors = [
    ["PrG", "PrM"],
    ["CoG", "CuG", "RuG", "PlG"],
    ["CoM", "CuM", "RuM", "PlM"],
    [],
]

elevator = {
    "floor": 0,
    "items": [],
}

# elevator stops at each floor to recharge
# XxM can't be with non-matching XxG without a matching XxG on the same floor
# max 2 items in elevator
# min 1 item in elevator
# how many moves to bring everything to 4th floor


