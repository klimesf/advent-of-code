use std::collections::HashMap;
use std::fs;

use regex::Regex;

use crate::utils::toolbox::parse_i32;

pub(crate) fn day08() {
    let input = fs::read_to_string("input/2017/day08/input.txt").unwrap();
    let instructions = parse(input.as_str());
    let (part_a, part_b) = interpret(&instructions);
    println!("{}", part_a);
    println!("{}", part_b);
}

fn parse(input: &str) -> Vec<(&str, &str, i32, &str, &str, i32)> {
    let re =
        Regex::new(r"^(.+) (inc|dec) ([\-0-9]+) if (.+) (==|>|<|>=|<=|!=) ([\-0-9]+)$").unwrap();
    input
        .lines()
        .map(|line| {
            let g = re.captures(line).unwrap();
            let target = g.get(1).unwrap().as_str();
            let operand = g.get(2).unwrap().as_str();
            let argument = parse_i32(g.get(3));
            let condition_left = g.get(4).unwrap().as_str();
            let condition = g.get(5).unwrap().as_str();
            let condition_right = parse_i32(g.get(6));
            (
                target,
                operand,
                argument,
                condition_left,
                condition,
                condition_right,
            )
        })
        .collect()
}

fn interpret(instructions: &Vec<(&str, &str, i32, &str, &str, i32)>) -> (i32, i32) {
    let mut registers: HashMap<&str, i32> = HashMap::new();
    let mut global_max = i32::MIN;
    for (target, operand, argument, condition_left, condition, condition_right) in instructions {
        let value_left = *registers.entry(condition_left).or_insert(0);
        let condition_met = match *condition {
            "==" => value_left == *condition_right,
            ">" => value_left > *condition_right,
            "<" => value_left < *condition_right,
            ">=" => value_left >= *condition_right,
            "<=" => value_left <= *condition_right,
            "!=" => value_left != *condition_right,
            _ => panic!("Unknown condition {}", condition),
        };

        if !condition_met {
            continue;
        }
        match *operand {
            "inc" => *registers.entry(target).or_insert(0) += argument,
            "dec" => *registers.entry(target).or_insert(0) -= argument,
            _ => panic!("Unknown operand {}", operand),
        }

        let local_max = *registers.values().max().unwrap();
        if local_max > global_max {
            global_max = local_max
        }
    }

    (*registers.values().max().unwrap(), global_max)
}

#[cfg(test)]
mod day08_tests {
    use std::fs;

    use crate::y2017::day08::{interpret, parse};

    #[test]
    fn test_works() {
        let input = fs::read_to_string("input/2017/day08/test.txt").unwrap();
        let instructions = parse(input.as_str());
        assert_eq!(1, interpret(&instructions).0);
        assert_eq!(10, interpret(&instructions).1);
    }

    #[test]
    fn input_works() {
        let input = fs::read_to_string("input/2017/day08/input.txt").unwrap();
        let instructions = parse(input.as_str());
        assert_eq!(4832, interpret(&instructions).0);
        assert_eq!(5443, interpret(&instructions).1);
    }
}
