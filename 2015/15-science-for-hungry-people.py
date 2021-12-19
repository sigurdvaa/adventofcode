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


def all_recipes(ingredients: dict, amount: int):
    recipes = [{}]
    for item in ingredients:
        next_recipes = []
        for recipe in recipes:
            for i in range(amount + 1 - sum(recipe.values())):
                next_recipe = recipe.copy()
                next_recipe[item] = i
                next_recipes.append(next_recipe)

        recipes = next_recipes

    valid_recipes = []
    for recipe in recipes:
        if sum(recipe.values()) == 100:
            valid_recipes.append(recipe)

    return valid_recipes


def recipe_score(ingredients: dict, recipe: dict, calories: int = -1):
    score = {}
    for item in recipe:
        for prop in ingredients[item]:
            if prop in score:
                score[prop] += recipe[item] * ingredients[item][prop]
            else:
                score[prop] = recipe[item] * ingredients[item][prop]

    if calories != -1:
        if score["calories"] != calories:
            return 0

    total_score = 1
    for prop in score:
        if prop != "calories":
            if score[prop] < 1:
                return 0
            else:
                total_score *= score[prop]

    return total_score


def find_highest_score(ingredients: dict, recipes: list, calories: int = -1):
    highest_score = 0
    for recipe in recipes:
        score = recipe_score(ingredients, recipe, calories)
        if highest_score < score:
            highest_score = score

    return highest_score


ingredients = parse_ingredients(input_raw)
recipes = all_recipes(ingredients, 100)

print(f"Part One: {find_highest_score(ingredients, recipes)}")
print(f"Part Two: {find_highest_score(ingredients, recipes, 500)}")
