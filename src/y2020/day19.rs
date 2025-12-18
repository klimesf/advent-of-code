use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::fs;

pub(crate) fn day19() {
    println!("{}", part_a(fs::read_to_string("input/2020/day19/input.txt").unwrap()));
    println!("{}", part_b(fs::read_to_string("input/2020/day19/input.txt").unwrap()));
}

fn part_a(input: String) -> usize {
    let (rules_str, messages) = input.split_once("\n\n").unwrap();
    let rules = parse_rules(rules_str);

    let re = Regex::new(format!("^{}$", build_regex(0, &rules)).as_str()).unwrap();
    messages.lines().filter(|msg| re.is_match(msg)).count()
}

fn part_b(input: String) -> usize {
    let (rules_str, messages) = input.split_once("\n\n").unwrap();
    let mut rules = parse_rules(rules_str);
    rules.insert(8, Rule::Or(vec![42], vec![42, 8]));
    rules.insert(11, Rule::Or(vec![42, 31], vec![42, 11, 31]));

    let re0 = Regex::new(format!("^{}$", build_regex(0, &rules)).as_str()).unwrap();
    let re8 = Regex::new(format!("^{}$", build_regex(8, &rules)).as_str()).unwrap();
    let re11 = Regex::new(format!("^{}$", build_regex(11, &rules)).as_str()).unwrap();
    messages
        .lines()
        .filter(|msg| match_rec(msg.to_string(), 0, &re0, &re8, &re11))
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
            let ls = l.iter().map(|rrr| build_regex(*rrr, rules)).join("");
            let rs = r
                .iter()
                .map(|rrr| {
                    if *rrr == rule {
                        format!("(?P<rec{}>.+)", rrr) // The recursive is always on right
                    } else {
                        build_regex(*rrr, rules)
                    }
                })
                .join("");
            format!("({}|{})", ls, rs)
        }
    }
}

fn match_rec(msg: String, re_use: usize, re0: &Regex, re8: &Regex, re11: &Regex) -> bool {
    let regex = match re_use {
        0 => re0,
        8 => re8,
        11 => re11,
        _ => panic!("Unsupported regex use: {}", re_use),
    };

    if !regex.is_match(msg.as_str()) {
        return false;
    }

    regex
        .captures_iter(msg.as_str())
        .filter(|caps| caps.name("rec8").is_some())
        .map(|caps| caps.name("rec8").unwrap())
        .filter(|cap| cap.as_str().len() > 0)
        .map(|cap| {
            let res = match_rec(cap.as_str().to_string(), 8, re0, re8, re11);
            if !res {
                println!("{:?} {}", regex, cap.as_str())
            }
            res
        })
        .all(|ok| ok)
        && regex
            .captures_iter(msg.as_str())
            .filter(|caps| caps.name("rec11").is_some())
            .map(|caps| caps.name("rec11").unwrap())
            .filter(|cap| cap.as_str().len() > 0)
            .map(|cap| {
                let res = match_rec(cap.as_str().to_string(), 11, re0, re8, re11);
                if !res {
                    println!("{:?} {}", regex, cap.as_str())
                }
                res
            })
            .all(|ok| ok)
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
        assert_eq!(12, part_b(fs::read_to_string("input/2020/day19/test_b.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!(147, part_a(fs::read_to_string("input/2020/day19/input.txt").unwrap()));
        assert_eq!(263, part_b(fs::read_to_string("input/2020/day19/input.txt").unwrap()));
    }
}
