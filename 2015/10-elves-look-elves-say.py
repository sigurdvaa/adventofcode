input_raw = "1113222113"


def look_and_say(string: str, repeat: int = 1):

    for _ in range(repeat):
        new_string = list()
        i = 0
        prev_char = ""
        prev_count = 0
        for char in string:
            if prev_count > 0:
                if prev_char == char:
                    prev_count += 1
                else:
                    new_string.append(f"{prev_count}{prev_char}")
                    prev_char = char
                    prev_count = 1
            else:
                prev_char = char
                prev_count = 1

        new_string.append(f"{prev_count}{prev_char}")
        string = "".join(new_string)

    return string


print(f"Part One: {len(look_and_say(input_raw, 40))}")
print(f"Part Two: {len(look_and_say(input_raw, 50))}")
