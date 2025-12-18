use std::collections::HashMap;
use std::fs;

pub(crate) fn day23() {
    println!("{}", computor(fs::read_to_string("input/2015/day23/input.txt").unwrap(), "b", 0));
    println!("{}", computor(fs::read_to_string("input/2015/day23/input.txt").unwrap(), "b", 1));
}

fn computor(input: String, res_reg: &str, initial_a: usize) -> usize {
    let instructions: Vec<&str> = input.lines().collect();
    let mut i = 0;
    let mut regs: HashMap<&str, usize> = HashMap::new();
    regs.insert("a", initial_a);

    while i < instructions.len() {
        let (instr, args) = instructions[i].split_once(" ").unwrap();
        match instr {
            "hlf" => {
                *regs.entry(args).or_insert(0) /= 2;
                i += 1;
            }
            "inc" => {
                *regs.entry(args).or_insert(0) += 1;
                i += 1;
            }
            "tpl" => {
                *regs.entry(args).or_insert(0) *= 3;
                i += 1;
            }
            "jmp" => {
                let jump = args.parse::<i32>().unwrap();
                i = (i as i32 + jump) as usize;
            }
            "jie" => {
                let (l, r) = args.split_once(", ").unwrap();
                let test = regs.get(l).unwrap_or(&0);
                let jump = r.parse::<i32>().unwrap();
                if test % 2 == 0 {
                    i = (i as i32 + jump) as usize;
                } else {
                    i += 1;
                }
            }
            "jio" => {
                let (l, r) = args.split_once(", ").unwrap();
                let test = regs.get(l).unwrap_or(&0);
                let jump = r.parse::<i32>().unwrap();
                if *test == 1 {
                    i = (i as i32 + jump) as usize;
                } else {
                    i += 1;
                }
            }
            _ => {
                panic!("Unknown instruction {}", instr)
            }
        }
    }

    *regs.get(res_reg).unwrap_or(&0)
}

#[cfg(test)]
mod day23_tests {
    use std::fs;

    use crate::y2015::day23::computor;

    #[test]
    fn test_works() {
        assert_eq!(2, computor(fs::read_to_string("input/2015/day23/test.txt").unwrap(), "a", 0));
    }

    #[test]
    fn input_works() {
        assert_eq!(307, computor(fs::read_to_string("input/2015/day23/input.txt").unwrap(), "b", 0));
        assert_eq!(160, computor(fs::read_to_string("input/2015/day23/input.txt").unwrap(), "b", 1));
    }
}
