use std::collections::HashMap;
use std::fs;

pub(crate) fn day12() {
    println!(
        "{}",
        solve(fs::read_to_string("input/2016/day12/input.txt").unwrap(), 0)
    );
    println!(
        "{}",
        solve(fs::read_to_string("input/2016/day12/input.txt").unwrap(), 1)
    );
}

fn solve(input: String, c_val: i32) -> i32 {
    let mut registers: HashMap<&str, i32> = HashMap::new();
    registers.insert("a", 0);
    registers.insert("b", 0);
    registers.insert("c", c_val);
    registers.insert("d", 0);
    let instructions: Vec<&str> = input.lines().collect();

    let mut i = 0;
    while i < instructions.len() as i32 {
        let line = instructions[i as usize];
        let (instr, args) = line.split_once(" ").unwrap();
        match instr {
            "cpy" => {
                let (what, reg) = args.split_once(" ").unwrap();
                let val = if let Some(x) = registers.get(what) {
                    *x
                } else {
                    what.parse::<i32>().unwrap()
                };
                registers.insert(reg, val);
                i += 1;
            }
            "inc" => {
                *registers.entry(args).or_insert(0) += 1;
                i += 1;
            }
            "dec" => {
                *registers.entry(args).or_insert(0) -= 1;
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
            _ => {
                panic!("unknown instruction {}", instr)
            }
        }
    }

    *registers.get("a").unwrap()
}

#[cfg(test)]
mod day12_tests {
    use std::fs;

    use crate::y2016::day12::solve;

    #[test]
    fn test_works() {
        assert_eq!(
            42,
            solve(fs::read_to_string("input/2016/day12/test.txt").unwrap(), 0)
        );
    }

    #[test]
    fn input_works() {
        assert_eq!(
            318083,
            solve(fs::read_to_string("input/2016/day12/input.txt").unwrap(), 0)
        );
        assert_eq!(
            9227737,
            solve(fs::read_to_string("input/2016/day12/input.txt").unwrap(), 1)
        );
    }
}
