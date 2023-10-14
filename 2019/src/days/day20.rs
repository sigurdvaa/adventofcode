use std::collections::{HashSet, VecDeque};
use std::fs;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Portal {
    name: String,
    pos: (usize, usize),
    is_outer: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct State {
    steps: usize,
    pos: (usize, usize),
    level: usize,
    seen_portals: String,
    jumps: usize,
}

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

fn find_portals(map: &Vec<Vec<char>>) -> Vec<Portal> {
    let dirs = [(0, 1), (1, 0)];
    let mut portals = vec![];
    for y in 0..map.len() - 1 {
        for x in 0..map[y].len() - 1 {
            if map[y][x].is_uppercase() {
                for dir in dirs {
                    if map[y + dir.1][x + dir.0].is_uppercase() {
                        let p = format!("{}{}", map[y][x], map[y + dir.1][x + dir.0]);
                        if let Some(pos) = find_portal(&map, (x, y), dir) {
                            let is_outer = pos.0 == 2
                                || pos.1 == 2
                                || pos.0 == map[pos.1].len() - 3
                                || pos.1 == map.len() - 3;
                            portals.push(Portal {
                                name: p,
                                pos,
                                is_outer,
                            });
                        }
                    }
                }
            }
        }
    }
    portals
}

fn shortest_path_recursive(map: &Vec<Vec<char>>) -> Option<usize> {
    let portals = find_portals(map);
    let start_portal = portals.iter().find(|portal| portal.name == "AA").unwrap();
    let end_portal = portals.iter().find(|portal| portal.name == "ZZ").unwrap();
    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(State {
        steps: 0,
        pos: start_portal.pos,
        level: 0,
        jumps: 0,
        seen_portals: start_portal.name.clone() + "0",
    });

    while !queue.is_empty() {
        let curr = queue.pop_front().unwrap();
        println!("{:?}", (curr.steps, curr.jumps, portals.len(), queue.len()));
        for dir in 0..4 {
            let next = match dir {
                0 if curr.pos.0 > 0 => (curr.pos.0 - 1, curr.pos.1),
                1 if curr.pos.1 > 0 => (curr.pos.0, curr.pos.1 - 1),
                2 if curr.pos.0 < map[curr.pos.1].len() - 1 => (curr.pos.0 + 1, curr.pos.1),
                3 if curr.pos.1 < map.len() - 1 => (curr.pos.0, curr.pos.1 + 1),
                _ => unreachable!(),
            };

            if !seen.insert((next, curr.seen_portals.clone())) {
                continue;
            }

            if next == end_portal.pos && curr.level == 0 {
                return Some(curr.steps + 1);
            }

            match map[next.1][next.0] {
                '.' => queue.push_back(State {
                    steps: curr.steps + 1,
                    pos: next,
                    level: curr.level,
                    jumps: curr.jumps,
                    seen_portals: curr.seen_portals.clone(),
                }),
                c if c.is_uppercase() => {
                    let portal = portals
                        .iter()
                        .find(|portal| portal.pos == curr.pos)
                        .unwrap();

                    if curr.level == 0 && portal.is_outer {
                        continue;
                    }

                    let next_level = match portal.is_outer {
                        true => curr.level - 1,
                        false => curr.level + 1,
                    };

                    if curr.jumps > portals.len() * 2 {
                        continue;
                    }

                    let mut port = portal.name.clone();
                    port.push_str(&next_level.to_string());
                    if curr.seen_portals[curr.seen_portals.len() - port.len()..] == port {
                        continue;
                    }

                    let mut next_portals = curr.seen_portals.clone();
                    next_portals.push_str(&portal.name);
                    next_portals.push_str(&curr.level.to_string());

                    if let Some(goto) = portals
                        .iter()
                        .find(|other| other.pos != portal.pos && other.name == portal.name)
                    {
                        queue.push_back(State {
                            steps: curr.steps + 1,
                            pos: goto.pos,
                            level: next_level,
                            jumps: curr.jumps + 1,
                            seen_portals: next_portals,
                        });
                    }
                }
                _ => (),
            }
        }
    }

    None
}

fn shortest_path(map: &Vec<Vec<char>>) -> Option<usize> {
    let portals = find_portals(map);
    let start = portals
        .iter()
        .find(|portal| portal.name == "AA")
        .unwrap()
        .pos;
    let end = portals
        .iter()
        .find(|portal| portal.name == "ZZ")
        .unwrap()
        .pos;
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
                    let portal = portals.iter().find(|portal| portal.pos == curr).unwrap();
                    if let Some(goto) = portals
                        .iter()
                        .find(|other| other.pos != curr && other.name == portal.name)
                    {
                        queue.push_back((steps + 1, goto.pos, seen.clone()));
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
    //println!("Part Two: {}", shortest_path_recursive(&map).unwrap());

    const TESTINPUT2: &'static str = concat!(
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
    const TESTINPUT3: &'static str = concat!(
        "             Z L X W       C                 \n",
        "             Z P Q B       K                 \n",
        "  ###########.#.#.#.#######.###############  \n",
        "  #...#.......#.#.......#.#.......#.#.#...#  \n",
        "  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###  \n",
        "  #.#...#.#.#...#.#.#...#...#...#.#.......#  \n",
        "  #.###.#######.###.###.#.###.###.#.#######  \n",
        "  #...#.......#.#...#...#.............#...#  \n",
        "  #.#########.#######.#.#######.#######.###  \n",
        "  #...#.#    F       R I       Z    #.#.#.#  \n",
        "  #.###.#    D       E C       H    #.#.#.#  \n",
        "  #.#...#                           #...#.#  \n",
        "  #.###.#                           #.###.#  \n",
        "  #.#....OA                       WB..#.#..ZH\n",
        "  #.###.#                           #.#.#.#  \n",
        "CJ......#                           #.....#  \n",
        "  #######                           #######  \n",
        "  #.#....CK                         #......IC\n",
        "  #.###.#                           #.###.#  \n",
        "  #.....#                           #...#.#  \n",
        "  ###.###                           #.#.#.#  \n",
        "XF....#.#                         RF..#.#.#  \n",
        "  #####.#                           #######  \n",
        "  #......CJ                       NM..#...#  \n",
        "  ###.#.#                           #.###.#  \n",
        "RE....#.#                           #......RF\n",
        "  ###.###        X   X       L      #.#.#.#  \n",
        "  #.....#        F   Q       P      #.#.#.#  \n",
        "  ###.###########.###.#######.#########.###  \n",
        "  #.....#...#.....#.......#...#.....#.#...#  \n",
        "  #####.#.###.#######.#######.###.###.#.#.#  \n",
        "  #.......#.......#.#.#.#.#...#...#...#.#.#  \n",
        "  #####.###.#####.#.#.#.#.###.###.#.###.###  \n",
        "  #.......#.....#.#...#...............#...#  \n",
        "  #############.#.#.###.###################  \n",
        "               A O F   N                     \n",
        "               A A D   M                     "
    );

    //let map = map_str_to_vec(TESTINPUT2);
    //assert_eq!(shortest_path_recursive(&map), None);
    let map = map_str_to_vec(TESTINPUT3);
    assert_eq!(shortest_path_recursive(&map), Some(396));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTINPUT1: &'static str = concat!(
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

    const TESTINPUT2: &'static str = concat!(
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

    const TESTINPUT3: &'static str = concat!(
        "             Z L X W       C                 \n",
        "             Z P Q B       K                 \n",
        "  ###########.#.#.#.#######.###############  \n",
        "  #...#.......#.#.......#.#.......#.#.#...#  \n",
        "  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###  \n",
        "  #.#...#.#.#...#.#.#...#...#...#.#.......#  \n",
        "  #.###.#######.###.###.#.###.###.#.#######  \n",
        "  #...#.......#.#...#...#.............#...#  \n",
        "  #.#########.#######.#.#######.#######.###  \n",
        "  #...#.#    F       R I       Z    #.#.#.#  \n",
        "  #.###.#    D       E C       H    #.#.#.#  \n",
        "  #.#...#                           #...#.#  \n",
        "  #.###.#                           #.###.#  \n",
        "  #.#....OA                       WB..#.#..ZH\n",
        "  #.###.#                           #.#.#.#  \n",
        "CJ......#                           #.....#  \n",
        "  #######                           #######  \n",
        "  #.#....CK                         #......IC\n",
        "  #.###.#                           #.###.#  \n",
        "  #.....#                           #...#.#  \n",
        "  ###.###                           #.#.#.#  \n",
        "XF....#.#                         RF..#.#.#  \n",
        "  #####.#                           #######  \n",
        "  #......CJ                       NM..#...#  \n",
        "  ###.#.#                           #.###.#  \n",
        "RE....#.#                           #......RF\n",
        "  ###.###        X   X       L      #.#.#.#  \n",
        "  #.....#        F   Q       P      #.#.#.#  \n",
        "  ###.###########.###.#######.#########.###  \n",
        "  #.....#...#.....#.......#...#.....#.#...#  \n",
        "  #####.#.###.#######.#######.###.###.#.#.#  \n",
        "  #.......#.......#.#.#.#.#...#...#...#.#.#  \n",
        "  #####.###.#####.#.#.#.#.###.###.#.###.###  \n",
        "  #.......#.....#.#...#...............#...#  \n",
        "  #############.#.#.###.###################  \n",
        "               A O F   N                     \n",
        "               A A D   M                     "
    );

    #[test]
    fn test_part_one() {
        let map = map_str_to_vec(TESTINPUT1);
        assert_eq!(shortest_path(&map), Some(23));

        let map = map_str_to_vec(TESTINPUT2);
        assert_eq!(shortest_path(&map), Some(58));
    }

    #[test]
    fn test_part_two() {
        let map = map_str_to_vec(TESTINPUT1);
        assert_eq!(shortest_path_recursive(&map), Some(26));

        let map = map_str_to_vec(TESTINPUT2);
        assert_eq!(shortest_path_recursive(&map), None);

        let map = map_str_to_vec(TESTINPUT3);
        assert_eq!(shortest_path_recursive(&map), Some(396));
    }
}
