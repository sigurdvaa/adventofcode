use std::fs;

fn direction(a: &(i32, i32), b: &(i32, i32)) -> (i32, i32) {
    let dx = b.0 - a.0;
    let dy = b.1 - a.1;
    (
        if dx == 0 {
            0
        } else {
            if dx > 0 {
                1
            } else {
                -1
            }
        },
        if dy == 0 {
            0
        } else {
            if dy > 0 {
                1
            } else {
                -1
            }
        },
    )
}

fn gradient(a: &(i32, i32), b: &(i32, i32)) -> f32 {
    let dx = b.0 - a.0;
    let dy = b.1 - a.1;
    dy as f32 / dx as f32
}

fn best_detection(points: &Vec<(i32, i32)>) -> usize {
    let mut max = 0;
    for p1 in points {
        let mut gradients = vec![];
        for p2 in points {
            if p1 == p2 {
                continue;
            }
            let dg = (direction(p1, p2), gradient(p1, p2));
            if !gradients.contains(&dg) {
                gradients.push(dg);
            }
        }
        if gradients.len() > max {
            max = gradients.len();
        }
    }
    max
}

fn parse_map(map: &str) -> Vec<(i32, i32)> {
    let mut points = vec![];
    for (y, line) in map.lines().enumerate() {
        for (x, c) in line.trim().chars().enumerate() {
            if c == '#' {
                points.push((x as i32, y as i32));
            }
        }
    }
    points
}

pub fn run() {
    println!("Day 10: Monitoring Station");
    let file_path = "inputs/day10.txt";
    let input_raw =
        fs::read_to_string(file_path).expect(format!("Error reading file '{file_path}'").as_str());

    let points = parse_map(&input_raw);
    println!("Part One: {}", best_detection(&points));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let map = ".#..#\n.....\n#####\n....#\n...##";
        let points = parse_map(&map);
        assert_eq!(best_detection(&points), 8);

        let map = "......#.#.\n#..#.#....\n..#######.\n.#.#.###..\n.#..#.....\n\
                   ..#....#.#\n#..#....#.\n.##.#..###\n##...#..#.\n.#....####";
        let points = parse_map(&map);
        assert_eq!(best_detection(&points), 33);

        let map = "#.#...#.#.\n.###....#.\n.#....#...\n##.#.#.#.#\n....#.#.#.\n\
                   .##..###.#\n..#...##..\n..##....##\n......#...\n.####.###.";
        let points = parse_map(&map);
        assert_eq!(best_detection(&points), 35);

        let map = ".#..#..###\n####.###.#\n....###.#.\n..###.##.#\n##.##.#.#.\n\
                   ....###..#\n..#.#..#.#\n#..#.#.###\n.##...##.#\n.....#.#..";
        let points = parse_map(&map);
        assert_eq!(best_detection(&points), 41);
    }

    #[test]
    fn test_part_two() {
        let map = ".#..##.###...#######\n##.############..##.\n.#.######.########.#\n\
            .###.#######.####.#.\n#####.##.#.##.###.##\n..#####..#.#########\n\
            ####################\n#.####....###.#.#.##\n##.#################\n\
            #####.##.###..####..\n..######..##.#######\n####.##.####...##..#\n\
            .#####..#.######.###\n##...#.##########...\n#.##########.#######\n\
            .####.#.###.###.#.##\n....##.##.###..#####\n.#.#.###########.###\n\
            #.#.#.#####.####.###\n###.##.####.##.#..##";
        let points = parse_map(&map);
        assert_eq!(best_detection(&points), 210);
    }
}
