use std::fs;

#[derive(Debug, Clone)]
enum Shuffle {
    Cut(isize),
    Into,
    With(usize),
}

#[derive(Clone)]
struct Deck {
    cards: Vec<usize>,
    shuffle: Option<Vec<Shuffle>>,
}

impl Deck {
    fn new(n: usize) -> Self {
        Self {
            cards: (0..n).collect(),
            shuffle: None,
        }
    }

    fn parse_shuffle(&mut self, instructions: &str) {
        let mut shuffle = vec![];
        for line in instructions.lines() {
            let mut split = line.split(" ");
            match split.next() {
                Some("deal") => match split.next() {
                    Some("into") => shuffle.push(Shuffle::Into),
                    Some("with") => shuffle.push(Shuffle::With(
                        split.skip(1).next().unwrap().parse().unwrap(),
                    )),
                    _ => unreachable!(),
                },
                Some("cut") => shuffle.push(Shuffle::Cut(split.next().unwrap().parse().unwrap())),
                _ => (),
            }
        }
        if shuffle.len() > 0 {
            self.shuffle = Some(shuffle);
        }
    }

    fn shuffle(&mut self) {
        if let Some(shuffle) = &self.shuffle {
            let shuffle = shuffle.clone();
            for s in shuffle {
                match s {
                    Shuffle::Cut(n) => self.cut(n),
                    Shuffle::Into => self.cards.reverse(),
                    Shuffle::With(n) => self.deal_with(n),
                }
            }
        }
    }

    fn cut(&mut self, n: isize) {
        let mut next = Vec::with_capacity(self.cards.len());
        if n < 0 {
            let n = self.cards.len() - n.abs() as usize;
            next.extend(self.cards.iter().skip(n).cloned());
            next.extend(self.cards.iter().take(n).cloned());
        } else {
            next.extend(self.cards.iter().skip(n as usize).cloned());
            next.extend(self.cards.iter().take(n as usize).cloned());
        }
        self.cards = next;
    }

    fn deal_with(&mut self, n: usize) {
        let mut next = vec![0; self.cards.len()];
        for (i, v) in self.cards.iter().enumerate() {
            next[(i * n) % self.cards.len()] = *v;
        }
        self.cards = next;
    }
}

pub fn run() {
    println!("Day 22: Slam Shuffle");
    let file_path = "inputs/day22.txt";
    let input_raw =
        fs::read_to_string(file_path).expect(format!("Error reading file '{file_path}'").as_str());

    let mut deck = Deck::new(10007);
    deck.parse_shuffle(&input_raw);
    deck.shuffle();
    println!(
        "Part One: {}",
        deck.cards.iter().position(|n| *n == 2019).unwrap()
    );

    // deck of 119315717514047
    // shuffle 101741582076661 times
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input_test = concat!(
            "deal with increment 7\n",
            "deal into new stack\n",
            "deal into new stack\n",
        );
        let mut deck = Deck::new(10);
        deck.parse_shuffle(&input_test);
        deck.shuffle();
        assert_eq!(deck.cards, [0, 3, 6, 9, 2, 5, 8, 1, 4, 7]);

        let input_test = concat!(
            "cut 6\n",
            "deal with increment 7\n",
            "deal into new stack\n",
        );
        let mut deck = Deck::new(10);
        deck.parse_shuffle(&input_test);
        deck.shuffle();
        assert_eq!(deck.cards, [3, 0, 7, 4, 1, 8, 5, 2, 9, 6]);

        let input_test = concat!(
            "deal with increment 7\n",
            "deal with increment 9\n",
            "cut -2\n",
        );
        let mut deck = Deck::new(10);
        deck.parse_shuffle(&input_test);
        deck.shuffle();
        assert_eq!(deck.cards, [6, 3, 0, 7, 4, 1, 8, 5, 2, 9]);

        let input_test = concat!(
            "deal into new stack\n",
            "cut -2\n",
            "deal with increment 7\n",
            "cut 8\n",
            "cut -4\n",
            "deal with increment 7\n",
            "cut 3\n",
            "deal with increment 9\n",
            "deal with increment 3\n",
            "cut -1\n",
        );
        let mut deck = Deck::new(10);
        deck.parse_shuffle(&input_test);
        deck.shuffle();
        assert_eq!(deck.cards, [9, 2, 5, 8, 1, 4, 7, 0, 3, 6]);
    }

    #[test]
    fn test_part_two() {}
}
