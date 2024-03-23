use std::fs;

fn count_neighbors_immediately(cells: &[char], width: usize, i: usize, neighbor: char) -> usize {
    let mut count = 0;
    let len = (cells.len() / width) as i32;
    let width = width as i32;
    let x = i as i32 % width;
    let y = i as i32 / width;
    let dirs: &[(i32, i32)] = &[
        (-1, 0),  // left
        (1, 0),   // right
        (0, -1),  // up
        (0, 1),   // down
        (-1, -1), // left up
        (-1, 1),  // left down
        (1, -1),  // right up
        (1, 1),   // right down
    ];

    for dir in dirs {
        let dx = x + dir.0;
        let dy = y + dir.1;
        if dx >= 0
            && dy >= 0
            && dx < width
            && dy < len
            && cells[((dy * width) + dx) as usize] == neighbor
        {
            count += 1;
        }
    }

    count
}

fn occupied_cells_immediately(cells: &[char], width: usize) -> usize {
    const OCCUPIED: char = '#';
    const EMPTY: char = 'L';
    let mut prev = cells.to_vec();
    loop {
        let mut next = prev.clone();
        for (i, cell) in prev.iter().enumerate() {
            let neighbors = count_neighbors_immediately(&prev, width, i, OCCUPIED);
            if *cell == EMPTY && neighbors == 0 {
                next[i] = OCCUPIED;
            } else if *cell == OCCUPIED && neighbors > 3 {
                next[i] = EMPTY;
            }
        }
        if prev == next {
            break;
        }
        prev = next;
    }
    prev.iter().filter(|&c| *c == OCCUPIED).count()
}

fn count_neighbors_direction(
    cells: &[char],
    width: usize,
    i: usize,
    neighbor: char,
    free: char,
) -> usize {
    let mut count = 0;
    let len = (cells.len() / width) as i32;
    let width = width as i32;
    let x = i as i32 % width;
    let y = i as i32 / width;
    let dirs: &[(i32, i32)] = &[
        (-1, 0),  // left
        (1, 0),   // right
        (0, -1),  // up
        (0, 1),   // down
        (-1, -1), // left up
        (-1, 1),  // left down
        (1, -1),  // right up
        (1, 1),   // right down
    ];

    for dir in dirs {
        let mut s = 1;
        let mut dx = x + (dir.0 * s);
        let mut dy = y + (dir.1 * s);
        while dx >= 0 && dy >= 0 && dx < width && dy < len {
            let di = ((dy * width) + dx) as usize;
            if cells[di] == neighbor {
                count += 1;
                break;
            } else if cells[di] == free {
                break;
            }
            s += 1;
            dx = x + (dir.0 * s);
            dy = y + (dir.1 * s);
        }
    }

    count
}

fn occupied_cells_direction(cells: &[char], width: usize) -> usize {
    const OCCUPIED: char = '#';
    const EMPTY: char = 'L';
    let mut prev = cells.to_vec();
    loop {
        let mut next = prev.clone();
        for (i, cell) in prev.iter().enumerate() {
            let neighbors = count_neighbors_direction(&prev, width, i, OCCUPIED, EMPTY);
            if *cell == EMPTY && neighbors == 0 {
                next[i] = OCCUPIED;
            } else if *cell == OCCUPIED && neighbors > 4 {
                next[i] = EMPTY;
            }
        }
        if prev == next {
            break;
        }
        prev = next;
    }
    prev.iter().filter(|&c| *c == OCCUPIED).count()
}

fn parse_cells(input: &str) -> (Vec<char>, usize) {
    let width = input.lines().next().unwrap().len();
    let cells = input.chars().filter(|&c| !c.is_whitespace()).collect();
    (cells, width)
}

pub fn run() {
    println!("Day 11: Seating System");
    let file_path = "inputs/day11.txt";
    let input_raw = fs::read_to_string(file_path)
        .unwrap_or_else(|err| panic!("Error reading file '{file_path}': {err}"));

    let (cells, width) = parse_cells(&input_raw);
    println!("Part One: {}", occupied_cells_immediately(&cells, width));
    println!("Part Two: {}", occupied_cells_direction(&cells, width));
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
        let (cells, width) = parse_cells(INPUT_TEST);
        assert_eq!(occupied_cells_immediately(&cells, width), 37);
    }

    #[test]
    fn test_part_two() {
        let (cells, width) = parse_cells(INPUT_TEST);
        assert_eq!(occupied_cells_direction(&cells, width), 26);
    }
}
