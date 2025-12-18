use std::fs;

pub(crate) fn day02() {
    let input = fs::read_to_string("input/2019/day02/input.txt").unwrap();
    let intcode: Vec<usize> = input.trim().split(',').map(|c| c.parse().unwrap()).collect();

    part_a(intcode.clone());
    part_b(intcode.clone());
}

fn part_a(mut intcode: Vec<usize>) {
    let noun = 12;
    let verb = 2;
    intcode[1] = noun;
    intcode[2] = verb;
    let output = run_intcode(intcode.clone());
    println!("{}", output);
}

fn part_b(mut intcode: Vec<usize>) {
    for noun in 0..=99 {
        for verb in 0..=99 {
            intcode[1] = noun;
            intcode[2] = verb;
            let output = run_intcode(intcode.clone());
            if output == 19690720 {
                println!("{}", 100 * noun + verb);
                return;
            }
        }
    }
}

fn run_intcode(mut intcode: Vec<usize>) -> usize {
    let mut instruction_ptr = 0;
    loop {
        let opcode = intcode[instruction_ptr];
        match opcode {
            1 => {
                let a = intcode[instruction_ptr + 1];
                let b = intcode[instruction_ptr + 2];
                let c = intcode[instruction_ptr + 3];
                intcode[c] = intcode[a] + intcode[b];
            }
            2 => {
                let a = intcode[instruction_ptr + 1];
                let b = intcode[instruction_ptr + 2];
                let c = intcode[instruction_ptr + 3];
                intcode[c] = intcode[a] * intcode[b];
            }
            99 => {
                break;
            }
            _ => {
                panic!("Unknown opcode: {}", opcode)
            }
        }
        instruction_ptr += 4;
    }

    return intcode[0];
}
