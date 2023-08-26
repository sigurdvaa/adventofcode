use std::cmp::Ordering;
use std::collections::{HashSet, VecDeque};
use std::fs;

#[derive(Debug, Clone)]
struct Adjacent {
    to: char,
    weight: usize,
    doors: Vec<char>,
}

impl Adjacent {
    fn new(to: char, weight: usize, doors: Vec<char>) -> Adjacent {
        Adjacent { weight, doors, to }
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

    fn add(&mut self, node: char, adj: Vec<Adjacent>) -> Result<(), &'static str> {
        if self.nodes.contains(&node) {
            return Err("Node already added");
        }
        self.nodes.push(node);
        self.adjacency.push(adj);
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct MinHeap<T: Ord + Clone> {
    data: Vec<T>,
    length: usize,
}

impl<T: Ord + Clone> MinHeap<T> {
    fn new() -> Self {
        Self {
            data: Vec::new(),
            length: 0,
        }
    }

    fn parent(&self, id: usize) -> Option<usize> {
        if id == 0 {
            return None;
        }
        Some((id - 1) / 2)
    }

    fn left_child(&self, id: usize) -> usize {
        id * 2 + 1
    }

    fn right_child(&self, id: usize) -> usize {
        id * 2 + 2
    }

    fn heap_up(&mut self, mut id: usize) {
        while let Some(p) = self.parent(id) {
            if self.data[id] < self.data[p] {
                self.data.swap(id, p);
                id = p;
            } else {
                break;
            }
        }
    }

    fn heap_down(&mut self, mut id: usize) {
        loop {
            let left_id = self.left_child(id);
            let right_id = self.right_child(id);

            if right_id >= self.length {
                break;
            }

            if self.data[left_id] < self.data[right_id] && self.data[left_id] < self.data[id] {
                self.data.swap(id, left_id);
                id = left_id;
            } else if self.data[right_id] < self.data[id] {
                self.data.swap(id, right_id);
                id = right_id;
            } else {
                break;
            }
        }
    }

    fn add(&mut self, value: T) {
        if self.length == self.data.len() {
            self.data.push(value);
        } else {
            self.data[self.length] = value;
        }
        self.heap_up(self.length);
        self.length += 1;
    }

    fn pop(&mut self) -> Option<T> {
        if self.length == 0 {
            return None;
        }

        let val = self.data.first().cloned().unwrap();
        self.length -= 1;
        if self.length > 0 {
            self.data.swap(0, self.length);
            self.heap_down(0);
        }
        Some(val)
    }
}

impl<T: Ord + Clone> From<Vec<T>> for MinHeap<T> {
    fn from(data: Vec<T>) -> Self {
        let mut new = Self::new();
        for d in data {
            new.add(d);
        }
        new
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
                    adj.push(Adjacent::new(c, next_steps, doors.clone()));
                }
                '.' | '@' => (),
                _ => continue,
            }
            queue.push_back((next_steps, next_pos, next_doors));
        }
    }
    adj
}

fn create_key_graph(map: &str) -> Result<Graph, &'static str> {
    let map = map
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let nodes = find_entrace_and_keys(&map);
    let mut graph = Graph::new();
    for n in nodes {
        graph.add(n.0, find_adjacent_nodes(&map, n.1))?;
    }
    Ok(graph)
}

#[derive(PartialEq, Eq, PartialOrd, Clone, Debug)]
struct Dist {
    dist: usize,
    idx: usize,
    keys: Vec<char>,
}

impl Ord for Dist {
    fn cmp(&self, other: &Self) -> Ordering {
        self.dist.cmp(&other.dist)
    }
}

fn shortest_path_to_keys(map: &str) -> Option<usize> {
    let key_graph = create_key_graph(map).unwrap();
    let mut minheap = MinHeap::new();
    let mut seen = HashSet::new();

    let start_idx = key_graph.nodes.iter().position(|&c| c == '@').unwrap();
    minheap.add(Dist {
        dist: 0,
        idx: start_idx,
        keys: vec!['@'],
    });

    while let Some(curr) = minheap.pop() {
        if curr.keys.len() == key_graph.nodes.len() {
            return Some(curr.dist);
        }
        for edge in &key_graph.adjacency[curr.idx] {
            if !curr.keys.contains(&edge.to) && edge.doors.iter().all(|&x| curr.keys.contains(&x)) {
                let edge_idx = key_graph.nodes.iter().position(|&c| c == edge.to).unwrap();
                let edge_dist = curr.dist + edge.weight;
                let mut edge_keys = curr.keys.clone();
                edge_keys.push(edge.to);
                edge_keys.sort();
                if !seen.insert((edge_dist, edge_keys.clone())) {
                    continue;
                }
                minheap.add(Dist {
                    dist: edge_dist,
                    idx: edge_idx,
                    keys: edge_keys,
                });
            }
        }
    }

    None
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

    /*
        let map = "\
                #########\n\
                #b.A.@.a#\n\
                #########";
        assert_eq!(shortest_path_to_keys(map).unwrap(), 8);
    */

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
