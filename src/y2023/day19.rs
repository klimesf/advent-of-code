use crate::utils::toolbox::parse_usize;
use regex::Regex;
use std::collections::HashMap;
use std::fs;

pub(crate) fn day19() {
    println!("{}", part_a(fs::read_to_string("input/2023/day19/input.txt").unwrap()));
    println!("{}", part_b(fs::read_to_string("input/2023/day19/input.txt").unwrap()));
}

fn part_a(input: String) -> usize {
    let (workflows_input, ratings_input) = input.split_once("\n\n").unwrap();
    let workflows = parse_workflows(workflows_input);

    let rer = Regex::new(r"^\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}$").unwrap();
    let ratings: Vec<HashMap<&str, usize>> = ratings_input
        .lines()
        .map(|line| {
            let caps = rer.captures(line).unwrap();
            let x = parse_usize(caps.get(1));
            let m = parse_usize(caps.get(2));
            let a = parse_usize(caps.get(3));
            let s = parse_usize(caps.get(4));

            let mut vals = HashMap::new();
            vals.insert("x", x);
            vals.insert("m", m);
            vals.insert("a", a);
            vals.insert("s", s);
            vals
        })
        .collect();

    let mut accepted: Vec<HashMap<&str, usize>> = vec![];
    'outer: for rating in ratings {
        let mut wf_key = "in";
        'middle: loop {
            if wf_key == "A" {
                accepted.push(rating.clone());
                continue 'outer;
            } else if wf_key == "R" {
                continue 'outer;
            }

            let rules = workflows.get(wf_key).unwrap();
            for rule in rules {
                match rule {
                    Rule::ACCEPTED => {
                        accepted.push(rating.clone());
                        continue 'outer;
                    }
                    Rule::REJECTED => {
                        continue 'outer;
                    }
                    Rule::GOTO(new_wf_key) => {
                        wf_key = new_wf_key;
                        continue 'middle;
                    }
                    Rule::GT(prop, val, to) => {
                        if rating.get(prop.as_str()).unwrap() > val {
                            wf_key = to;
                            continue 'middle;
                        }
                    }
                    Rule::LT(prop, val, to) => {
                        if rating.get(prop.as_str()).unwrap() < val {
                            wf_key = to;
                            continue 'middle;
                        }
                    }
                }
            }
        }
    }

    accepted
        .iter()
        .map(|rating| rating.values().sum::<usize>())
        .sum::<usize>()
}

fn part_b(input: String) -> usize {
    let (workflows_input, _) = input.split_once("\n\n").unwrap();
    let workflows = parse_workflows(workflows_input);

    let mut stack: Vec<((usize, usize), (usize, usize), (usize, usize), (usize, usize), &str, usize)> =
        vec![((1, 4000), (1, 4000), (1, 4000), (1, 4000), "in", 0)];
    let mut accepted: Vec<((usize, usize), (usize, usize), (usize, usize), (usize, usize))> = vec![];
    while let Some(range) = stack.pop() {
        let (x, m, a, s, wf_key, rule_key) = range;
        if wf_key == "A" {
            accepted.push((x, m, a, s));
            continue;
        } else if wf_key == "R" {
            continue;
        }

        // Invalid bounds check
        if x.0 > x.1 || m.0 > m.1 || a.0 > a.1 || s.0 > s.1 {
            continue;
        }

        let rules = workflows.get(wf_key).unwrap();
        let rule = &rules[rule_key];
        match rule {
            Rule::ACCEPTED => {
                accepted.push((x, m, a, s));
                continue;
            }
            Rule::REJECTED => {
                continue;
            }
            Rule::GOTO(new_wf_key) => {
                stack.push((x, m, a, s, new_wf_key, 0));
                continue;
            }
            Rule::GT(prop, val, to) => match prop.as_str() {
                "x" => {
                    stack.push(((val + 1, x.1), m, a, s, to.as_str(), 0));
                    stack.push(((x.0, *val), m, a, s, wf_key, rule_key + 1));
                }
                "m" => {
                    stack.push((x, (val + 1, m.1), a, s, to.as_str(), 0));
                    stack.push((x, (m.0, *val), a, s, wf_key, rule_key + 1));
                }
                "a" => {
                    stack.push((x, m, (val + 1, a.1), s, to.as_str(), 0));
                    stack.push((x, m, (a.0, *val), s, wf_key, rule_key + 1));
                }
                "s" => {
                    stack.push((x, m, a, (val + 1, s.1), to.as_str(), 0));
                    stack.push((x, m, a, (s.0, *val), wf_key, rule_key + 1));
                }
                _ => {
                    panic!("unknown prop {}", prop)
                }
            },
            Rule::LT(prop, val, to) => match prop.as_str() {
                "x" => {
                    stack.push(((x.0, val - 1), m, a, s, to.as_str(), 0));
                    stack.push(((*val, x.1), m, a, s, wf_key, rule_key + 1));
                }
                "m" => {
                    stack.push((x, (m.0, val - 1), a, s, to.as_str(), 0));
                    stack.push((x, (*val, m.1), a, s, wf_key, rule_key + 1));
                }
                "a" => {
                    stack.push((x, m, (a.0, val - 1), s, to.as_str(), 0));
                    stack.push((x, m, (*val, a.1), s, wf_key, rule_key + 1));
                }
                "s" => {
                    stack.push((x, m, a, (s.0, val - 1), to.as_str(), 0));
                    stack.push((x, m, a, (*val, s.1), wf_key, rule_key + 1));
                }
                _ => {
                    panic!("unknown prop {}", prop)
                }
            },
        }
    }

    accepted
        .iter()
        .map(|(x, m, a, s)| (x.1 - x.0 + 1) * (m.1 - m.0 + 1) * (a.1 - a.0 + 1) * (s.1 - s.0 + 1))
        .sum()
}

fn parse_workflows(workflows_input: &str) -> HashMap<&str, Vec<Rule>> {
    let mut workflows: HashMap<&str, Vec<Rule>> = HashMap::new();
    let rew = Regex::new(r"^(.+)\{(.+)\}$").unwrap();
    workflows_input.lines().for_each(|line| {
        let caps = rew.captures(line).unwrap();
        let key = caps.get(1).unwrap().as_str();
        let rules_str = caps.get(2).unwrap().as_str();

        let rules = rules_str
            .split(",")
            .map(|expr| {
                if expr == "A" {
                    return Rule::ACCEPTED;
                }
                if expr == "R" {
                    return Rule::REJECTED;
                }
                if !expr.contains(":") {
                    return Rule::GOTO(expr.to_string());
                }

                let (rule, to) = expr.split_once(":").unwrap();

                return if rule.contains(">") {
                    let (prop, val) = rule.split_once(">").unwrap();
                    Rule::GT(prop.to_string(), val.parse().unwrap(), to.to_string())
                } else if rule.contains("<") {
                    let (prop, val) = rule.split_once("<").unwrap();
                    Rule::LT(prop.to_string(), val.parse().unwrap(), to.to_string())
                } else {
                    panic!("Unknown rule {}", rule)
                };
            })
            .collect();

        workflows.insert(key, rules);
    });
    workflows
}

enum Rule {
    ACCEPTED,
    REJECTED,
    GOTO(String),
    GT(String, usize, String),
    LT(String, usize, String),
}

#[cfg(test)]
mod day19_tests {
    use std::fs;

    use crate::y2023::day19::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(19114, part_a(fs::read_to_string("input/2023/day19/test.txt").unwrap()));
        assert_eq!(167409079868000, part_b(fs::read_to_string("input/2023/day19/test.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!(367602, part_a(fs::read_to_string("input/2023/day19/input.txt").unwrap()));
        assert_eq!(125317461667458, part_b(fs::read_to_string("input/2023/day19/input.txt").unwrap()));
    }
}
