use std::collections::HashMap;

fn parse_cubes(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect()
}

fn get_coord_neighbors(coord: &[i32]) -> Vec<Vec<i32>> {
    let mut neighbors = vec![vec![]];
    let mut next_neighbors = vec![];
    for d in coord.iter() {
        next_neighbors.clear();
        for curr_neighbor in neighbors.iter() {
            for n in -1i32..2 {
                let nd = d + n;
                let mut next_neighbor = curr_neighbor.clone();
                next_neighbor.push(nd);
                if next_neighbor != coord {
                    next_neighbors.push(next_neighbor);
                }
            }
        }
        std::mem::swap(&mut neighbors, &mut next_neighbors);
    }
    neighbors
}

fn active_cubes_after_cycles(init_cubes: &[Vec<bool>], dim: u32, cycles: u32) -> usize {
    let mut state: HashMap<Vec<i32>, (bool, u16)> =
        HashMap::from_iter(init_cubes.iter().enumerate().flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, cube)| **cube)
                .map(move |(x, _)| {
                    let mut coords = vec![0; dim as usize];
                    coords[(dim - 1) as usize] = x as i32;
                    coords[(dim - 2) as usize] = y as i32;
                    (coords, (true, 0))
                })
        }));

    for _ in 0..cycles {
        let mut next_state = state.clone();

        for coord in state.keys() {
            for neighbor in get_coord_neighbors(coord) {
                let cube = next_state.entry(neighbor).or_insert((false, 0));
                cube.1 += 1;
            }
        }

        next_state.retain(|_, cube| {
            if cube.0 && !(cube.1 == 2 || cube.1 == 3) {
                cube.0 = false
            } else if !cube.0 && cube.1 == 3 {
                cube.0 = true
            };
            cube.1 = 0;
            cube.0
        });

        std::mem::swap(&mut state, &mut next_state);
    }

    state.len()
}

pub fn run() {
    let input_raw = crate::load_input(module_path!());
    let initial_cubes = parse_cubes(&input_raw);
    println!("Day 17: Conway Cubes");
    println!(
        "Part One: {}",
        active_cubes_after_cycles(&initial_cubes, 3, 6)
    );
    println!(
        "Part Two: {}",
        active_cubes_after_cycles(&initial_cubes, 4, 6)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = concat!(".#.\n", "..#\n", "###");
    #[test]
    fn test_part_one() {
        let initial_cubes = parse_cubes(INPUT_TEST);
        assert_eq!(active_cubes_after_cycles(&initial_cubes, 3, 6), 112);
    }

    #[test]
    fn test_part_two() {
        let initial_cubes = parse_cubes(INPUT_TEST);
        assert_eq!(active_cubes_after_cycles(&initial_cubes, 4, 6), 848);
    }
}
