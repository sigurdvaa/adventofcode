#[derive(Debug, Clone)]
enum Shuffle {
    Cut(i128),
    Into,
    With(i128),
}

fn parse_shuffle(instructions: &str) -> Vec<Shuffle> {
    let mut shuffle = vec![];
    for line in instructions.lines() {
        let mut split = line.split(' ');
        match split.next() {
            Some("deal") => match split.next() {
                Some("into") => shuffle.push(Shuffle::Into),
                Some("with") => shuffle.push(Shuffle::With(split.nth(1).unwrap().parse().unwrap())),
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

pub fn mod_exp(mut base: i128, mut exponent: i128, modulus: i128) -> i128 {
    let mut result = 1;
    base %= modulus;

    while exponent > 0 {
        if exponent % 2 == 1 {
            result = (result * base) % modulus;
        }

        exponent >>= 1;
        base = (base * base) % modulus;
    }

    result
}

fn pos_after_shuffle_reverse(shuffle: &Vec<Shuffle>, size: i128, times: i128, pos: i128) -> i128 {
    let mut factor: i128 = 1;
    let mut offset: i128 = 0;
    for s in shuffle {
        match s {
            Shuffle::Cut(n) => {
                offset += factor * n;
            }
            Shuffle::Into => {
                offset -= factor;
                factor *= -1;
            }
            Shuffle::With(n) => {
                factor = (factor * mod_exp(*n, size - 2, size)).rem_euclid(size);
            }
        }
    }
    offset = (offset * mod_exp(1 - factor, size - 2, size)).rem_euclid(size);
    factor = mod_exp(factor, times, size);
    (pos * factor + (1 - factor) * offset).rem_euclid(size)
}

pub fn run() {
    println!("Day 22: Slam Shuffle");
    let input_raw = crate::load_input(module_path!());
    let shuffle = parse_shuffle(&input_raw);
    println!("Part One: {}", pos_after_shuffle(&shuffle, 10007, 2019));
    println!(
        "Part Two: {}",
        pos_after_shuffle_reverse(&shuffle, 119315717514047, 101741582076661, 2020)
    );
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
    fn test_part_two() {
        let input_raw = crate::load_input("day22");
        let shuffle = parse_shuffle(&input_raw);
        assert_eq!(pos_after_shuffle_reverse(&shuffle, 10007, 1, 8379), 2019)
    }
}
