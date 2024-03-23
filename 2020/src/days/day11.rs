use std::fs;

fn find_neighbors_immediately(
    cells: &[char],
    width: usize,
    seat: char,
    floor: char,
) -> Vec<Vec<usize>> {
    let mut neighbors = vec![];
    let len = (cells.len() / width) as i32;
    let width = width as i32;
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

    for i in 0..cells.len() {
        neighbors.push(vec![]);
        if cells[i] == floor {
            continue;
        }

        let x = i as i32 % width;
        let y = i as i32 / width;

        for dir in dirs {
            let dx = x + dir.0;
            let dy = y + dir.1;
            if dx >= 0
                && dy >= 0
                && dx < width
                && dy < len
                && cells[((dy * width) + dx) as usize] == seat
            {
                neighbors[i].push(((dy * width) + dx) as usize);
            }
        }
    }
    neighbors
}

fn occupied_cells_immediately(cells: &[char], width: usize) -> usize {
    const OCCUPIED: char = '#';
    const EMPTY: char = 'L';
    const FLOOR: char = '.';
    let neighbors = find_neighbors_immediately(cells, width, EMPTY, FLOOR);
    let mut occupied_cells = cells.iter().map(|&c| c == OCCUPIED).collect::<Vec<bool>>();
    let mut change = vec![];
    loop {
        for (i, cell) in occupied_cells.iter().enumerate() {
            if neighbors[i].is_empty() {
                continue;
            }
            let count = neighbors[i].iter().filter(|&n| occupied_cells[*n]).count();
            if (!*cell && count == 0) || (*cell && count > 3) {
                change.push(i);
            }
        }

        if change.is_empty() {
            break;
        }
        for &i in &change {
            occupied_cells[i] = !occupied_cells[i];
        }
        change.clear();
    }
    occupied_cells.iter().filter(|&c| *c).count()
}

fn find_neighbors_direction(
    cells: &[char],
    width: usize,
    seat: char,
    floor: char,
) -> Vec<Vec<usize>> {
    let mut neighbors = vec![];
    let len = (cells.len() / width) as i32;
    let width = width as i32;
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

    for i in 0..cells.len() {
        neighbors.push(vec![]);
        if cells[i] == floor {
            continue;
        }

        let x = i as i32 % width;
        let y = i as i32 / width;

        for dir in dirs {
            let mut dx = x + dir.0;
            let mut dy = y + dir.1;
            while dx >= 0 && dy >= 0 && dx < width && dy < len {
                let di = ((dy * width) + dx) as usize;
                if cells[di] == seat {
                    neighbors[i].push(di);
                    break;
                }
                dx += dir.0;
                dy += dir.1;
            }
        }
    }
    neighbors
}

fn occupied_cells_direction(cells: &[char], width: usize) -> usize {
    const OCCUPIED: char = '#';
    const EMPTY: char = 'L';
    const FLOOR: char = '.';
    let neighbors = find_neighbors_direction(cells, width, EMPTY, FLOOR);
    let mut occupied_cells = cells.iter().map(|&c| c == OCCUPIED).collect::<Vec<bool>>();
    let mut change = vec![];
    loop {
        for (i, cell) in occupied_cells.iter().enumerate() {
            if neighbors[i].is_empty() {
                continue;
            }
            let count = neighbors[i].iter().filter(|&n| occupied_cells[*n]).count();
            if (!*cell && count == 0) || (*cell && count > 4) {
                change.push(i);
            }
        }

        if change.is_empty() {
            break;
        }
        for &i in &change {
            occupied_cells[i] = !occupied_cells[i];
        }
        change.clear();
    }
    occupied_cells.iter().filter(|&c| *c).count()
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
