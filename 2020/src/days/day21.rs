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

fn ingredients_without_allergens(food: &[Food]) -> usize {
    0
}

pub fn run() {
    let input_raw = crate::load_input(module_path!());
    let food = parse_food(&input_raw);
    println!("Day 21: Allergen Assessment");
    println!("Part One: {}", "TODO");
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
