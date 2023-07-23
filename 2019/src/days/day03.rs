use std::collections::HashMap;
use std::fs;

fn parse_path(wire_path: &str) -> HashMap<(i32, i32), i32> {
    let mut path = HashMap::new();
    let mut pos = (0, 0);
    let mut s = 1;
    for step in wire_path.split(",") {
        let d = step.chars().next().unwrap();
        let n = step[1..].parse::<usize>().unwrap();
        let add = match d {
            'R' => (1, 0),
            'L' => (-1, 0),
            'D' => (0, 1),
            'U' => (0, -1),
            _ => panic!("invalid direction: {}", d),
        };
        for _ in 0..n {
            pos = (pos.0 + add.0, pos.1 + add.1);
            path.insert(pos.clone(), s);
            s += 1;
        }
    }
    path
}

fn closest_and_shortest_intersection(
    path1: &HashMap<(i32, i32), i32>,
    path2: &HashMap<(i32, i32), i32>,
) -> (i32, i32) {
    let mut closest = i32::MAX;
    let mut shortest = i32::MAX;
    for (pos1, s1) in path1 {
        if let Some(s2) = path2.get(pos1) {
            let steps = s1 + s2;
            if steps < shortest {
                shortest = steps;
            }
            let dist = pos1.0.abs() + pos1.1.abs();
            if dist < closest {
                closest = dist;
            }
        }
    }
    (closest, shortest)
}

pub fn run() {
    println!("Day 3: Crossed Wires");
    let file_path = "inputs/day03.txt";
    let input_raw =
        fs::read_to_string(file_path).expect(format!("Error reading file '{file_path}'").as_str());

    let mut wires = input_raw.lines();
    let wire1 = parse_path(wires.next().unwrap());
    let wire2 = parse_path(wires.next().unwrap());

    let (closest, shortest) = closest_and_shortest_intersection(&wire1, &wire2);
    println!("Part One: {}", closest);
    println!("Part Two: {}", shortest);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let mut wires = "R8,U5,L5,D3\nU7,R6,D4,L4".lines();
        let wire1 = parse_path(wires.next().unwrap());
        let wire2 = parse_path(wires.next().unwrap());
        assert_eq!(closest_and_shortest_intersection(&wire1, &wire2).0, 6);

        let mut wires =
            "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83".lines();
        let wire1 = parse_path(wires.next().unwrap());
        let wire2 = parse_path(wires.next().unwrap());
        assert_eq!(closest_and_shortest_intersection(&wire1, &wire2).0, 159);

        let mut wires =
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
                .lines();
        let wire1 = parse_path(wires.next().unwrap());
        let wire2 = parse_path(wires.next().unwrap());
        assert_eq!(closest_and_shortest_intersection(&wire1, &wire2).0, 135);
    }

    #[test]
    fn test_part_two() {
        let mut wires = "R8,U5,L5,D3\nU7,R6,D4,L4".lines();
        let wire1 = parse_path(wires.next().unwrap());
        let wire2 = parse_path(wires.next().unwrap());
        assert_eq!(closest_and_shortest_intersection(&wire1, &wire2).1, 30);

        let mut wires =
            "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83".lines();
        let wire1 = parse_path(wires.next().unwrap());
        let wire2 = parse_path(wires.next().unwrap());
        assert_eq!(closest_and_shortest_intersection(&wire1, &wire2).1, 610);

        let mut wires =
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
                .lines();
        let wire1 = parse_path(wires.next().unwrap());
        let wire2 = parse_path(wires.next().unwrap());
        assert_eq!(closest_and_shortest_intersection(&wire1, &wire2).1, 410);
    }
}
