use std::collections::{HashSet, VecDeque};
use std::fs;

fn map_str_to_vec(map: &str) -> Vec<Vec<char>> {
    map.lines().map(|line| line.chars().collect()).collect()
}

fn find_portal(
    map: &Vec<Vec<char>>,
    pos: (usize, usize),
    dir: (usize, usize),
) -> Option<(usize, usize)> {
    if pos.1 > 1 && pos.0 > 1 {
        if map[pos.1 - dir.1][pos.0 - dir.0] == '.' {
            return Some((pos.0 - dir.0, pos.1 - dir.1));
        }
    }
    if pos.1 < map.len() - 2 && pos.0 < map[pos.1].len() - 2 {
        let x = pos.0 + (dir.0 * 2);
        let y = pos.1 + (dir.1 * 2);
        if map[y][x] == '.' {
            return Some((x, y));
        }
    }
    None
}

fn find_portals(map: &Vec<Vec<char>>) -> Vec<(String, (usize, usize))> {
    let dirs = [(0, 1), (1, 0)];
    let mut portals = vec![];
    for y in 0..map.len() - 1 {
        for x in 0..map[y].len() - 1 {
            if map[y][x].is_uppercase() {
                for dir in dirs {
                    if map[y + dir.1][x + dir.0].is_uppercase() {
                        let p = format!("{}{}", map[y][x], map[y + dir.1][x + dir.0]);
                        if let Some(pos) = find_portal(&map, (x, y), dir) {
                            portals.push((p, pos));
                        }
                    }
                }
            }
        }
    }
    portals
}

fn shortest_path(map: &Vec<Vec<char>>) -> Option<usize> {
    let portals = find_portals(map);
    let start = portals.iter().find(|(portal, _)| portal == "AA").unwrap().1;
    let end = portals.iter().find(|(portal, _)| portal == "ZZ").unwrap().1;
    let mut queue = VecDeque::new();
    queue.push_back((0, start, HashSet::new()));

    while !queue.is_empty() {
        let (steps, curr, mut seen) = queue.pop_front().unwrap();
        for dir in 0..4 {
            let next = match dir {
                0 if curr.0 > 0 => (curr.0 - 1, curr.1),
                1 if curr.1 > 0 => (curr.0, curr.1 - 1),
                2 if curr.0 < map[curr.1].len() - 1 => (curr.0 + 1, curr.1),
                3 if curr.1 < map.len() - 1 => (curr.0, curr.1 + 1),
                _ => unreachable!(),
            };

            if !seen.insert(next) {
                continue;
            }

            if next == end {
                return Some(steps + 1);
            }

            match map[next.1][next.0] {
                '.' => queue.push_back((steps + 1, next, seen.clone())),
                c if c.is_uppercase() => {
                    let portal = portals.iter().find(|(_, pos)| *pos == curr).unwrap();
                    if let Some(goto) = portals
                        .iter()
                        .find(|(name, pos)| *pos != curr && name == &portal.0)
                    {
                        queue.push_back((steps + 1, goto.1, seen.clone()));
                    }
                }
                _ => (),
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
            "             Z       "
        );
        let map = map_str_to_vec(input);
        assert_eq!(shortest_path(&map), Some(23));

        let input = concat!(
            "                   A               \n",
            "                   A               \n",
            "  #################.#############  \n",
            "  #.#...#...................#.#.#  \n",
            "  #.#.#.###.###.###.#########.#.#  \n",
            "  #.#.#.......#...#.....#.#.#...#  \n",
            "  #.#########.###.#####.#.#.###.#  \n",
            "  #.............#.#.....#.......#  \n",
            "  ###.###########.###.#####.#.#.#  \n",
            "  #.....#        A   C    #.#.#.#  \n",
            "  #######        S   P    #####.#  \n",
            "  #.#...#                 #......VT\n",
            "  #.#.#.#                 #.#####  \n",
            "  #...#.#               YN....#.#  \n",
            "  #.###.#                 #####.#  \n",
            "DI....#.#                 #.....#  \n",
            "  #####.#                 #.###.#  \n",
            "ZZ......#               QG....#..AS\n",
            "  ###.###                 #######  \n",
            "JO..#.#.#                 #.....#  \n",
            "  #.#.#.#                 ###.#.#  \n",
            "  #...#..DI             BU....#..LF\n",
            "  #####.#                 #.#####  \n",
            "YN......#               VT..#....QG\n",
            "  #.###.#                 #.###.#  \n",
            "  #.#...#                 #.....#  \n",
            "  ###.###    J L     J    #.#.###  \n",
            "  #.....#    O F     P    #.#...#  \n",
            "  #.###.#####.#.#####.#####.###.#  \n",
            "  #...#.#.#...#.....#.....#.#...#  \n",
            "  #.#####.###.###.#.#.#########.#  \n",
            "  #...#.#.....#...#.#.#.#.....#.#  \n",
            "  #.###.#####.###.###.#.#.#######  \n",
            "  #.#.........#...#.............#  \n",
            "  #########.###.###.#############  \n",
            "           B   J   C               \n",
            "           U   P   P               "
        );

        let map = map_str_to_vec(input);
        assert_eq!(shortest_path(&map), Some(58));
    }

    #[test]
    fn test_part_two() {}
}
