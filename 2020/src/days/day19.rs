use regex::Regex;
use std::collections::HashMap;

fn resolve_rules<'a>(rules: &'a [&str], start: &'a str) -> HashMap<&'a str, String> {
    let mut resolved = HashMap::new();
    let mut parsed = HashMap::new();

    for rule in rules {
        let mut split = rule.split(": ");
        let nr = split.next().unwrap();
        let value = split.next().unwrap();

        if value.contains('"') {
            resolved.insert(nr, value[1..value.len() - 1].to_string());
        } else {
            let subrules = value
                .split(" | ")
                .map(|s| s.split_whitespace().collect())
                .collect::<Vec<Vec<&str>>>();
            parsed.insert(nr, subrules);
        }
    }

    let mut stack = vec![start];
    while let Some(rule_nr) = stack.last().cloned() {
        if resolved.contains_key(rule_nr) {
            stack.pop();
            continue;
        }

        let mut missing = false;
        for subrule in parsed.get(rule_nr).unwrap() {
            for sub_nr in subrule {
                if !resolved.contains_key(sub_nr) {
                    stack.push(sub_nr);
                    missing = true;
                }
            }
        }
        if missing {
            continue;
        }

        // build strings from subrules
        let mut resolved_rule = vec![];
        for sub_rule in parsed[rule_nr].iter() {
            let mut resolved_sub = String::new();
            for sub_idx in sub_rule {
                resolved_sub.push_str(&resolved[sub_idx]);
            }
            resolved_rule.push(resolved_sub);
        }

        // add to resolved
        resolved.insert(rule_nr, format!("({})", resolved_rule.join("|")));
        stack.pop();
    }

    resolved
}

fn messages_match_rule(rules: &[&str], messages: &[&str], rule_idx: &str) -> usize {
    let resolved = resolve_rules(rules, rule_idx);
    let start = std::time::Instant::now();
    let re = Regex::new(&format!("^{}$", &resolved[rule_idx])).unwrap();
    let mut count = 0;
    for msg in messages {
        if re.is_match(msg) {
            count += 1;
        }
    }
    println!("Regex Match Elapsed: {:?}", start.elapsed());
    count
}

fn parse_rules_and_messages(input: &str) -> (Vec<&str>, Vec<&str>) {
    let mut rules = vec![];
    let mut lines = input.lines();

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
    let (rules, messages) = parse_rules_and_messages(&input_raw);
    println!("Day 19: Monster Messages");
    println!("Part One: {}", messages_match_rule(&rules, &messages, "0"));
    println!("Part Two: {}", "TODO");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
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
        let (rules, messages) = parse_rules_and_messages(INPUT_TEST);
        assert_eq!(messages_match_rule(&rules, &messages, "0"), 2);
        assert_eq!(messages_match_rule(&rules, &messages, "0"), 2);
    }

    #[test]
    fn test_part_two() {
        const INPUT_TEST: &str = concat!(
            "42: 9 14 | 10 1\n",
            "9: 14 27 | 1 26\n",
            "10: 23 14 | 28 1\n",
            "1: \"a\"\n",
            "11: 42 31\n",
            "5: 1 14 | 15 1\n",
            "19: 14 1 | 14 14\n",
            "12: 24 14 | 19 1\n",
            "16: 15 1 | 14 14\n",
            "31: 14 17 | 1 13\n",
            "6: 14 14 | 1 14\n",
            "2: 1 24 | 14 4\n",
            "0: 8 11\n",
            "13: 14 3 | 1 12\n",
            "15: 1 | 14\n",
            "17: 14 2 | 1 7\n",
            "23: 25 1 | 22 14\n",
            "28: 16 1\n",
            "4: 1 1\n",
            "20: 14 14 | 1 15\n",
            "3: 5 14 | 16 1\n",
            "27: 1 6 | 14 18\n",
            "14: \"b\"\n",
            "21: 14 1 | 1 14\n",
            "25: 1 1 | 1 14\n",
            "22: 14 14\n",
            "8: 42\n",
            "26: 14 22 | 1 20\n",
            "18: 15 15\n",
            "7: 14 5 | 1 21\n",
            "24: 14 1\n",
            "\n",
            "abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa\n",
            "bbabbbbaabaabba\n",
            "babbbbaabbbbbabbbbbbaabaaabaaa\n",
            "aaabbbbbbaaaabaababaabababbabaaabbababababaaa\n",
            "bbbbbbbaaaabbbbaaabbabaaa\n",
            "bbbababbbbaaaaaaaabbababaaababaabab\n",
            "ababaaaaaabaaab\n",
            "ababaaaaabbbaba\n",
            "baabbaaaabbaaaababbaababb\n",
            "abbbbabbbbaaaababbbbbbaaaababb\n",
            "aaaaabbaabaaaaababaa\n",
            "aaaabbaaaabbaaa\n",
            "aaaabbaabbaaaaaaabbbabbbaaabbaabaaa\n",
            "babaaabbbaaabaababbaabababaaab\n",
            "aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba",
        );
        let (rules, messages) = parse_rules_and_messages(INPUT_TEST);
        assert_eq!(messages_match_rule(&rules, &messages, "0"), 3);
        assert_eq!(messages_match_rule(&rules, &messages, "0"), 3);
    }
}
