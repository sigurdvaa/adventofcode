use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

fn gradient_circle(a: &Point, b: &Point) -> f32 {
    let mut g = (b.y - a.y) as f32 / (b.x - a.x) as f32;
    g = g.atan();
    if a.x > b.x {
        g += std::f32::consts::PI;
    }
    g
}

fn best_detection(points: &Vec<Point>) -> HashMap<String, Vec<Point>> {
    let mut max = HashMap::new();
    for p1 in points {
        let mut gradients = HashMap::new();
        for p2 in points {
            if p1 == p2 {
                continue;
            }
            let g = gradient_circle(p1, p2);
            gradients
                .entry(g.to_string())
                .or_insert(vec![])
                .push(p2.clone());
        }
        if gradients.len() > max.len() {
            max = gradients;
        }
    }
    max
}

fn bet_200th(mut targets: HashMap<String, Vec<Point>>) -> i32 {
    let mut dir: Vec<String> = targets.keys().map(|x| x.clone()).collect();
    dir.sort();
    let mut i = 0;
    loop {
        let prev = i;
        for d in &dir {
            let t = targets.get_mut(d).unwrap();
            if t.len() == 0 {
                continue;
            }
            let p = t.first().unwrap();
            i += 1;
            if i == 200 {
                return p.x * 100 + p.y;
            }
        }
        if prev == i {
            break;
        }
    }
    0
}

fn parse_map(map: &str) -> Vec<Point> {
    let mut points = vec![];
    for (y, line) in map.lines().enumerate() {
        for (x, c) in line.trim().chars().enumerate() {
            if c == '#' {
                points.push(Point::new(x as i32, y as i32));
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
    let best = best_detection(&points);
    println!("Part One: {}", best.len());
    println!("Part Two: {}", bet_200th(best));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let map = ".#..#\n.....\n#####\n....#\n...##";
        let points = parse_map(&map);
        assert_eq!(best_detection(&points).len(), 8);

        let map = "......#.#.\n#..#.#....\n..#######.\n.#.#.###..\n.#..#.....\n\
                   ..#....#.#\n#..#....#.\n.##.#..###\n##...#..#.\n.#....####";
        let points = parse_map(&map);
        assert_eq!(best_detection(&points).len(), 33);

        let map = "#.#...#.#.\n.###....#.\n.#....#...\n##.#.#.#.#\n....#.#.#.\n\
                   .##..###.#\n..#...##..\n..##....##\n......#...\n.####.###.";
        let points = parse_map(&map);
        assert_eq!(best_detection(&points).len(), 35);

        let map = ".#..#..###\n####.###.#\n....###.#.\n..###.##.#\n##.##.#.#.\n\
                   ....###..#\n..#.#..#.#\n#..#.#.###\n.##...##.#\n.....#.#..";
        let points = parse_map(&map);
        assert_eq!(best_detection(&points).len(), 41);
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
        let best = best_detection(&points);
        assert_eq!(best.len(), 210);
        assert_eq!(bet_200th(best), 802);
    }
}
