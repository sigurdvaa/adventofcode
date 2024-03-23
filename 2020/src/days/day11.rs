use std::fs;

fn count_neighbors_immediately(cells: &[Vec<char>], (x, y): (i32, i32), neighbor: char) -> usize {
    let mut count = 0;
    let len = cells.len() as i32;
    let width = cells[0].len() as i32;
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

    for d in dirs {
        let dx = x + d.0;
        let dy = y + d.1;
        if dx >= 0
            && dy >= 0
            && dx < width
            && dy < len
            && cells[dy as usize][dx as usize] == neighbor
        {
            count += 1;
        }
    }

    count
}

fn occupied_cells_immediately(cells: &[Vec<char>]) -> usize {
    const OCCUPIED: char = '#';
    const EMPTY: char = 'L';
    let mut prev = cells.to_vec();
    loop {
        let mut next = prev.clone();
        for (y, row) in prev.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let neighbors = count_neighbors_immediately(&prev, (x as i32, y as i32), OCCUPIED);
                if *cell == EMPTY && neighbors == 0 {
                    next[y][x] = OCCUPIED;
                } else if *cell == OCCUPIED && neighbors > 3 {
                    next[y][x] = EMPTY;
                }
            }
        }
        if prev == next {
            break;
        }
        prev = next;
    }
    prev.iter()
        .map(|row| row.iter().filter(|&c| *c == OCCUPIED).count())
        .sum()
}

fn count_neighbors_direction(
    cells: &[Vec<char>],
    (x, y): (i32, i32),
    neighbor: char,
    free: char,
) -> usize {
    let mut count = 0;
    let len = cells.len() as i32;
    let width = cells[0].len() as i32;
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

    for d in dirs {
        let mut s = 1;
        let mut dx = x + (d.0 * s);
        let mut dy = y + (d.1 * s);
        while dx >= 0 && dy >= 0 && dx < width && dy < len {
            if cells[dy as usize][dx as usize] == neighbor {
                count += 1;
                break;
            } else if cells[dy as usize][dx as usize] == free {
                break;
            }
            s += 1;
            dx = x + (d.0 * s);
            dy = y + (d.1 * s);
        }
    }

    count
}

fn occupied_cells_direction(cells: &[Vec<char>]) -> usize {
    const OCCUPIED: char = '#';
    const EMPTY: char = 'L';
    let mut prev = cells.to_vec();
    loop {
        let mut next = prev.clone();
        for (y, row) in prev.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let neighbors =
                    count_neighbors_direction(&prev, (x as i32, y as i32), OCCUPIED, EMPTY);
                if *cell == EMPTY && neighbors == 0 {
                    next[y][x] = OCCUPIED;
                } else if *cell == OCCUPIED && neighbors > 4 {
                    next[y][x] = EMPTY;
                }
            }
        }
        if prev == next {
            break;
        }
        prev = next;
    }
    prev.iter()
        .map(|row| row.iter().filter(|&c| *c == OCCUPIED).count())
        .sum()
}

fn parse_cells(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect()
}

pub fn run() {
    println!("Day 11: Seating System");
    let file_path = "inputs/day11.txt";
    let input_raw = fs::read_to_string(file_path)
        .unwrap_or_else(|err| panic!("Error reading file '{file_path}': {err}"));

    let cells = parse_cells(&input_raw);
    println!("Part One: {}", occupied_cells_immediately(&cells));
    // println!("Part Two: {}", occupied_cells_direction(&cells));
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
        let cells = parse_cells(INPUT_TEST);
        assert_eq!(occupied_cells_immediately(&cells), 37);
    }

    #[test]
    fn test_part_two() {
        let cells = parse_cells(INPUT_TEST);
        assert_eq!(occupied_cells_direction(&cells), 26);
    }
}
