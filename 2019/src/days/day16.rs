use std::fs;

fn parse_numbers(input: &str) -> Vec<i32> {
    input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .map(|x| x as i32)
        .collect()
}

fn create_subpattern(base: &Vec<i32>, len: usize, offset: usize) -> Vec<i32> {
    let mut sub = Vec::with_capacity(len);
    for i in 0..len {
        sub.push(base[((i + 1) / (offset + 1)) % base.len()]);
    }
    sub
}

fn create_patterns(base: Vec<i32>, len: usize) -> Vec<Vec<i32>> {
    let mut patterns = Vec::with_capacity(len);
    for i in 0..len {
        patterns.push(create_subpattern(&base, len, i));
    }
    patterns
}

fn fft_phases(mut numbers: Vec<i32>, phases: i32) -> Vec<i32> {
    let base = vec![0, 1, 0, -1];
    let patterns = create_patterns(base, numbers.len());
    for _ in 0..phases {
        let mut next = Vec::with_capacity(numbers.len());
        for i in 0..next.capacity() {
            next.push(
                numbers
                    .iter()
                    .zip(&patterns[i])
                    .skip(i)
                    .map(|(a, b)| *a * b)
                    .sum::<i32>()
                    .abs()
                    % 10,
            );
        }
        numbers = next;
    }
    numbers
}

fn fft_phases_real(mut numbers: Vec<i32>) -> Vec<i32> {
    let offset = numbers[..7].iter().fold(0, |acc, x| acc * 10 + x) as usize;
    let input_len = numbers.len();
    numbers = numbers
        .into_iter()
        .cycle()
        .take(input_len * 10_000)
        .skip(offset)
        .collect();
    fft_phases(numbers, 100)
}

pub fn run() {
    println!("Day 16: Flawed Frequency Transmission");
    let file_path = "inputs/day16.txt";
    let input_raw =
        fs::read_to_string(file_path).expect(format!("Error reading file '{file_path}'").as_str());

    let numbers = parse_numbers(&input_raw);
    println!(
        "Part One: {}",
        fft_phases(numbers, 100)[..8]
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join("")
    );

    let input_raw = "03036732577212944063491565474664";
    let numbers = parse_numbers(&input_raw);
    //assert_eq!(fft_phases_real(numbers)[..8], [8, 4, 4, 6, 2, 0, 2, 6]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input_raw = "12345678";
        let numbers = parse_numbers(&input_raw);
        assert_eq!(fft_phases(numbers, 4), [0, 1, 0, 2, 9, 4, 9, 8]);

        let input_raw = "80871224585914546619083218645595";
        let numbers = parse_numbers(&input_raw);
        assert_eq!(fft_phases(numbers, 100)[..8], [2, 4, 1, 7, 6, 1, 7, 6]);

        let input_raw = "19617804207202209144916044189917";
        let numbers = parse_numbers(&input_raw);
        assert_eq!(fft_phases(numbers, 100)[..8], [7, 3, 7, 4, 5, 4, 1, 8]);

        let input_raw = "69317163492948606335995924319873";
        let numbers = parse_numbers(&input_raw);
        assert_eq!(fft_phases(numbers, 100)[..8], [5, 2, 4, 3, 2, 1, 3, 3]);
    }

    #[test]
    fn test_part_two() {
        let input_raw = "03036732577212944063491565474664";
        let numbers = parse_numbers(&input_raw);
        assert_eq!(fft_phases_real(numbers)[..8], [8, 4, 4, 6, 2, 0, 2, 6]);

        //let input_raw = "02935109699940807407585447034323";
        //let numbers = parse_numbers(&input_raw);
        //assert_eq!(fft_phases_real(numbers)[..8], [7, 8, 7, 2, 5, 2, 7, 0]);

        //let input_raw = "03081770884921959731165446850517";
        //let numbers = parse_numbers(&input_raw);
        //assert_eq!(fft_phases_real(numbers)[..8], [5, 3, 5, 5, 3, 7, 3, 1]);
    }
}
