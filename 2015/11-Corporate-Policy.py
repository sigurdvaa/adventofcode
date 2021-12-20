from string import ascii_lowercase

input_raw = "hxbxwxba"


def next_pw(pw: list):
    for i in range(len(pw) - 1, -1, -1):
        if pw[i] == "z":
            pw[i] = "a"
        else:
            pw[i] = ascii_lowercase[ascii_lowercase.index(pw[i]) + 1]
            return


def valid_pw(pw: list):
    invalid_char = ["i", "o", "l"]
    for c in invalid_char:
        if c in pw:
            return False

    straight = False
    for i in range(len(pw) - 2):
        if pw[i] != "y" and pw[i] != "z":
            if pw[i + 1] == ascii_lowercase[ascii_lowercase.index(pw[i]) + 1]:
                if pw[i + 2] == ascii_lowercase[ascii_lowercase.index(pw[i]) + 2]:
                    straight = True
                    break
    if not straight:
        return False

    pairs = list()
    i = 0
    while i < len(pw) - 1:
        if pw[i] == pw[i + 1]:
            if pw[i] + pw[i + 1] not in pairs:
                pairs.append(pw[i] + pw[i + 1])
            i += 2
        else:
            i += 1
    if len(pairs) < 2:
        return False

    return True


def increment_password(password: str):
    pw = list(password)
    while True:
        next_pw(pw)
        if valid_pw(pw):
            return "".join(pw)


new_pw = increment_password(input_raw)
print(f"Part One: {new_pw}")
new_pw = increment_password(new_pw)
print(f"Part Two: {new_pw}")
