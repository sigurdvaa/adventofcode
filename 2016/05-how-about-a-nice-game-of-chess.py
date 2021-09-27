import os, string, hashlib

input_raw = "wtnhxymk"
password = []
password_len = 0
i = 0
while True:
    md5 = hashlib.md5(f"{input_raw}{i}".encode()).hexdigest()
    if md5[:5] == "00000":
        password += [md5[5]]
        password_len += 1
        if password_len == 8:
            print("Part One: " + "".join(password))
            break
    i += 1

password = ["" for x in range(8)]
password_len = 0
string_ints = [str(x) for x in range(8)]
i = 0
while True:
    md5 = hashlib.md5(f"{input_raw}{i}".encode()).hexdigest()
    if md5[:5] == "00000" and md5[5] in string_ints:
        if password[int(md5[5])] == "":
            password[int(md5[5])] = md5[6]
            password_len += 1
            if password_len == 8:
                print("Part Two: " + "".join(password))
                break
    i += 1
