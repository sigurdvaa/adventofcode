use std::fs;

fn gradient(a: (i32, i32), b: (i32, i32)) -> f32 {
    (b.0 - a.0) as f32 / (b.1 - a.1) as f32
}

pub fn run() {
    println!("Day 10: Monitoring Station");
    let file_path = "inputs/day10.txt";
    let input_raw =
        fs::read_to_string(file_path).expect(format!("Error reading file '{file_path}'").as_str());

    let s = (0, 0);
    let points = vec![(3, 1), (6, 2), (9, 3)];
    for p in points {
        let g = gradient(s, p);
        println!("{}", g);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {}

    #[test]
    fn test_part_two() {}
}
