use std::collections::HashMap;
use std::fs;

pub(crate) fn day19() {
    let input = fs::read_to_string("input/2019/day19/input.txt").unwrap();
    let code: Vec<i64> = input
        .trim()
        .split(',')
        .map(|c| c.parse().unwrap())
        .collect();

    part_a(&code);
    part_b(&code);
}

fn part_a(code: &Vec<i64>) {
    let mut total = 0;
    let mut map = [[0; 1000]; 200];
    for x in 0..50 {
        for y in 0..50 {
            let mut intcode = intcode_instance(code);
            intcode.run(&mut vec![y, x]);
            let val = intcode.output.pop().unwrap();
            total += val;
            map[y as usize][x as usize] = val;
        }
    }
    println!(
        "There are {} points affected by the beam in 50x50 area",
        total
    );
    for y in 0..50 {
        for x in 0..50 {
            match map[y][x] {
                1 => print!("#"),
                _ => print!("."),
            }
        }
        println!();
    }
}

fn part_b(code: &Vec<i64>) {
    let mut map = [[0; 100]; 100];
    let x_from = 1067;
    let y_from = 1712;

    for x in (x_from)..(x_from + 100) {
        for y in (y_from)..(y_from + 100) {
            let mut intcode = intcode_instance(code);
            intcode.run(&mut vec![y, x]);
            let val = intcode.output.pop().unwrap();
            map[(y - y_from) as usize][(x - x_from) as usize] = val;
        }
    }
    println!();
    for y in 0..100 {
        for x in 0..100 {
            match map[y][x] {
                1 => print!("#"),
                _ => print!("."),
            }
        }
        println!();
    }

    // 1067 * 10000 + 1712
    // 10671712
}

struct IntcodeProcessor {
    instruction_ptr: i64,
    memory: HashMap<i64, i64>,
    output: Vec<i64>,
    relative_base: i64,
}

impl IntcodeProcessor {
    fn run(&mut self, input: &mut Vec<i64>) {
        loop {
            let _instr = self.memory[&self.instruction_ptr];
            let opcode = self.memory[&self.instruction_ptr] % 100;
            let a_mode = (self.memory[&self.instruction_ptr] / 100 % 10) as i32;
            let b_mode = (self.memory[&self.instruction_ptr] / 1000 % 10) as i32;
            let c_mode = (self.memory[&self.instruction_ptr] / 10000 % 10) as i32;

            match opcode {
                1 => {
                    let a = self.memory[&(self.instruction_ptr + 1)];
                    let b = self.memory[&(self.instruction_ptr + 2)];
                    let c = self.memory[&(self.instruction_ptr + 3)];

                    let a_val = self.mem_read(a_mode, a);
                    let b_val = self.mem_read(b_mode, b);

                    self.mem_write(c_mode, c, a_val + b_val);
                    //println!("{}: {}x{} = {} + {}", self.instruction_ptr, c_mode, c, a_val, b_val);
                    self.instruction_ptr += 4;
                }
                2 => {
                    let a = self.memory[&(self.instruction_ptr + 1)];
                    let b = self.memory[&(self.instruction_ptr + 2)];
                    let c = self.memory[&(self.instruction_ptr + 3)];

                    let a_val = self.mem_read(a_mode, a);
                    let b_val = self.mem_read(b_mode, b);

                    self.mem_write(c_mode, c, a_val * b_val);
                    //println!("{}: {}x{} = {} - {}", self.instruction_ptr, c_mode, c, a_val, b_val);
                    self.instruction_ptr += 4;
                }
                3 => {
                    let a = self.memory[&(self.instruction_ptr + 1)];
                    let inp_val = input.pop().unwrap();
                    self.mem_write(a_mode, a, inp_val);
                    //println!("{}: {}x{} = input({})", self.instruction_ptr, a_mode, a, inp_val);
                    self.instruction_ptr += 2;
                }
                4 => {
                    let a = self.memory[&(self.instruction_ptr + 1)];
                    let a_val = self.mem_read(a_mode, a);
                    self.output.push(a_val);
                    //println!("{}: output({}x{})", self.instruction_ptr, a_mode, a_val);
                    self.instruction_ptr += 2;
                }
                5 => {
                    let a = self.memory[&(self.instruction_ptr + 1)];
                    let b = self.memory[&(self.instruction_ptr + 2)];
                    let a_val = self.mem_read(a_mode, a);
                    if a_val != 0 {
                        let b_val = self.mem_read(b_mode, b);
                        //println!("{}: jmp to {} if {} != 0", self.instruction_ptr, b_val, a_val);
                        self.instruction_ptr = b_val;
                    } else {
                        self.instruction_ptr += 3;
                    }
                }
                6 => {
                    let a = self.memory[&(self.instruction_ptr + 1)];
                    let b = self.memory[&(self.instruction_ptr + 2)];
                    let a_val = self.mem_read(a_mode, a);
                    if a_val == 0 {
                        let b_val = self.mem_read(b_mode, b);
                        //println!("{}: jmp to {} if {} == 0", self.instruction_ptr, b_val, a_val);
                        self.instruction_ptr = b_val;
                    } else {
                        self.instruction_ptr += 3;
                    }
                }
                7 => {
                    let a = self.memory[&(self.instruction_ptr + 1)];
                    let b = self.memory[&(self.instruction_ptr + 2)];
                    let c = self.memory[&(self.instruction_ptr + 3)];

                    let a_val = self.mem_read(a_mode, a);
                    let b_val = self.mem_read(b_mode, b);

                    self.mem_write(c_mode, c, if a_val < b_val { 1 } else { 0 });
                    //println!("{}: {}x{} = {} < {}", self.instruction_ptr, c_mode, c, a_val, b_val);
                    self.instruction_ptr += 4;
                }
                8 => {
                    let a = self.memory[&(self.instruction_ptr + 1)];
                    let b = self.memory[&(self.instruction_ptr + 2)];
                    let c = self.memory[&(self.instruction_ptr + 3)];

                    let a_val = self.mem_read(a_mode, a);
                    let b_val = self.mem_read(b_mode, b);

                    self.mem_write(c_mode, c, if a_val == b_val { 1 } else { 0 });
                    //println!("{}: {}x{} = {} == {}", self.instruction_ptr, c_mode, c, a_val, b_val);
                    self.instruction_ptr += 4;
                }
                9 => {
                    let a = self.memory[&(self.instruction_ptr + 1)];
                    let a_val = self.mem_read(a_mode, a);
                    self.relative_base += a_val;
                    //println!("{}: relative base += {}", self.instruction_ptr, a_val);
                    self.instruction_ptr += 2;
                }
                99 => {
                    break;
                }
                _ => {
                    panic!("Unknown opcode {} at pos {}", opcode, self.instruction_ptr);
                }
            }
        }
    }

    fn mem_read(&mut self, mode: i32, val: i64) -> i64 {
        match mode {
            0 => {
                if val < 0 {
                    panic!("Invalid memory address: {}", val)
                }
                *self.memory.entry(val).or_insert(0)
            }
            1 => val,
            2 => {
                let addr = val + self.relative_base;
                if addr < 0 {
                    panic!("Invalid memory address: {}", addr)
                }
                *self.memory.entry(addr).or_insert(0)
            }
            _ => panic!("Unknown mode: {}", mode),
        }
    }

    fn mem_write(&mut self, mode: i32, addr: i64, val: i64) {
        match mode {
            0 => {
                if addr < 0 {
                    panic!("Invalid memory address: {}", addr);
                }
                self.memory.insert(addr, val);
            }
            1 => panic!("How to write in mode 1?"),
            2 => {
                let i = addr + self.relative_base;
                if i < 0 {
                    panic!("Invalid memory address: {}", i)
                }
                self.memory.insert(i, val);
            }
            _ => panic!("Unknown mode: {}", mode),
        };
    }
}

fn intcode_instance(code: &Vec<i64>) -> IntcodeProcessor {
    let mut memory = HashMap::new();
    for i in 0..code.len() {
        memory.insert(i as i64, code[i]);
    }
    IntcodeProcessor {
        instruction_ptr: 0,
        memory,
        output: vec![],
        relative_base: 0,
    }
}
