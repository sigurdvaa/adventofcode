#[derive(Debug, Clone)]
struct Tile {
    id: u32,
    image: Vec<Vec<char>>,
    edges: Vec<Vec<char>>,
}

fn mark_sea_monster(image: &mut [Vec<char>]) {
    // const SEA_MONSTER: [[char; 20]; 3] = [
    //     [
    //         ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
    //         ' ', '#', ' ',
    //     ],
    //     [
    //         '#', ' ', ' ', ' ', ' ', '#', '#', ' ', ' ', ' ', ' ', '#', '#', ' ', ' ', ' ', ' ',
    //         '#', '#', '#',
    //     ],
    //     [
    //         ' ', '#', ' ', ' ', '#', ' ', ' ', '#', ' ', ' ', '#', ' ', ' ', '#', ' ', ' ', '#',
    //         ' ', ' ', ' ',
    //     ],
    // ];
    const SEA_MONSTER: [(usize, usize); 15] = [
        (18, 0),
        (0, 1),
        (5, 1),
        (6, 1),
        (11, 1),
        (12, 1),
        (17, 1),
        (18, 1),
        (19, 1),
        (1, 2),
        (4, 2),
        (7, 2),
        (10, 2),
        (13, 2),
        (16, 2),
    ];

    for _ in 0..4 {
        let mut monsters_found = false;
        for y in 0..image.len() - 2 {
            for x in 0..image[y].len() - 19 {
                let mut found = true;
                for (sx, sy) in SEA_MONSTER {
                    if image[y + sy][x + sx] != '#' {
                        found = false;
                    }
                }
                if found {
                    monsters_found = true;
                    for (sx, sy) in SEA_MONSTER {
                        image[y + sy][x + sx] = 'O';
                    }
                }
            }
        }
        if monsters_found {
            break;
        }
        flip_image_col(image);
        for y in 0..image.len() - 2 {
            for x in 0..image[y].len() - 19 {
                let mut found = true;
                for (sx, sy) in SEA_MONSTER {
                    if image[y + sy][x + sx] != '#' {
                        found = false;
                    }
                }
                if found {
                    monsters_found = true;
                    for (sx, sy) in SEA_MONSTER {
                        image[y + sy][x + sx] = 'O';
                    }
                }
            }
        }
        if monsters_found {
            break;
        }
        flip_image_col(image);
        rotate_image(image);
    }
}

fn parse_tiles(input: &str) -> Vec<Tile> {
    let mut tiles = vec![];
    let mut lines = input.lines();

    while let Some(line) = lines.next() {
        let id = line.split("Tile ").nth(1).unwrap();
        let id = id[..id.len() - 1].parse().unwrap();
        let mut image: Vec<Vec<char>> = vec![];
        for row in lines.by_ref() {
            if row.is_empty() {
                break;
            }
            image.push(row.chars().collect());
        }

        let top = image.first().unwrap().clone();
        let bot = image.last().unwrap().clone();
        let left = image
            .iter()
            .map(|row| row.iter().next().cloned().unwrap())
            .collect::<Vec<_>>();
        let right = image
            .iter()
            .map(|row| row.iter().last().cloned().unwrap())
            .collect::<Vec<_>>();

        let edges = vec![
            top.iter().cloned().rev().collect(),
            top,
            bot.iter().cloned().rev().collect(),
            bot,
            left.iter().cloned().rev().collect(),
            left,
            right.iter().cloned().rev().collect(),
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

fn tiles_to_image(image: &[Vec<Tile>]) -> Vec<Vec<char>> {
    let mut new_image = vec![];

    for row in image {
        let mut rows: Vec<Vec<char>> = vec![vec![]; row[0].image.len() - 2];
        for tile in row {
            for (i, v) in tile.image.iter().skip(1).take(rows.len()).enumerate() {
                rows[i].extend(v.iter().skip(1).take(v.len() - 2));
            }
        }
        new_image.extend(rows);
    }

    new_image
}

fn build_image(tiles: &[Tile], graph: &[Vec<usize>]) -> Vec<Vec<char>> {
    let mut used = vec![false; tiles.len()];
    let mut image = vec![];

    // first row, first corner
    let first = graph.iter().position(|edges| edges.len() == 2).unwrap();
    used[first] = true;
    let mut row = vec![first];
    let mut prev = &tiles[first];

    // follow the first row
    loop {
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

    // find last corner in first row
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

    let mut image = image
        .iter()
        .map(|row| row.iter().map(|i| tiles[*i].clone()).collect())
        .collect::<Vec<_>>();

    align_tiles_in_image(&mut image);
    tiles_to_image(&image)
}

fn rotate_image(image: &mut [Vec<char>]) {
    // transpose
    for y in 0..image.len() - 1 {
        for x in 0..image[y].len() - y - 1 {
            let swap_y = image.len() - 1 - x;
            let swap_x = image.len() - 1 - y;
            let tmp = image[y][x];
            image[y][x] = image[swap_y][swap_x];
            image[swap_y][swap_x] = tmp;
        }
    }

    // reverse
    image.reverse();
}

fn tiles_aligned_row(lhs: &Tile, rhs: &Tile) -> bool {
    for i in 0..lhs.image.len() {
        if lhs.image[i].last().unwrap() != rhs.image[i].first().unwrap() {
            return false;
        }
    }
    true
}

fn flip_image_col(image: &mut [Vec<char>]) {
    image.reverse();
}

fn tiles_aligned_col(top: &Tile, bot: &Tile) -> bool {
    top.image.last().unwrap() == bot.image.first().unwrap()
}

fn align_tile_row(row: &mut [Tile], lhs: usize, rhs: usize) {
    for _ in 0..4 {
        if tiles_aligned_row(&row[lhs], &row[rhs]) {
            return;
        }
        flip_image_col(&mut row[rhs].image);

        if tiles_aligned_row(&row[lhs], &row[rhs]) {
            return;
        }
        flip_image_col(&mut row[rhs].image);
        rotate_image(&mut row[rhs].image);
    }
    unreachable!("align tile row failed");
}

fn align_tile_col(image: &mut [Vec<Tile>], row: usize, col: usize) {
    for _ in 0..4 {
        if tiles_aligned_col(&image[row - 1][col], &image[row][col]) {
            return;
        }
        flip_image_col(&mut image[row][col].image);

        if tiles_aligned_col(&image[row - 1][col], &image[row][col]) {
            return;
        }
        flip_image_col(&mut image[row][col].image);
        rotate_image(&mut image[row][col].image);
    }
    unreachable!("align tile col failed");
}

fn align_tile_corner(image: &mut [Vec<Tile>]) {
    // align towards row
    for _ in 0..4 {
        let right = image[0][0]
            .image
            .iter()
            .map(|row| row.iter().last().cloned().unwrap())
            .collect::<Vec<_>>();
        if image[0][1].edges.contains(&right) {
            break;
        }
        rotate_image(&mut image[0][0].image);
    }

    // align towards col
    let bot = image[0][0].image.last().cloned().unwrap();
    if !image[1][0].edges.contains(&bot) {
        flip_image_col(&mut image[0][0].image);
    }
}

fn align_tiles_in_image(image: &mut [Vec<Tile>]) {
    // align first corner
    align_tile_corner(image);

    // align first row
    for i in 1..image[0].len() {
        align_tile_row(&mut image[0], i - 1, i);
    }

    // align the rest by col
    for r in 1..image.len() {
        for i in 0..image[r].len() {
            align_tile_col(image, r, i);
        }
    }
}

pub fn run() {
    let input_raw = crate::load_input(module_path!());
    let tiles = parse_tiles(&input_raw);
    let graph = find_matching_tiles(&tiles);
    let mut image = build_image(&tiles, &graph);
    mark_sea_monster(&mut image);
    println!("Day 20: Jurassic Jigsaw");
    println!("Part One: {}", corners_product(&tiles, &graph));
    println!(
        "Part Two: {}",
        image
            .iter()
            .map(|row| row.iter().filter(|&c| *c == '#').count())
            .sum::<usize>()
    );
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
        mark_sea_monster(&mut image);
        assert_eq!(
            image
                .iter()
                .map(|row| row.iter().filter(|&c| *c == '#').count())
                .sum::<usize>(),
            273
        );
    }
}
