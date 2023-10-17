use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fs;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Portal {
    name: String,
    pos: (usize, usize),
    is_outer: bool,
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

fn get_portal_adj_list(map: &Vec<Vec<char>>) -> HashMap<Portal, Vec<(usize, Portal)>> {
    let mut adj = HashMap::new();
    let portals = find_portals(map);
    for portal in portals.iter() {
        let portal_adj = adj.entry(portal.clone()).or_insert(Vec::new());

        let mut seen = HashSet::new();
        let mut queue = VecDeque::new();
        seen.insert(portal.pos);
        queue.push_back((0, portal.pos));

        while let Some((weight, pos)) = queue.pop_front() {
            for dir in 0..4 {
                let next = match dir {
                    0 if pos.0 > 0 => (pos.0 - 1, pos.1),
                    1 if pos.1 > 0 => (pos.0, pos.1 - 1),
                    2 if pos.0 < map[pos.1].len() - 1 => (pos.0 + 1, pos.1),
                    3 if pos.1 < map.len() - 1 => (pos.0, pos.1 + 1),
                    _ => unreachable!(),
                };

                if !seen.insert(next) {
                    continue;
                }

                if map[next.1][next.0] == '.' {
                    queue.push_back((weight + 1, next));
                }

                if let Some(other) = portals.iter().find(|p| p.pos == next) {
                    portal_adj.push((weight + 1, other.clone()));
                }
            }
        }
    }
    adj
}

fn shortest_path(map: &Vec<Vec<char>>, with_levels: bool) -> Option<usize> {
    #[derive(PartialEq, Eq, Clone, Debug)]
    struct State {
        weight: usize,
        steps: usize,
        level: usize,
        portal: Portal,
    }

    impl Ord for State {
        fn cmp(&self, other: &Self) -> Ordering {
            other.weight.cmp(&self.weight)
        }
    }
    impl PartialOrd for State {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(other.weight.cmp(&self.weight))
        }
    }

    let start = "AA";
    let end = "ZZ";
    let graph = get_portal_adj_list(map);
    let start_p = graph.keys().find(|portal| portal.name == start).unwrap();
    let mut queue = BinaryHeap::from([State {
        weight: 0,
        steps: 0,
        level: 0,
        portal: start_p.clone(),
    }]);

    while let Some(state) = queue.pop() {
        if state.portal.name == end && state.level == 0 {
            return Some(state.steps);
        }
        if let Some(adj) = graph.get(&state.portal) {
            for (weight, portal) in adj {
                if portal.name == end && state.level == 0 {
                    queue.push(State {
                        weight: state.weight + weight,
                        steps: state.steps + weight,
                        level: state.level,
                        portal: portal.clone(),
                    });
                    continue;
                }

                let mut next_level = 0;
                if with_levels {
                    if portal.is_outer && state.level == 0 && portal.name != end {
                        continue;
                    }
                    next_level = match portal.is_outer {
                        true => state.level - 1,
                        false => state.level + 1,
                    };
                }

                let next_weight = state.weight + ((next_level + 1) * weight);
                if let Some(next_portal) = graph
                    .keys()
                    .find(|other| other.name == portal.name && other.pos != portal.pos)
                {
                    queue.push(State {
                        weight: next_weight,
                        steps: state.steps + weight + 1,
                        level: next_level,
                        portal: next_portal.clone(),
                    });
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
    println!("Part One: {}", shortest_path(&map, false).unwrap());
    println!("Part Two: {}", shortest_path(&map, true).unwrap());
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
        assert_eq!(shortest_path(&map, false), Some(23));

        let map = map_str_to_vec(TESTINPUT2);
        assert_eq!(shortest_path(&map, false), Some(58));
    }

    #[test]
    fn test_part_two() {
        let map = map_str_to_vec(TESTINPUT1);
        assert_eq!(shortest_path(&map, true), Some(26));

        let map = map_str_to_vec(TESTINPUT3);
        assert_eq!(shortest_path(&map, true), Some(396));
    }
}
