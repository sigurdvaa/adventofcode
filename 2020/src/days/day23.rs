fn parse_cups(input: &str) -> Vec<u32> {
    input.chars().map(|c| c.to_digit(10).expect("valid int")).collect()
}

fn crab_game(cups: &[u32], iter: usize) -> String {
    let mut order = [0; 9];
    for i in 0..8 {
        order[cups[i] as usize - 1] = cups[i + 1] as usize - 1;
    }
    order[cups[8] as usize - 1] = cups[0] as usize - 1;

    let mut curr = cups[0] as usize - 1;
    for _ in 0..iter {
        let pickup_1 = order[curr];
        let pickup_2 = order[pickup_1];
        let pickup_3 = order[pickup_2];
        let next = order[pickup_3];

        // dbg!(curr, pickup_1, pickup_2, pickup_3, next);

        let mut dest = match curr {
            0 => 8,
            _ => curr - 1,
        };
        while dest == pickup_1 || dest == pickup_2 || dest == pickup_3 {
            dest = match dest {
                0 => 8,
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
    for _ in 0..8 {
        output.push_str((curr + 1).to_string().as_str());
        curr = order[curr];
    }
    output
}

pub fn run() {
    let input_raw = "123487596";
    let cups = parse_cups(input_raw);
    println!("Day 23: Crab Cups");
    println!("Part One: {}", crab_game(&cups, 100));
    println!("Part Two: {}", "TODO");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = "389125467";

    #[test]
    fn test_part_one() {
        let cups = parse_cups(INPUT_TEST);
        assert_eq!(crab_game(&cups, 10), "92658374".to_string());
        assert_eq!(crab_game(&cups, 100), "67384529".to_string());
    }

    #[test]
    fn test_part_two() {}
}

/*
0: 3
1: 4
2: 0
3: 6
4: 5
5: 7
6: 8
7: 1
8: 2
 */
