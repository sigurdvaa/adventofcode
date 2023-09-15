use std::fs;

fn map_str_to_vec(map: &str) -> Vec<Vec<char>> {
    map.lines().map(|line| line.chars().collect()).collect()
}

fn find_start(map: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for y in 0..map.len() - 1 {
        for x in 0..map[y].len() - 1 {
            if map[y][x] == 'A' {
                if map[y][x] == map[y][x + 1] {
                    return Some((x + 2, y));
                }
                if map[y][x] == map[y + 1][x] {
                    return Some((x, y + 2));
                }
            }
        }
    }
    None
}

fn shortest_path(map: &Vec<Vec<char>>) -> Option<usize> {
    let _start = find_start(map)?;
    None
}

pub fn run() {
    println!("Day 20: Donut Maze");
    let file_path = "inputs/day20.txt";
    let _input_raw =
        fs::read_to_string(file_path).expect(format!("Error reading file '{file_path}'").as_str());

    let map = map_str_to_vec(_input_raw.as_str());
    println!("Part One: {}", shortest_path(&map).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = concat!(
            "         A           \n",
            "         A           \n",
            "  #######.#########  \n",
            "  #######.........#  \n",
            "  #######.#######.#  \n",
            "  #######.#######.#  \n",
            "  #######.#######.#  \n",
            "  #####  B    ###.#  \n",
            "BC...##  C    ###.#  \n",
            "  ##.##       ###.#  \n",
            "  ##...DE  F  ###.#  \n",
            "  #####    G  ###.#  \n",
            "  #########.#####.#  \n",
            "DE..#######...###.#  \n",
            "  #.#########.###.#  \n",
            "FG..#########.....#  \n",
            "  ###########.#####  \n",
            "             Z       \n",
            "             Z       ");
        let map = map_str_to_vec(input);
        assert_eq!(find_start(&map), Some((9, 2)));
        assert_eq!(shortest_path(&map), Some(23));
    }

    #[test]
    fn test_part_two() {}
}
