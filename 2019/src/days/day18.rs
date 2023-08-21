use std::collections::{HashSet, VecDeque};
use std::fs;

#[derive(Debug, Clone)]
struct Adjacent {
    from: char,
    to: char,
    steps: usize,
    doors: Vec<char>,
}

impl Adjacent {
    fn new(from: char, to: char, steps: usize, doors: Vec<char>) -> Adjacent {
        Adjacent {
            steps,
            doors,
            from,
            to,
        }
    }
}

fn find_entrace_and_keys(map: &Vec<Vec<char>>) -> Vec<(char, (usize, usize))> {
    let mut nodes = vec![];
    for (y, line) in map.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            match c {
                c if c.is_lowercase() => nodes.push((*c, (x, y))),
                '@' => nodes.push((*c, (x, y))),
                _ => (),
            }
        }
    }
    nodes
}

fn find_adjacent_nodes(map: &Vec<Vec<char>>, pos: (usize, usize)) -> Vec<Adjacent> {
    let mut adj = Vec::new();
    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();
    let from = map[pos.1][pos.0];

    seen.insert(pos);
    queue.push_back((0, pos, vec![]));

    while !queue.is_empty() {
        let (steps, pos, doors) = queue.pop_front().unwrap();
        for i in 0..4 {
            let next_pos = match i {
                0 => (pos.0 + 1, pos.1),
                1 => (pos.0 - 1, pos.1),
                2 => (pos.0, pos.1 + 1),
                3 => (pos.0, pos.1 - 1),
                _ => unreachable!(),
            };

            if seen.contains(&next_pos) {
                continue;
            }
            seen.insert(next_pos.clone());

            let mut next_doors = doors.clone();
            let next_steps = steps + 1;
            match map[next_pos.1][next_pos.0] {
                c if c.is_uppercase() => {
                    next_doors.push(c);
                }
                c if c.is_lowercase() => {
                    adj.push(Adjacent::new(from, c, next_steps, next_doors));
                    continue;
                }
                '.' => (),
                _ => continue,
            }
            queue.push_back((next_steps, next_pos, next_doors));
        }
    }
    adj
}

fn create_key_graph(map: &str) -> Vec<Adjacent> {
    let map = map
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let nodes = find_entrace_and_keys(&map);
    let mut adj = vec![];
    for n in nodes {
        adj.extend(find_adjacent_nodes(&map, n.1));
    }
    adj
}

fn shortest_path_to_keys(map: &str) -> Option<usize> {
    let adj_keys = create_key_graph(map);

    // BFS or djikstra's

    Some(0)
}

pub fn run() {
    println!("Day 18: Many-Worlds Interpretation");
    let file_path = "inputs/day18.txt";
    let _input_raw =
        fs::read_to_string(file_path).expect(format!("Error reading file '{file_path}'").as_str());

    //println!("Part One: {}", shortest_path_to_keys(&input_raw).unwrap());

    /*
    println!(
        "Part Two: {}",
        shortest_path_to_keys_all_entrances(&input_raw).unwrap()
    );
    */
    // high 1828

    let map = "\
            #########\n\
            #b.A.@.a#\n\
            #########";
    assert_eq!(shortest_path_to_keys(map).unwrap(), 8);

    /*
    let map = "\
            #################\n\
            #i.G..c...e..H.p#\n\
            ########.########\n\
            #j.A..b...f..D.o#\n\
            ########@########\n\
            #k.E..a...g..B.n#\n\
            ########.########\n\
            #l.F..d...h..C.m#\n\
            #################";
    assert_eq!(shortest_path_to_keys(map).unwrap(), 136);
    */

    /*
    let map = "\
            #############\n\
            #g#f.D#..h#l#\n\
            #F###e#E###.#\n\
            #dCba@#@BcIJ#\n\
            #############\n\
            #nK.L@#@G...#\n\
            #M###N#H###.#\n\
            #o#m..#i#jk.#\n\
            #############";
    assert_eq!(shortest_path_to_keys_all_entrances(map).unwrap(), 72);
    */
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let map = "\
            #########\n\
            #b.A.@.a#\n\
            #########";
        assert_eq!(shortest_path_to_keys(map).unwrap(), 8);

        let map = "\
            ########################\n\
            #f.D.E.e.C.b.A.@.a.B.c.#\n\
            ######################.#\n\
            #d.....................#\n\
            ########################";
        assert_eq!(shortest_path_to_keys(map).unwrap(), 86);

        let map = "\
            ########################\n\
            #...............b.C.D.f#\n\
            #.######################\n\
            #.....@.a.B.c.d.A.e.F.g#\n\
            ########################";
        assert_eq!(shortest_path_to_keys(map).unwrap(), 132);

        let map = "\
            #################\n\
            #i.G..c...e..H.p#\n\
            ########.########\n\
            #j.A..b...f..D.o#\n\
            ########@########\n\
            #k.E..a...g..B.n#\n\
            ########.########\n\
            #l.F..d...h..C.m#\n\
            #################";
        assert_eq!(shortest_path_to_keys(map).unwrap(), 136);

        let map = "\
            ########################\n\
            #@..............ac.GI.b#\n\
            ###d#e#f################\n\
            ###A#B#C################\n\
            ###g#h#i################\n\
            ########################";
        assert_eq!(shortest_path_to_keys(map).unwrap(), 81);
    }

    #[test]
    fn test_part_two() {
        /*
        let map = "\
            #######\n\
            #a.#Cd#\n\
            ##@#@##\n\
            #######\n\
            ##@#@##\n\
            #cB#Ab#\n\
            #######";
        assert_eq!(shortest_path_to_keys_all_entrances(map).unwrap(), 8);

        let map = "\
            ###############\n\
            #d.ABC.#.....a#\n\
            ######@#@######\n\
            ###############\n\
            ######@#@######\n\
            #b.....#.....c#\n\
            ###############";
        assert_eq!(shortest_path_to_keys_all_entrances(map).unwrap(), 24);

        let map = "\
            #############\n\
            #DcBa.#.GhKl#\n\
            #.###@#@#I###\n\
            #e#d#####j#k#\n\
            ###C#@#@###J#\n\
            #fEbA.#.FgHi#\n\
            #############";
        assert_eq!(shortest_path_to_keys_all_entrances(map).unwrap(), 32);

        let map = "\
            #############\n\
            #g#f.D#..h#l#\n\
            #F###e#E###.#\n\
            #dCba@#@BcIJ#\n\
            #############\n\
            #nK.L@#@G...#\n\
            #M###N#H###.#\n\
            #o#m..#i#jk.#\n\
            #############";
        assert_eq!(shortest_path_to_keys_all_entrances(map).unwrap(), 72);
        */
    }
}
