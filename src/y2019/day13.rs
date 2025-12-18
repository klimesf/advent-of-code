use std::collections::HashMap;
use std::fs;

pub(crate) fn day13() {
    let input = fs::read_to_string("input/2019/day13/input.txt").unwrap();
    let code: Vec<i64> = input.trim().split(',').map(|c| c.parse().unwrap()).collect();
    part_a(&code);
    part_b(&code);
}

fn part_a(code: &Vec<i64>) {
    let mut intcode = intcode_instance(&code);
    let mut input = |_: &Vec<i64>| 0;
    intcode.run(&mut input);

    let mut screen = HashMap::new();
    let mut i = 0;
    while i < intcode.output.len() {
        let pos = (intcode.output[i], intcode.output[i + 1]);
        screen.insert(pos, intcode.output[i + 2]);
        i += 3;
    }

    print_screen(&screen);
    let block_count = screen.values().into_iter().filter(|v| **v == 2).count();
    println!("{}", block_count);
}

fn part_b(code: &Vec<i64>) {
    let mut intcode = intcode_instance(&code);
    intcode.memory.insert(0, 2);

    let input = |output: &Vec<i64>| {
        let mut platform_pos = 0;
        let mut ball_pos = 0;

        let mut i = 0;
        while i < output.len() {
            if output[i + 2] == 3 {
                platform_pos = output[i];
            } else if output[i + 2] == 4 {
                ball_pos = output[i];
            }
            i += 3;
        }

        // Just follow the ball with the platform, cannot miss
        if platform_pos > ball_pos {
            return -1;
        }
        if platform_pos < ball_pos {
            return 1;
        }
        return 0;
    };

    intcode.run(&input);
    let mut screen = HashMap::new();
    let mut i = 0;
    while i < intcode.output.len() {
        let pos = (intcode.output[i], intcode.output[i + 1]);
        screen.insert(pos, intcode.output[i + 2]);
        i += 3;
    }

    print_screen(&screen);
    println!("Score: {}", screen.get(&(-1, 0)).unwrap());
}

fn print_screen(screen: &HashMap<(i64, i64), i64>) {
    let max_x = screen.iter().map(|(p, _)| p.0).max().unwrap();
    let max_y = screen.iter().map(|(p, _)| p.1).max().unwrap();

    for y in 0..=max_y {
        for x in 0..=max_x {
            match screen.get(&(x, y)) {
                Some(o) => match o {
                    0 => print!(" "),
                    1 => print!("#"),
                    2 => print!("X"),
                    3 => print!("_"),
                    4 => print!("o"),
                    _ => panic!("Unknown object {}", o),
                },
                None => print!(" "),
            }
        }
        println!();
    }
    for _ in 0..=max_x {
        print!("_");
    }
    println!();
}

struct IntcodeProcessor {
    instruction_ptr: i64,
    memory: HashMap<i64, i64>,
    output: Vec<i64>,
    relative_base: i64,
}

impl IntcodeProcessor {
    fn run(&mut self, input: &dyn Fn(&Vec<i64>) -> i64) {
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
                    self.instruction_ptr += 4;
                }
                2 => {
                    let a = self.memory[&(self.instruction_ptr + 1)];
                    let b = self.memory[&(self.instruction_ptr + 2)];
                    let c = self.memory[&(self.instruction_ptr + 3)];

                    let a_val = self.mem_read(a_mode, a);
                    let b_val = self.mem_read(b_mode, b);

                    self.mem_write(c_mode, c, a_val * b_val);
                    self.instruction_ptr += 4;
                }
                3 => {
                    let a = self.memory[&(self.instruction_ptr + 1)];
                    self.mem_write(a_mode, a, input(&self.output));
                    self.instruction_ptr += 2;
                }
                4 => {
                    let a = self.memory[&(self.instruction_ptr + 1)];
                    let a_val = self.mem_read(a_mode, a);
                    self.output.push(a_val);
                    self.instruction_ptr += 2;
                }
                5 => {
                    let a = self.memory[&(self.instruction_ptr + 1)];
                    let b = self.memory[&(self.instruction_ptr + 2)];
                    let a_val = self.mem_read(a_mode, a);
                    if a_val != 0 {
                        let b_val = self.mem_read(b_mode, b);
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
                    self.instruction_ptr += 4;
                }
                8 => {
                    let a = self.memory[&(self.instruction_ptr + 1)];
                    let b = self.memory[&(self.instruction_ptr + 2)];
                    let c = self.memory[&(self.instruction_ptr + 3)];

                    let a_val = self.mem_read(a_mode, a);
                    let b_val = self.mem_read(b_mode, b);

                    self.mem_write(c_mode, c, if a_val == b_val { 1 } else { 0 });
                    self.instruction_ptr += 4;
                }
                9 => {
                    let a = self.memory[&(self.instruction_ptr + 1)];
                    let a_val = self.mem_read(a_mode, a);
                    self.relative_base += a_val;
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
