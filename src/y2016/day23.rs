use std::collections::HashMap;
use std::fs;

pub(crate) fn day23() {
    println!("{}", solve(fs::read_to_string("input/2016/day23/input.txt").unwrap(), 7));
    println!("{}", solve(fs::read_to_string("input/2016/day23/input.txt").unwrap(), 12));
}

fn solve(input: String, a_initial: i32) -> i32 {
    let mut registers: HashMap<String, i32> = HashMap::new();
    registers.insert("a".to_string(), a_initial);
    registers.insert("b".to_string(), 0);
    registers.insert("c".to_string(), 0);
    registers.insert("d".to_string(), 0);
    let mut instructions: Vec<String> = input.lines().map(|line| line.to_string()).collect();

    let mut i = 0;
    while i < instructions.len() as i32 {
        println!("a:{:3} b:{:3} c:{:3} d:{:3}", registers.get(&"a".to_string()).unwrap(), registers.get(&"b".to_string()).unwrap(), registers.get(&"c".to_string()).unwrap(), registers.get(&"d".to_string()).unwrap());
        let line = instructions[i as usize].clone();
        let (instr, args) = line.split_once(" ").unwrap();
        match instr {
            "cpy" => {
                let (what, reg) = args.split_once(" ").unwrap();
                let val = if let Some(x) = registers.get(what) {
                    *x
                } else {
                    what.parse::<i32>().unwrap()
                };
                registers.insert(reg.to_string(), val);
                i += 1;
            }
            "inc" => {
                *registers.entry(args.to_string()).or_insert(0) += 1;
                i += 1;
            }
            "dec" => {
                *registers.entry(args.to_string()).or_insert(0) -= 1;
                i += 1;
            }
            "jnz" => {
                let (test, jump) = args.split_once(" ").unwrap();
                let test_val = if let Some(x) = registers.get(test) {
                    *x
                } else {
                    test.parse::<i32>().unwrap()
                };

                if test_val != 0 {
                    let jump_val = if let Some(x) = registers.get(jump) {
                        *x
                    } else {
                        jump.parse::<i32>().unwrap()
                    };
                    i += jump_val;
                } else {
                    i += 1;
                }
            }
            "tgl" => {
                let what = *registers.entry(args.to_string()).or_insert(0);
                if i + what < 0 || i + what >= instructions.len() as i32 {
                    i += 1;
                    continue
                }
                let mut new_instructions = instructions.clone();

                let to_toggle = instructions[(i + what) as usize].clone();
                let (to_toggle_instr, to_toggle_args) = to_toggle.split_once(" ").unwrap();
                match to_toggle_instr {
                    "cpy" => {
                        new_instructions[(i + what) as usize] = format!("jnz {}", to_toggle_args);
                    }
                    "inc" => {
                        new_instructions[(i + what) as usize] = format!("dec {}", to_toggle_args);
                    }
                    "dec" => {
                        new_instructions[(i + what) as usize] = format!("inc {}", to_toggle_args);
                    }
                    "jnz" => {
                        new_instructions[(i + what) as usize] = format!("cpy {}", to_toggle_args);
                    }
                    "tgl" => {
                        new_instructions[(i + what) as usize] = format!("inc {}", to_toggle_args);
                    }
                    _ => { panic!("unknown instruction {}", instr) }
                }
                instructions = new_instructions;
                i += 1;
            }
            _ => { panic!("unknown instruction {}", instr) }
        }
    }

    *registers.get("a").unwrap()
}

#[cfg(test)]
mod day23_tests {
    use std::fs;

    use crate::y2016::day23::{solve};

    #[test]
    fn test_works() {
        assert_eq!(3, solve(fs::read_to_string("input/2016/day23/test.txt").unwrap(), 7));
    }

    #[test]
    fn input_works() {
        assert_eq!(14065, solve(fs::read_to_string("input/2016/day23/input.txt").unwrap(), 7));
        assert_eq!(0, solve(fs::read_to_string("input/2016/day23/input.txt").unwrap(), 12));
    }
}
