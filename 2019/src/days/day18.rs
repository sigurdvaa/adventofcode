use core::cell::RefCell;
use std::collections::{HashSet, VecDeque};
use std::fs;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn new(x: usize, y: usize) -> Pos {
        Pos { x, y }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Node {
    name: char,
    pos: Pos,
    adjacent: Vec<(RefCell<Node>, usize, Vec<char>)>,
}

impl Node {
    fn new(name: char, x: usize, y: usize) -> Node {
        Node {
            name,
            pos: Pos { x, y },
            adjacent: Vec::new(),
        }
    }

    fn add(mut self, adj: Node, steps: usize, doors: Vec<char>) {
        self.adjacent.push((RefCell::new(adj), steps, doors));
    }
}

fn create_nodes(map: &Vec<Vec<char>>) -> Vec<Node> {
    let mut nodes = vec![];
    for (y, line) in map.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            match c {
                c if c.is_lowercase() => nodes.push(Node::new(*c, x, y)),
                '@' => nodes.push(Node::new(*c, x, y)),
                _ => (),
            }
        }
    }
    nodes
}

fn find_adjacent_nodes(map: &Vec<Vec<char>>, pos: Pos) -> Vec<(char, usize, Vec<char>)> {
    let mut nodes = Vec::new();
    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((0, pos, vec![]));

    while !queue.is_empty() {
        let (steps, pos, doors) = queue.pop_front().unwrap();
        for i in 0..4 {
            let next_pos = match i {
                0 => Pos::new(pos.x + 1, pos.y),
                1 => Pos::new(pos.x - 1, pos.y),
                2 => Pos::new(pos.x, pos.y + 1),
                3 => Pos::new(pos.x, pos.y - 1),
                _ => unreachable!(),
            };

            if seen.contains(&next_pos) {
                continue;
            }
            seen.insert(next_pos.clone());

            let mut next_doors = doors.clone();
            let next_steps = steps + 1;
            match map[next_pos.y][next_pos.x] {
                c if c.is_uppercase() => {
                    next_doors.push(c);
                }
                c if c.is_lowercase() => {
                    nodes.push((c, next_steps, next_doors));
                    continue;
                }

                '@' => nodes.push(('@', next_steps, next_doors.clone())),
                '.' => (),
                _ => continue,
            }
            queue.push_back((next_steps, next_pos, next_doors));
        }
    }

    nodes
}

fn create_key_graph(map: &str) -> () {
    let map = map
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let nodes = create_nodes(&map);
    for i1 in 0..nodes.len() {
        let adjacent = find_adjacent_nodes(&map, nodes[i1].pos);
        'outer: for (c, steps, doors) in adjacent.iter() {
            for i2 in 0..nodes.len() {
                if nodes[i2].name == *c {
                    nodes[i1].add(nodes[i2], *steps, *doors);
                    break 'outer;
                }
            }
        }
        println!("{:?}", adjacent);
    }
}

fn steps_collect_keys(map: &str) -> Option<usize> {
    // use bfs to precalc graph between all keys (this includes entrance), cost is steps
    let _adj_keys = create_key_graph(map);

    // then djikstra to find shortest path based on cost

    Some(0)
}

pub fn run() {
    println!("Day 18: Many-Worlds Interpretation");
    let file_path = "inputs/day18.txt";
    let input_raw =
        fs::read_to_string(file_path).expect(format!("Error reading file '{file_path}'").as_str());

    //println!("Part One: {}", steps_collect_keys(&input_raw).unwrap());

    /*
    println!(
        "Part Two: {}",
        steps_collect_keys_all_entrances(&input_raw).unwrap()
    );
    */
    // high 1828

    let map = "\
        #########\n\
        #b.A.@.a#\n\
        #########";
    assert_eq!(steps_collect_keys(map).unwrap(), 8);

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
    assert_eq!(steps_collect_keys(map).unwrap(), 136);
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
    assert_eq!(steps_collect_keys_all_entrances(map).unwrap(), 72);
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
        assert_eq!(steps_collect_keys(map).unwrap(), 8);

        let map = "\
            ########################\n\
            #f.D.E.e.C.b.A.@.a.B.c.#\n\
            ######################.#\n\
            #d.....................#\n\
            ########################";
        assert_eq!(steps_collect_keys(map).unwrap(), 86);

        let map = "\
            ########################\n\
            #...............b.C.D.f#\n\
            #.######################\n\
            #.....@.a.B.c.d.A.e.F.g#\n\
            ########################";
        assert_eq!(steps_collect_keys(map).unwrap(), 132);

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
        assert_eq!(steps_collect_keys(map).unwrap(), 136);

        let map = "\
            ########################\n\
            #@..............ac.GI.b#\n\
            ###d#e#f################\n\
            ###A#B#C################\n\
            ###g#h#i################\n\
            ########################";
        assert_eq!(steps_collect_keys(map).unwrap(), 81);
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
        assert_eq!(steps_collect_keys_all_entrances(map).unwrap(), 8);

        let map = "\
            ###############\n\
            #d.ABC.#.....a#\n\
            ######@#@######\n\
            ###############\n\
            ######@#@######\n\
            #b.....#.....c#\n\
            ###############";
        assert_eq!(steps_collect_keys_all_entrances(map).unwrap(), 24);

        let map = "\
            #############\n\
            #DcBa.#.GhKl#\n\
            #.###@#@#I###\n\
            #e#d#####j#k#\n\
            ###C#@#@###J#\n\
            #fEbA.#.FgHi#\n\
            #############";
        assert_eq!(steps_collect_keys_all_entrances(map).unwrap(), 32);

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
        assert_eq!(steps_collect_keys_all_entrances(map).unwrap(), 72);
        */
    }
}
