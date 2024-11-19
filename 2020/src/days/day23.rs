fn parse_cups(input: &str) -> Vec<u32> {
    input.chars().map(|c| c.to_digit(10).expect("valid int")).collect()
}

fn crab_game_order(cups: &[u32], iter: usize) -> String {
    // build a "linked list" by using vec index as value, and value as next node
    let mut order = vec![0; cups.len()];
    let last_idx = cups.len() - 1;
    // fill in the input cups
    for i in 0..last_idx {
        order[cups[i] as usize - 1] = cups[i + 1] as usize - 1;
    }
    // add last link to wrap around
    order[cups[last_idx] as usize - 1] = cups[0] as usize - 1;

    // get first cup and run game
    let mut curr = cups[0] as usize - 1;
    for _ in 0..iter {
        let pickup_1 = order[curr];
        let pickup_2 = order[pickup_1];
        let pickup_3 = order[pickup_2];
        let next = order[pickup_3];

        // find dest (where to move the pickups)
        let mut dest = curr;
        while dest == curr || dest == pickup_1 || dest == pickup_2 || dest == pickup_3 {
            dest = match dest {
                0 => last_idx,
                _ => dest - 1,
            };
        }

        // move the pickups to after dest
        order[pickup_3] = order[dest];
        order[dest] = pickup_1;

        // set current cup to point to next cup (skipping the moved pickups)
        order[curr] = next;

        // continue from next cup
        curr = next;
    }

    // get output
    let mut curr = order[0];
    let mut output = String::new();
    for _ in 0..last_idx {
        output.push_str((curr + 1).to_string().as_str());
        curr = order[curr];
    }
    output
}

fn crab_game_mult(cups: &[u32], iter: usize) -> usize {
    // build a "linked list" by using vec index as value, and value as next node
    const GAME_SIZE: usize = 1_000_000;
    let mut order = vec![0; GAME_SIZE];
    // fill in the input cups
    let last_idx = cups.len() - 1;
    for i in 0..last_idx {
        order[cups[i] as usize - 1] = cups[i + 1] as usize - 1;
    }
    // add last input cup to the fillers
    order[cups[last_idx] as usize - 1] = last_idx + 1;
    // add the fillers up to GAME_SIZE
    for i in (last_idx + 2)..GAME_SIZE {
        order[i - 1] = i;
    }
    // add last link to wrap around
    order[GAME_SIZE - 1] = cups[0] as usize - 1;

    // get first cup and run game
    let mut curr = cups[0] as usize - 1;
    for _ in 0..iter {
        let pickup_1 = order[curr];
        let pickup_2 = order[pickup_1];
        let pickup_3 = order[pickup_2];
        let next = order[pickup_3];

        // find dest (where to move the pickups)
        let mut dest = curr;
        while dest == curr || dest == pickup_1 || dest == pickup_2 || dest == pickup_3 {
            dest = match dest {
                0 => GAME_SIZE - 1,
                _ => dest - 1,
            };
        }

        // move the pickups to after dest
        order[pickup_3] = order[dest];
        order[dest] = pickup_1;

        // set current cup to point to next cup (skipping the moved pickups)
        order[curr] = next;

        // continue from next cup
        curr = next;
    }

    // get output
    let first = order[0];
    let second = order[first];
    (first + 1) * (second + 1)
}

pub fn run() {
    let input_raw = "123487596";
    let cups = parse_cups(input_raw);
    println!("Day 23: Crab Cups");
    println!("Part One: {}", crab_game_order(&cups, 100));
    println!("Part Two: {}", crab_game_mult(&cups, 10_000_000));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = "389125467";

    #[test]
    fn test_part_one() {
        let cups = parse_cups(INPUT_TEST);
        assert_eq!(crab_game_order(&cups, 10), "92658374".to_string());
        assert_eq!(crab_game_order(&cups, 100), "67384529".to_string());
    }

    #[test]
    fn test_part_two() {
        let cups = parse_cups(INPUT_TEST);
        assert_eq!(crab_game_mult(&cups, 10_000_000), 149245887792);
    }
}
