use crate::intcode::Program;
use std::fs;

fn build_map(input: &Vec<i64>) -> Vec<Vec<i64>> {
    let max_x = input.iter().step_by(3).max().unwrap() + 1;
    let max_y = input.iter().skip(1).step_by(3).max().unwrap() + 1;
    let mut map = vec![vec![0; max_x as usize]; max_y as usize];
    for i in (0..input.len()).step_by(3) {
        let x = input[i] as usize;
        let y = input[i + 1] as usize;
        let value = input[i + 2];
        map[y][x] = value;
    }
    map
}

pub fn run() {
    println!("Day 13: Care Package");
    let file_path = "inputs/day13.txt";
    let input_raw =
        fs::read_to_string(file_path).expect(format!("Error reading file '{file_path}'").as_str());

    let prog = Program::new(&input_raw);

    let mut prog1 = prog.clone();
    prog1.run();
    println!(
        "Part One: {}",
        prog1
            .output
            .iter()
            .skip(2)
            .step_by(3)
            .filter(|&x| *x == 2)
            .count()
    );

    let map = build_map(&prog1.output);
    let mut prog2 = prog.clone();
    prog2.input.push(2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let map = build_map(&vec![1, 2, 3, 6, 5, 4]);
        assert_eq!(
            map,
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 3, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 4],
            ]
        );
    }

    #[test]
    fn test_part_two() {}
}
