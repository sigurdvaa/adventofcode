fn messages_match_rule(
    resolved: &mut [Vec<String>],
    rules: &[Vec<Vec<usize>>],
    messages: &[&str],
    rule_idx: usize,
) -> usize {
    let mut unresolved = true;
    while unresolved {
        unresolved = false;
        for i in 0..resolved.len() {
            if !resolved[i].is_empty() {
                continue;
            }

            // check for missing resolved
            let mut missing = false;
            for sub_rule in rules[i].iter() {
                for sub_idx in sub_rule {
                    if resolved[*sub_idx].is_empty() {
                        missing = true;
                        break;
                    }
                }
                if missing {
                    break;
                }
            }
            if missing {
                continue;
            }

            // build strings from subrules
            let mut rule = vec![];
            for sub_rule in rules[i].iter() {
                let mut sub_res = vec![String::new()];
                for sub_idx in sub_rule {
                    let mut next_sub_res = vec![];
                    for sub_value in sub_res {
                        for res_value in resolved[*sub_idx] {
                            next_sub_res.push(format!("{sub_value}{res_value}"));
                        }
                    }
                    sub_res = next_sub_res;
                }
                rule.push(sub_rule);
            }

            // add to resolved
            resolved[i] = rule;
        }
    }

    let mut count = 0;
    count
}

fn parse_rules_and_messages(input: &str) -> (Vec<Vec<String>>, Vec<Vec<Vec<usize>>>, Vec<&str>) {
    let mut lines = input.lines();
    let mut raw_rules = vec![];

    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        raw_rules.push(line);
    }

    let mut rules = vec![];
    let mut resolved = vec![];
    for raw in raw_rules {
        let mut split = raw.split(": ");
        let idx = split.next().unwrap().parse::<usize>().unwrap();
        let value = split.next().unwrap();

        if value.contains('"') {
            resolved.push((idx, vec![value[1..value.len() - 1].to_string()]));
            rules.push((idx, vec![]));
            continue;
        }

        let subrules = value
            .split(" | ")
            .map(|s| {
                s.split_whitespace()
                    .map(|i| i.parse::<usize>().unwrap())
                    .collect()
            })
            .collect::<Vec<_>>();
        resolved.push((idx, vec![]));
        rules.push((idx, subrules));
    }

    resolved.sort_by(|a, b| a.0.cmp(&b.0));
    rules.sort_by(|a, b| a.0.cmp(&b.0));

    let resolved = resolved.into_iter().map(|(_, r)| r).collect();
    let rules = rules.into_iter().map(|(_, r)| r).collect();

    let messages = lines.collect();
    (resolved, rules, messages)
}

pub fn run() {
    let input_raw = crate::load_input(module_path!());
    let (mut resolved, rules, messages) = parse_rules_and_messages(&input_raw);
    println!("Day 19: Monster Messages");
    println!(
        "Part One: {}",
        messages_match_rule(&mut resolved, &rules, &messages, 0)
    );
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
        let (mut resolved, rules, messages) = parse_rules_and_messages(INPUT_TEST);
        assert_eq!(messages_match_rule(&mut resolved, &rules, &messages, 0), 2);
    }

    #[test]
    fn test_part_two() {}
}
