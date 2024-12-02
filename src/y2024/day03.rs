use std::fs;
use regex::Regex;
use crate::utils::toolbox::parse_usize;

pub(crate) fn day03() {
    println!("{}", part_a(fs::read_to_string("input/2024/day03/input.txt").unwrap()));
    println!("{}", part_b(fs::read_to_string("input/2024/day03/input.txt").unwrap()));
}

fn part_a(input: String) -> usize {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    input.lines()
        .map(|line| {
            re.captures_iter(line).map(|caps| {
                let a = parse_usize(caps.get(1));
                let b = parse_usize(caps.get(2));
                a * b
            }).sum::<usize>()
        })
        .sum()
}

fn part_b(input: String) -> usize {
    let re = Regex::new(r"do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut enabled = true; // Start of program, not line! (lost minutes here)
    input.lines()
        .map(|line| {
            re.captures_iter(line).map(|caps| {
                let split = caps.get(0).unwrap().as_str();
                if split.starts_with("don't") {
                    enabled = false;
                    0
                } else if split.starts_with("do") {
                    enabled = true;
                    0
                } else {
                    let a = parse_usize(caps.get(1));
                    let b = parse_usize(caps.get(2));

                    if enabled {
                        a * b
                    } else { 0 }
                }
            }).sum::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod day03_tests {
    use std::fs;

    use crate::y2024::day03::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(161, part_a(fs::read_to_string("input/2024/day03/test.txt").unwrap()));
        assert_eq!(48, part_b(fs::read_to_string("input/2024/day03/test2.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!(185797128, part_a(fs::read_to_string("input/2024/day03/input.txt").unwrap()));
        assert_eq!(89798695, part_b(fs::read_to_string("input/2024/day03/input.txt").unwrap()));
    }
}
