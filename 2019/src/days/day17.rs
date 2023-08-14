use crate::intcode::Program;
use std::collections::{HashMap, VecDeque};
use std::fs;

fn parse_map(map: &Vec<i64>) -> Vec<Vec<char>> {
    map.iter()
        .map(|&x| x as u8 as char)
        .collect::<String>()
        .trim()
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect::<Vec<Vec<char>>>()
}

fn find_start(map: &Vec<Vec<char>>) -> Option<(usize, usize)> {
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
            count.to_string()
        ));
        prev = path[i - 1];
        count = 1;
    }
    compact.push(format!(
        "{},{}",
        get_turn(*path.last().unwrap(), prev).unwrap(),
        count.to_string(),
    ));
    compact
}

fn sum_alignment_parameters(map: &Vec<Vec<char>>) -> usize {
    let mut sum = 0;
    for y in 1..map.len() - 1 {
        for x in 1..map[y].len() - 1 {
            let units = vec![
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

fn pattern_freq(path: &Vec<String>) -> HashMap<&[String], u32> {
    let mut patterns = HashMap::new();
    for i in 0..path.len() {
        for l in 0..=i {
            let curr = &path[l..=i];
            if let Some(_) = patterns.get(&curr) {
                continue;
            }
            let mut count = 1;
            for j in i + 1..path.len() - curr.len() {
                if *curr == path[j..j + curr.len()] {
                    count += 1;
                }
            }
            patterns.insert(curr, count);
        }
    }
    patterns
}

fn compression_indexes(path: &String, patterns: &Vec<String>) -> Option<Vec<usize>> {
    let mut deque = VecDeque::new();
    deque.push_back((0, vec![]));

    while deque.len() > 0 {
        let (s, indexes) = deque.pop_front().unwrap();
        if indexes.len() > 10 {
            continue;
        }

        for (i, p) in patterns.iter().enumerate() {
            let next_s = s + p.len();
            if next_s > path.len() {
                continue;
            }

            if *p == path[s..next_s] {
                let mut next_ids = indexes.clone();
                next_ids.push(i);

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

fn patterns_sorted(path: &Vec<String>) -> Vec<String> {
    let mut patterns = pattern_freq(&path).into_iter().collect::<Vec<_>>();
    patterns.sort_by_key(|x| (std::cmp::Reverse(x.1), x.0.len(), x.0));
    patterns
        .iter()
        .map(|x| x.0.join(","))
        .filter(|x| x.len() < 21)
        .collect()
}

fn pattern_combinations(path: &String, patterns: &Vec<String>) -> Vec<Vec<String>> {
    let mut combs = vec![];
    for base in patterns
        .iter()
        .filter(|x| path.starts_with(x.as_str()) || path.ends_with(x.as_str()))
    {
        combs.push(vec![base.to_string()]);
    }

    let mut next_combs = vec![];
    for _ in 0..2 {
        for b in &combs {
            for p in patterns {
                if !b.contains(p) {
                    let mut c = b.clone();
                    c.push(p.clone());
                    next_combs.push(c);
                }
            }
        }
        std::mem::swap(&mut next_combs, &mut combs);
        next_combs.clear();
    }

    combs
}

fn compressed_path(map: &Vec<Vec<char>>) -> Option<(String, Vec<String>)> {
    let path = trace_path(&map);
    let compact = compact_path(&path);
    let compact_str = compact.join(",");
    let patterns = patterns_sorted(&compact);
    let combinations = pattern_combinations(&compact_str, &patterns);
    println!("{}", combinations.len());
    for c in &combinations {
        if let Some(ids) = compression_indexes(&compact_str, &c) {
            return Some((
                ids.iter()
                    .map(|x| ((x + 65) as u8 as char).to_string())
                    .collect::<Vec<_>>()
                    .join(","),
                c.clone(),
            ));
        }
    }
    None
}

pub fn run() {
    println!("Day 17: Set and Forget");
    let file_path = "inputs/day17.txt";
    let input_raw =
        fs::read_to_string(file_path).expect(format!("Error reading file '{file_path}'").as_str());

    let mut prog = Program::new(&input_raw);

    let mut prog1 = prog.clone();
    prog1.run();
    let map = parse_map(&prog1.output);
    println!("Part One: {}", sum_alignment_parameters(&map));

    let compressed = compressed_path(&map).unwrap();
    let mut fn_main = compressed.0.chars().map(|x| x as i64).collect();
    let mut fn_a = compressed.1[0].chars().map(|x| x as i64).collect();
    let mut fn_b = compressed.1[1].chars().map(|x| x as i64).collect();
    let mut fn_c = compressed.1[2].chars().map(|x| x as i64).collect();

    prog.input.append(&mut fn_main);
    prog.input.push('\n' as i64);
    prog.input.append(&mut fn_a);
    prog.input.push('\n' as i64);
    prog.input.append(&mut fn_b);
    prog.input.push('\n' as i64);
    prog.input.append(&mut fn_c);
    prog.input.push('\n' as i64);
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
        let map = parse_map(&map_str.chars().map(|x| x as i64).collect());
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
        let map = parse_map(&map_str.chars().map(|x| x as i64).collect());
        let path = trace_path(&map);
        let compact = compact_path(&path);
        assert_eq!(
            compact.join(","),
            "R,8,R,8,R,4,R,4,R,8,L,6,L,2,R,4,R,4,R,8,R,8,R,8,L,6,L,2"
        );
    }
}

/*
R,8,L,4,R,4,R,10,R,8
R,8,L,4,R,4,R,10,R,8

L,12,L,12,R,8,R,8

R,10,R,4,R,4

L,12,L,12,R,8,R,8

R,10,R,4,R,4

L,12,L,12,R,8,R,8

R,10,R,4,R,4
R,10,R,4,R,4

R,8,L,4,R,4,R,10,R,8

#####

R,8,L,4,R,4,R,10,R,8
L,12,L,12,R,8,R,8
R,10,R,4,R,4
*/
