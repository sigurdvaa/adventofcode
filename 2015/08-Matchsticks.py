with open("08_input.txt", "r") as f:
    input_raw = [x.strip() for x in f.readlines()]


def mem_size(string: str):
    size = -2
    i = 0
    while i < len(string):
        size += 1
        if string[i] == "\\":
            if string[i + 1] == "x":
                i += 4
            else:
                i += 2
        else:
            i += 1

    return size


def encoded_size(string: str):
    size = 2
    i = 0
    while i < len(string):
        if string[i] == "\\" or string[i] == '"':
            size += 1
        size += 1
        i += 1

    return size


def size_diff_mem(strings: list):
    sizediff = 0

    for string in strings:
        sizediff += len(string) - mem_size(string)

    return sizediff


def size_diff_encoded(strings: list):
    sizediff = 0

    for string in strings:
        sizediff += encoded_size(string) - len(string)

    return sizediff


print(f"Part One: {size_diff_mem(input_raw)}")
print(f"Part Two: {size_diff_encoded(input_raw)}")
