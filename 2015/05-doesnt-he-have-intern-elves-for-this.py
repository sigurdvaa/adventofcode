with open("05-input.txt", "r") as f:
    input_raw = [x.strip() for x in f.readlines()]


def is_nice(string: str):
    from re import findall

    bad_strings = ["ab", "cd", "pq", "xy"]

    found_vowels = findall("[aeiou]", string)
    if len(found_vowels) < 3:
        return False

    twice_in_row = findall("(.)\\1+", string)
    if len(twice_in_row) < 1:
        return False

    for bad in bad_strings:
        if bad in string:
            return False

    return True


def find_repeated_pairs(string: str):
    repeated_pairs = list()
    i = 0
    while i < len(string) - 3:
        match = False
        pair = string[i] + string[i + 1]
        for s in range(i + 2, len(string) - 1):
            if pair == string[s] + string[s + 1]:
                repeated_pairs.append(pair)
                match = True
                break

        if match:
            i += 2
        else:
            i += 1

    return repeated_pairs


def find_repeated_letters(string: str):
    repeated_letters = list()
    i = 0
    while i < len(string) - 2:
        if string[i] == string[i + 2]:
            repeated_letters.append(string[i : i + 3])
            i += 3
        else:
            i += 1

    return repeated_letters


def is_nice2(string: str):
    pairs = find_repeated_pairs(string)
    if len(pairs) == 0:
        return False

    letters = find_repeated_letters(string)
    if len(letters) == 0:
        return False

    return True


def count_nice(strings: list, part: int = 1):
    count = 0
    for string in strings:
        if part == 1:
            if is_nice(string):
                count += 1
        elif part == 2:
            if is_nice2(string):
                count += 1
    return count


print(f"Part One: {count_nice(input_raw)}")
print(f"Part Two: {count_nice(input_raw, 2)}")
