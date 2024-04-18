// enum Rule {
//     Unresolved(String),
//     Resolved(String),
//     Parsed(Vec<Vec<usize>>),
// }
//
// impl Rule {
//     fn value(&self) -> String {
//         match self {
//             Self::Unresolved(value) => value.into(),
//             Self::Resolved(value) => value.into(),
//             Self(value) => value.into(),
//         }
//     }
// }
//
fn parse_rules_and_messages(input: &str) -> (Vec<Vec<String>>, Vec<&str>) {
    let mut lines = input.lines();
    let mut rules = vec![];
    let mut unresolved = vec![];

    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        if line.contains('"') {
            rules.push(Some(line.to_string()));
            unresolved.push(vec![]);
        } else {
            rules.push(None);
            let pointers = line
                .split('|')
                .map(|or| {
                    or.split_whitespace()
                        .map(|point| point.trim().parse().unwrap())
                        .collect()
                })
                .collect::<Vec<Vec<usize>>>();
            unresolved.push(pointers);
        }
    }

    let mut resolved = false;
    while !resolved {
        resolved = true;
    }

    let messages = lines.collect();
    (rules, messages)
}

fn messages_match_rule(rules: &[String], messages: &[String], rule: usize) -> usize {
    0
}

pub fn run() {
    let input_raw = crate::load_input(module_path!());
    println!("Day 19: Monster Messages");
    println!("Part One: {}", "TODO");
    println!("Part Two: {}", "TODO");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = concat!(
        "0: 4 1 5\n",
        "1: 2 3 | 3 2\n",
        "2: 4 4 | 5 5\n",
        "3: 4 5 | 5 4\n",
        "4: \"a\"\n",
        "5: \"b\"\n",
        "\n",
        "ababbb\n",
        "bababa\n",
        "abbbab\n",
        "aaabbb\n",
        "aaaabbb",
    );

    #[test]
    fn test_part_one() {
        let (rules, messages) = parse_rules_and_messages(INPUT_TEST);
        assert_eq!(messages_match_rule(&rules, &messages, 0), 2);
    }

    #[test]
    fn test_part_two() {}
}
