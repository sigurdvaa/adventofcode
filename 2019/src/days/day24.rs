use std::collections::HashSet;
use std::fs;

fn biodiversity(cells: &Vec<Vec<char>>) -> usize {
    let mut sum = 0;
    for (y, row) in cells.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == '#' {
                sum += 2usize.pow((x + cells[y].len() * y) as u32);
            }
        }
    }
    sum
}

fn biodiversity_first_repeating_layout(layout: &str) -> usize {
    let mut seen = HashSet::new();
    let mut cells = layout
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    seen.insert(cells.clone());
    loop {
        let next = game_of_bugs(&cells);
        //println!();
        //for row in next.iter() {
        //    println!("{:?}", row.iter().collect::<String>())
        //}

        if !seen.insert(next.clone()) {
            return biodiversity(&next);
        }

        cells = next;
    }
}

fn count_adjacent_bugs(cells: &Vec<Vec<char>>, x: usize, y: usize) -> usize {
    let mut count = 0;

    if y > 0 {
        if cells[y - 1][x] == '#' {
            count += 1;
        }
    }

    if y < cells.len() - 1 {
        if cells[y + 1][x] == '#' {
            count += 1;
        }
    }

    if x > 0 {
        if cells[y][x - 1] == '#' {
            count += 1;
        }
    }

    if x < cells[y].len() - 1 {
        if cells[y][x + 1] == '#' {
            count += 1;
        }
    }

    count
}

fn game_of_bugs(cells: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut next = cells.clone();

    for (y, row) in cells.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            let adjacent = count_adjacent_bugs(&cells, x, y);
            //println!("x: {x}, y: {y}, adjacent: {adjacent}");
            match cell {
                '.' => {
                    if adjacent > 0 && adjacent < 3 {
                        next[y][x] = '#'
                    }
                }
                '#' => {
                    if adjacent != 1 {
                        next[y][x] = '.'
                    };
                }
                _ => unreachable!(),
            }
        }
    }

    next
}

pub fn run() {
    println!("Day 24: Planet of Discord");
    let file_path = "inputs/day24.txt";
    let input_raw =
        fs::read_to_string(file_path).expect(format!("Error reading file '{file_path}'").as_str());

    println!(
        "Part One: {}",
        biodiversity_first_repeating_layout(&input_raw)
    );
    println!("Part Two: {}", "TODO");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input_test = "....#\n#..#.\n#..##\n..#..\n#....";
        assert_eq!(biodiversity_first_repeating_layout(&input_test), 2129920);
    }
    #[test]
    fn test_part_two() {}
}
