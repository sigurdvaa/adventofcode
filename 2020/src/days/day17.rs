fn parse_cubes(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect()
}

fn simulate_cycle(state: &mut Vec<Vec<Vec<bool>>>) {}

fn active_cubes_after_cycle(cubes: &[Vec<bool>], cycles: usize) -> usize {
    let mut state: Vec<Vec<Vec<bool>>> = vec![vec![vec![]], cubes.to_vec(), vec![vec![]]];

    for _ in 0..cycles {
        simulate_cycle(&mut state);
    }

    state
        .iter()
        .map(|z| {
            z.iter()
                .map(|y| y.iter().filter(|x| **x).count())
                .sum::<usize>()
        })
        .sum()
}

pub fn run() {
    let input_raw = crate::load_input(module_path!());
    println!("Day 17: Conway Cubes");
    println!("Part One: {}", "TODO");
    println!("Part Two: {}", "TODO");
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
