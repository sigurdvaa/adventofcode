use std::fs;

fn count_neighbors(width: usize, cells: &[char], i: usize, neighbor: char) -> usize {
    let mut count = 0;
    let rows = cells.len() / width;
    let x = i % width;
    let y = i / width;

    // left
    if x > 0 && cells[i - 1] == neighbor {
        count += 1;
    }

    // right
    if x < width - 1 && cells[i + 1] == neighbor {
        count += 1;
    }

    // up
    if y > 0 && cells[i - width] == neighbor {
        count += 1;
    }

    // down
    if y < rows - 1 && cells[i + width] == neighbor {
        count += 1;
    }

    // left up
    if x > 0 && y > 0 && cells[i - width - 1] == neighbor {
        count += 1;
    }

    // right up
    if x < width - 1 && y > 0 && cells[i - width + 1] == neighbor {
        count += 1;
    }

    // left down
    if x > 0 && y < rows - 1 && cells[i - 1 + width] == neighbor {
        count += 1;
    }

    // right down
    if x < width - 1 && y < rows - 1 && cells[i + 1 + width] == neighbor {
        count += 1;
    }

    count
}

fn occupied_cells(width: usize, cells: &[char]) -> usize {
    const OCCUPIED: char = '#';
    const EMPTY: char = 'L';
    let mut prev = cells.to_vec();
    loop {
        let mut next = prev.clone();
        for (i, c) in prev.iter().enumerate() {
            let neighbors = count_neighbors(width, &prev, i, OCCUPIED);
            if *c == EMPTY && neighbors == 0 {
                next[i] = OCCUPIED;
            } else if *c == OCCUPIED && neighbors > 3 {
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

fn parse_cells(input: &str) -> (usize, Vec<char>) {
    let width = input.chars().position(|c| c == '\n').unwrap();
    let cells = input.chars().filter(|c| *c != '\n').collect();
    (width, cells)
}

pub fn run() {
    println!("Day 11: Seating System");
    let file_path = "inputs/day11.txt";
    let input_raw = fs::read_to_string(file_path)
        .unwrap_or_else(|err| panic!("Error reading file '{file_path}': {err}"));

    let (width, cells) = parse_cells(&input_raw);
    println!("Part One: {}", occupied_cells(width, &cells));
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
        let (width, cells) = parse_cells(INPUT_TEST);
        assert_eq!(occupied_cells(width, &cells), 37);
    }

    #[test]
    fn test_part_two() {}
}
