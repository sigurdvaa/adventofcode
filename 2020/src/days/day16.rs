type Field = (usize, usize, usize, usize, String);
type Ticket = Vec<usize>;

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
    let tickets = lines
        .map(|line| line.split(',').map(|c| c.parse().unwrap()).collect())
        .collect();

    (fields, my_ticket, tickets)
}

fn valid_value(fields: &[Field], value: usize) -> bool {
    for field in fields {
        if (field.0 <= value && value <= field.1) || (field.2 <= value && value <= field.3) {
            return true;
        }
    }
    false
}

fn validate_tickets(fields: &[Field], tickets: &[Ticket]) -> (Vec<Ticket>, usize) {
    let mut invalid_values = vec![];
    let mut valid_tickets = vec![];
    for ticket in tickets {
        let mut valid_ticket = true;
        for &value in ticket {
            if !valid_value(fields, value) {
                invalid_values.push(value);
                valid_ticket = false;
            }
        }
        if valid_ticket {
            valid_tickets.push(ticket.clone());
        }
    }

    (valid_tickets, invalid_values.iter().sum())
}

fn map_field_to_ticket_col(fields: &[Field], tickets: &[Ticket]) -> Vec<Option<usize>> {
    let mut field_map = vec![vec![]; fields.len()];
    for (f, field) in fields.iter().enumerate() {
        for i in 0..fields.len() {
            let mut valid = true;
            for ticket in tickets.iter() {
                let value = ticket[i];
                if !((field.0 <= value && value <= field.1)
                    || (field.2 <= value && value <= field.3))
                {
                    valid = false;
                    break;
                }
            }
            if valid {
                field_map[f].push(i)
            }
        }
    }

    let mut field_to_col: Vec<Option<usize>> = vec![None; fields.len()];
    while field_map.iter().map(|m| m.len()).sum::<usize>() > 0 {
        let known = field_map.iter().position(|m| m.len() == 1).unwrap();
        let value = *field_map[known].first().unwrap();
        field_to_col[known] = Some(value);
        for map in field_map.iter_mut() {
            map.retain(|v| *v != value);
        }
    }

    field_to_col
}

fn multiply_ticket_fields_like(
    fields: &[Field],
    tickets: &[Ticket],
    my_ticket: &Ticket,
    like: &str,
) -> usize {
    let field_to_col = map_field_to_ticket_col(fields, tickets);

    let mut product = 1;
    for (i, _) in fields
        .iter()
        .enumerate()
        .filter(|(_, f)| f.4.contains(like))
    {
        let col = field_to_col[i].unwrap();
        product *= my_ticket[col];
    }

    product
}

pub fn run() {
    println!("Day 16: Ticket Translation");
    let input_raw = crate::load_input(module_path!());
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
        let (valid_tickets, _error_rate) = validate_tickets(&fields, &tickets);
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
