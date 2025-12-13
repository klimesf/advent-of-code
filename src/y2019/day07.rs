use std::{fs, thread};

use itertools::Itertools;

use blockingqueue::BlockingQueue;
use std::thread::JoinHandle;

pub(crate) fn day07() {
    let input = fs::read_to_string("input/2019/day07/input.txt").unwrap();
    let intcode: Vec<i32> = input
        .trim()
        .split(',')
        .map(|c| c.parse().unwrap())
        .collect();

    part_a(&intcode);
    part_b(&intcode);
}

fn part_a(intcode: &Vec<i32>) {
    let mut max_e_output = i32::MIN;
    let phases = vec![0, 1, 2, 3, 4];
    for perm in phases.iter().permutations(phases.len()).unique() {
        let a_output = run_intcode(intcode.clone(), vec![0, *perm[0]])[0];
        let b_output = run_intcode(intcode.clone(), vec![a_output, *perm[1]])[0];
        let c_output = run_intcode(intcode.clone(), vec![b_output, *perm[2]])[0];
        let d_output = run_intcode(intcode.clone(), vec![c_output, *perm[3]])[0];
        let e_output = run_intcode(intcode.clone(), vec![d_output, *perm[4]])[0];
        max_e_output = max_e_output.max(e_output);
    }
    println!("{}", max_e_output);
}

fn part_b(intcode: &Vec<i32>) {
    let input_a = BlockingQueue::new();
    let input_b = BlockingQueue::new();
    let input_c = BlockingQueue::new();
    let input_d = BlockingQueue::new();
    let input_e = BlockingQueue::new();

    let mut max_e_output = i32::MIN;
    let phases = vec![5, 6, 7, 8, 9];
    for perm in phases.iter().permutations(phases.len()).unique() {
        input_a.push(*perm[0]);
        input_a.push(0);
        input_b.push(*perm[1]);
        input_c.push(*perm[2]);
        input_d.push(*perm[3]);
        input_e.push(*perm[4]);

        spawn_amp(&intcode, &input_a, &input_b);
        spawn_amp(&intcode, &input_b, &input_c);
        spawn_amp(&intcode, &input_c, &input_d);
        spawn_amp(&intcode, &input_d, &input_e);
        spawn_amp(&intcode, &input_e, &input_a).join().unwrap();

        let e_output = input_a.pop();
        max_e_output = max_e_output.max(e_output);
    }
    println!("{}", max_e_output);
}

fn spawn_amp(
    intcode: &Vec<i32>,
    input: &BlockingQueue<i32>,
    output: &BlockingQueue<i32>,
) -> JoinHandle<()> {
    let intcode_clone = intcode.clone();
    let input_clone = input.clone();
    let output_clone = output.clone();
    return thread::spawn(move || {
        run_intcode_parallel(intcode_clone, &input_clone, &output_clone);
    });
}

fn run_intcode(mut intcode: Vec<i32>, mut input: Vec<i32>) -> Vec<i32> {
    let mut instruction_ptr = 0;
    let mut output = vec![];

    loop {
        if intcode[instruction_ptr] > 9999 {
            panic!("c mode > 0, what to do?")
        }

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
            99 => {
                break;
            }
            _ => {
                panic!("Unknown opcode {} at pos {}", opcode, instruction_ptr)
            }
        }
    }

    return output;
}

fn run_intcode_parallel(
    mut intcode: Vec<i32>,
    input: &BlockingQueue<i32>,
    output: &BlockingQueue<i32>,
) {
    let mut instruction_ptr = 0;

    loop {
        if intcode[instruction_ptr] > 9999 {
            panic!("c mode > 0, what to do?")
        }

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
                intcode[c] = input.pop();
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
            99 => {
                break;
            }
            _ => {
                panic!("Unknown opcode {} at pos {}", opcode, instruction_ptr)
            }
        }
    }
}
