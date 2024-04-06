fn parse_cubes(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect()
}

fn count_active_neighbors(state: &[Vec<Vec<bool>>], x: isize, y: isize, z: isize) -> usize {
    let mut count = 0;
    for dz in -1isize..2 {
        for dy in -1isize..2 {
            for dx in -1isize..2 {
                if dz == 0 && dy == 0 && dx == 0 {
                    continue;
                }
                if state[(z + dz) as usize][(y + dy) as usize][(x + dx) as usize] {
                    count += 1;
                }
            }
        }
    }
    count
}

fn simulate_cycle(state: &mut Vec<Vec<Vec<bool>>>) {
    let mut cols = 0;
    let mut rows = 0;
    for layer in &mut *state {
        for row in &mut *layer {
            // pad x
            row.insert(0, false);
            row.push(false);
            cols = row.len();
        }
        // pad y
        layer.insert(0, vec![false; cols]);
        layer.push(vec![false; cols]);
        rows = layer.len();
    }
    // pad z
    state.insert(0, vec![vec![false; cols]; rows]);
    state.push(vec![vec![false; cols]; rows]);

    let mut next_state = state.clone();

    for (z, layer) in state.iter().enumerate().skip(1).take(state.len() - 2) {
        for (y, row) in layer.iter().enumerate().skip(1).take(layer.len() - 2) {
            for (x, cube) in row.iter().enumerate().skip(1).take(row.len() - 2) {
                let neighbors = count_active_neighbors(state, x as isize, y as isize, z as isize);
                if *cube && !(neighbors == 2 || neighbors == 3) {
                    next_state[z][y][x] = false;
                } else if !*cube && neighbors == 3 {
                    next_state[z][y][x] = true;
                }
            }
        }
    }

    std::mem::swap(state, &mut next_state);
}

fn active_cubes_after_cycle(initial_cubes: &[Vec<bool>], cycles: usize) -> usize {
    let mut state: Vec<Vec<Vec<bool>>> = vec![initial_cubes.to_vec()];

    for i in 0..cycles {
        println!("\ncycle: {i}");
        println!("Before: ");
        for layer in &state {
            println!();
            for row in layer {
                println!(
                    "{}",
                    row.iter()
                        .map(|c| if *c { '#' } else { '.' })
                        .collect::<String>()
                );
            }
        }

        simulate_cycle(&mut state);

        println!("After: ");
        for layer in &state {
            println!();
            for row in layer {
                println!(
                    "{}",
                    row.iter()
                        .map(|c| if *c { '#' } else { '.' })
                        .collect::<String>()
                );
            }
        }
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
    let initial_cubes = parse_cubes(&input_raw);
    println!("Day 17: Conway Cubes");
    println!("Part One: {}", active_cubes_after_cycle(&initial_cubes, 6));
    println!("Part Two: {}", "TODO");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = concat!(".#.\n", "..#\n", "###");
    #[test]
    fn test_part_one() {
        let initial_cubes = parse_cubes(INPUT_TEST);
        assert_eq!(active_cubes_after_cycle(&initial_cubes, 6), 112);
    }

    #[test]
    fn test_part_two() {}
}
