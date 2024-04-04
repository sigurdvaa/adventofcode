pub fn run() {
    let input_raw = crate::load_input(module_path!());
    println!("Day 17: Conway Cubes");
    println!("Part One: {}", "TODO");
    println!("Part Two: {}", "TODO");
}

fn parse_cubes(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect()
}

fn active_cubes_after_cycle(cubes: &[Vec<bool>], cycles: usize) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = concat!(".#.\n", "..#\n", "###");
    #[test]
    fn test_part_one() {
        let cubes = parse_cubes(INPUT_TEST);
        assert_eq!(active_cubes_after_cycle(&cubes, 6), 112);
    }

    #[test]
    fn test_part_two() {}
}
