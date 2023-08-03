use crate::intcode::Program;
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

fn trace_path(map: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut path = vec![];
    let mut prev = find_start(map).unwrap();
    let mut dir = 0;
    let mut turns = 0;
    while turns < 2 {
        let next = match dir {
            0 if prev.1 > 0 => (prev.0, prev.1 - 1),
            1 if prev.0 < map[0].len() - 1 => (prev.0 + 1, prev.1),
            2 if prev.1 < map.len() - 1 => (prev.0, prev.1 + 1),
            3 if prev.0 > 0 => (prev.0 - 1, prev.1),
            _ => prev,
        };
        println!(
            "dir: {}, turns: {}, prev: {:?}, next: {:?}",
            dir, turns, prev, next
        );
        if next == prev {
            dir = (dir + 1) % 4;
            turns += 1;
            continue;
        }
        if map[next.1][next.0] == '#' {
            path.push(next);
            prev = next;
            turns = 0;
        } else {
            dir = (dir + 1) % 4;
            turns += 1;
        }
    }
    path
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
    //for l in map {
    //    println!("{}", l.iter().collect::<String>());
    //}
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
    assert_eq!(false, true);
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
        assert_eq!(false, true);
        let _strpath = "R,8,R,8,R,4,R,4,R,8,L,6,L,2,R,4,R,4,R,8,R,8,R,8,L,6,L,2";
    }
}
