use std::fs;

const SLOPES: &[(usize, usize); 5] = &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

fn trees_encountered_product(map: &Vec<Vec<char>>, slopes: &[(usize, usize)]) -> usize {
    let mut product = 1;

    for slope in slopes {
        let mut slope_count = 0;
        let mut x = 0;
        let mut y = 0;

        while y < map.len() {
            if map[y][x] == '#' {
                slope_count += 1;
            }
            x = (x + slope.0) % map[y].len();
            y += slope.1;
        }

        product *= slope_count;
    }

    product
}

pub fn run() {
    println!("Day 3: Toboggan Trajectory");
    let file_path = "inputs/day03.txt";
    let input_raw = fs::read_to_string(file_path)
        .unwrap_or_else(|err| panic!("Error reading file '{file_path}': {err}"));

    let map = input_raw
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<_>>>();
    println!(
        "Part One: {}",
        trees_encountered_product(&map, &SLOPES[1..2])
    );
    println!("Part Two: {}", trees_encountered_product(&map, SLOPES));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = concat!(
        "..##.......\n",
        "#...#...#..\n",
        ".#....#..#.\n",
        "..#.#...#.#\n",
        ".#...##..#.\n",
        "..#.##.....\n",
        ".#.#.#....#\n",
        ".#........#\n",
        "#.##...#...\n",
        "#...##....#\n",
        ".#..#...#.#"
    );

    #[test]
    fn test_part_one() {
        let map = INPUT_TEST
            .lines()
            .map(|line| line.chars().collect())
            .collect::<Vec<Vec<_>>>();
        assert_eq!(trees_encountered_product(&map, &SLOPES[1..2]), 7);
    }

    #[test]
    fn test_part_two() {
        let map = INPUT_TEST
            .lines()
            .map(|line| line.chars().collect())
            .collect::<Vec<Vec<_>>>();
        assert_eq!(trees_encountered_product(&map, SLOPES), 336);
    }
}
