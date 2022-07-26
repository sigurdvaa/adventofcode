input_raw = "110201"


def create_recipes(target: str, backwards: bool = False) -> int:
    recipes: list[int] = [3, 7]
    elf1: int = 0
    elf2: int = 1

    if not backwards:
        target: int = int(target)

    while True:
        new_recipe: int = recipes[elf1] + recipes[elf2]
        if new_recipe < 10:
            recipes.append(new_recipe)
        else:
            recipes.append(1)
            recipes.append(new_recipe - 10)

        recipes_len: int = len(recipes)
        elf1 = (elf1 + recipes[elf1] + 1) % recipes_len
        elf2 = (elf2 + recipes[elf2] + 1) % recipes_len

        if backwards:
            if "".join(map(str, recipes[-len(target):])) == target:
                return len(recipes) - len(target)
        else:
            if len(recipes) >= target + 10:
                return "".join(map(str, recipes[-10:]))


print(f"Part One: {create_recipes(input_raw)}")
print(f"Part Two: 2018: {create_recipes('59414', backwards=True)}")
#print(f"Part Two: {create_recipes(input_raw, backwards=True)}")
