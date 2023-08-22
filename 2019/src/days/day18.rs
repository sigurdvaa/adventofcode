use std::collections::{HashSet, VecDeque};
use std::fs;

#[derive(Debug, Clone)]
struct Adjacent {
    to: char,
    steps: usize,
    doors: Vec<char>,
}

impl Adjacent {
    fn new(to: char, steps: usize, doors: Vec<char>) -> Adjacent {
        Adjacent { steps, doors, to }
    }
}

#[derive(Debug, Clone)]
struct Graph {
    nodes: Vec<char>,
    adjacency: Vec<Vec<Adjacent>>,
}

impl Graph {
    fn new() -> Graph {
        Graph {
            nodes: vec![],
            adjacency: vec![],
        }
    }

    fn add(&mut self, node: char, adj: Vec<Adjacent>) {
        if !self.nodes.contains(&node) {
            self.nodes.push(node);
        }
        self.adjacency.push(adj);
    }
}

struct PriQueue<T> {
    data: Vec<T>,
    length: usize,
}

impl<T> PriQueue<T> {
    fn new() -> PriQueue<T> {
        PriQueue {
            data: Vec::new(),
            length: 0,
        }
    }

    fn heap_up(&mut self, id: usize) {}

    fn heap_down(&mut self, id: usize) {}

    fn insert(&mut self, value: T) {
        self.length += 1;
        if self.length == self.data.len() {
            self.data.push(value);
        } else {
            self.data[self.length] = value;
        }
        self.heap_up(self.data.len() - 1);
    }

    fn delete(&mut self) -> Option<T> {
        if self.length == 0 {
            return None;
        }
        if self.length == 1 {
            self.length -= 1;
            return Some(self.data[0]);
        }
        let value = self.data[0];
        self.length -= 1;
        self.data[0] = self.data[self.length];
        self.heap_down(0);
        Some(value)
    }

    fn parent(id: usize) -> usize {
        if id == 0 {
            return 0;
        }
        (id - 1) / 2
    }

    fn left_child(id: usize) -> usize {
        id * 2 + 1
    }

    fn right_child(id: usize) -> usize {
        id * 2 + 2
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
                    next_doors.push(c.to_lowercase().next().unwrap());
                }
                c if c.is_lowercase() => {
                    adj.push(Adjacent::new(c, next_steps, next_doors));
                    continue;
                }
                '.' | '@' => (),
                _ => continue,
            }
            queue.push_back((next_steps, next_pos, next_doors));
        }
    }
    adj
}

fn create_key_graph(map: &str) -> Graph {
    let map = map
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let nodes = find_entrace_and_keys(&map);
    let mut graph = Graph::new();
    for n in nodes {
        graph.add(n.0, find_adjacent_nodes(&map, n.1));
    }
    graph
}

fn shortest_path_to_keys(map: &str) -> Option<usize> {
    let key_graph = create_key_graph(map);
    for a in key_graph.adjacency.iter() {
        println!("{:?}", a);
    }
    // djikstra's (with priority queue)
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
