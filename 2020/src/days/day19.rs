#[derive(Clone)]
enum RuleToken {
    Ref(usize),
    Val(char),
}

fn value_matches_rule(rules: &[Vec<Vec<RuleToken>>], rule: usize, value: &str) -> bool {
    let value = value.chars().collect::<Vec<_>>();
    let mut queue = vec![];
    for subrule in rules[rule].iter().rev() {
        queue.push((0, vec![(0, subrule)]));
    }

    while let Some((len, mut stack)) = queue.pop() {
        if let Some((i, rule)) = stack.pop() {
            if i + 1 < rule.len() {
                stack.push((i + 1, rule));
            }
            match rule[i] {
                RuleToken::Ref(ruleref) => {
                    for subrule in rules[ruleref].iter().rev() {
                        let mut next_stack = stack.clone();
                        next_stack.push((0, subrule));
                        queue.push((len, next_stack));
                    }
                }
                RuleToken::Val(c) => {
                    if c == value[len] {
                        if stack.is_empty() {
                            if len + 1 == value.len() {
                                return true;
                            }
                        } else if len + 1 < value.len() {
                            queue.push((len + 1, stack));
                        }
                    }
                }
            }
        }
    }

    false
}

fn parse_rule_tokens(rules: &[&str]) -> Vec<Vec<Vec<RuleToken>>> {
    let mut tokens = vec![];
    for rule in rules {
        let mut split = rule.split(": ");
        let nr = split.next().unwrap().parse::<usize>().unwrap();
        let value = split.next().unwrap();
        let value = if value.contains('"') {
            let c = value.chars().nth(1).unwrap();
            vec![vec![RuleToken::Val(c)]]
        } else {
            value
                .split(" | ")
                .map(|s| {
                    s.split_whitespace()
                        .map(|s| s.parse().unwrap())
                        .map(RuleToken::Ref)
                        .collect()
                })
                .collect::<Vec<Vec<RuleToken>>>()
        };
        if nr >= tokens.len() {
            tokens.resize(nr + 1, vec![]);
        }
        tokens[nr] = value;
    }
    tokens
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

fn updated_rules<'a>(rules: &[&'a str]) -> Vec<&'a str> {
    rules
        .iter()
        .map(|r| {
            if r.starts_with("8:") {
                "8: 42 | 42 8"
            } else if r.starts_with("11:") {
                "11: 42 31 | 42 11 31"
            } else {
                r
            }
        })
        .collect::<Vec<&str>>()
}

fn messages_match_rule(rules: &[&str], messages: &[&str], rule: usize) -> usize {
    let parsed_rules = parse_rule_tokens(rules);
    let mut count = 0;
    for msg in messages {
        if value_matches_rule(&parsed_rules, rule, msg) {
            count += 1;
        }
    }
    count
}

pub fn run() {
    let input_raw = crate::load_input(module_path!());
    let (rules, messages) = parse_rules_and_messages(&input_raw);
    println!("Day 19: Monster Messages");
    println!("Part One: {}", messages_match_rule(&rules, &messages, 0));
    let rules = updated_rules(&rules);
    println!("Part Two: {}", messages_match_rule(&rules, &messages, 0));
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
        assert_eq!(messages_match_rule(&rules, &messages, 0), 2);
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
        assert_eq!(messages_match_rule(&rules, &messages, 0), 3);
        let rules = updated_rules(&rules);
        assert_eq!(messages_match_rule(&rules, &messages, 0), 12);
    }
}
