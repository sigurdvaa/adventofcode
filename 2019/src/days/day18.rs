use std::collections::{HashMap, HashSet, VecDeque};

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
    nodes: HashMap<char, usize>,
    adjacency: Vec<Vec<Adjacent>>,
}

impl Graph {
    fn new() -> Graph {
        Graph {
            nodes: HashMap::new(),
            adjacency: vec![],
        }
    }

    fn add(&mut self, node: char, adj: Vec<Adjacent>) -> Result<(), &'static str> {
        if self.nodes.get(&node).is_some() {
            return Err("Node already added");
        }
        self.nodes.insert(node, self.adjacency.len());
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

    fn push(&mut self, value: T) {
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

fn find_entrances(map: &[Vec<char>]) -> Vec<(usize, usize)> {
    let mut entrances = vec![];
    for (y, line) in map.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == '@' {
                entrances.push((x, y));
            }
        }
    }
    entrances
}

fn find_node(map: &[Vec<char>], node: &char) -> Option<(usize, usize)> {
    for (y, line) in map.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            match c {
                c if c == node => return Some((x, y)),
                _ => (),
            }
        }
    }
    None
}

fn find_adjacent_nodes(map: &[Vec<char>], start: &(usize, usize)) -> Vec<Adjacent> {
    let mut adj = Vec::new();
    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();

    seen.insert(*start);
    queue.push_back((0, *start, vec![]));

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
            seen.insert(next_pos);

            let mut next_doors = doors.clone();
            let next_steps = steps + 1;
            match map[next_pos.1][next_pos.0] {
                c if c.is_uppercase() => {
                    next_doors.push(c.to_lowercase().next().unwrap());
                }
                c if c.is_lowercase() => {
                    adj.push(Adjacent::new(c, next_steps, doors.clone()));
                    continue;
                }
                '#' => continue,
                _ => (),
            }
            queue.push_back((next_steps, next_pos, next_doors));
        }
    }
    adj
}

fn create_key_graph(map: &[Vec<char>], start: &(usize, usize)) -> Result<Graph, &'static str> {
    let mut graph = Graph::new();
    graph.add(map[start.1][start.0], find_adjacent_nodes(map, start))?;
    let mut i = 0;
    while i < graph.nodes.len() {
        for edge in graph.adjacency[i].clone() {
            if graph.nodes.get(&edge.to).is_some() {
                continue;
            }
            graph.add(
                edge.to,
                find_adjacent_nodes(map, &find_node(map, &edge.to).unwrap()),
            )?;
        }
        i += 1;
    }
    Ok(graph)
}

fn bfs_dist_collect_keys(map: &[Vec<char>]) -> Option<usize> {
    #[derive(PartialEq, Eq, Clone, Debug, PartialOrd, Ord)]
    struct State {
        dist: usize,
        nodes: Vec<char>,
        keys: Vec<char>,
    }

    let entrances = find_entrances(map);
    let mut graphs = vec![];
    for e in &entrances {
        graphs.push(create_key_graph(map, e).unwrap());
    }
    let num_keys = graphs.iter().map(|g| g.nodes.len()).sum::<usize>() - entrances.len();
    let mut minheap = MinHeap::new();
    let mut seen = HashMap::new();

    minheap.push(State {
        dist: 0,
        nodes: vec!['@'; entrances.len()],
        keys: vec![],
    });

    while let Some(curr) = minheap.pop() {
        if curr.keys.len() == num_keys {
            return Some(curr.dist);
        }
        for (graph_idx, node) in curr.nodes.iter().enumerate() {
            let key_graph = &graphs[graph_idx];
            let node_idx = key_graph.nodes.get(node).unwrap();
            for edge in &key_graph.adjacency[*node_idx] {
                if edge.doors.iter().all(|&x| curr.keys.contains(&x)) {
                    let edge_dist = curr.dist + edge.weight;
                    let mut edge_ids = curr.nodes.clone();
                    edge_ids[graph_idx] = edge.to;

                    let mut edge_keys = curr.keys.clone();
                    if !edge_keys.contains(&edge.to) {
                        edge_keys.push(edge.to);
                        edge_keys.sort();
                    }

                    let best_dist = seen
                        .entry((
                            edge_ids.iter().collect::<String>(),
                            edge_keys.iter().collect::<String>(),
                        ))
                        .or_insert(usize::MAX);
                    if edge_dist < *best_dist {
                        *best_dist = edge_dist;
                        minheap.push(State {
                            dist: edge_dist,
                            nodes: edge_ids,
                            keys: edge_keys,
                        });
                    }
                }
            }
        }
    }

    None
}

fn shortest_dist_collect_keys(map: &str) -> Option<usize> {
    let map = map
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    bfs_dist_collect_keys(&map)
}

fn shortest_dist_collect_keys_four_entrances(map: &str) -> Option<usize> {
    let mut map = map
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let entrance = find_node(&map, &'@').unwrap();

    map[entrance.1][entrance.0] = '#';
    map[entrance.1 - 1][entrance.0] = '#';
    map[entrance.1 + 1][entrance.0] = '#';
    map[entrance.1][entrance.0 - 1] = '#';
    map[entrance.1][entrance.0 + 1] = '#';

    map[entrance.1 - 1][entrance.0 - 1] = '@';
    map[entrance.1 - 1][entrance.0 + 1] = '@';
    map[entrance.1 + 1][entrance.0 - 1] = '@';
    map[entrance.1 + 1][entrance.0 + 1] = '@';

    bfs_dist_collect_keys(&map)
}

pub fn run() {
    println!("Day 18: Many-Worlds Interpretation");
    let input_raw = crate::load_input(module_path!());

    println!(
        "Part One: {}",
        shortest_dist_collect_keys(&input_raw).unwrap()
    );
    println!(
        "Part Two: {}",
        shortest_dist_collect_keys_four_entrances(&input_raw).unwrap()
    );
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
        assert_eq!(shortest_dist_collect_keys(map).unwrap(), 8);

        let map = "\
            ########################\n\
            #f.D.E.e.C.b.A.@.a.B.c.#\n\
            ######################.#\n\
            #d.....................#\n\
            ########################";
        assert_eq!(shortest_dist_collect_keys(map).unwrap(), 86);

        let map = "\
            ########################\n\
            #...............b.C.D.f#\n\
            #.######################\n\
            #.....@.a.B.c.d.A.e.F.g#\n\
            ########################";
        assert_eq!(shortest_dist_collect_keys(map).unwrap(), 132);

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
        assert_eq!(shortest_dist_collect_keys(map).unwrap(), 136);

        let map = "\
            ########################\n\
            #@..............ac.GI.b#\n\
            ###d#e#f################\n\
            ###A#B#C################\n\
            ###g#h#i################\n\
            ########################";
        assert_eq!(shortest_dist_collect_keys(map).unwrap(), 81);
    }

    #[test]
    fn test_part_two() {
        let map = "\
            #######\n\
            #a.#Cd#\n\
            #######\n\
            ###@###\n\
            #######\n\
            #cB#Ab#\n\
            #######";
        assert_eq!(shortest_dist_collect_keys_four_entrances(map).unwrap(), 8);

        let map = "\
            ###############\n\
            #d.ABC.#.....a#\n\
            ###############\n\
            #######@#######\n\
            ###############\n\
            #b.....#.....c#\n\
            ###############";
        assert_eq!(shortest_dist_collect_keys_four_entrances(map).unwrap(), 24);

        let map = "\
            #############\n\
            #DcBa.#.GhKl#\n\
            #.#######I###\n\
            #e#d##@##j#k#\n\
            ###C#######J#\n\
            #fEbA.#.FgHi#\n\
            #############";
        assert_eq!(shortest_dist_collect_keys_four_entrances(map).unwrap(), 32);

        let map = "\
            #############\n\
            #g#f.D#..h#l#\n\
            #F###e#E###.#\n\
            #dCba###BcIJ#\n\
            ######@######\n\
            #nK.L###G...#\n\
            #M###N#H###.#\n\
            #o#m..#i#jk.#\n\
            #############";
        assert_eq!(shortest_dist_collect_keys_four_entrances(map).unwrap(), 72);
    }
}
