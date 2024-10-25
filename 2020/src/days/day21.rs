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
            Some(allergens_str) => allergens_str[..allergens_str.len() - 1]
                .split(", ")
                .collect(),
        };
        food.push(Food {
            ingredients,
            allergens,
        });
    }
    food
}

fn ingredients_without_allergens<'a>(food: &'a [Food]) -> usize {
    let mut allergens_map: HashMap<&str, Vec<&'a str>> = HashMap::new();
    for f in food.iter() {
        for allergene in &f.allergens {
            let map = allergens_map
                .entry(allergene)
                .or_insert(f.ingredients.clone());
            if !map.is_empty() {
                let mut new_map = vec![];
                for ingredient in map.iter() {
                    if f.ingredients.contains(ingredient) {
                        new_map.push(*ingredient);
                    }
                }
                *map = new_map;
            }
        }
    }

    let mut unique = vec![];
    for (_, map) in allergens_map.iter() {
        for a in map.iter() {
            if !unique.contains(a) {
                unique.push(*a);
            }
        }
    }

    let mut count = 0;
    for f in food.iter() {
        for i in &f.ingredients {
            if !unique.contains(i) {
                count += 1;
            }
        }
    }

    count
}

pub fn run() {
    let input_raw = crate::load_input(module_path!());
    let food = parse_food(&input_raw);
    println!("Day 21: Allergen Assessment");
    println!("Part One: {}", ingredients_without_allergens(&food));
    println!("Part Two: {}", "TODO");
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
        assert_eq!(ingredients_without_allergens(&food), 5);
    }

    #[test]
    fn test_part_two() {}
}
