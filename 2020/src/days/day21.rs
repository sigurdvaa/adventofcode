use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Food<'a> {
    ingredients: Vec<&'a str>,
    allergens: Vec<&'a str>,
}

fn parse_food(list: &str) -> Vec<Food> {
    let mut food = vec![];
    for line in list.lines() {
        let mut split = line.split(" (contains ");
        let ingredients = split.next().unwrap().split(' ').collect();
        let allergens = match split.next() {
            None => vec![],
            Some(allergens_str) => allergens_str[..allergens_str.len() - 1].split(", ").collect(),
        };
        food.push(Food { ingredients, allergens });
    }
    food
}

fn create_allergens_map<'a>(food: &'a [Food]) -> HashMap<&'a str, &'a str> {
    let mut allergens_to_ingredients: HashMap<&str, Vec<&str>> = HashMap::new();
    for f in food.iter() {
        for allergene in &f.allergens {
            allergens_to_ingredients
                .entry(allergene)
                .and_modify(|map| {
                    let mut new_map = vec![];
                    for ingredient in map.iter() {
                        if f.ingredients.contains(ingredient) {
                            new_map.push(*ingredient);
                        }
                    }
                    *map = new_map;
                })
                .or_insert(f.ingredients.clone());
        }
    }

    let mut allergens_map = HashMap::new();
    loop {
        let mut eliminate: Option<(&str, &str)> = None;
        for (a, i) in allergens_to_ingredients.iter() {
            if i.len() == 1 {
                eliminate = Some((a, i[0]));
                allergens_map.insert(*a, i[0]);
            }
        }
        match eliminate {
            None => break,
            Some((allergene, ingredient)) => {
                allergens_to_ingredients.remove(allergene);
                for (_, i) in allergens_to_ingredients.iter_mut() {
                    if let Some(idx) = i.iter().position(|&v| v == ingredient) {
                        i.swap_remove(idx);
                    }
                }
            }
        }
    }

    allergens_map
}

fn ingredients_without_allergens<'a>(food: &'a [Food], allergens_map: &HashMap<&'a str, &'a str>) -> usize {
    let mut count = 0;
    for f in food.iter() {
        count += f
            .ingredients
            .iter()
            .filter(|&i| !allergens_map.values().any(|allergene_i| i == allergene_i))
            .count();
    }
    count
}

fn canonical_dangerous_ingredient_list<'a>(allergens_map: &HashMap<&'a str, &'a str>) -> String {
    let mut dangerous = allergens_map.iter().collect::<Vec<_>>();
    dangerous.sort_by_key(|(a, _)| *a);
    dangerous
        .iter()
        .map(|(_, i)| i.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

pub fn run() {
    let input_raw = crate::load_input(module_path!());
    let food = parse_food(&input_raw);
    let allergens_map = create_allergens_map(&food);
    println!("Day 21: Allergen Assessment");
    println!("Part One: {}", ingredients_without_allergens(&food, &allergens_map));
    println!("Part Two: {}", canonical_dangerous_ingredient_list(&allergens_map));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = concat!(
        "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)\n",
        "trh fvjkl sbzzf mxmxvkd (contains dairy)\n",
        "sqjhc fvjkl (contains soy)\n",
        "sqjhc mxmxvkd sbzzf (contains fish)\n",
    );

    #[test]
    fn test_part_one() {
        let food = parse_food(TEST_INPUT);
        let allergens_map = create_allergens_map(&food);
        assert_eq!(ingredients_without_allergens(&food, &allergens_map), 5);
    }

    #[test]
    fn test_part_two() {
        let food = parse_food(TEST_INPUT);
        let allergens_map = create_allergens_map(&food);
        assert_eq!(
            canonical_dangerous_ingredient_list(&allergens_map),
            "mxmxvkd,sqjhc,fvjkl".to_string()
        );
    }
}
