use std::collections::HashMap;
use std::fs;

const MYBAG: &str = "shiny gold";

fn num_bags_can_contain_bag(rules: &str, bag: &str) -> usize {
    let mut outer = vec![bag];
    let lines = rules.lines().collect::<Vec<_>>();
    let mut i = 0;
    while i < outer.len() {
        for line in lines.iter() {
            let mut split = line.split(" bags contain ");
            let name = split.next().unwrap();
            let contain = split.next().unwrap();
            if contain.contains(outer[i]) && !outer.contains(&name) {
                outer.push(name);
            }
        }
        i += 1;
    }
    outer.len() - 1
}

fn parse_rules(rules: &str) -> HashMap<String, Vec<(usize, String)>> {
    let mut bags = HashMap::new();
    for line in rules.lines() {
        let mut split = line.split(" bags contain ");
        let name = split.next().unwrap();
        let contain = split.next().unwrap().split(", ");

        let edges = bags.entry(name.to_string()).or_insert(vec![]);
        for bag2 in contain {
            let mut split = bag2.split(' ');
            let num = split.next().unwrap().parse().unwrap_or(0);
            let name2 = format!("{} {}", split.next().unwrap(), split.next().unwrap());
            if num > 0 {
                edges.push((num, name2));
            }
        }
    }
    bags
}

fn recurse_bags(bags: &HashMap<String, Vec<(usize, String)>>, bag: &str) -> usize {
    let mut num = 1;
    if let Some(edges) = bags.get(bag) {
        for edge in edges.iter() {
            num += edge.0 * recurse_bags(bags, &edge.1);
        }
    }
    num
}

fn num_bags_in_bag(rules: &str, bag: &str) -> usize {
    let bags = parse_rules(rules);
    recurse_bags(&bags, bag) - 1
}

pub fn run() {
    println!("Day 7: Handy Haversacks");
    let file_path = "inputs/day07.txt";
    let input_raw = fs::read_to_string(file_path)
        .unwrap_or_else(|err| panic!("Error reading file '{file_path}': {err}"));

    println!("Part One: {}", num_bags_can_contain_bag(&input_raw, MYBAG));
    println!("Part Two: {}", num_bags_in_bag(&input_raw, MYBAG));
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
        assert_eq!(num_bags_can_contain_bag(INPUT_TEST, MYBAG), 4);
    }

    #[test]
    fn test_part_two() {
        const INPUT_TEST: &str = concat!(
            "shiny gold bags contain 2 dark red bags.\n",
            "dark red bags contain 2 dark orange bags.\n",
            "dark orange bags contain 2 dark yellow bags.\n",
            "dark yellow bags contain 2 dark green bags.\n",
            "dark green bags contain 2 dark blue bags.\n",
            "dark blue bags contain 2 dark violet bags.\n",
            "dark violet bags contain no other bags.",
        );
        assert_eq!(num_bags_in_bag(INPUT_TEST, MYBAG), 126);
    }
}
