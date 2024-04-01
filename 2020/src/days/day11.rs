fn occupied_seats(cells: &[char], width: usize, visible: bool) -> usize {
    const EMPTY: char = 'L';
    const FLOOR: char = '.';
    let threshold = if visible { 4 } else { 3 };
    let seat_neighbors = find_seat_neighbors(cells, width, EMPTY, FLOOR, visible);
    let mut curr = vec![false; cells.len()];
    let mut next = curr.clone();
    loop {
        for (i, neighbors) in seat_neighbors.iter() {
            let count = neighbors.iter().filter(|&n| curr[*n]).count();
            let curr_seat = curr[*i];
            let next_seat = &mut next[*i];
            if (!curr_seat && count == 0) || (curr_seat && count > threshold) {
                *next_seat = !curr_seat;
            } else {
                *next_seat = curr_seat;
            }
        }
        std::mem::swap(&mut curr, &mut next);
        if curr == next {
            break;
        }
    }
    curr.iter().filter(|&c| *c).count()
}

fn find_seat_neighbors(
    cells: &[char],
    width: usize,
    seat: char,
    floor: char,
    visible: bool,
) -> Vec<(usize, Vec<usize>)> {
    let mut seat_neighbors = vec![];
    let len = (cells.len() / width) as isize;
    let width = width as isize;
    let dirs: &[(isize, isize)] = &[
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
        if cells[i] == floor {
            continue;
        }

        let mut neighbors = vec![];
        let x = i as isize % width;
        let y = i as isize / width;

        for dir in dirs {
            let mut dx = x + dir.0;
            let mut dy = y + dir.1;
            while dx >= 0 && dy >= 0 && dx < width && dy < len {
                let di = ((dy * width) + dx) as usize;
                if cells[di] == seat {
                    neighbors.push(di);
                    break;
                }
                if !visible {
                    break;
                }
                dx += dir.0;
                dy += dir.1;
            }
        }
        seat_neighbors.push((i, neighbors));
    }
    seat_neighbors
}

fn parse_cells(input: &str) -> (Vec<char>, usize) {
    let width = input.lines().next().unwrap().len();
    let cells = input.chars().filter(|&c| !c.is_whitespace()).collect();
    (cells, width)
}

pub fn run() {
    println!("Day 11: Seating System");
    let input_raw = crate::load_input(module_path!());
    let (cells, width) = parse_cells(&input_raw);
    println!("Part One: {}", occupied_seats(&cells, width, false));
    println!("Part Two: {}", occupied_seats(&cells, width, true));
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
        assert_eq!(occupied_seats(&cells, width, false), 37);
    }

    #[test]
    fn test_part_two() {
        let (cells, width) = parse_cells(INPUT_TEST);
        assert_eq!(occupied_seats(&cells, width, true), 26);
    }
}
