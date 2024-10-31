use std::collections::HashSet;
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

fn recursive_combat_subgame(mut p1: VecDeque<usize>, mut p2: VecDeque<usize>) -> (VecDeque<usize>, VecDeque<usize>) {
    let mut seen = HashSet::new();

    while !p1.is_empty() && !p2.is_empty() {
        // if state seen before, the subgame is over
        if !seen.insert((p1.clone(), p2.clone())) {
            return (p1, VecDeque::new());
        }

        // draw cards
        let v1 = p1.pop_front().expect("deck can't be empty while playing");
        let v2 = p2.pop_front().expect("deck can't be empty while playing");

        // recurse if cards left in deck >= value of drawn card, and end round
        if v1 <= p1.len() && v2 <= p2.len() {
            let (n1, _n2) = recursive_combat_subgame(
                p1.iter().take(v1).copied().collect(),
                p2.iter().take(v2).copied().collect(),
            );
            if !n1.is_empty() {
                p1.push_back(v1);
                p1.push_back(v2);
            } else {
                p2.push_back(v2);
                p2.push_back(v1);
            }
            continue;
        }

        // regular round
        if v1 > v2 {
            p1.push_back(v1);
            p1.push_back(v2);
        } else {
            p2.push_back(v2);
            p2.push_back(v1);
        }
    }

    (p1, p2)
}

fn recursive_combat(p1: VecDeque<usize>, p2: VecDeque<usize>) -> usize {
    let (p1, p2) = recursive_combat_subgame(p1, p2);
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
