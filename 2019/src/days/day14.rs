use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Reaction {
    amount: i64,
    reactants: Vec<(i64, String)>,
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
            .collect::<Vec<(i64, String)>>();
        let mut produces = parts.next().unwrap().split(" ");
        let amount = produces.next().unwrap().parse().unwrap();
        let product = produces.next().unwrap();
        reactions.insert(product.to_string(), Reaction { amount, reactants });
    }
    reactions
}

fn ores_required(
    reactions: &HashMap<String, Reaction>,
    available: &mut HashMap<String, i64>,
    product: &str,
    amount: i64,
) -> i64 {
    let mut ores = 0;
    let mut has = *available.get(product).unwrap_or(&0);

    if has < amount {
        let product = reactions.get(product).unwrap();
        let diff = (amount - has - 1) / product.amount + 1;
        for (amount, reactant) in &product.reactants {
            ores += match reactant {
                n if n == "ORE" => *amount * diff,
                _ => ores_required(reactions, available, reactant, *amount * diff),
            }
        }
        has += product.amount * diff;
    }

    available.insert(product.to_string(), has - amount);
    ores
}

fn ores_required_for_fuel(reactions: &HashMap<String, Reaction>) -> i64 {
    ores_required(&reactions, &mut HashMap::new(), "FUEL", 1)
}

fn max_fuel(reactions: &HashMap<String, Reaction>) -> i64 {
    let available_ore = 1000000000000;
    let ores_per_fuel = ores_required_for_fuel(reactions);
    let mut max_fuel = available_ore / ores_per_fuel;

    let mut ores = ores_required(&reactions, &mut HashMap::new(), "FUEL", max_fuel);
    while ores <= available_ore {
        max_fuel += (available_ore - ores - 1) / ores_per_fuel + 1;
        ores = ores_required(&reactions, &mut HashMap::new(), "FUEL", max_fuel);
    }
    max_fuel - 1
}

pub fn run() {
    println!("Day 14: Space Stoichiometry");
    let file_path = "inputs/day14.txt";
    let input_raw =
        fs::read_to_string(file_path).expect(format!("Error reading file '{file_path}'").as_str());

    let reactions = parse_reactions(&input_raw);
    println!("Part One: {}", ores_required_for_fuel(&reactions));
    println!("Part Two: {}", max_fuel(&reactions));
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT_TEST1: &str = "157 ORE => 5 NZVS\n165 ORE => 6 DCFZ\n\
        44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL\n\
        12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ\n179 ORE => 7 PSHF\n\
        177 ORE => 5 HKGWZ\n7 DCFZ, 7 PSHF => 2 XJWVT\n165 ORE => 2 GPVTF\n\
        3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";

    const INPUT_TEST2: &str = "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG\n\
        17 NVRVD, 3 JNWZP => 8 VPVL\n\
        53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL\n\
        22 VJHF, 37 MNCFX => 5 FWMGM\n139 ORE => 4 NVRVD\n144 ORE => 7 JNWZP\n\
        5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC\n\
        5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV\n145 ORE => 6 MNCFX\n\
        1 NVRVD => 8 CXFTF\n1 VJHF, 6 MNCFX => 4 RFSQX\n176 ORE => 6 VJHF";

    const INPUT_TEST3: &str = "171 ORE => 8 CNZTR\n\
        7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL\n\
        114 ORE => 4 BHXH\n14 VRPVC => 6 BMBT\n\
        6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL\n\
        6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT\n\
        15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW\n\
        13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW\n\
        5 BMBT => 4 WPTQ\n189 ORE => 9 KTJDG\n1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP\n\
        12 VRPVC, 27 CNZTR => 2 XDBXC\n15 KTJDG, 12 BHXH => 5 XCVML\n\
        3 BHXH, 2 VRPVC => 7 MZWV\n121 ORE => 7 VRPVC\n7 XCVML => 6 RJRHP\n\
        5 BHXH, 4 VRPVC => 5 LTCX";

    #[test]
    fn test_part_one() {
        let input_raw = "10 ORE => 10 A\n1 ORE => 1 B\n7 A, 1 B => 1 C\n\
            7 A, 1 C => 1 D\n7 A, 1 D => 1 E\n7 A, 1 E => 1 FUEL";
        let reactions = parse_reactions(&input_raw);
        assert_eq!(ores_required_for_fuel(&reactions), 31);

        let input_raw = "9 ORE => 2 A\n8 ORE => 3 B\n7 ORE => 5 C\n3 A, 4 B => 1 AB\n\
            5 B, 7 C => 1 BC\n4 C, 1 A => 1 CA\n2 AB, 3 BC, 4 CA => 1 FUEL";
        let reactions = parse_reactions(&input_raw);
        assert_eq!(ores_required_for_fuel(&reactions), 165);

        let reactions = parse_reactions(&INPUT_TEST1);
        assert_eq!(ores_required_for_fuel(&reactions), 13312);

        let reactions = parse_reactions(&INPUT_TEST2);
        assert_eq!(ores_required_for_fuel(&reactions), 180697);

        let reactions = parse_reactions(&INPUT_TEST3);
        assert_eq!(ores_required_for_fuel(&reactions), 2210736);
    }

    #[test]
    fn test_part_two() {
        let reactions = parse_reactions(&INPUT_TEST1);
        assert_eq!(max_fuel(&reactions), 82892753);

        let reactions = parse_reactions(&INPUT_TEST2);
        assert_eq!(max_fuel(&reactions), 5586022);

        let reactions = parse_reactions(&INPUT_TEST3);
        assert_eq!(max_fuel(&reactions), 460664);
    }
}
