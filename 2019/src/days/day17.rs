use crate::intcode::Program;
use std::collections::HashMap;
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
    match next {
        _ if prev == 3 && next == 0 => Some('R'),
        _ if next == 3 && prev == 0 => Some('L'),
        _ if next < prev => Some('L'),
        _ if next > prev => Some('R'),
        _ => None,
    }
}

fn compress_path(path: &Vec<u32>) -> Vec<(char, String)> {
    let mut compressed = vec![];
    let mut count = 1;
    let mut prev = 0;
    for i in 1..path.len() {
        if path[i - 1] == path[i] {
            count += 1;
            continue;
        }
        compressed.push((get_turn(path[i - 1], prev).unwrap(), count.to_string()));
        prev = path[i - 1];
        count = 1;
    }
    compressed.push((
        get_turn(*path.last().unwrap(), prev).unwrap(),
        count.to_string(),
    ));
    compressed
}

fn compressed_to_string(compressed: &Vec<(char, String)>) -> String {
    compressed
        .iter()
        .map(|(c, n)| format!("{c},{n}"))
        .collect::<Vec<_>>()
        .join(",")
}

fn count_patterns(path: &Vec<(char, String)>) -> HashMap<&[(char, String)], u32> {
    let mut patterns = HashMap::new();
    for i in 0..path.len() {
        for l in 0..=i {
            let curr = &path[l..=i];
            if let Some(_) = patterns.get(curr) {
                continue;
            }
            let mut count = 1;
            for j in i + 1..path.len() {
                if *curr == path[l..=j] {
                    count += 1;
                }
            }
            patterns.insert(curr.clone(), count);
        }
    }
    patterns
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

pub fn run() {
    println!("Day 17: Set and Forget");
    let file_path = "inputs/day17.txt";
    let input_raw =
        fs::read_to_string(file_path).expect(format!("Error reading file '{file_path}'").as_str());

    let prog = Program::new(&input_raw);

    let mut prog1 = prog.clone();
    prog1.run();
    let map = parse_map(&prog1.output);
    println!("Part One: {}", sum_alignment_parameters(&map));

    let path = trace_path(&map);
    let _compressed = compress_path(&path);

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
    let compressed = compress_path(&path);
    let _cmp_str = compressed_to_string(&compressed);
    let patterns = count_patterns(&compressed);
    println!("{:?}", patterns);
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
        let compressed = compress_path(&path);
        assert_eq!(
            compressed_to_string(&compressed),
            "R,8,R,8,R,4,R,4,R,8,L,6,L,2,R,4,R,4,R,8,R,8,R,8,L,6,L,2"
        );
    }
}
