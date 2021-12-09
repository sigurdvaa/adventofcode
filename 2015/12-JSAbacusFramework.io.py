import json
from re import findall


with open("12-input.txt", "r") as f:
    input_raw = f.readline()


def findsum(string):
    numbers = findall("-?\d+", string)
    totalsum = 0
    for num in numbers:
        totalsum += int(num)
    return totalsum


def findsum_notred(obj):
    totalsum = 0
    obj_type = type(obj)

    if obj_type == int:
        return obj

    elif obj_type == dict:
        for item in obj.values():
            if type(item) == str:
                if item == "red":
                    return 0
        for item in obj.values():
            totalsum += findsum_notred(item)

    elif obj_type == list:
        for item in obj:
            totalsum += findsum_notred(item)

    return totalsum


print(f"Part One: {findsum(input_raw)}")
print(f"Part Two: {findsum_notred(json.loads(input_raw))}")
