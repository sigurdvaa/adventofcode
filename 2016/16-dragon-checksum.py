input_raw = "01111001100111011"
disk_size_p1 = 272
disk_size_p2 = 35651584


def checksum(data: list):
    consider = list(data)
    while (len(consider) % 2) == 0:
        new_consider = list()
        for i1, i2 in zip(consider[::2], consider[1::2]):
            if i1 == i2:
                new_consider.append(1)
            else:
                new_consider.append(0)
        consider = new_consider
    return consider


def dragon_curve(data: list):
    data_len = len(data)
    for bit in reversed(data):
        if bit:
            data.append(0)
        else:
            data.append(1)
    data.insert(data_len, 0)
    return data


def random_data(data: str, size: int):
    data = list(map(int, data))
    while len(data) < size:
        data = dragon_curve(data)
    return "".join(map(str, checksum(data[:size])))


print(f"Part One: {random_data(input_raw, disk_size_p1)}")
print(f"Part Two: {random_data(input_raw, disk_size_p2)}")
