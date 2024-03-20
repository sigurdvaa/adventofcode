use std::fs;

fn occupied_cells(cells: &mut [char]) -> usize {
    let prev = "".to_string();
    while prev != cells.iter().collect::<String>() {}
    cells.iter().filter(|&c| *c == 'L').count()
}

fn parse_cells(input: &str) -> Vec<char> {
    input.chars().filter(|c| *c != '\n').collect()
}

pub fn run() {
    println!("Day 11: Seating System");
    let file_path = "inputs/day11.txt";
    let input_raw = fs::read_to_string(file_path)
        .unwrap_or_else(|err| panic!("Error reading file '{file_path}': {err}"));

    let mut cells = parse_cells(&input_raw);
    println!("Part One: {}", occupied_cells(&mut cells));
    println!("Part Two: {}", "TODO");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = concat!(
        "L.LL.LL.LL\n",
        "LLLLLLL.LL\n",
        "L.L.L..L..\n",
        "LLLL.LL.LL\n",
        "L.LL.LL.LL\n",
        "L.LLLLL.LL\n",
        "..L.L.....\n",
        "LLLLLLLLLL\n",
        "L.LLLLLL.L\n",
        "L.LLLLL.LL"
    );

    #[test]
    fn test_part_one() {
        let mut cells = parse_cells(INPUT_TEST);
        assert_eq!(occupied_cells(&mut cells), 37);
    }

    #[test]
    fn test_part_two() {}
}
