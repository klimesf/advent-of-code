use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::fs;

pub(crate) fn day19() {
    println!("{}", part_a(fs::read_to_string("input/2020/day19/input.txt").unwrap()));
    println!("{}", part_b(fs::read_to_string("input/2020/day19/input.txt").unwrap(), 8));
}

fn part_a(input: String) -> usize {
    let (rules_str, messages) = input.split_once("\n\n").unwrap();
    let rules = parse_rules(rules_str);

    let re = Regex::new(format!("^{}$", build_regex(0, &rules)).as_str()).unwrap();
    messages.lines().filter(|msg| re.is_match(msg)).count()
}

fn part_b(input: String, len: usize) -> usize {
    let (rules_str, messages) = input.split_once("\n\n").unwrap();
    let rules = parse_rules(rules_str);
    let re_42_str = build_regex(42, &rules);
    let re_31_str = build_regex(31, &rules);
    let re_42 = Regex::new(&re_42_str).unwrap();
    let re_31 = Regex::new(&re_31_str).unwrap();
    // The rule 0: 8 11
    // With rule 8: 42 | 42 8 ... meaning (42)+
    // And rule 11: 42 31 | 42 11 31 ... meaning (42)+(31)+
    // If you join it, it means the strings 42 and 31 must be repeated at least once in this order, where 31 must be
    // repeated at least 1 less time than 42
    // 42 42 31 is valid ....... 8 11
    // 42 42 42 31 is valid .... 8 8 11
    // 42 42 42 31 31 is valid . 8 11
    // etc.
    // also, 42 and 31 always match the same length strings, 5 for example, 8 for my input
    messages
        .lines()
        .filter(|m| {
            let mut checking_42 = true;
            let mut count_42 = 0;
            let mut count_31 = 0;

            let mut pos = 0;

            while pos < m.len() {
                let part = m[pos..pos + len].to_string();
                pos += len;
                if checking_42 && re_42.is_match(&part) {
                    count_42 += 1;
                    continue;
                } else if checking_42 && re_31.is_match(&part) {
                    checking_42 = false;
                    count_31 += 1;
                    continue;
                } else if re_31.is_match(&part) {
                    count_31 += 1;
                    continue;
                } else {
                    return false;
                }
            }
            count_42 > 0 && count_31 > 0 && count_42 > count_31
        })
        .count()
}

fn parse_rules(rules_str: &str) -> HashMap<usize, Rule> {
    rules_str
        .lines()
        .map(|line| {
            let (num, rest) = line.split_once(": ").unwrap();

            let rule = if rest == "\"a\"" {
                Rule::Literal('a')
            } else if rest == "\"b\"" {
                Rule::Literal('b')
            } else if rest.contains("|") {
                let (l, r) = rest.split_once(" | ").unwrap();
                let ls = l.split_whitespace().map(|c| c.parse().unwrap()).collect();
                let rs = r.split_whitespace().map(|c| c.parse().unwrap()).collect();
                Rule::Or(ls, rs)
            } else {
                let seq = rest.split_whitespace().map(|c| c.parse().unwrap()).collect();
                Rule::Sequence(seq)
            };

            (num.parse::<usize>().unwrap(), rule)
        })
        .collect::<HashMap<usize, Rule>>()
}

fn build_regex(rule: usize, rules: &HashMap<usize, Rule>) -> String {
    match rules.get(&rule).unwrap() {
        Rule::Literal(char) => format!("{}", char),
        Rule::Sequence(seq) => seq.iter().map(|rule| build_regex(*rule, rules)).join(""),
        Rule::Or(l, r) => {
            let ls = l.iter().map(|rul| build_regex(*rul, rules)).join("");
            let rs = r.iter().map(|rul| build_regex(*rul, rules)).join("");
            format!("({}|{})", ls, rs)
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Rule {
    Literal(char),
    Sequence(Vec<usize>),
    Or(Vec<usize>, Vec<usize>),
}

#[cfg(test)]
mod day19_tests {
    use std::fs;

    use crate::y2020::day19::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(2, part_a(fs::read_to_string("input/2020/day19/test_a.txt").unwrap()));
        assert_eq!(12, part_b(fs::read_to_string("input/2020/day19/test_b.txt").unwrap(), 5));
    }

    #[test]
    fn input_works() {
        assert_eq!(147, part_a(fs::read_to_string("input/2020/day19/input.txt").unwrap()));
        assert_eq!(263, part_b(fs::read_to_string("input/2020/day19/input.txt").unwrap(), 8));
    }
}
