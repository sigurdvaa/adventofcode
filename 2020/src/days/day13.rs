use std::fs;

fn parse_timestamp_and_ids(input: &str) -> (usize, Vec<usize>) {
    let mut lines = input.lines();
    let timestamp = lines.next().unwrap().parse::<usize>().unwrap();
    let ids = lines
        .next()
        .unwrap()
        .split(',')
        .map(|x| if x == "x" { "0" } else { x })
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    (timestamp, ids)
}

fn earliest_id_times_minutes(timestamp: usize, ids: &[usize]) -> usize {
    let mut bus_id = 0;
    let mut bus_wait = usize::MAX;

    for id in ids.iter().filter(|x| **x != 0) {
        let wait = ((timestamp / id) + 1) * id - timestamp;
        if wait < bus_wait {
            bus_wait = wait;
            bus_id = *id;
        }
    }

    bus_id * bus_wait
}

fn earliest_timestamp_match_list(ids: &[usize]) -> usize {
    0
}

pub fn run() {
    println!("Day 13: Shuttle Search");
    let file_path = "inputs/day13.txt";
    let input_raw = fs::read_to_string(file_path)
        .unwrap_or_else(|err| panic!("Error reading file '{file_path}': {err}"));

    let (timestamp, ids) = parse_timestamp_and_ids(&input_raw);
    println!("Part One: {}", earliest_id_times_minutes(timestamp, &ids));
    println!("Part Two: {}", earliest_timestamp_match_list(&ids));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = "939\n7,13,x,x,59,x,31,19";

    #[test]
    fn test_part_one() {
        let (timestamp, ids) = parse_timestamp_and_ids(INPUT_TEST);
        assert_eq!(timestamp, 939);
        assert_eq!(ids, vec![7, 13, 0, 0, 59, 0, 31, 19]);
        assert_eq!(earliest_id_times_minutes(timestamp, &ids), 295);
    }

    #[test]
    fn test_part_two() {
        let (_timestamp, ids) = parse_timestamp_and_ids(INPUT_TEST);
        assert_eq!(earliest_timestamp_match_list(&ids), 1068781);

        let (_timestamp, ids) = parse_timestamp_and_ids("0\n17,x,13,19");
        assert_eq!(earliest_timestamp_match_list(&ids), 1068781);

        let (_timestamp, ids) = parse_timestamp_and_ids("0\n67,7,59,61");
        assert_eq!(earliest_timestamp_match_list(&ids), 754018);

        let (_timestamp, ids) = parse_timestamp_and_ids("0\n67,x,7,59,61");
        assert_eq!(earliest_timestamp_match_list(&ids), 779210);

        let (_timestamp, ids) = parse_timestamp_and_ids("0\n67,7,x,59,61");
        assert_eq!(earliest_timestamp_match_list(&ids), 1261476);

        let (_timestamp, ids) = parse_timestamp_and_ids("0\n1789,37,47,1889");
        assert_eq!(earliest_timestamp_match_list(&ids), 1202161486);
    }
}
