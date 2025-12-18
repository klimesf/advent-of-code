use itertools::Itertools;
use std::fs;

pub fn day17(print: fn(String)) {
    print(part_a(fs::read_to_string("input/2024/day17/input.txt").unwrap()));
    print(format!("{}", part_b(fs::read_to_string("input/2024/day17/input.txt").unwrap())));
}

fn part_a(input: String) -> String {
    let (reg_a, reg_b, reg_c, program) = parse_input(input);
    let output = interpret(reg_a, reg_b, reg_c, &program);
    output.iter().map(|x| x.to_string()).join(",")
}

fn part_b(input: String) -> usize {
    let (_, reg_b, reg_c, program) = parse_input(input);

    // The input program works in a loop
    // - it takes a number from register A, does some calculations, outputs a digit,
    // - then it divides the register A by 8, and if A is 0, it exits the loop
    //
    // This means every 3 bits in the register A input will produce 1 digit on the output.
    // Also, going from right to left, the first 3 bits in register A represent the first
    // number on the output, the second 3 bits in register A represent the second number on the
    // output etc.
    //
    // However, the output digits are also influenced by the bits to the left in the number.
    // I.e., the 1st output digit is not influenced only by the first 3 bits in register A,
    // but by all the bits to the left. Only the last output digit is influenced only by it's
    // corresponding 3 bits.
    //
    // Hence, we cannot simply get the number for each output digit and call it a day.
    //
    // So, we are going from the end of the program here.
    // We iterate from 0b000 to 0b111 and shift it left by 3 * position in the program.
    // We try to add this number to every solution from the previous program digit and then use
    // it as the input for register A.
    // If the number at the position in the output matches the number at the position in the program,
    // we will add it to the list of possible solutions.
    // Finally, we move to the next left position in the program.

    let mut stack: Vec<usize> = vec![0];
    for pos in (0..program.len()).rev() {
        let mut new_stack = vec![];
        for i in 0b000..=0b111 {
            for s in &stack {
                let possible_solution = s + (i << (3 * pos as u32));
                let out = interpret(possible_solution, reg_b, reg_c, &program);
                if out.len() == program.len() && out[pos] == program[pos] {
                    new_stack.push(possible_solution);
                }
            }
        }
        stack = new_stack;
    }

    assert!(stack.len() == 1); // We assume only one solution will be valid
    stack.pop().unwrap()
}

fn parse_input(input: String) -> (usize, usize, usize, Vec<usize>) {
    let (regs, prog) = input.trim().split_once("\n\n").unwrap();
    let regs_lines = regs.lines().collect_vec();
    let (_, reg_a_s) = regs_lines[0].split_once(": ").unwrap();
    let reg_a = reg_a_s.parse::<usize>().unwrap();
    let (_, reg_b_s) = regs_lines[1].split_once(": ").unwrap();
    let reg_b = reg_b_s.parse::<usize>().unwrap();
    let (_, reg_c_s) = regs_lines[2].split_once(": ").unwrap();
    let reg_c = reg_c_s.parse::<usize>().unwrap();

    let (_, prog) = prog.split_once(": ").unwrap();
    let program = prog.split(',').map(|x| x.parse::<usize>().unwrap()).collect_vec();
    (reg_a, reg_b, reg_c, program)
}

fn interpret(mut reg_a: usize, mut reg_b: usize, mut reg_c: usize, program: &Vec<usize>) -> Vec<usize> {
    let mut instruction_pointer = 0;
    let mut output = vec![];
    while instruction_pointer < program.len() {
        let instr = program[instruction_pointer];
        let operand = program[instruction_pointer + 1];

        let value = match operand {
            0 | 1 | 2 | 3 => operand,
            4 => reg_a,
            5 => reg_b,
            6 => reg_c,
            _ => 0,
        };

        match instr {
            0 => {
                // adv
                let numerator = reg_a;
                let denominator = 2_usize.pow(value as u32);
                // Truncate int
                reg_a = numerator / denominator;
                instruction_pointer += 2;
            }
            1 => {
                // bxl
                reg_b = reg_b ^ operand;
                instruction_pointer += 2;
            }
            2 => {
                // bst
                reg_b = value % 8;
                instruction_pointer += 2;
            }
            3 => {
                // jnz
                if reg_a == 0 {
                    instruction_pointer += 2;
                } else {
                    instruction_pointer = operand;
                    continue; // Do not incr
                }
            }
            4 => {
                reg_b = reg_b ^ reg_c;
                instruction_pointer += 2;
            }
            5 => {
                // out
                output.push(value % 8);
                instruction_pointer += 2;
            }
            6 => {
                // bdv
                let numerator = reg_a;
                let denominator = 2_usize.pow(value as u32);
                // Truncate int
                reg_b = numerator / denominator;
                instruction_pointer += 2;
            }
            7 => {
                // cdv
                let numerator = reg_a;
                let denominator = 2_usize.pow(value as u32);
                // Truncate int
                reg_c = numerator / denominator;
                instruction_pointer += 2;
            }
            _ => panic!("{}", instr),
        }
    }
    output
}

#[cfg(test)]
mod day17_tests {
    use std::fs;

    use crate::y2024::day17::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!("4,6,3,5,6,3,5,2,1,0", part_a(fs::read_to_string("input/2024/day17/test.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!("1,5,0,1,7,4,1,0,3", part_a(fs::read_to_string("input/2024/day17/input.txt").unwrap()));
        assert_eq!(47910079998866, part_b(fs::read_to_string("input/2024/day17/input.txt").unwrap()));
    }
}
