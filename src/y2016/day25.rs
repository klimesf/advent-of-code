use std::collections::HashMap;
use std::fs;

pub(crate) fn day25() {
    println!(
        "{}",
        part_a(fs::read_to_string("input/2016/day25/input.txt").unwrap())
    );
}

fn part_a(input: String) -> i64 {
    let mut i = 0;
    loop {
        let out = easter_bunny_processor(input.clone(), i);
        let mut got_it = false;
        for j in 0..out.len() {
            if out[j] == ((j as i64) % 2) {
                got_it = true;
            } else {
                got_it = false;
                break;
            }
        }

        if got_it {
            return i;
        }
        i += 1
    }
}

fn easter_bunny_processor(input: String, initial_a: i64) -> Vec<i64> {
    let mut registers: HashMap<&str, i64> = HashMap::new();
    registers.insert("a", initial_a);
    registers.insert("b", 0);
    registers.insert("c", 0);
    registers.insert("d", 0);
    let instructions: Vec<&str> = input.lines().collect();

    let mut out = vec![];
    let mut i = 0;
    while i < instructions.len() as i64 {
        if out.len() > 10 {
            return out;
        }
        let line = instructions[i as usize];
        let (instr, args) = line.split_once(" ").unwrap();
        match instr {
            "cpy" => {
                let (what, reg) = args.split_once(" ").unwrap();
                let val = if let Some(x) = registers.get(what) {
                    *x
                } else {
                    what.parse::<i64>().unwrap()
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
                    test.parse::<i64>().unwrap()
                };

                if test_val != 0 {
                    let jump_val = if let Some(x) = registers.get(jump) {
                        *x
                    } else {
                        jump.parse::<i64>().unwrap()
                    };
                    i += jump_val;
                } else {
                    i += 1;
                }
            }
            "out" => {
                let val = *registers.get(args).unwrap();
                out.push(val);
                i += 1;
            }
            _ => {
                panic!("unknown instruction {}", instr)
            }
        }
    }

    out
}

#[cfg(test)]
mod day25_tests {
    use std::fs;

    use crate::y2016::day25::part_a;

    #[test]
    fn input_works() {
        assert_eq!(
            0,
            part_a(fs::read_to_string("input/2016/day25/input.txt").unwrap())
        );
    }
}
