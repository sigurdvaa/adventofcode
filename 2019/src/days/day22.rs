use std::fs;

#[derive(Debug, Clone)]
enum Shuffle {
    Cut(i128),
    Into,
    With(i128),
}

fn parse_shuffle(instructions: &str) -> Vec<Shuffle> {
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
    shuffle
}

fn pos_after_shuffle(shuffle: &Vec<Shuffle>, size: i128, mut pos: i128) -> i128 {
    for s in shuffle {
        match s {
            Shuffle::Cut(n) => pos = (pos - n).rem_euclid(size),
            Shuffle::Into => pos = size - 1 - pos,
            Shuffle::With(n) => pos = (pos * *n).rem_euclid(size),
        }
    }
    pos
}

pub fn run() {
    println!("Day 22: Slam Shuffle");
    let file_path = "inputs/day22.txt";
    let input_raw =
        fs::read_to_string(file_path).expect(format!("Error reading file '{file_path}'").as_str());

    let shuffle = parse_shuffle(&input_raw);
    println!("Part One: {}", pos_after_shuffle(&shuffle, 10007, 2019));

    //let shuffle = parse_shuffle(&input_raw);
    //println!(
    //    "Part Two: {}",
    //    pos_after_shuffle(shuffle, 119315717514047, 101741582076661, 2020)
    //);

    // deck of 119315717514047
    // shuffle 101741582076661 times
    // number at pos 2020
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let shuffle = parse_shuffle(concat!(
            "deal with increment 7\n",
            "deal into new stack\n",
            "deal into new stack",
        ));
        assert_eq!(pos_after_shuffle(&shuffle, 10, 3), 1);
        assert_eq!(pos_after_shuffle(&shuffle, 10, 7), 9);

        let shuffle = parse_shuffle(concat!(
            "cut 6\n",
            "deal with increment 7\n",
            "deal into new stack",
        ));
        assert_eq!(pos_after_shuffle(&shuffle, 10, 4), 3);
        assert_eq!(pos_after_shuffle(&shuffle, 10, 8), 5);

        let shuffle = parse_shuffle(concat!(
            "deal with increment 7\n",
            "deal with increment 9\n",
            "cut -2",
        ));
        assert_eq!(pos_after_shuffle(&shuffle, 10, 0), 2);
        assert_eq!(pos_after_shuffle(&shuffle, 10, 5), 7);

        let shuffle = parse_shuffle(concat!(
            "deal into new stack\n",
            "cut -2\n",
            "deal with increment 7\n",
            "cut 8\n",
            "cut -4\n",
            "deal with increment 7\n",
            "cut 3\n",
            "deal with increment 9\n",
            "deal with increment 3\n",
            "cut -1",
        ));
        assert_eq!(pos_after_shuffle(&shuffle, 10, 9), 0);
        assert_eq!(pos_after_shuffle(&shuffle, 10, 4), 5);
    }

    #[test]
    fn test_part_two() {}
}
