with open("04_input.txt", "r") as f:
    input_raw = [x.strip() for x in f.readlines()]


def parse_guard_records(strings: str) -> dict:
    guards = {}
    for line in sorted(strings):
        split = line.split()
        if split[2] == "Guard":
            guard_id = int(split[3][1:])
            if not guard_id in guards:
                guards[guard_id] = [0] * 60
        elif split[2] == "falls":
            sleep_start = int(split[1][3:5])
        elif split[2] == "wakes":
            sleep_end = int(split[1][3:5])
            for m in range(sleep_start, sleep_end):
                guards[guard_id][m] += 1

    return guards


def guard_strategy1(guards: dict) -> int:
    max_sleep = 0
    for guard_id in guards:
        sleep = sum(guards[guard_id])
        if sleep > max_sleep:
            max_sleep = sleep
            max_guard_id = guard_id

    max_per_minute = 0
    for m in range(len(guards[max_guard_id])):
        if guards[max_guard_id][m] > max_per_minute:
            max_per_minute = guards[max_guard_id][m]
            top_minute = m

    return max_guard_id * top_minute


def guard_strategy2(guards: dict) -> int:
    max_per_minute = 0
    for guard_id in guards:
        for m in range(len(guards[guard_id])):
            if guards[guard_id][m] > max_per_minute:
                max_per_minute = guards[guard_id][m]
                max_guard_id = guard_id
                max_minute = m

    return max_guard_id * max_minute


guards = parse_guard_records(input_raw)
print(f"Part One: { guard_strategy1(guards) }")
print(f"Part Two: { guard_strategy2(guards) }")
