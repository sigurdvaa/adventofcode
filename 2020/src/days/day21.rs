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

    #[test]
    fn test_part_one() {}

    #[test]
    fn test_part_two() {}
}
