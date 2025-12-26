use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;

pub(crate) fn day19() {
    println!("{}", part_a(fs::read_to_string("input/2020/day19/input.txt").unwrap()));
    println!("{}", part_b(fs::read_to_string("input/2020/day19/input.txt").unwrap()));
}

fn part_a(input: String) -> usize {
    let (rules_str, messages) = input.split_once("\n\n").unwrap();
    let rules = parse_rules(rules_str);
    let strings = build_strings(0, &rules);
    messages.lines().filter(|msg| strings.contains(*msg)).count()
}

fn part_b(input: String) -> usize {
    let (rules_str, messages) = input.split_once("\n\n").unwrap();
    let rules = parse_rules(rules_str);
    let re = Regex::new(format!("^{}$", build_regex(0, &rules, true)).as_str()).unwrap();
    messages
        .lines()
        .filter(|msg| match_rec(msg.to_string(), &re, &rules))
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

fn build_strings(rule: usize, rules: &HashMap<usize, Rule>) -> HashSet<String> {
    match rules.get(&rule).unwrap() {
        Rule::Literal(c) => HashSet::from_iter(vec![c.to_string()]),
        Rule::Sequence(seq) => {
            let mut ans = HashSet::new();
            ans.insert(String::new());
            for i in 0..seq.len() {
                let mut new_ans = HashSet::new();
                let app = build_strings(seq[i], rules);
                for s in &ans {
                    for s2 in &app {
                        let res = format!("{}{}", s, s2);
                        new_ans.insert(res);
                    }
                }
                ans = new_ans;
            }
            ans
        }
        Rule::Or(ls, rs) => {
            let mut ans1 = HashSet::new();
            ans1.insert(String::new());
            for i in 0..ls.len() {
                let mut new_ans = HashSet::new();
                let app = build_strings(ls[i], rules);
                for s in &ans1 {
                    for s2 in &app {
                        let res = format!("{}{}", s, s2);
                        new_ans.insert(res);
                    }
                }
                ans1 = new_ans;
            }

            let mut ans2 = HashSet::new();
            ans2.insert(String::new());
            for i in 0..rs.len() {
                let mut new_ans = HashSet::new();
                let app = build_strings(rs[i], rules);
                for s in &ans2 {
                    for s2 in &app {
                        let res = format!("{}{}", s, s2);
                        new_ans.insert(res);
                    }
                }
                ans2 = new_ans;
            }

            ans1.union(&ans2).cloned().collect()
        }
    }
}

fn build_regex(rule: usize, rules: &HashMap<usize, Rule>, part_b: bool) -> String {
    if part_b && rule == 8 {
        return format!("({})+", build_regex(42, &rules, part_b));
    }
    if part_b && rule == 11 {
        return format!(
            "(?P<left>{})+?(?P<right>{})+?",
            build_regex(42, &rules, part_b),
            build_regex(31, &rules, part_b)
        );
    }
    match rules.get(&rule).unwrap() {
        Rule::Literal(char) => format!("{}", char),
        Rule::Sequence(seq) => seq.iter().map(|rule| build_regex(*rule, rules, part_b)).join(""),
        Rule::Or(l, r) => {
            let ls = l.iter().map(|ru| build_regex(*ru, rules, part_b)).join("");
            let rs = r.iter().map(|ru| build_regex(*ru, rules, part_b)).join("");
            format!("({}|{})", ls, rs)
        }
    }
}

fn match_rec(msg: String, regex: &Regex, rules: &HashMap<usize, Rule>) -> bool {
    if !regex.is_match(msg.as_str()) {
        return false;
    }

    let left_match = regex
        .captures_iter(msg.as_str())
        .filter(|caps| caps.name("left").is_some())
        .map(|caps| caps.name("left").unwrap().as_str())
        .last()
        .unwrap();

    let right_match = regex
        .captures_iter(msg.as_str())
        .filter(|caps| caps.name("right").is_some())
        .map(|caps| caps.name("right").unwrap().as_str())
        .last()
        .unwrap();

    if left_match.len() == 0 || right_match.len() == 0 {
        return true;
    }

    let left_re = Regex::new(build_regex(42, &rules, false).as_str()).unwrap();
    let right_re = Regex::new(build_regex(31, &rules, false).as_str()).unwrap();

    let left_cnt = left_re.find_iter(left_match).count();
    let right_cnt = right_re.find_iter(right_match).count();

    left_cnt == right_cnt
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
