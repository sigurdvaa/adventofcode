use std::collections::{HashSet, VecDeque};
use std::fs;

fn find_entrance_and_keys(map: &Vec<Vec<char>>) -> ((usize, usize), usize) {
    let mut entrance = (0, 0);
    let mut keys = 0;
    for (y, line) in map.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            match c {
                '@' => entrance = (x, y),
                _ if c.is_lowercase() => keys += 1,
                _ => (),
            }
        }
    }
    (entrance, keys)
}

fn steps_collect_keys(map: &str) -> Option<usize> {
    let map = map
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let (entrance, num_keys) = find_entrance_and_keys(&map);
    let mut queue = VecDeque::new();
    queue.push_back((0, entrance, vec![], HashSet::new()));
    while !queue.is_empty() {
        let (steps, pos, mut keys, mut seen) = queue.pop_front().unwrap();
        let state = (pos.clone(), keys.clone());
        println!("s: {}, pos: {:?}, keys: {:?}", steps, pos, keys);
        if seen.contains(&state) {
            continue;
        }
        seen.insert(state);
        match map[pos.1][pos.0] {
            c if c.is_uppercase() => {
                if !keys.contains(&c.to_lowercase().next().unwrap()) {
                    continue;
                }
            }
            c if c.is_lowercase() => {
                if !keys.contains(&c) {
                    keys.push(c);
                    if keys.len() >= num_keys {
                        return Some(steps);
                    }
                }
            }
            '.' | '@' => (),
            '#' => continue,
            _ => unreachable!(),
        }
        for i in 0..4 {
            let new_pos = match i {
                0 => (pos.0 + 1, pos.1),
                1 => (pos.0, pos.1 + 1),
                2 => (pos.0, pos.1 - 1),
                3 => (pos.0 - 1, pos.1),
                _ => unreachable!(),
            };
            queue.push_back((steps + 1, new_pos, keys.clone(), seen.clone()));
        }
    }
    None
}

pub fn run() {
    println!("Day 18: Many-Worlds Interpretation");
    let file_path = "inputs/day18.txt";
    let input_raw =
        fs::read_to_string(file_path).expect(format!("Error reading file '{file_path}'").as_str());

    //println!("Part One: {}", steps_collect_keys(&input_raw).unwrap());
    let map = "\
            ########################\n\
            #...............b.C.D.f#\n\
            #.######################\n\
            #.....@.a.B.c.d.A.e.F.g#\n\
            ########################";
    assert_eq!(steps_collect_keys(map).unwrap(), 132);
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

        /*
        let map = "\
            ########################\n\
            #@..............ac.GI.b#\n\
            ###d#e#f################\n\
            ###A#B#C################\n\
            ###g#h#i################\n\
            ########################";
        assert_eq!(steps_collect_keys(map).unwrap(), 81);
        */
    }

    #[test]
    fn test_part_two() {}
}
