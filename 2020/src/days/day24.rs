use std::collections::HashMap;

#[derive(Debug)]
enum Dir {
    E,
    SE,
    SW,
    W,
    NW,
    NE,
}

impl TryFrom<&str> for Dir {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "e" => Ok(Self::E),
            "se" => Ok(Self::SE),
            "sw" => Ok(Self::SW),
            "w" => Ok(Self::W),
            "nw" => Ok(Self::NW),
            "ne" => Ok(Self::NE),
            _ => Err("Invalid Dir"),
        }
    }
}

fn parse_paths(input: &str) -> Vec<Vec<Dir>> {
    let mut paths = vec![];
    for line in input.lines() {
        let mut path = vec![];
        let mut start = 0;
        let mut end = 1;
        while end <= line.len() {
            match Dir::try_from(&line[start..end]) {
                Ok(dir) => {
                    path.push(dir);
                    start = end;
                    end += 1;
                }
                Err(_) => end += 1,
            }
        }
        paths.push(path);
    }
    paths
}

fn black_tiles_after_flip(paths: &[Vec<Dir>]) -> usize {
    let mut map: HashMap<(i32, i32), bool> = HashMap::new();

    for path in paths {
        let mut x = 0;
        let mut y = 0;
        for dir in path {
            let offset = y % 2;
            let (vx, vy) = match dir {
                Dir::E => (1, 0),
                Dir::SE if offset == 0 => (0, 1),
                Dir::SE => (1, 1),
                Dir::SW if offset == 0 => (-1, 1),
                Dir::SW => (0, 1),
                Dir::W => (-1, 0),
                Dir::NW if offset == 0 => (-1, -1),
                Dir::NW => (0, -1),
                Dir::NE if offset == 0 => (0, -1),
                Dir::NE => (1, -1),
            };
            x += vx;
            y += vy;
        }
        map.entry((x, y)).and_modify(|state| *state = !*state).or_insert(false);
    }

    map.values().filter(|state| !**state).count()
}

pub fn run() {
    let input_raw = crate::load_input(module_path!());
    let paths = parse_paths(&input_raw);
    println!("Day 24: Lobby Layout");
    println!("Part One: {}", black_tiles_after_flip(&paths));
    println!("Part Two: {}", "TODO");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = concat!(
        "sesenwnenenewseeswwswswwnenewsewsw\n",
        "neeenesenwnwwswnenewnwwsewnenwseswesw\n",
        "seswneswswsenwwnwse\n",
        "nwnwneseeswswnenewneswwnewseswneseene\n",
        "swweswneswnenwsewnwneneseenw\n",
        "eesenwseswswnenwswnwnwsewwnwsene\n",
        "sewnenenenesenwsewnenwwwse\n",
        "wenwwweseeeweswwwnwwe\n",
        "wsweesenenewnwwnwsenewsenwwsesesenwne\n",
        "neeswseenwwswnwswswnw\n",
        "nenwswwsewswnenenewsenwsenwnesesenew\n",
        "enewnwewneswsewnwswenweswnenwsenwsw\n",
        "sweneswneswneneenwnewenewwneswswnese\n",
        "swwesenesewenwneswnwwneseswwne\n",
        "enesenwswwswneneswsenwnewswseenwsese\n",
        "wnwnesenesenenwwnenwsewesewsesesew\n",
        "nenewswnwewswnenesenwnesewesw\n",
        "eneswnwswnwsenenwnwnwwseeswneewsenese\n",
        "neswnwewnwnwseenwseesewsenwsweewe\n",
        "wseweeenwnesenwwwswnew\n",
    );

    #[test]
    fn test_part_one() {
        let paths = parse_paths(INPUT_TEST);
        assert_eq!(black_tiles_after_flip(&paths), 10);
    }

    #[test]
    fn test_part_two() {}
}
