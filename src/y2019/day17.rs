use std::collections::HashMap;
use std::fs;

pub(crate) fn day17() {
    let input = fs::read_to_string("input/2019/day17/input.txt").unwrap();
    let code: Vec<i64> = input.trim().split(',').map(|c| c.parse().unwrap()).collect();

    part_a(code.clone());
    part_b(code.clone());
}

fn part_a(code: Vec<i64>) {
    let mut intcode = intcode_instance(&code);
    intcode.run(&mut vec!());
    let map = screen_to_2d_map(&intcode.output);

    // Part A -> count intersections
    let intersections = get_intersections(&map);
    print_screen(&map, &intersections);
    println!("{:?}", intersections);
    println!("Total intersection score is: {}", intersections.iter().map(|(y, x)| y * x).sum::<usize>());
}

fn part_b(mut code: Vec<i64>) {
    // Part B -> use handsolved solution and pass it to the intcode program
    code[0] = 2; // Activate part B

    let mut input = vec!();

    // Add main movement routine A,C,A,C,A,B,C,B,A,B\n
    ['A', ',', 'C', ',', 'A', ',', 'C', ',', 'A', ',', 'B', ',', 'C', ',', 'B', ',', 'A', ',', 'B', '\n'].iter()
        .map(|c| *c as i64)
        .for_each(|c| input.push(c));

    // Add movement function A L10,L12,R6,\n
    ['L', ',', '1', '0', ',', 'L', ',', '1', '2', ',', 'R', ',', '6', '\n'].iter()
        .map(|c| *c as i64)
        .for_each(|c| input.push(c));

    // Add movement function B L,10,R,10,R,6,L,4\n
    ['L', ',', '1', '0', ',', 'R', ',', '1', '0', ',', 'R', ',', '6', ',', 'L', ',', '4', '\n'].iter()
        .map(|c| *c as i64)
        .for_each(|c| input.push(c));

    // Add movement function C R,10,L,4,L,4,L,12\n
    ['R', ',', '1', '0', ',', 'L', ',', '4', ',', 'L', ',', '4', ',', 'L', ',', '1', '2', '\n'].iter()
        .map(|c| *c as i64)
        .for_each(|c| input.push(c));

    // Continuous stream
    ['n', '\n'].iter()
        .map(|c| *c as i64)
        .for_each(|c| input.push(c));

    println!("Passing this sequence to intcode:");
    input.iter().map(|i| *i as u8 as char).for_each(|c| print!("{}", c));
    println!();
    input.reverse();

    let mut intcode = intcode_instance(&code);
    intcode.run(&mut input);

    // Output
    let map = screen_to_2d_map(&intcode.output);
    print_screen(&map, &vec!());

    println!("The cleanup score is: {:?}", intcode.output.last().unwrap());
}

fn get_intersections(input: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut intersections = vec!();
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            if input[y][x] != '#' {
                continue;
            }

            let mut n_count = 0;
            if y > 0 && input[y - 1][x] == '#' {
                n_count += 1;
            }
            if x > 0 && input[y][x - 1] == '#' {
                n_count += 1;
            }
            if y < (input.len() - 1) && input[y + 1][x] == '#' {
                n_count += 1;
            }
            if x < (input[y].len() - 1) && input[y][x + 1] == '#' {
                n_count += 1;
            }

            if n_count > 3 {
                intersections.push((x, y));
            }
        }
    }
    return intersections;
}

fn screen_to_2d_map(input: &Vec<i64>) -> Vec<Vec<char>> {
    let mut map = vec!();
    let mut row = vec!();
    for i in input {
        if *i == 10 {
            if !row.is_empty() {
                map.push(row.clone());
            }
            row = vec!();
        } else {
            row.push(*i as u8 as char);
        }
    }
    return map;
}

fn print_screen(input: &Vec<Vec<char>>, intersections: &Vec<(usize, usize)>) {
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            if intersections.contains(&(x, y)) {
                print!("O");
            } else {
                print!("{}", input[y][x]);
            }
        }
        println!();
    }
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
                    self.mem_write(a_mode, a, input.pop().unwrap());
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
                99 => { break; }
                _ => { panic!("Unknown opcode {} at pos {}", opcode, self.instruction_ptr); }
            }
        }
    }

    fn mem_read(&mut self, mode: i32, val: i64) -> i64 {
        match mode {
            0 => {
                if val < 0 { panic!("Invalid memory address: {}", val) }
                *self.memory.entry(val).or_insert(0)
            }
            1 => val,
            2 => {
                let addr = val + self.relative_base;
                if addr < 0 { panic!("Invalid memory address: {}", addr) }
                *self.memory.entry(addr).or_insert(0)
            }
            _ => panic!("Unknown mode: {}", mode),
        }
    }

    fn mem_write(&mut self, mode: i32, addr: i64, val: i64) {
        match mode {
            0 => {
                if addr < 0 { panic!("Invalid memory address: {}", addr); }
                self.memory.insert(addr, val);
            }
            1 => panic!("How to write in mode 1?"),
            2 => {
                let i = addr + self.relative_base;
                if i < 0 { panic!("Invalid memory address: {}", i) }
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
        output: vec!(),
        relative_base: 0,
    }
}
