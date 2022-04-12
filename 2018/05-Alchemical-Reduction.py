with open("05_input.txt", "r") as f:
    input_raw = f.readline().strip()


def polymer_len_after_react(polymer: str) -> int:
    polymer = list(polymer)

    reacting = True
    while reacting:
        reacting = False
        i = 0
        polymer_len = len(polymer)
        while i < polymer_len - 1:
            if (
                polymer[i].lower() == polymer[i + 1].lower()
                and polymer[i] != polymer[i + 1]
            ):
                reacting = True
                del polymer[i]
                del polymer[i]
                polymer_len -= 2
                i -= 1
            i += 1

    return len(polymer)


class Unit:
    def __init__(self, prev, data):
        self.prev = prev
        self.next = None
        self.data = data

    def __repr__(self):
        return self.data


class Polymer:
    def __init__(self, units):
        self.head = None
        for u in units:
            if self.head == None:
                self.head = Unit(None, u)
                unit = self.head
            else:
                unit.next = Unit(unit, u)
                unit = unit.next

    def __repr__(self):
        unit = self.head
        units = []
        while unit is not None:
            units.append(unit.data)
            unit = unit.next
        return " <-> ".join(units)

    def __len__(self):
        if self.head == None:
            return 0
        else:
            i = 0
            unit = self.head
            while unit is not None:
                i += 1
                unit = unit.next
            return i

    def react(self, unit):
        next_unit = unit.next.next
        if unit.prev == None:
            self.head = next_unit
            next_unit.prev = None
            return next_unit
        else:
            next_unit.prev = unit.prev
            unit.prev.next = next_unit
            return unit.prev


def polymer_len_after_react2(polymer: str) -> int:
    polymer = Polymer(polymer)

    unit = polymer.head
    while unit.next is not None:
        if unit.data.lower() == unit.next.data.lower() and unit.data != unit.next.data:
            reacting = True
            unit = polymer.react(unit)
        else:
            unit = unit.next

    return len(polymer)


def polymer_len_after_best_react(polymer: str) -> int:
    units = set()
    for u in polymer:
        units.add(u.lower())

    min_polymer_len = len(polymer)
    for u in units:
        new_polymer = polymer.translate({ord(u): None, ord(u.upper()): None})
        polymer_len = polymer_len_after_react(new_polymer)
        if polymer_len < min_polymer_len:
            min_polymer_len = polymer_len

    return min_polymer_len


print(f"Part One: { polymer_len_after_react2(input_raw) }")
print(f"Part Two: { polymer_len_after_best_react(input_raw) }")
