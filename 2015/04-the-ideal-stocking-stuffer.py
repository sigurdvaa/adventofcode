input_raw = "iwrupvqb"


def find_startwith_zero(secret: str, length: int = 5):
    from hashlib import md5
    startwithzero = "0" * length
    i = 0
    hashvalue = md5((secret + str(i)).encode()).hexdigest()
    
    while not hashvalue.startswith(startwithzero):
        i += 1
        hashvalue = md5((secret + str(i)).encode()).hexdigest()

    return i


print(f"Part One: {find_startwith_zero(input_raw)}")
print(f"Part Two: {find_startwith_zero(input_raw, 6)}")
