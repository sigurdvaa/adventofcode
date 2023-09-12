use std::fs;

fn map_str_to_vec(map: &str) -> Vec<Vec<char>> {
    map.lines().map(|line| line.chars().collect()).collect()
}

fn find_start(map: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for y in 0..map.len() - 1 {
        for x in 0..map[y].len() - 1 {
            if map[y][x] == 'A' {
                if map[y][x] == map[y][x + 1] {
                    return Some((y, x + 2));
                }
                if map[y][x] == map[y + 1][x] {
                    return Some((y + 2, x));
                }
            }
        }
    }
    None
}

pub fn run() {
    println!("Day 20: Donut Maze");
    let file_path = "inputs/day20.txt";
    let _input_raw =
        fs::read_to_string(file_path).expect(format!("Error reading file '{file_path}'").as_str());

    let map = map_str_to_vec(_input_raw.as_str());
    let start = find_start(&map);
    println!("{:?}", start);
}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn test_part_one() {}

    #[test]
    fn test_part_two() {}
}
