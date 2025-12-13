use std::collections::HashMap;
use std::fs;

pub(crate) fn day23() {
    let input = fs::read_to_string("input/2017/day23/input.txt").unwrap();
    let instructions: Vec<&str> = input.lines().collect();
    println!("{}", coprocess_debug_mode(&instructions));
    println!("{}", coprocess(&instructions));
}

fn coprocess_debug_mode(instructions: &Vec<&str>) -> i64 {
    let mut register: HashMap<&str, i64> = HashMap::new();
    let mut i: i64 = 0;
    let mut mul = 0;

    while (i as usize) < instructions.len() {
        let instruction = instructions[i as usize];
        let (name, args) = instruction.split_once(" ").unwrap();
        match name {
            "set" => {
                let (addr, num) = parse_args(args, &register, 0);
                register.insert(addr, num);
            }
            "sub" => {
                let (addr, num) = parse_args(args, &register, 0);
                *register.entry(addr).or_insert(0) -= num;
            }
            "mul" => {
                let (addr, num) = parse_args(args, &register, 0);
                *register.entry(addr).or_insert(0) *= num;
                mul += 1;
            }
            "jnz" => {
                let (check, offset) = parse_args(args, &register, 0);
                let check_num = match check.parse::<i64>() {
                    Ok(n) => n,
                    Err(_) => *register.get(check).unwrap_or(&0),
                };
                if check_num != 0 {
                    i += offset;
                    continue;
                }
            }
            _ => panic!("Unknown instruction {}", name),
        }
        i += 1;
    }
    mul
}

fn coprocess(instructions: &Vec<&str>) -> i64 {
    // This is here jsut so I'm not leaking my whole input. I'm not sure what instructions do other people have in common
    let line_31 = instructions[30];
    let b_inc = line_31[6..line_31.len()].parse::<i32>().unwrap();

    let mut b = 108100;
    let mut f;
    let mut h = 0;

    // Rewritten from the input
    loop {
        f = 1;
        'outer: for d in 2..=b {
            for e in 2..=b {
                if (d * e) == b {
                    f = 0; // <-- important
                    break 'outer;
                }
                if d * e > b {
                    break;
                }
            }
            if d * 2 > b {
                break;
            }
        }
        if f == 0 {
            h += 1;
        }
        if b - 125100 == 0 {
            break;
        }
        b -= b_inc;
    }

    h
}

fn parse_args<'a>(args: &'a str, register: &HashMap<&'a str, i64>, default: i64) -> (&'a str, i64) {
    let (addr, val) = args.split_once(" ").unwrap();
    let num = match val.parse::<i64>() {
        Ok(num) => num,
        Err(_) => *register.get(val).unwrap_or(&default),
    };
    (addr, num)
}

#[cfg(test)]
mod day23_tests {
    use crate::y2017::day23::{coprocess, coprocess_debug_mode};
    use std::fs;

    #[test]
    fn input_works() {
        let input = fs::read_to_string("input/2017/day23/input.txt").unwrap();
        let instructions: Vec<&str> = input.lines().collect();
        assert_eq!(6241, coprocess_debug_mode(&instructions));
        assert_eq!(909, coprocess(&instructions));
    }
}
