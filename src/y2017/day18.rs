use std::collections::{HashMap, VecDeque};
use std::fs;

pub(crate) fn day18() {
    let input = fs::read_to_string("input/2017/day18/input.txt").unwrap();
    let instructions: Vec<&str> = input.lines().collect();
    println!("{}", duet(&instructions));
    println!("{}", dual_core_duet(&instructions));
}

fn duet(instructions: &Vec<&str>) -> i64 {
    let mut register: HashMap<&str, i64> = HashMap::new();
    let mut last_played_sound = 0;
    let mut i = 0;

    loop {
        let instruction = instructions[i as usize];
        let (name, args) = instruction.split_once(" ").unwrap();
        match name {
            "set" => {
                let (addr, num) = parse_args(args, &register, 0);
                register.insert(addr, num);
            }
            "snd" => {
                last_played_sound = *register.get(args).unwrap_or(&0);
            }
            "add" => {
                let (addr, num) = parse_args(args, &register, 0);
                *register.entry(addr).or_insert(0) += num;
            }
            "mul" => {
                let (addr, num) = parse_args(args, &register, 0);
                *register.entry(addr).or_insert(0) *= num;
            }
            "mod" => {
                let (addr, num) = parse_args(args, &register, 0);
                *register.entry(addr).or_insert(0) %= num;
            }
            "rcv" => {
                let val = *register.get(args).unwrap();
                if val != 0 {
                    return last_played_sound;
                }
            }
            "jgz" => {
                let (check, offset) = parse_args(args, &register, 0);
                if *register.get(check).unwrap_or(&0) > 0 {
                    i += offset;
                    continue;
                }
            }
            _ => panic!("Unknown instruction {}", name),
        }
        i += 1;
    }
}

fn dual_core_duet(instructions: &Vec<&str>) -> i64 {
    let mut registers = [HashMap::new(), HashMap::new()];
    let mut current_program = 0;
    let mut pos = [0; 2];
    let mut queues = [VecDeque::new(), VecDeque::new()];
    let mut sent_ctr = [0; 2];
    let mut stuck_ctr = [0; 2];

    loop {
        let instruction = instructions[pos[current_program] as usize];
        let register = &mut registers[current_program];
        let (name, args) = instruction.split_once(" ").unwrap();
        let default = current_program as i64;
        match name {
            "set" => {
                let (addr, num) = parse_args(args, &register, default);
                register.insert(addr, num);
            }
            "snd" => {
                let num = match args.parse::<i64>() {
                    Ok(num) => num,
                    Err(_) => *register.get(args).unwrap_or(&default),
                };
                // println!("prg {} sending {}", current_program, num);
                queues[current_program].push_back(num);
                sent_ctr[current_program] += 1;
            }
            "add" => {
                let (addr, num) = parse_args(args, &register, default);
                *register.entry(addr).or_insert(default) += num;
            }
            "mul" => {
                let (addr, num) = parse_args(args, &register, default);
                *register.entry(addr).or_insert(default) *= num;
            }
            "mod" => {
                let (addr, num) = parse_args(args, &register, default);
                *register.entry(addr).or_insert(default) %= num;
            }
            "rcv" => {
                let queue = &mut queues[(current_program + 1) % 2];
                if queue.is_empty() {
                    stuck_ctr[current_program] += 1;
                    if stuck_ctr[0] > 1 && stuck_ctr[1] > 1 {
                        break; // deadlock
                    }
                    // println!("{} stuck on {}, switching to {} to pos {}", current_program, pos[current_program], (current_program + 1) % 2, pos[(current_program + 1) % 2]);
                    current_program = (current_program + 1) % 2;
                    continue;
                } else {
                    if stuck_ctr[current_program] > 0 {
                        stuck_ctr[current_program] = 0
                    };
                    let num = queue.pop_front().unwrap();
                    // println!("prg {} recieving {}", current_program, num);
                    register.insert(args, num);
                }
            }
            "jgz" => {
                let (check, offset) = parse_args(args, &register, default);
                let check_num = match check.parse::<i64>() {
                    Ok(n) => n,
                    Err(_) => *register.get(check).unwrap_or(&0),
                };
                if check_num > 0 {
                    pos[current_program] += offset;
                    continue;
                }
            }
            _ => panic!("Unknown instruction {}", name),
        }
        pos[current_program] += 1;
    }

    sent_ctr[1]
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
mod day18_tests {
    use crate::y2017::day18::{dual_core_duet, duet};
    use std::fs;

    #[test]
    fn test_works() {
        let input = fs::read_to_string("input/2017/day18/test.txt").unwrap();
        let instructions: Vec<&str> = input.lines().collect();
        assert_eq!(4, duet(&instructions));
    }

    #[test]
    fn test_2_works() {
        let input = fs::read_to_string("input/2017/day18/test_2.txt").unwrap();
        let instructions: Vec<&str> = input.lines().collect();
        assert_eq!(3, dual_core_duet(&instructions));
    }

    #[test]
    fn input_works() {
        let input = fs::read_to_string("input/2017/day18/input.txt").unwrap();
        let instructions: Vec<&str> = input.lines().collect();
        assert_eq!(3188, duet(&instructions));
        assert_eq!(3175, dual_core_duet(&instructions));
    }
}
