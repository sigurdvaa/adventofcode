use crate::load_input;

type Field = (u32, u32, u32, u32, String);
type Ticket = Vec<u32>;

fn parse_input(input: &str) -> (Vec<Field>, Ticket, Vec<Ticket>) {
    let mut lines = input.lines();

    let mut fields = vec![];
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        let mut split = line.split(": ");
        let name = split.next().unwrap();
        let mut ranges = split.next().unwrap().split(" or ");
        let mut range1 = ranges.next().unwrap().split('-');
        let mut range2 = ranges.next().unwrap().split('-');
        fields.push((
            range1.next().unwrap().parse().unwrap(),
            range1.next().unwrap().parse().unwrap(),
            range2.next().unwrap().parse().unwrap(),
            range2.next().unwrap().parse().unwrap(),
            name.to_string(),
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

fn validate_tickets(fields: &[Field], tickets: &[Ticket]) -> (Vec<Ticket>, u32) {
    let mut invalid_fields = vec![];
    let mut valid_tickets = vec![];
    for ticket in tickets {
        for &value in ticket {
            let mut valid = false;
            for field in fields {
                if (field.0 <= value && value <= field.1) || (field.2 <= value && value <= field.3)
                {
                    valid = true;
                    valid_tickets.push(ticket.clone());
                    break;
                }
            }
            if !valid {
                invalid_fields.push(value);
            }
        }
    }

    (valid_tickets, invalid_fields.iter().sum())
}

fn multiply_ticket_fields_like(
    fields: &[Field],
    tickets: &[Ticket],
    my_ticket: &Ticket,
    like: &str,
) -> u32 {
    0
}

pub fn run() {
    println!("Day 16: Ticket Translation");
    let input_raw = load_input(module_path!());
    let (fields, my_ticket, tickets) = parse_input(&input_raw);
    let (valid_tickets, error_rate) = validate_tickets(&fields, &tickets);
    println!("Part One: {}", error_rate);
    println!(
        "Part Two: {}",
        multiply_ticket_fields_like(&fields, &valid_tickets, &my_ticket, "departure")
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
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

        let (fields, my_ticket, tickets) = parse_input(INPUT_TEST);
        assert_eq!(
            fields,
            [
                (1, 3, 5, 7, "class".to_string()),
                (6, 11, 33, 44, "row".to_string()),
                (13, 40, 45, 50, "seat".to_string())
            ]
        );
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
        assert_eq!(validate_tickets(&fields, &tickets).1, 71);
    }

    #[test]
    fn test_part_two() {
        const INPUT_TEST: &str = concat!(
            "class: 0-1 or 4-19\n",
            "row: 0-5 or 8-19\n",
            "seat: 0-13 or 16-19\n",
            "\n",
            "your ticket:\n",
            "11,12,13\n",
            "\n",
            "nearby tickets:\n",
            "3,9,18\n",
            "15,1,5\n",
            "5,14,9",
        );
        let (fields, my_ticket, tickets) = parse_input(INPUT_TEST);
        let (valid_tickets, error_rate) = validate_tickets(&fields, &tickets);
        assert_eq!(
            multiply_ticket_fields_like(&fields, &valid_tickets, &my_ticket, "row"),
            11
        );
        assert_eq!(
            multiply_ticket_fields_like(&fields, &valid_tickets, &my_ticket, "class"),
            12
        );
        assert_eq!(
            multiply_ticket_fields_like(&fields, &valid_tickets, &my_ticket, "seat"),
            13
        );
    }
}
