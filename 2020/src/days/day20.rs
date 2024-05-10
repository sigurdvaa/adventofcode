#[derive(Debug, Clone)]
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

fn find_matching_tiles(tiles: &[Tile]) -> Vec<Vec<usize>> {
    let mut graph = vec![vec![]; tiles.len()];
    for i in 0..tiles.len() {
        for s in 0..tiles.len() {
            if tiles[i].id == tiles[s].id {
                continue;
            }
            if tiles_match(&tiles[i], &tiles[s]) {
                graph[i].push(s);
            }
        }
    }
    graph
}

fn corners_product(tiles: &[Tile], graph: &[Vec<usize>]) -> usize {
    let mut prod = 1;
    for (i, n) in graph.iter().enumerate() {
        if n.len() == 2 {
            prod *= tiles[i].id as usize;
        }
    }
    prod
}

fn build_image(tiles: &[Tile], graph: &[Vec<usize>]) -> Vec<Vec<Tile>> {
    let mut used = vec![false; tiles.len()];
    let mut image = vec![];

    // first row, first corner
    let first = graph.iter().position(|edges| edges.len() == 2).unwrap();
    used[first] = true;
    let mut row = vec![first];
    let mut prev = &tiles[first];
    loop {
        // follow the edge
        let mut found_edge = false;
        for (i, edges) in graph.iter().enumerate() {
            if !used[i] && edges.len() == 3 && tiles_match(prev, &tiles[i]) {
                used[i] = true;
                row.push(i);
                found_edge = true;
                prev = &tiles[i];
                break;
            }
        }
        if !found_edge {
            break;
        }
    }
    // find last corner in row
    for (i, edges) in graph.iter().enumerate() {
        if !used[i] && edges.len() == 2 && tiles_match(prev, &tiles[i]) {
            used[i] = true;
            row.push(i);
            break;
        }
    }
    image.push(row);

    // rest of the rows
    loop {
        let prev = image.last().unwrap();
        let mut row = vec![];
        for i in prev {
            for n in graph[*i].iter() {
                if !used[*n] && tiles_match(&tiles[*i], &tiles[*n]) {
                    used[*n] = true;
                    row.push(*n);
                    break;
                }
            }
        }
        if row.is_empty() {
            break;
        }
        image.push(row);
    }

    image
        .iter()
        .map(|row| row.iter().map(|i| tiles[*i].clone()).collect())
        .collect()
}

fn rotate_tile(tile: &mut Tile) {
    let mut rotated_right = vec![];
    for i in (0..tile.image[0].len()).rev() {
        rotated_right.push(
            tile.image
                .iter()
                .map(|row| row.chars().nth(i).unwrap())
                .collect::<String>(),
        );
    }
    tile.image = rotated_right;
}

fn align_tile_right(row: &mut [Tile], lhs: usize, rhs: usize) {
    let lhs_side = &row[lhs].edges[7];
    let rhs_edge_match = row[rhs].edges.iter().position(|e| *e == *lhs_side).unwrap();
    rotate_tile(&mut row[rhs]);
}

fn align_tiles_in_image(image: &mut [Vec<Tile>]) {
    // align first row
    for i in 1..image[0].len() {
        println!("##############");
        dbg!(&image[0][i - 1].image);
        println!("pre");
        dbg!(&image[0][i].image);
        align_tile_right(&mut image[0], i - 1, i);
        println!("post");
        dbg!(&image[0][i].image);
    }

    // align rest of the rows
}

pub fn run() {
    let input_raw = crate::load_input(module_path!());
    let tiles = parse_tiles(&input_raw);
    let graph = find_matching_tiles(&tiles);
    let mut image = build_image(&tiles, &graph);
    align_tiles_in_image(&mut image);
    println!("Day 20: Jurassic Jigsaw");
    println!("Part One: {}", corners_product(&tiles, &graph));
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
        let graph = find_matching_tiles(&tiles);
        assert_eq!(corners_product(&tiles, &graph), 20899048083289);
    }

    #[test]
    fn test_part_two() {
        let tiles = parse_tiles(INPUT_TEST);
        let graph = find_matching_tiles(&tiles);
        let mut image = build_image(&tiles, &graph);
        align_tiles_in_image(&mut image);
        for row in image {
            println!(
                "{}",
                row.iter()
                    .map(|t| t.id.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
        assert_eq!(false, true);
    }
}
