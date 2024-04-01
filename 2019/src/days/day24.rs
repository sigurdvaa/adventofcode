use std::collections::HashSet;

fn biodiversity(cells: &[Vec<char>]) -> usize {
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

fn biodiversity_repeated(layout: &str) -> usize {
    let mut seen = HashSet::new();
    let mut cells = layout
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    seen.insert(cells.clone());
    loop {
        let next = game_of_bugs(&cells);
        if !seen.insert(next.clone()) {
            return biodiversity(&next);
        }
        cells = next;
    }
}

fn count_adjacent_bugs(cells: &Vec<Vec<char>>, x: usize, y: usize) -> usize {
    let mut count = 0;

    if y > 0 && cells[y - 1][x] == '#' {
        count += 1;
    }

    if y < cells.len() - 1 && cells[y + 1][x] == '#' {
        count += 1;
    }

    if x > 0 && cells[y][x - 1] == '#' {
        count += 1;
    }

    if x < cells[y].len() - 1 && cells[y][x + 1] == '#' {
        count += 1;
    }

    count
}

fn count_adjacent_bugs_recursive(levels: &[Vec<Vec<char>>], i: usize, x: usize, y: usize) -> usize {
    let mut count = 0;

    // check above
    if y == 0 {
        if i > 0 && levels[i - 1][1][2] == '#' {
            count += 1;
        }
    } else if y == 3 && x == 2 {
        if let Some(row) = levels.get(i + 1).and_then(|level| level.last()) {
            count += row.iter().filter(|&&c| c == '#').count();
        }
    } else if let Some(row) = levels[i].get(y - 1) {
        if row[x] == '#' {
            count += 1;
        }
    }

    // check below
    if y == 4 {
        if i > 0 && levels[i - 1][3][2] == '#' {
            count += 1;
        }
    } else if y == 1 && x == 2 {
        if let Some(row) = levels.get(i + 1).and_then(|level| level.first()) {
            count += row.iter().filter(|&&c| c == '#').count();
        }
    } else if let Some(row) = levels[i].get(y + 1) {
        if row[x] == '#' {
            count += 1;
        }
    }

    // check left
    if x == 0 {
        if i > 0 && levels[i - 1][2][1] == '#' {
            count += 1;
        }
    } else if y == 2 && x == 3 {
        if let Some(level) = levels.get(i + 1) {
            count += level
                .iter()
                .map(|row| match row.last() {
                    Some('#') => 1,
                    _ => 0,
                })
                .sum::<usize>();
        }
    } else if let Some(cell) = levels[i][y].get(x - 1) {
        if *cell == '#' {
            count += 1;
        }
    }

    // check right
    if x == 4 {
        if i > 0 && levels[i - 1][2][3] == '#' {
            count += 1;
        }
    } else if y == 2 && x == 1 {
        if let Some(level) = levels.get(i + 1) {
            count += level
                .iter()
                .map(|row| match row.first() {
                    Some('#') => 1,
                    _ => 0,
                })
                .sum::<usize>();
        }
    } else if let Some(cell) = levels[i][y].get(x + 1) {
        if *cell == '#' {
            count += 1;
        }
    }

    count
}

fn game_of_bugs(cells: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut next = cells.clone();

    for (y, row) in cells.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            let adjacent = count_adjacent_bugs(cells, x, y);
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

fn game_of_bugs_recursive(levels: &[Vec<Vec<char>>], i: usize) -> Vec<Vec<char>> {
    let mut next = levels[i].clone();

    for (y, row) in levels[i].iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == '?' {
                continue;
            }

            let adjacent = count_adjacent_bugs_recursive(levels, i, x, y);
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

fn bugs_after_minutes(layout: &str, minutes: usize) -> usize {
    let mut start_lvl = layout
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut empty_lvl = vec![vec!['.'; start_lvl[0].len()]; start_lvl.len()];
    start_lvl[2][2] = '?';
    empty_lvl[2][2] = '?';
    let mut levels = Vec::from([empty_lvl.clone(), start_lvl, empty_lvl.clone()]);

    for _ in 0..minutes {
        let mut next_levels = Vec::with_capacity(levels.capacity());

        if let Some(level) = levels.first() {
            if level.iter().any(|row| row.contains(&'#')) {
                next_levels.push(empty_lvl.clone());
            }
        };

        for i in 0..levels.len() {
            next_levels.push(game_of_bugs_recursive(&levels, i));
        }

        if let Some(level) = levels.last() {
            if level.iter().any(|row| row.contains(&'#')) {
                next_levels.push(empty_lvl.clone());
            }
        };

        levels = next_levels;
    }

    levels
        .iter()
        .map(|cells| {
            cells
                .iter()
                .map(|row| row.iter().filter(|&&c| c == '#').count())
                .sum::<usize>()
        })
        .sum()
}

pub fn run() {
    println!("Day 24: Planet of Discord");
    let input_raw = crate::load_input(module_path!());
    println!("Part One: {}", biodiversity_repeated(&input_raw));
    println!("Part Two: {}", bugs_after_minutes(&input_raw, 200));
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT_TEST: &str = "....#\n#..#.\n#..##\n..#..\n#....";

    #[test]
    fn test_part_one() {
        assert_eq!(biodiversity_repeated(INPUT_TEST), 2129920);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(bugs_after_minutes(INPUT_TEST, 10), 99);
    }
}
