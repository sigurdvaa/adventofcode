use std::collections::HashMap;

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

impl Dir {
    fn value(&self) -> (i32, i32) {
        match self {
            Self::E => (1, 0),
            Self::SE => (1, 1),
            Self::SW => (0, 1),
            Self::W => (-1, 0),
            Self::NW => (-1, -1),
            Self::NE => (0, -1),
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

fn flip_tiles(paths: &[Vec<Dir>]) -> HashMap<(i32, i32), bool> {
    let mut map: HashMap<(i32, i32), bool> = HashMap::new();
    for path in paths {
        let mut x = 0;
        let mut y = 0;
        for dir in path {
            let (vx, vy) = dir.value();
            x += vx;
            y += vy;
        }
        map.entry((x, y)).and_modify(|white| *white = !*white).or_insert(false);
    }
    map
}

fn living_art(tiles: &HashMap<(i32, i32), bool>, days: u32) -> HashMap<(i32, i32), (bool, u32)> {
    let directions =
        [Dir::E.value(), Dir::SE.value(), Dir::SW.value(), Dir::W.value(), Dir::NW.value(), Dir::NE.value()];
    // init state with only black tiles, track white side up and black neighbor count
    let mut state = HashMap::from_iter(
        tiles
            .iter()
            .filter(|(_tile, white)| !*white)
            .map(|(tile, white)| (*tile, (*white, 0))),
    );
    for _ in 0..days {
        let mut next_state = state.clone();

        // for all black tiles, update their neighbors black_neighbor count
        for tile in state.keys() {
            for dir in directions {
                let neighbor = (tile.0 + dir.0, tile.1 + dir.1);
                let tile = next_state.entry(neighbor).or_insert((true, 0));
                tile.1 += 1;
            }
        }

        // check tiles and update their state based on state and black neighbor count
        // keep only black tiles for next loop, reset neighbor count
        next_state.retain(|_, tile| {
            if tile.0 && tile.1 == 2 {
                tile.0 = false
            } else if !tile.0 && tile.1 != 1 && tile.1 != 2 {
                tile.0 = true
            };
            tile.1 = 0;
            !tile.0
        });

        state = next_state;
    }
    state
}

pub fn run() {
    let input_raw = crate::load_input(module_path!());
    let paths = parse_paths(&input_raw);
    let init_tiles = flip_tiles(&paths);
    let art_tiles = living_art(&init_tiles, 100);
    println!("Day 24: Lobby Layout");
    println!("Part One: {}", init_tiles.values().filter(|white| !**white).count());
    println!("Part Two: {}", art_tiles.values().filter(|white| !white.0).count());
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
        let tiles = flip_tiles(&paths);
        assert_eq!(tiles.values().filter(|white| !**white).count(), 10);
    }

    #[test]
    fn test_part_two() {
        let paths = parse_paths(INPUT_TEST);
        let init_tiles = flip_tiles(&paths);
        let art_tiles = living_art(&init_tiles, 100);
        assert_eq!(art_tiles.values().filter(|white| !white.0).count(), 2208);
    }
}
