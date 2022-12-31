use std::collections::HashMap;
use std::fs;
use regex::Regex;

pub(crate) fn day25() {
    let input = fs::read_to_string("input/2017/day25/input.txt").unwrap();
    let (start, machines) = parse(input.as_str());
    println!("{}", turing_machine(start, &machines));
}

fn parse(input: &str) -> (i32, HashMap<&str, ((i32, i32, &str), (i32, i32, &str))>) {
    let re = Regex::new(r"state ([A-Z])").unwrap();
    let parts: Vec<&str> = input.split("\n\n").collect();
    let (_, steps_str) = parts[0].split_once(" after ").unwrap();
    let (steps, _) = steps_str.split_once(" ").unwrap();

    let mut states = HashMap::new();
    for i in 1..parts.len() {
        let lines: Vec<&str> = parts[i].split("\n").collect();
        let g = re.captures(lines[0]).unwrap();
        let name = g.get(1).unwrap().as_str();

        let zero_write = if lines[2].contains("the value 1") { 1 } else { 0 };
        let zero_move = if lines[3].contains("left") { -1 } else { 1 };
        let zero_next = re.captures(lines[4]).unwrap().get(1).unwrap().as_str();

        let one_write = if lines[6].contains("the value 1") { 1 } else { 0 };
        let one_move = if lines[7].contains("left") { -1 } else { 1 };
        let one_next = re.captures(lines[8]).unwrap().get(1).unwrap().as_str();

        states.insert(name, ((zero_write, zero_move, zero_next), (one_write, one_move, one_next)));
    }

    (steps.parse::<i32>().unwrap(), states)
}

fn turing_machine(steps: i32, states: &HashMap<&str, ((i32, i32, &str), (i32, i32, &str))>) -> usize {
    let mut tape: HashMap<i32, i32> = HashMap::new();
    let mut state = states.get("A").unwrap();
    let mut pos = 0;

    for _ in 0..steps {
        let (write, dx, next) = match *tape.get(&pos).unwrap_or(&0) {
            0 => { state.0 }
            1 => { state.1 }
            _ => panic!("tape corrupted")
        };
        tape.insert(pos, write);
        pos += dx;
        state = states.get(next).unwrap();
    }

    tape.values().filter(|v| **v == 1).count()
}

#[cfg(test)]
mod day25_tests {
    use std::fs;
    use crate::y2017::day25::{parse, turing_machine};

    #[test]
    fn test_works() {
        let input = fs::read_to_string("input/2017/day25/test.txt").unwrap();
        let (start, machines) = parse(input.as_str());
        assert_eq!(3, turing_machine(start, &machines));
    }

    #[test]
    fn input_works() {
        let input = fs::read_to_string("input/2017/day25/input.txt").unwrap();
        let (start, machines) = parse(input.as_str());
        assert_eq!(2474, turing_machine(start, &machines));
    }
}
