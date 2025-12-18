use std::fs;

pub(crate) fn day08() {
    println!("{}", part_a(fs::read_to_string("input/2020/day08/input.txt").unwrap()));
    println!("{}", part_b(fs::read_to_string("input/2020/day08/input.txt").unwrap()));
}

fn part_a(input: String) -> isize {
    let instructions = input
        .lines()
        .map(|line| {
            let (instr, val) = line.split_once(" ").unwrap();
            (instr.to_string(), val.parse::<isize>().unwrap())
        })
        .collect::<Vec<(String, isize)>>();

    let mut visited = vec![false; instructions.len()];
    let mut pos: isize = 0;
    let mut acc: isize = 0;

    while (pos as usize) < instructions.len() {
        let (instruction, value) = &instructions[pos as usize];
        if visited[pos as usize] {
            return acc;
        }
        visited[pos as usize] = true;

        match instruction.as_str() {
            "nop" => {
                pos += 1;
            }
            "acc" => {
                acc += value;
                pos += 1;
            }
            "jmp" => {
                pos += value;
            }
            _ => panic!("Unknown instruction {}", instruction),
        }
    }
    panic!("No instruction repeats twice");
}

fn part_b(input: String) -> isize {
    let instructions = input
        .lines()
        .map(|line| {
            let (instr, val) = line.split_once(" ").unwrap();
            (instr.to_string(), val.parse::<isize>().unwrap())
        })
        .collect::<Vec<(String, isize)>>();

    'outer: for i in 0..instructions.len() {
        let mut new_instructions = instructions.clone();
        if instructions[i].0 == "nop" {
            new_instructions[i].0 = "jmp".to_string();
        } else if instructions[i].0 == "jmp" {
            new_instructions[i].0 = "nop".to_string();
        } else {
            continue;
        }

        let mut visited = vec![false; new_instructions.len()];
        let mut pos: isize = 0;
        let mut acc: isize = 0;

        while (pos as usize) < new_instructions.len() {
            let (instruction, value) = &new_instructions[pos as usize];
            if visited[pos as usize] {
                continue 'outer;
            }
            visited[pos as usize] = true;

            match instruction.as_str() {
                "nop" => {
                    pos += 1;
                }
                "acc" => {
                    acc += value;
                    pos += 1;
                }
                "jmp" => {
                    pos += value;
                }
                _ => panic!("Unknown instruction {}", instruction),
            }
        }
        return acc;
    }

    panic!("No instruction change breaks the infinite loop");
}

#[cfg(test)]
mod day08_tests {
    use std::fs;

    use crate::y2020::day08::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(5, part_a(fs::read_to_string("input/2020/day08/test.txt").unwrap()));
        assert_eq!(8, part_b(fs::read_to_string("input/2020/day08/test.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!(1521, part_a(fs::read_to_string("input/2020/day08/input.txt").unwrap()));
        assert_eq!(1016, part_b(fs::read_to_string("input/2020/day08/input.txt").unwrap()));
    }
}
