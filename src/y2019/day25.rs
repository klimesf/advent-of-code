use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::fs;

pub(crate) fn day25() {
    let input = fs::read_to_string("input/2019/day25/input.txt").unwrap();
    let code: Vec<i64> = input
        .trim()
        .split(',')
        .map(|c| c.parse().unwrap())
        .collect();

    let items = [
        "dark matter",
        "hypercube",
        "coin",
        "klein bottle",
        "shell",
        "easter egg",
        "astrolabe",
        "tambourine",
    ];

    for i in 0..256 {
        let mut intcode = intcode_instance(&code);
        let mut input = VecDeque::new();
        path_to_pressure_room(&mut input);

        let str: Vec<char> = format!("{:08b}", i).chars().collect();
        for j in 0..8 {
            if str[j] == '1' {
                format!("drop {}\n", items[j])
                    .chars()
                    .for_each(|c| input.push_back(c as i64));
            }
        }
        "south\n".chars().for_each(|c| input.push_back(c as i64));

        intcode.run(&mut input);

        let mut s = String::new();
        for i in (intcode.output.len() - 249)..(intcode.output.len()) {
            s.push(intcode.output[i]);
        }
        if !s.contains("Security Checkpoint") {
            println!(
                "{}",
                intcode.output.iter().map(|c| *c as u8 as char).join("")
            );
            break;
        }
    }
}

fn path_to_pressure_room(input: &mut VecDeque<i64>) {
    //                                                                                                                     HALLWAY (easter egg)
    //                                                                                                                          n
    //                                                                           OBSERVATORY        e   ENGINEERING              NAVIGATION (klein bottle)
    //                                                                                n                                               n
    //     STORAGE (coin)  w      KITCHEN (lava)   e   WARP (escape pod) e ARC |   HOLODECK (tamb)  e   SICK BAY (astrolabe)  e   CORRIDOR
    //         s                       n                                              n                       s
    //     SECURITY            HOT CHOC FNT (ge)   w   SCIENCE LAB (dm)        w  HULL BREACH          GIFT WRAPPING CENTER (shell)
    //         s                                                                      s
    //      PRESSURE                                      CREW QUARTERS        w  PASSAGES
    //                                                                                s
    //                                                                       STABLES (hyperc)

    "north\n".chars().for_each(|c| input.push_back(c as i64));
    "take tambourine\n"
        .chars()
        .for_each(|c| input.push_back(c as i64));
    "east\n".chars().for_each(|c| input.push_back(c as i64));
    "take astrolabe\n"
        .chars()
        .for_each(|c| input.push_back(c as i64));
    "east\n".chars().for_each(|c| input.push_back(c as i64));
    "north\n".chars().for_each(|c| input.push_back(c as i64));
    "take klein bottle\n"
        .chars()
        .for_each(|c| input.push_back(c as i64));
    "north\n".chars().for_each(|c| input.push_back(c as i64));
    "take easter egg\n"
        .chars()
        .for_each(|c| input.push_back(c as i64));
    "south\n".chars().for_each(|c| input.push_back(c as i64));
    "south\n".chars().for_each(|c| input.push_back(c as i64));
    "west\n".chars().for_each(|c| input.push_back(c as i64));
    "south\n".chars().for_each(|c| input.push_back(c as i64));
    "take shell\n"
        .chars()
        .for_each(|c| input.push_back(c as i64));
    "north\n".chars().for_each(|c| input.push_back(c as i64));
    "west\n".chars().for_each(|c| input.push_back(c as i64));
    "south\n".chars().for_each(|c| input.push_back(c as i64));
    "south\n".chars().for_each(|c| input.push_back(c as i64));
    "south\n".chars().for_each(|c| input.push_back(c as i64));
    "take hypercube\n"
        .chars()
        .for_each(|c| input.push_back(c as i64));
    "inv\n".chars().for_each(|c| input.push_back(c as i64));
    "north\n".chars().for_each(|c| input.push_back(c as i64));
    "north\n".chars().for_each(|c| input.push_back(c as i64));
    "west\n".chars().for_each(|c| input.push_back(c as i64));
    "take dark matter\n"
        .chars()
        .for_each(|c| input.push_back(c as i64));
    "west\n".chars().for_each(|c| input.push_back(c as i64));
    //"take giant electromagnet\n".chars().for_each(|c| input.push_back(c as i64));
    "north\n".chars().for_each(|c| input.push_back(c as i64));
    //"take molten lava\n".chars().for_each(|c| input.push_back(c as i64));
    "east\n".chars().for_each(|c| input.push_back(c as i64));
    //"take escape pod\n".chars().for_each(|c| input.push_back(c as i64));
    "east\n".chars().for_each(|c| input.push_back(c as i64));
    //"take infinite loop\n".chars().for_each(|c| input.push_back(c as i64));
    "west\n".chars().for_each(|c| input.push_back(c as i64));
    "west\n".chars().for_each(|c| input.push_back(c as i64));
    "west\n".chars().for_each(|c| input.push_back(c as i64));
    "take coin\n"
        .chars()
        .for_each(|c| input.push_back(c as i64));
    "south\n".chars().for_each(|c| input.push_back(c as i64));
    "inv\n".chars().for_each(|c| input.push_back(c as i64));
}

struct IntcodeProcessor {
    instruction_ptr: i64,
    memory: HashMap<i64, i64>,
    relative_base: i64,
    output: Vec<char>,
}

impl IntcodeProcessor {
    fn run(&mut self, input: &mut VecDeque<i64>) {
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
                    if input.is_empty() {
                        return;
                    }
                    let inp_val = input.pop_front().unwrap();
                    self.mem_write(a_mode, a, inp_val);
                    self.instruction_ptr += 2;
                }
                4 => {
                    let a = self.memory[&(self.instruction_ptr + 1)];
                    let a_val = self.mem_read(a_mode, a);
                    //print!("{}", a_val as u8 as char);
                    self.output.push(a_val as u8 as char);
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
        relative_base: 0,
        output: vec![],
    }
}
