input_raw = "110201"


def create_recipes(target: str, backwards: bool = False) -> str:
    recipes: str = "37"
    elf1: int = 0
    elf2: int = 1
    # need to increase len by 1, as we're adding 1 or 2 recipes each loop
    target_len: int = len(target) + 1

    while True:
        # using ord() - 48 is faster then int()
        # only works when converting single digit
        recipes += str(ord(recipes[elf1]) + ord(recipes[elf2]) - 96)

        recipes_len: int = len(recipes)
        elf1 = (elf1 + ord(recipes[elf1]) - 48 + 1) % recipes_len
        elf2 = (elf2 + ord(recipes[elf2]) - 48 + 1) % recipes_len

        if backwards:
            if target in recipes[-target_len:]:
                if target == recipes[-target_len + 1 :]:
                    return str(len(recipes) - target_len + 1)
                else:
                    return str(len(recipes) - target_len)
        else:
            if len(recipes) >= int(target) + 10:
                return str(recipes[-10:])


print(f"Part One: {create_recipes(input_raw)}")
print(f"Part Two: {create_recipes(input_raw, backwards=True)}")
