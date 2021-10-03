string = "14	0	15	12	11	11	3	5	1	6	8	4	9	1	8	4"
string2 = "0\t2\t7\t0"

banks = [int(x) for x in string.split("\t")]
seen = [banks[:]]
size = len(banks)
seen_before = False
cycles = 0
while not seen_before:
    cycles += 1

    # Find biggest
    biggest_value = 0
    biggest_index = 0
    for s in range(size):
        value = banks[s]
        if value > biggest_value:
            biggest_index = s
            biggest_value = value

    # Set biggest value to 0 and redistribute
    banks[biggest_index] = 0
    while biggest_value > 0:
        biggest_index += 1
        if biggest_index == size:
            biggest_index = 0
        banks[biggest_index] += 1
        biggest_value -= 1

    # State seen before?
    if not banks in seen:
        seen += [banks[:]]
    else:
        loop_size = len(seen) - seen.index(banks)
        seen_before = True

print("Part 1\n" + str(cycles))
print("Part 2\n" + str(loop_size))
