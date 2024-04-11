use std::collections::{HashMap, HashSet};

fn parse_cubes(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect()
}

fn count_active_neighbors_3d(state: &[Vec<Vec<bool>>], x: isize, y: isize, z: isize) -> usize {
    let mut count = 0;
    let layers_max = (state.len() as isize) - 1;
    let rows_max = (state[0].len() as isize) - 1;
    let cols_max = (state[0][0].len() as isize) - 1;
    for dz in -1isize..2 {
        for dy in -1isize..2 {
            for dx in -1isize..2 {
                if dz == 0 && dy == 0 && dx == 0 {
                    continue;
                }

                let nz = z + dz;
                let ny = y + dy;
                let nx = x + dx;
                if nz < 0 || ny < 0 || nx < 0 {
                    continue;
                }
                if nz >= layers_max || ny >= rows_max || nx >= cols_max {
                    continue;
                }

                if state[nz as usize][ny as usize][nx as usize] {
                    count += 1;
                }
            }
        }
    }
    count
}

fn count_active_neighbors_4d(
    state: &[Vec<Vec<Vec<bool>>>],
    x: isize,
    y: isize,
    z: isize,
    w: isize,
) -> usize {
    let mut count = 0;
    let hypers_max = (state.len() as isize) - 1;
    let layers_max = (state[0].len() as isize) - 1;
    let rows_max = (state[0][0].len() as isize) - 1;
    let cols_max = (state[0][0][0].len() as isize) - 1;
    for dw in -1isize..2 {
        for dz in -1isize..2 {
            for dy in -1isize..2 {
                for dx in -1isize..2 {
                    if dw == 0 && dz == 0 && dy == 0 && dx == 0 {
                        continue;
                    }

                    let nw = w + dw;
                    let nz = z + dz;
                    let ny = y + dy;
                    let nx = x + dx;
                    if nw < 0 || nz < 0 || ny < 0 || nx < 0 {
                        continue;
                    }
                    if nw >= hypers_max || nz >= layers_max || ny >= rows_max || nx >= cols_max {
                        continue;
                    }

                    if state[nw as usize][nz as usize][ny as usize][nx as usize] {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}
fn pad_state_3d(state: &mut Vec<Vec<Vec<bool>>>) {
    let mut z_pad_start = false;
    let mut z_pad_end = false;
    let mut y_pad_start = false;
    let mut y_pad_end = false;
    let mut x_pad_start = false;
    let mut x_pad_end = false;

    for (z, layer) in state.iter().enumerate() {
        for (y, row) in layer.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                if *col && x == 0 {
                    x_pad_start = true;
                }
                if *col && x == row.len() - 1 {
                    x_pad_end = true;
                }

                if *col && y == 0 {
                    y_pad_start = true;
                }
                if *col && y == layer.len() - 1 {
                    y_pad_end = true;
                }

                if *col && z == 0 {
                    z_pad_start = true;
                }
                if *col && z == state.len() - 1 {
                    z_pad_end = true;
                }
            }
        }
    }

    let mut rows = 0;
    let mut cols = 0;
    for layer in &mut *state {
        for row in &mut *layer {
            if x_pad_start {
                row.insert(0, false);
            }
            if x_pad_end {
                row.push(false);
            }
            cols = row.len();
        }
        if y_pad_start {
            layer.insert(0, vec![false; cols]);
        }
        if y_pad_end {
            layer.push(vec![false; cols]);
        }
        rows = layer.len();
    }
    if z_pad_start {
        state.insert(0, vec![vec![false; cols]; rows]);
    }
    if z_pad_end {
        state.push(vec![vec![false; cols]; rows]);
    }
}

fn pad_state_4d(state: &mut Vec<Vec<Vec<Vec<bool>>>>) {
    let mut w_pad_start = false;
    let mut w_pad_end = false;
    let mut z_pad_start = false;
    let mut z_pad_end = false;
    let mut y_pad_start = false;
    let mut y_pad_end = false;
    let mut x_pad_start = false;
    let mut x_pad_end = false;

    for (w, hyper) in state.iter().enumerate() {
        for (z, layer) in hyper.iter().enumerate() {
            for (y, row) in layer.iter().enumerate() {
                for (x, col) in row.iter().enumerate() {
                    if *col && x == 0 {
                        x_pad_start = true;
                    }
                    if *col && x == row.len() - 1 {
                        x_pad_end = true;
                    }

                    if *col && y == 0 {
                        y_pad_start = true;
                    }
                    if *col && y == layer.len() - 1 {
                        y_pad_end = true;
                    }

                    if *col && z == 0 {
                        z_pad_start = true;
                    }
                    if *col && z == hyper.len() - 1 {
                        z_pad_end = true;
                    }

                    if *col && w == 0 {
                        w_pad_start = true;
                    }
                    if *col && w == state.len() - 1 {
                        w_pad_end = true;
                    }
                }
            }
        }
    }

    let mut rows = 0;
    let mut cols = 0;
    let mut layers = 0;
    for hyper in &mut *state {
        for layer in &mut *hyper {
            for row in &mut *layer {
                if x_pad_start {
                    row.insert(0, false);
                }
                if x_pad_end {
                    row.push(false);
                }
                cols = row.len();
            }
            if y_pad_start {
                layer.insert(0, vec![false; cols]);
            }
            if y_pad_end {
                layer.push(vec![false; cols]);
            }
            rows = layer.len();
        }
        if z_pad_start {
            hyper.insert(0, vec![vec![false; cols]; rows]);
        }
        if z_pad_end {
            hyper.push(vec![vec![false; cols]; rows]);
        }
        layers = hyper.len();
    }
    if w_pad_start {
        state.insert(0, vec![vec![vec![false; cols]; rows]; layers]);
    }
    if w_pad_end {
        state.push(vec![vec![vec![false; cols]; rows]; layers]);
    }
}
fn simulate_cycle_3d(state: &mut Vec<Vec<Vec<bool>>>) {
    pad_state_3d(state);
    let mut next_state = state.clone();

    for (z, layer) in state.iter().enumerate() {
        for (y, row) in layer.iter().enumerate() {
            for (x, cube) in row.iter().enumerate() {
                let neighbors =
                    count_active_neighbors_3d(state, x as isize, y as isize, z as isize);
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
fn simulate_cycle_4d(state: &mut Vec<Vec<Vec<Vec<bool>>>>) {
    pad_state_4d(state);
    let mut next_state = state.clone();

    for (w, hyper) in state.iter().enumerate() {
        for (z, layer) in hyper.iter().enumerate() {
            for (y, row) in layer.iter().enumerate() {
                for (x, cube) in row.iter().enumerate() {
                    let neighbors = count_active_neighbors_4d(
                        state, x as isize, y as isize, z as isize, w as isize,
                    );
                    if *cube && !(neighbors == 2 || neighbors == 3) {
                        next_state[w][z][y][x] = false;
                    } else if !*cube && neighbors == 3 {
                        next_state[w][z][y][x] = true;
                    }
                }
            }
        }
    }

    std::mem::swap(state, &mut next_state);
}

fn active_cubes_after_cycle_3d(initial_cubes: &[Vec<bool>], cycles: usize) -> usize {
    let mut state: Vec<Vec<Vec<bool>>> = vec![initial_cubes.to_vec()];

    for _ in 0..cycles {
        simulate_cycle_3d(&mut state);
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

fn active_cubes_after_cycle_4d(initial_cubes: &[Vec<bool>], cycles: usize) -> usize {
    let mut state: Vec<Vec<Vec<Vec<bool>>>> = vec![vec![initial_cubes.to_vec()]];

    for _ in 0..cycles {
        simulate_cycle_4d(&mut state);
    }

    state
        .iter()
        .map(|w| {
            w.iter()
                .map(|z| {
                    z.iter()
                        .map(|y| y.iter().filter(|x| **x).count())
                        .sum::<usize>()
                })
                .sum::<usize>()
        })
        .sum()
}

fn get_coord_neighbors(coord: &[i32]) -> Vec<Vec<i32>> {
    let mut neighbors = vec![vec![]];
    for d in coord.iter() {
        let mut next_neighbors = vec![];
        for curr_neighbor in neighbors {
            for n in -1i32..2 {
                let nd = d + n;
                let mut next_neighbor = curr_neighbor.clone();
                next_neighbor.push(nd);
                if next_neighbor != coord {
                    next_neighbors.push(next_neighbor);
                }
            }
        }
        neighbors = next_neighbors;
    }
    neighbors
}

fn simulate_conway_cubes(
    state: &mut HashSet<Vec<i32>>,
    neighbor_buf: &mut HashMap<Vec<i32>, Vec<Vec<i32>>>,
) {
    let mut next_state: HashSet<Vec<i32>> = HashSet::new();
    let mut check_cubes = state.clone();
    for active_cube in state.iter() {
        let neighbors = neighbor_buf
            .entry(active_cube.clone())
            .or_insert_with(|| get_coord_neighbors(active_cube));
        check_cubes.extend(neighbors.clone());
    }

    for cube in check_cubes.iter() {
        let active = state.contains(cube);
        let neighbors = neighbor_buf
            .entry(cube.clone())
            .or_insert_with(|| get_coord_neighbors(cube));
        let count = neighbors.iter().filter(|n| state.contains(*n)).count();

        if (active && (count == 2 || count == 3)) || (!active && count == 3) {
            next_state.insert(cube.clone());
        }
    }
    std::mem::swap(state, &mut next_state);
}

fn active_cubes_after_cycles(init_cubes: &[Vec<bool>], dim: u32, cycles: u32) -> usize {
    let mut neighbor_buf: HashMap<Vec<i32>, Vec<Vec<i32>>> = HashMap::new();
    let mut state: HashSet<Vec<i32>> =
        HashSet::from_iter(init_cubes.iter().enumerate().flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, cube)| **cube)
                .map(move |(x, _)| {
                    let mut coords = vec![0; dim as usize];
                    coords[(dim - 1) as usize] = x as i32;
                    coords[(dim - 2) as usize] = y as i32;
                    coords
                })
        }));

    for _ in 0..cycles {
        simulate_conway_cubes(&mut state, &mut neighbor_buf);
    }

    state.len()
}

fn simulate_conway_cubes2(
    state: &mut HashMap<Vec<i32>, (bool, u16)>,
    neighbor_buf: &mut HashMap<Vec<i32>, Vec<Vec<i32>>>,
) {
    let mut next_state: HashMap<Vec<i32>, (bool, u16)> = HashMap::new();

    for (coord, (active, _count)) in state.iter() {
        if *active {
            let curr = next_state.entry(coord.clone()).or_insert((true, 0));
            *curr = (true, curr.1);

            let neighbors = neighbor_buf
                .entry(coord.clone())
                .or_insert_with(|| get_coord_neighbors(coord));
            for neighbor in neighbors {
                let cube = next_state.entry(neighbor.clone()).or_insert((false, 0));
                *cube = (cube.0, cube.1 + 1);
            }
        }
    }

    for cube in next_state.values_mut() {
        if cube.0 && !(cube.1 == 2 || cube.1 == 3) {
            *cube = (false, cube.1);
        } else if !cube.0 && cube.1 == 3 {
            *cube = (true, cube.1);
        }
    }

    std::mem::swap(state, &mut next_state);
}

fn active_cubes_after_cycles2(init_cubes: &[Vec<bool>], dim: u32, cycles: u32) -> usize {
    let mut neighbor_buf: HashMap<Vec<i32>, Vec<Vec<i32>>> = HashMap::new();
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
        simulate_conway_cubes2(&mut state, &mut neighbor_buf);
    }

    state.values().filter(|(active, _)| *active).count()
}

pub fn run() {
    let input_raw = crate::load_input(module_path!());
    let initial_cubes = parse_cubes(&input_raw);
    println!("Day 17: Conway Cubes");
    println!(
        "Part One: {}",
        active_cubes_after_cycle_3d(&initial_cubes, 6)
    );

    println!(
        "Part One: {}",
        active_cubes_after_cycles(&initial_cubes, 3, 6)
    );

    let now = std::time::Instant::now();
    println!(
        "Part Two: {}",
        active_cubes_after_cycle_4d(&initial_cubes, 6)
    );
    println!("Elapsed: {:?}", now.elapsed());

    let now = std::time::Instant::now();
    println!(
        "Part Two: {}",
        active_cubes_after_cycles(&initial_cubes, 4, 6)
    );
    println!("Elapsed: {:?}", now.elapsed());

    let now = std::time::Instant::now();
    println!(
        "Part Two: {}",
        active_cubes_after_cycles2(&initial_cubes, 4, 6)
    );
    println!("Elapsed: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = concat!(".#.\n", "..#\n", "###");
    #[test]
    fn test_part_one() {
        let initial_cubes = parse_cubes(INPUT_TEST);
        assert_eq!(active_cubes_after_cycle_3d(&initial_cubes, 6), 112);
        assert_eq!(active_cubes_after_cycles(&initial_cubes, 3, 6), 112);
        assert_eq!(active_cubes_after_cycles2(&initial_cubes, 3, 6), 112);
    }

    #[test]
    fn test_part_two() {
        let initial_cubes = parse_cubes(INPUT_TEST);
        assert_eq!(active_cubes_after_cycle_4d(&initial_cubes, 6), 848);
        assert_eq!(active_cubes_after_cycles(&initial_cubes, 4, 6), 848);
        assert_eq!(active_cubes_after_cycles2(&initial_cubes, 4, 6), 848);
    }
}
