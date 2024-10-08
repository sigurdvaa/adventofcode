use crate::intcode::Program;
use std::collections::{HashSet, VecDeque};

fn parse_map(map: &[i64]) -> Vec<Vec<char>> {
    map.iter()
        .map(|&x| x as u8 as char)
        .collect::<String>()
        .trim()
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect::<Vec<Vec<char>>>()
}

fn find_start(map: &[Vec<char>]) -> Option<(usize, usize)> {
    for (y, line) in map.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == '^' {
                return Some((x, y));
            }
        }
    }
    None
}

fn trace_path(map: &Vec<Vec<char>>) -> Vec<u32> {
    let mut path = vec![];
    let mut prev = find_start(map).unwrap();
    let mut curr = prev;
    let mut dir = 0;
    let mut turns = 0;
    while turns < 4 {
        let next = match dir {
            0 if curr.1 > 0 => (curr.0, curr.1 - 1),
            1 if curr.0 < map[0].len() - 1 => (curr.0 + 1, curr.1),
            2 if curr.1 < map.len() - 1 => (curr.0, curr.1 + 1),
            3 if curr.0 > 0 => (curr.0 - 1, curr.1),
            _ => curr,
        };
        if map[next.1][next.0] == '#' && next != prev && next != curr {
            path.push(dir);
            prev = curr;
            curr = next;
            turns = 0;
        } else {
            dir = (dir + 1) % 4;
            turns += 1;
        }
    }
    path
}

fn get_turn(next: u32, prev: u32) -> Option<char> {
    match (prev, next) {
        (3, 0) => Some('R'),
        (0, 3) => Some('L'),
        (a, b) if b < a => Some('L'),
        (a, b) if b > a => Some('R'),
        _ => None,
    }
}

fn compact_path(path: &Vec<u32>) -> Vec<String> {
    let mut compact = vec![];
    let mut count = 1;
    let mut prev = 0;
    for i in 1..path.len() {
        if path[i - 1] == path[i] {
            count += 1;
            continue;
        }
        compact.push(format!(
            "{},{}",
            get_turn(path[i - 1], prev).unwrap(),
            count
        ));
        prev = path[i - 1];
        count = 1;
    }
    compact.push(format!(
        "{},{}",
        get_turn(*path.last().unwrap(), prev).unwrap(),
        count,
    ));
    compact
}

fn sum_alignment_parameters(map: &Vec<Vec<char>>) -> usize {
    let mut sum = 0;
    for y in 1..map.len() - 1 {
        for x in 1..map[y].len() - 1 {
            let units = [
                map[y][x],
                map[y][x + 1],
                map[y][x - 1],
                map[y - 1][x],
                map[y + 1][x],
            ];
            if units.iter().all(|&c| c == '#') {
                sum += x * y;
            }
        }
    }
    sum
}

fn path_patterns(path: &Vec<String>) -> Vec<String> {
    let mut patterns = HashSet::new();
    for i in 0..path.len() {
        for l in 0..=i {
            patterns.insert(&path[l..=i]);
        }
    }
    patterns
        .iter()
        .map(|x| x.join(","))
        .filter(|x| x.len() < 21)
        .collect()
}

fn compression_indexes(path: &String, patterns: &[String]) -> Option<Vec<usize>> {
    let mut deque = VecDeque::new();
    deque.push_back((0, vec![]));

    while !deque.is_empty() {
        let (s, indexes) = deque.pop_front().unwrap();
        for (i, p) in patterns.iter().enumerate() {
            let next_s = s + p.len();
            if next_s > path.len() {
                continue;
            }

            if *p == path[s..next_s] {
                let mut next_ids = indexes.clone();
                next_ids.push(i);

                if next_ids.len() > 10 {
                    continue;
                }
                if next_s == path.len() {
                    return Some(next_ids);
                }
                if &path[next_s..=next_s] == "," {
                    deque.push_back((next_s + 1, next_ids));
                }
            }
        }
    }

    None
}

fn pattern_combinations(path: &str, patterns: &Vec<String>) -> Vec<Vec<String>> {
    let mut combs = vec![];
    for base in patterns
        .iter()
        .filter(|x| path.starts_with(x.as_str()) || path.ends_with(x.as_str()))
    {
        combs.push(vec![base.to_string()]);
    }

    let mut next_combs = vec![];
    for i1 in 0..patterns.len() - 1 {
        for i2 in i1 + 1..patterns.len() {
            for b in &combs {
                let p1 = &patterns[i1];
                let p2 = &patterns[i2];

                if p1 != &b[0] && p2 != &b[0] {
                    let mut next = b.clone();
                    next.push(p1.clone());
                    next.push(p2.clone());
                    next_combs.push(next);
                }
            }
        }
    }

    next_combs
}

fn compressed_path_funcs(map: &Vec<Vec<char>>) -> Option<Vec<String>> {
    let path = trace_path(map);
    let compact = compact_path(&path);
    let compact_str = compact.join(",");
    let patterns = path_patterns(&compact);
    let combinations = pattern_combinations(&compact_str, &patterns);
    for mut c in combinations {
        if let Some(ids) = compression_indexes(&compact_str, &c) {
            let main = ids
                .iter()
                .map(|x| ((x + 65) as u8 as char).to_string())
                .collect::<Vec<_>>()
                .join(",");
            c.insert(0, main);
            return Some(c.clone());
        }
    }
    None
}

pub fn run() {
    println!("Day 17: Set and Forget");
    let input_raw = crate::load_input(module_path!());
    let mut prog = Program::new(&input_raw);

    let mut prog1 = prog.clone();
    prog1.run();
    let map = parse_map(&prog1.output);
    println!("Part One: {}", sum_alignment_parameters(&map));

    let funcs = compressed_path_funcs(&map).unwrap();
    for f in funcs {
        prog.input
            .append(&mut f.chars().map(|x| x as i64).collect());
        prog.input.push('\n' as i64);
    }
    prog.input.push('n' as i64);
    prog.input.push('\n' as i64);
    prog.input.reverse();
    prog.intcode[0] = 2;
    prog.run();
    println!("Part Two: {}", prog.output.last().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let map_str = "\
            ..#..........\n\
            ..#..........\n\
            #######...###\n\
            #.#...#...#.#\n\
            #############\n\
            ..#...#...#..\n\
            ..#####...^..";
        let map = parse_map(&map_str.chars().map(|x| x as i64).collect::<Vec<_>>());
        assert_eq!(sum_alignment_parameters(&map), 76);
    }

    #[test]
    fn test_part_two() {
        let map_str = "\
            #######...#####\n\
            #.....#...#...#\n\
            #.....#...#...#\n\
            ......#...#...#\n\
            ......#...###.#\n\
            ......#.....#.#\n\
            ^########...#.#\n\
            ......#.#...#.#\n\
            ......#########\n\
            ........#...#..\n\
            ....#########..\n\
            ....#...#......\n\
            ....#...#......\n\
            ....#...#......\n\
            ....#####......";
        let map = parse_map(&map_str.chars().map(|x| x as i64).collect::<Vec<_>>());
        let path = trace_path(&map);
        let compact = compact_path(&path);
        assert_eq!(
            compact.join(","),
            "R,8,R,8,R,4,R,4,R,8,L,6,L,2,R,4,R,4,R,8,R,8,R,8,L,6,L,2"
        );
    }
}
