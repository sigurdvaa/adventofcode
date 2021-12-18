input_raw = """Sugar: capacity 3, durability 0, flavor 0, texture -3, calories 2
Sprinkles: capacity -3, durability 3, flavor 0, texture 0, calories 9
Candy: capacity -1, durability 0, flavor 4, texture 0, calories 1
Chocolate: capacity 0, durability 0, flavor -2, texture 2, calories 8"""

input_test = """Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3"""


def parse_ingredients(string: str):
    ingredients = {}
    for line in string.splitlines():
        split = line.split()
        ingredients[split[0][:-1]] = {
            "capacity": int(split[2][:-1]),
            "durability": int(split[4][:-1]),
            "flavor": int(split[6][:-1]),
            "texture": int(split[8][:-1]),
            "calories": int(split[10]),
        }

    return ingredients


print(parse_ingredients(input_test))
