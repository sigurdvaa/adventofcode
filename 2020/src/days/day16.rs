use std::fs;

type Field = (u32, u32, u32, u32);
type Ticket = Vec<u32>;

fn parse_input(input: &str) -> (Vec<Field>, Ticket, Vec<Ticket>) {
    let mut lines = input.lines();

    let mut fields = vec![];
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        let mut split = line.split(": ");
        let _name = split.next().unwrap();
        let mut ranges = split.next().unwrap().split(" or ");
        let mut range1 = ranges.next().unwrap().split('-');
        let mut range2 = ranges.next().unwrap().split('-');
        fields.push((
            range1.next().unwrap().parse().unwrap(),
            range1.next().unwrap().parse().unwrap(),
            range2.next().unwrap().parse().unwrap(),
            range2.next().unwrap().parse().unwrap(),
        ));
    }

    let _ = lines.next(); // your ticket:
    let my_ticket = lines
        .next()
        .unwrap()
        .split(',')
        .map(|c| c.parse().unwrap())
        .collect();

    let _ = lines.next(); // empty line
    let _ = lines.next(); // nearby tickets:
    let mut tickets = vec![];
    for line in lines {
        let ticket = line.split(',').map(|c| c.parse().unwrap()).collect();
        tickets.push(ticket);
    }

    (fields, my_ticket, tickets)
}

pub fn run() {
    println!("Day 16: Ticket Translation");
    let file_path = "inputs/day16.txt";
    let input_raw = fs::read_to_string(file_path)
        .unwrap_or_else(|err| panic!("Error reading file '{file_path}': {err}"));

    // let (fields, my_ticket, tickets) = parse_input(&input_raw);
    println!("Part One: {}", "TODO");
    println!("Part Two: {}", "TODO");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = concat!(
        "class: 1-3 or 5-7\n",
        "row: 6-11 or 33-44\n",
        "seat: 13-40 or 45-50\n",
        "\n",
        "your ticket:\n",
        "7,1,14\n",
        "\n",
        "nearby tickets:\n",
        "7,3,47\n",
        "40,4,50\n",
        "55,2,20\n",
        "38,6,12",
    );

    #[test]
    fn test_part_one() {
        let (fields, my_ticket, tickets) = parse_input(INPUT_TEST);
        assert_eq!(fields, [(1, 3, 5, 7), (6, 11, 33, 44), (13, 40, 45, 50)]);
        assert_eq!(my_ticket, vec![7, 1, 14]);
        assert_eq!(
            tickets,
            vec![
                vec![7, 3, 47],
                vec![40, 4, 50],
                vec![55, 2, 20],
                vec![38, 6, 12]
            ]
        );
    }

    #[test]
    fn test_part_two() {}
}
