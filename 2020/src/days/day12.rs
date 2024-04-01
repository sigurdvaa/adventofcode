enum Move {
    North(isize),
    South(isize),
    East(isize),
    West(isize),
    Forward(isize),
    Right(isize),
    Left(isize),
}

impl From<&str> for Move {
    fn from(line: &str) -> Self {
        let mut chars = line.chars();
        let m = chars.next().unwrap();
        let value = chars.as_str().parse().unwrap();
        match m {
            'N' => Self::North(value),
            'S' => Self::South(value),
            'E' => Self::East(value),
            'W' => Self::West(value),
            'F' => Self::Forward(value),
            'R' => Self::Right(value),
            'L' => Self::Left(value),
            _ => panic!("Unknown move: {}", m),
        }
    }
}

fn parse_moves(input: &str) -> Vec<Move> {
    input.lines().map(Move::from).collect()
}

fn distance_after_moves(moves: &[Move]) -> isize {
    let mut x = 0;
    let mut y = 0;
    let mut d = 90;
    for m in moves {
        match m {
            Move::North(v) => y -= v,
            Move::South(v) => y += v,
            Move::East(v) => x += v,
            Move::West(v) => x -= v,
            Move::Forward(v) => match d {
                0 => y -= v,
                90 => x += v,
                180 => y += v,
                270 => x -= v,
                _ => panic!("Invalid direction: {}", d),
            },
            Move::Right(v) => d = (d + v).rem_euclid(360),
            Move::Left(v) => d = (d - v).rem_euclid(360),
        }
    }
    x.abs() + y.abs()
}

fn distance_after_waypoint(moves: &[Move]) -> isize {
    let mut ship_x = 0;
    let mut ship_y = 0;
    let mut waypoint_x = 10;
    let mut waypoint_y = -1;
    for m in moves {
        match m {
            Move::North(v) => waypoint_y -= v,
            Move::South(v) => waypoint_y += v,
            Move::East(v) => waypoint_x += v,
            Move::West(v) => waypoint_x -= v,
            Move::Forward(v) => {
                ship_x += waypoint_x * v;
                ship_y += waypoint_y * v;
            }
            Move::Left(v) | Move::Right(v) => match v {
                90 => {
                    std::mem::swap(&mut waypoint_x, &mut waypoint_y);
                    if let Move::Left(_) = m {
                        waypoint_y *= -1;
                    } else {
                        waypoint_x *= -1;
                    }
                }
                180 => {
                    waypoint_x *= -1;
                    waypoint_y *= -1;
                }
                270 => {
                    std::mem::swap(&mut waypoint_x, &mut waypoint_y);
                    if let Move::Left(_) = m {
                        waypoint_x *= -1;
                    } else {
                        waypoint_y *= -1;
                    }
                }
                _ => panic!("Invalid turn angle: {}", v),
            },
        }
    }
    ship_x.abs() + ship_y.abs()
}

pub fn run() {
    println!("Day 12: Rain Risk");
    let input_raw = crate::load_input(module_path!());
    let moves = parse_moves(&input_raw);
    println!("Part One: {}", distance_after_moves(&moves));
    println!("Part Two: {}", distance_after_waypoint(&moves));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = concat!("F10\n", "N3\n", "F7\n", "R90\n", "F11",);

    #[test]
    fn test_part_one() {
        let moves = parse_moves(INPUT_TEST);
        assert_eq!(distance_after_moves(&moves), 25);
    }

    #[test]
    fn test_part_two() {
        let moves = parse_moves(INPUT_TEST);
        assert_eq!(distance_after_waypoint(&moves), 286);
    }
}
