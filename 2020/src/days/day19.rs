fn resolve_rule(rules: &[&str], cache: &mut [Option<Vec<String>>], idx: usize) -> Vec<String> {
    if let Some(rule) = &cache[idx] {
        return rule.clone();
    }

    if rules[idx].contains('"') {
        return vec![rules[idx][1..rules[idx].len() - 1].to_string()];
    }

    // parse rule
    let mut rule = vec![];
    for alt in rules[idx][4..].split('|') {
        // build string
        let mut alt_rule = vec![String::new()];
        for alt_idx in alt.split_whitespace().map(|p| p.parse().unwrap()) {
            //   for number, resolve_rule
            let mut next_alt_rule = vec![];
            for alt_value in alt_rule {
                for res_value in resolve_rule(rules, cache, alt_idx) {
                    next_alt_rule.push(format!("{alt_value}{res_value}"));
                }
            }
            alt_rule = next_alt_rule;
        }
        rule.extend(alt_rule);
    }
    cache[idx] = Some(rule.clone());
    dbg!(cache);
    // insert to cache

    rule
}

fn messages_match_rule(rules: &[&str], messages: &[&str], rule: usize) -> usize {
    let mut cache = vec![None; rules.len()];
    let rule = resolve_rule(rules, &mut cache, rule);

    let mut count = 0;
    for msg in messages {
        for alt in rule.iter() {
            if !msg.contains(alt) {
                break;
            }
            count += 1;
        }
    }
    count
}

fn parse_rules_and_messages(input: &str) -> (Vec<&str>, Vec<&str>) {
    let mut lines = input.lines();
    let mut rules = vec![];

    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        rules.push(line);
    }

    let messages = lines.collect();
    (rules, messages)
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
