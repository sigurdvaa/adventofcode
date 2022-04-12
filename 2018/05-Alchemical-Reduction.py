with open("05_input.txt", "r") as f:
    input_raw = f.readline().strip()


class Unit:
    def __init__(self, prev, data):
        self.prev = prev
        self.data = data
        self.next = None

    def __repr__(self):
        return self.data


class Polymer:
    def __init__(self, units):
        self._len = 0
        self.head = None
        for u in units:
            if self.head == None:
                self.head = Unit(None, u)
                unit = self.head
            else:
                unit.next = Unit(unit, u)
                unit = unit.next
            self._len += 1

    def __repr__(self):
        unit = self.head
        units = []
        while unit is not None:
            units.append(unit.data)
            unit = unit.next
        return " <-> ".join(units)

    def __len__(self):
        return self._len

    def react(self, unit):
        self._len -= 2
        if unit.prev == None:
            self.head = unit.next.next
            self.head.prev = None
            return self.head
        elif unit.next.next == None:
            unit.prev.next = None
            return unit.prev
        else:
            unit.prev.next = unit.next.next
            unit.next.next.prev = unit.prev
            return unit.prev


def polymer_len_after_react(polymer: str) -> int:
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


print(f"Part One: { polymer_len_after_react(input_raw) }")
print(f"Part Two: { polymer_len_after_best_react(input_raw) }")
