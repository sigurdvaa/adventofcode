fn parse_cups(input: &str) -> Vec<u32> {
    input.chars().map(|c| c.to_digit(10).expect("valid int")).collect()
}

fn crab_game_output_order(cups: &[u32], iter: usize) -> String {
    let mut order = vec![0; cups.len()];
    let last_idx = cups.len() - 1;
    for i in 0..last_idx {
        order[cups[i] as usize - 1] = cups[i + 1] as usize - 1;
    }
    order[cups[last_idx] as usize - 1] = cups[0] as usize - 1;

    let mut curr = cups[0] as usize - 1;
    for _ in 0..iter {
        let pickup_1 = order[curr];
        let pickup_2 = order[pickup_1];
        let pickup_3 = order[pickup_2];
        let next = order[pickup_3];

        let mut dest = match curr {
            0 => last_idx,
            _ => curr - 1,
        };
        while dest == pickup_1 || dest == pickup_2 || dest == pickup_3 {
            dest = match dest {
                0 => last_idx,
                _ => dest - 1,
            };
        }

        order[curr] = next;
        curr = next;

        order[pickup_3] = order[dest];
        order[dest] = pickup_1;
    }
    let mut curr = order[0];
    let mut output = String::new();
    for _ in 0..last_idx {
        output.push_str((curr + 1).to_string().as_str());
        curr = order[curr];
    }
    output
}

fn crab_game_output_product(cups: &[u32], iter: usize) -> usize {
    const GAME_SIZE: usize = 1_000_000;
    let mut order = vec![0; GAME_SIZE];
    let last_idx = cups.len() - 1;
    for i in 0..last_idx {
        order[cups[i] as usize - 1] = cups[i + 1] as usize - 1;
    }
    order[cups[last_idx] as usize - 1] = last_idx + 1;
    for i in (last_idx + 2)..GAME_SIZE {
        order[i - 1] = i;
    }
    order[GAME_SIZE - 1] = cups[0] as usize - 1;

    let mut curr = cups[0] as usize - 1;
    for _ in 0..iter {
        let pickup_1 = order[curr];
        let pickup_2 = order[pickup_1];
        let pickup_3 = order[pickup_2];
        let next = order[pickup_3];

        let mut dest = match curr {
            0 => GAME_SIZE - 1,
            _ => curr - 1,
        };
        while dest == pickup_1 || dest == pickup_2 || dest == pickup_3 {
            dest = match dest {
                0 => GAME_SIZE - 1,
                _ => dest - 1,
            };
        }

        order[curr] = next;
        curr = next;

        order[pickup_3] = order[dest];
        order[dest] = pickup_1;
    }
    let first = order[0];
    let second = order[first];
    (first + 1) * (second + 1)
}

pub fn run() {
    let input_raw = "123487596";
    let cups = parse_cups(input_raw);
    println!("Day 23: Crab Cups");
    println!("Part One: {}", crab_game_output_order(&cups, 100));
    println!("Part Two: {}", crab_game_output_product(&cups, 10_000_000));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = "389125467";

    #[test]
    fn test_part_one() {
        let cups = parse_cups(INPUT_TEST);
        assert_eq!(crab_game_output_order(&cups, 10), "92658374".to_string());
        assert_eq!(crab_game_output_order(&cups, 100), "67384529".to_string());
    }

    #[test]
    fn test_part_two() {
        let cups = parse_cups(INPUT_TEST);
        assert_eq!(crab_game_output_product(&cups, 10_000_000), 149245887792);
    }
}
