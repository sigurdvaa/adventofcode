use std::collections::VecDeque;

fn parse_decks(input: &str) -> (VecDeque<usize>, VecDeque<usize>) {
    let mut lines = input.lines();
    let mut p1 = VecDeque::new();
    let mut p2 = VecDeque::new();

    if let Some("Player 1:") = lines.next() {
        for card in lines.by_ref() {
            match card {
                "" => break,
                n => p1.push_back(n.parse().expect("")),
            }
        }
    }

    if let Some("Player 2:") = lines.next() {
        for card in lines {
            match card {
                "" => break,
                n => p2.push_back(n.parse().expect("")),
            }
        }
    }

    (p1, p2)
}

fn combat(mut p1: VecDeque<usize>, mut p2: VecDeque<usize>) -> usize {
    while !p1.is_empty() && !p2.is_empty() {
        let v1 = p1.pop_front().expect("deck can't be empty while playing");
        let v2 = p2.pop_front().expect("deck can't be empty while playing");
        if v1 > v2 {
            p1.push_back(v1);
            p1.push_back(v2);
        } else {
            p2.push_back(v2);
            p2.push_back(v1);
        }
    }

    let winner = if p1.is_empty() { p2 } else { p1 };
    winner.iter().enumerate().map(|(i, val)| (winner.len() - i) * val).sum()
}

fn recursive_combat(mut p1: VecDeque<usize>, mut p2: VecDeque<usize>) -> usize {
    while !p1.is_empty() && !p2.is_empty() {
        let v1 = p1.pop_front().expect("deck can't be empty while playing");
        let v2 = p2.pop_front().expect("deck can't be empty while playing");
        if v1 > v2 {
            p1.push_back(v1);
            p1.push_back(v2);
        } else {
            p2.push_back(v2);
            p2.push_back(v1);
        }
    }

    let winner = if p1.is_empty() { p2 } else { p1 };
    winner.iter().enumerate().map(|(i, val)| (winner.len() - i) * val).sum()
}

pub fn run() {
    let input_raw = crate::load_input(module_path!());
    let (p1, p2) = parse_decks(&input_raw);
    println!("Day 22: Crab Combat");
    println!("Part One: {}", combat(p1.clone(), p2.clone()));
    println!("Part Two: {}", recursive_combat(p1, p2));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = concat!(
        "Player 1:\n",
        "9\n",
        "2\n",
        "6\n",
        "3\n",
        "1\n",
        "\n",
        "Player 2:\n",
        "5\n",
        "8\n",
        "4\n",
        "7\n",
        "10\n",
        ""
    );

    #[test]
    fn test_part_one() {
        let (p1, p2) = parse_decks(INPUT_TEST);
        assert_eq!(combat(p1, p2), 306);
    }

    #[test]
    fn test_part_two() {
        let (p1, p2) = parse_decks(INPUT_TEST);
        assert_eq!(recursive_combat(p1, p2), 291);
    }
}
