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



