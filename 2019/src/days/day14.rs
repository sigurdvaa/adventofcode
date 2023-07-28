use std::collections::HashMap;
use std::fs;

struct Reaction {
    product: String,
    amount: i32,
    reactants: Vec<(i32, String)>,
}

fn parse_reactions(input: &str) -> HashMap<String, Reaction> {
    let mut reactions = HashMap::new();
    for line in input.lines() {
        let mut parts = line.split(" => ");
        let reactants = parts
            .next()
            .unwrap()
            .split(", ")
            .map(|x| {
                let mut s = x.split(" ");
                (
                    s.next().unwrap().parse().unwrap(),
                    s.next().unwrap().to_string(),
                )
            })
            .collect::<Vec<(i32, String)>>();
        let mut produces = parts.next().unwrap().split(" ");
        let amount = produces.next().unwrap().parse().unwrap();
        let product = produces.next().unwrap().to_string();
        reactions.insert(
            product.clone(),
            Reaction {
                product,
                amount,
                reactants,
            },
        );
    }
    reactions
}

fn ores_required(reactions: &HashMap<String, Reaction>) -> i32 {
    0
}

pub fn run() {
    println!("Day 14: Space Stoichiometry");
    let file_path = "inputs/day14.txt";
    let input_raw =
        fs::read_to_string(file_path).expect(format!("Error reading file '{file_path}'").as_str());
    let reactions = parse_reactions(&input_raw);
    let ores = ores_required(&reactions);
    println!("Part One: {}", ores);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input_raw = "10 ORE => 10 A\n1 ORE => 1 B\n7 A, 1 B => 1 C\n\
            7 A, 1 C => 1 D\n7 A, 1 D => 1 E\n7 A, 1 E => 1 FUEL";
        let reactions = parse_reactions(&input_raw);
        assert_eq!(ores_required(&reactions), 31);
    }

    #[test]
    fn test_part_two() {}
}
