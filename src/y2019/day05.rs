use std::fs;

pub(crate) fn day05() {
    let input = fs::read_to_string("input/2019/day05/input.txt").unwrap();
    let intcode: Vec<i32> = input.trim().split(',').map(|c| c.parse().unwrap()).collect();

    part_a(&intcode);
    part_b(&intcode);
}

fn part_a(intcode: &Vec<i32>) {
    let output = run_intcode(intcode.clone(), vec![1]);
    println!("{:?}", output);
}

fn part_b(intcode: &Vec<i32>) {
    let output = run_intcode(intcode.clone(), vec![5]);
    println!("{:?}", output);
}

fn run_intcode(mut intcode: Vec<i32>, mut input: Vec<i32>) -> Vec<i32> {
    let mut instruction_ptr = 0;
    let mut output = vec!();

    loop {
        if intcode[instruction_ptr] > 9999 { panic!("c mode > 0, what to do?") }

        let opcode = intcode[instruction_ptr] % 100;
        let a_mode = intcode[instruction_ptr] / 100 % 10;
        let b_mode = intcode[instruction_ptr] / 1000 % 10;

        match opcode {
            1 => {
                let a = intcode[instruction_ptr + 1] as usize;
                let b = intcode[instruction_ptr + 2] as usize;
                let c = intcode[instruction_ptr + 3] as usize;

                let a_val = if a_mode == 1 { a as i32 } else { intcode[a] };
                let b_val = if b_mode == 1 { b as i32 } else { intcode[b] };
                intcode[c] = a_val + b_val;
                instruction_ptr += 4;
            }
            2 => {
                let a = intcode[instruction_ptr + 1] as usize;
                let b = intcode[instruction_ptr + 2] as usize;
                let c = intcode[instruction_ptr + 3] as usize;

                let a_val = if a_mode == 1 { a as i32 } else { intcode[a] };
                let b_val = if b_mode == 1 { b as i32 } else { intcode[b] };
                intcode[c] = a_val * b_val;
                instruction_ptr += 4;
            }
            3 => {
                let c = intcode[instruction_ptr + 1] as usize;
                intcode[c] = input.pop().unwrap();
                instruction_ptr += 2;
            }
            4 => {
                let c = intcode[instruction_ptr + 1] as usize;
                output.push(intcode[c]);
                instruction_ptr += 2;
            }
            5 => {
                let a = intcode[instruction_ptr + 1];
                let b = intcode[instruction_ptr + 2];
                let a_val = if a_mode > 0 { a } else { intcode[a as usize] };
                if a_val != 0 {
                    let b_val = if b_mode > 0 { b } else { intcode[b as usize] };
                    instruction_ptr = b_val as usize;
                } else {
                    instruction_ptr += 3;
                }
            }
            6 => {
                let a = intcode[instruction_ptr + 1];
                let b = intcode[instruction_ptr + 2];
                let a_val = if a_mode > 0 { a } else { intcode[a as usize] };
                if a_val == 0 {
                    let b_val = if b_mode > 0 { b } else { intcode[b as usize] };
                    instruction_ptr = b_val as usize;
                } else {
                    instruction_ptr += 3;
                }
            }
            7 => {
                let a = intcode[instruction_ptr + 1] as usize;
                let b = intcode[instruction_ptr + 2] as usize;
                let c = intcode[instruction_ptr + 3] as usize;

                let a_val = if a_mode == 1 { a as i32 } else { intcode[a] };
                let b_val = if b_mode == 1 { b as i32 } else { intcode[b] };
                intcode[c] = if a_val < b_val { 1 } else { 0 };
                instruction_ptr += 4;
            }
            8 => {
                let a = intcode[instruction_ptr + 1] as usize;
                let b = intcode[instruction_ptr + 2] as usize;
                let c = intcode[instruction_ptr + 3] as usize;

                let a_val = if a_mode == 1 { a as i32 } else { intcode[a] };
                let b_val = if b_mode == 1 { b as i32 } else { intcode[b] };
                intcode[c] = if a_val == b_val { 1 } else { 0 };
                instruction_ptr += 4;
            }
            99 => { break; }
            _ => { panic!("Unknown opcode {} at pos {}", opcode, instruction_ptr) }
        }
    }

    return output;
}
