use std::fs;

fn get_loc(pass: &str, start: usize, end: usize) -> usize {
    if pass.is_empty() {
        return end;
    }

    if pass.starts_with(|c| c == 'F' || c == 'L') {
        return get_loc(&pass[1..], start, start + (end - start) / 2);
    }

    get_loc(&pass[1..], start + (end - start) / 2, end)
}

fn seat_id(pass: &str) -> usize {
    let row = get_loc(&pass[..7], 0, 127);
    let col = get_loc(&pass[7..], 0, 7);
    row * 8 + col
}

fn highest_seat_id(boarding_passes: &str) -> usize {
    boarding_passes.lines().map(seat_id).max().unwrap_or(0)
}

fn my_seat_id(boarding_passes: &str) -> usize {
    let mut ids = boarding_passes.lines().map(seat_id).collect::<Vec<usize>>();
    ids.sort();
    let mut prev = ids[0];
    for id in ids.iter().skip(1) {
        if id - prev == 2 {
            return id - 1;
        }
        prev = *id;
    }
    0
}

pub fn run() {
    println!("Day 5: Binary Boarding");
    let file_path = "inputs/day05.txt";
    let input_raw = fs::read_to_string(file_path)
        .unwrap_or_else(|err| panic!("Error reading file '{file_path}': {err}"));

    println!("Part One: {}", highest_seat_id(&input_raw));
    println!("Part Two: {}", my_seat_id(&input_raw));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(seat_id("FBFBBFFRLR"), 357);
        assert_eq!(seat_id("BFFFBBFRRR"), 567);
        assert_eq!(seat_id("FFFBBBFRRR"), 119);
        assert_eq!(seat_id("BBFFBBFRLL"), 820);
    }

    #[test]
    fn test_part_two() {}
}
