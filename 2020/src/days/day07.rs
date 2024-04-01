use std::collections::HashMap;

type BagEdges = HashMap<String, Vec<(usize, String)>>;
const MYBAG: &str = "shiny gold";

fn num_bags_can_contain_bag(bags: &BagEdges, bag: &str) -> usize {
    let mut outer = vec![bag];
    let mut i = 0;
    while i < outer.len() {
        for (name, edges) in bags.iter() {
            for (_num, edge) in edges.iter() {
                if edge == outer[i] && !outer.contains(&name.as_str()) {
                    outer.push(name);
                    break;
                }
            }
        }
        i += 1;
    }
    outer.len() - 1
}

fn parse_rules_to_edgegraph(rules: &str) -> BagEdges {
    let mut bags = HashMap::new();
    for line in rules.lines() {
        let mut split = line.split(" bags contain ");
        let name = split.next().unwrap();
        let contain = split.next().expect("invalid rule").split(", ");

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

fn recurse_bags(bags: &BagEdges, bag: &str) -> usize {
    let mut num = 1;
    if let Some(edges) = bags.get(bag) {
        for edge in edges.iter() {
            num += edge.0 * recurse_bags(bags, &edge.1);
        }
    }
    num
}

fn num_bags_in_bag(bags: &BagEdges, bag: &str) -> usize {
    recurse_bags(bags, bag) - 1
}

pub fn run() {
    println!("Day 7: Handy Haversacks");
    let input_raw = crate::load_input(module_path!());
    let bags = parse_rules_to_edgegraph(&input_raw);
    println!("Part One: {}", num_bags_can_contain_bag(&bags, MYBAG));
    println!("Part Two: {}", num_bags_in_bag(&bags, MYBAG));
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
        let bags = parse_rules_to_edgegraph(INPUT_TEST);
        assert_eq!(num_bags_can_contain_bag(&bags, MYBAG), 4);
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
        let bags = parse_rules_to_edgegraph(INPUT_TEST);
        assert_eq!(num_bags_in_bag(&bags, MYBAG), 126);
    }
}
