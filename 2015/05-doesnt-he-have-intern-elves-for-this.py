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


def count_nice(strings: list):
    count = 0
    for string in strings:
        if is_nice(string):
            count += 1
    return count


print(f"Part One: {count_nice(input_raw)}")
