#[derive(Debug)]
struct Tile {
    id: u32,
    image: Vec<String>,
    edges: Vec<String>,
}

fn parse_tiles(input: &str) -> Vec<Tile> {
    let mut tiles = vec![];
    let mut lines = input.lines();

    while let Some(line) = lines.next() {
        let id = line.split("Tile ").nth(1).unwrap();
        let id = id[..id.len() - 1].parse().unwrap();
        let mut image = vec![];
        for row in lines.by_ref() {
            if row.is_empty() {
                break;
            }
            image.push(row.to_string());
        }

        let top = image.first().unwrap().clone();
        let bot = image.last().unwrap().clone();
        let left = image
            .iter()
            .map(|row| row.chars().next().unwrap())
            .collect::<String>();
        let right = image
            .iter()
            .map(|row| row.chars().last().unwrap())
            .collect::<String>();

        let edges = vec![
            top.chars().rev().collect::<String>(),
            top,
            bot.chars().rev().collect::<String>(),
            bot,
            left.chars().rev().collect::<String>(),
            left,
            right.chars().rev().collect::<String>(),
            right,
        ];

        tiles.push(Tile { id, image, edges });
    }

    tiles
}

fn tiles_match(a: &Tile, b: &Tile) -> bool {
    for edge in a.edges.iter() {
        if b.edges.contains(edge) {
            return true;
        }
    }

    false
}

fn find_matching_tiles(tiles: &[Tile]) -> usize {
    let mut matches = vec![0; tiles.len()];
    for i in 0..tiles.len() {
        for s in 0..tiles.len() {
            if tiles[i].id == tiles[s].id {
                continue;
            }
            if tiles_match(&tiles[i], &tiles[s]) {
                matches[i] += 1;
            }
        }
    }

    let mut prod = 1;
    for (i, n) in matches.iter().enumerate() {
        if *n == 2 {
            prod *= tiles[i].id as usize;
        }
    }
    prod
}

pub fn run() {
    let input_raw = crate::load_input(module_path!());
    let tiles = parse_tiles(&input_raw);
    println!("Day 20: Jurassic Jigsaw");
    println!("Part One: {}", find_matching_tiles(&tiles));
    println!("Part Two: {}", "TODO");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = concat!(
        "Tile 2311:\n",
        "..##.#..#.\n",
        "##..#.....\n",
        "#...##..#.\n",
        "####.#...#\n",
        "##.##.###.\n",
        "##...#.###\n",
        ".#.#.#..##\n",
        "..#....#..\n",
        "###...#.#.\n",
        "..###..###\n",
        "\n",
        "Tile 1951:\n",
        "#.##...##.\n",
        "#.####...#\n",
        ".....#..##\n",
        "#...######\n",
        ".##.#....#\n",
        ".###.#####\n",
        "###.##.##.\n",
        ".###....#.\n",
        "..#.#..#.#\n",
        "#...##.#..\n",
        "\n",
        "Tile 1171:\n",
        "####...##.\n",
        "#..##.#..#\n",
        "##.#..#.#.\n",
        ".###.####.\n",
        "..###.####\n",
        ".##....##.\n",
        ".#...####.\n",
        "#.##.####.\n",
        "####..#...\n",
        ".....##...\n",
        "\n",
        "Tile 1427:\n",
        "###.##.#..\n",
        ".#..#.##..\n",
        ".#.##.#..#\n",
        "#.#.#.##.#\n",
        "....#...##\n",
        "...##..##.\n",
        "...#.#####\n",
        ".#.####.#.\n",
        "..#..###.#\n",
        "..##.#..#.\n",
        "\n",
        "Tile 1489:\n",
        "##.#.#....\n",
        "..##...#..\n",
        ".##..##...\n",
        "..#...#...\n",
        "#####...#.\n",
        "#..#.#.#.#\n",
        "...#.#.#..\n",
        "##.#...##.\n",
        "..##.##.##\n",
        "###.##.#..\n",
        "\n",
        "Tile 2473:\n",
        "#....####.\n",
        "#..#.##...\n",
        "#.##..#...\n",
        "######.#.#\n",
        ".#...#.#.#\n",
        ".#########\n",
        ".###.#..#.\n",
        "########.#\n",
        "##...##.#.\n",
        "..###.#.#.\n",
        "\n",
        "Tile 2971:\n",
        "..#.#....#\n",
        "#...###...\n",
        "#.#.###...\n",
        "##.##..#..\n",
        ".#####..##\n",
        ".#..####.#\n",
        "#..#.#..#.\n",
        "..####.###\n",
        "..#.#.###.\n",
        "...#.#.#.#\n",
        "\n",
        "Tile 2729:\n",
        "...#.#.#.#\n",
        "####.#....\n",
        "..#.#.....\n",
        "....#..#.#\n",
        ".##..##.#.\n",
        ".#.####...\n",
        "####.#.#..\n",
        "##.####...\n",
        "##..#.##..\n",
        "#.##...##.\n",
        "\n",
        "Tile 3079:\n",
        "#.#.#####.\n",
        ".#..######\n",
        "..#.......\n",
        "######....\n",
        "####.#..#.\n",
        ".#...#.##.\n",
        "#.#####.##\n",
        "..#.###...\n",
        "..#.......\n",
        "..#.###...\n",
    );

    #[test]
    fn test_part_one() {
        let tiles = parse_tiles(INPUT_TEST);
        assert_eq!(find_matching_tiles(&tiles), 20899048083289);
    }

    #[test]
    fn test_part_two() {}
}
