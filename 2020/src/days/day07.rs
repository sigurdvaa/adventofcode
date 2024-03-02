use std::fs;

fn parse_rules(rules: &str) -> _ {}

fn num_bags_can_contain_bag(rules: &str, bag: &str) -> usize {
    0
}

pub fn run() {
    println!("Day 7: Handy Haversacks");
    let file_path = "inputs/day07.txt";
    let input_raw = fs::read_to_string(file_path)
        .unwrap_or_else(|err| panic!("Error reading file '{file_path}': {err}"));

    println!("Part One: {}", "TODO");
    println!("Part Two: {}", "TODO");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        const INPUT_TEST: &str = concat!(
            "light red bags contain 1 bright white bag, 2 muted yellow bags.\n",
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.\n",
            "bright white bags contain 1 shiny gold bag.\n",
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\n",
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\n",
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.\n",
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\n",
            "faded blue bags contain no other bags.\n",
            "dotted black bags contain no other bags.",
        );
        let mybag = "shiny gold";
        assert_eq!(num_bags_can_contain_bag(INPUT_TEST, mybag), 4);
    }

    #[test]
    fn test_part_two() {}
}
