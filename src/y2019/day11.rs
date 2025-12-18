use std::collections::HashMap;
use std::fs;

pub(crate) fn day11() {
    let input = fs::read_to_string("input/2019/day11/input.txt").unwrap();
    let code: Vec<i64> = input.trim().split(',').map(|c| c.parse().unwrap()).collect();
    let mut intcode = intcode_instance(&code);

    let mut painted_areas: HashMap<(i32, i32), i64> = HashMap::new();
    painted_areas.insert((0, 0), 1);
    let mut robot_orientation = 0;
    let mut robot_position = (0, 0);
    let mut painting = true;

    let get_area = |pa: &HashMap<(i32, i32), i64>, rp: &(i32, i32)| match pa.get(rp) {
        Some(color) => *color,
        None => 0,
    };

    let accept_instr = |pa: &mut HashMap<(i32, i32), i64>, rp: &mut (i32, i32), instr: i64| {
        if painting {
            pa.insert(*rp, instr);
            painting = false;
        } else {
            if instr == 1 {
                robot_orientation = (robot_orientation + 90) % 360;
            } else {
                robot_orientation = robot_orientation - 90;
                if robot_orientation < 0 {
                    robot_orientation += 360;
                }
            }

            match robot_orientation {
                0 => *rp = (rp.0 + 1, rp.1),
                90 => *rp = (rp.0, rp.1 + 1),
                180 => *rp = (rp.0 - 1, rp.1),
                270 => *rp = (rp.0, rp.1 - 1),
                _ => panic!("invalid robot orientation: {}", robot_orientation),
            }

            painting = true;
        }
    };

    intcode.run(&mut painted_areas, &mut robot_position, get_area, accept_instr);
    println!("{}", painted_areas.len());

    for x in (-5..1).rev() {
        for y in 0..40 {
            match painted_areas.get(&(x, y)) {
                Some(e) => match e {
                    1 => print!("#"),
                    _ => print!(" "),
                },
                None => print!(" "),
            }
        }
        println!();
    }
}

struct IntcodeProcessor {
    instruction_ptr: i64,
    memory: HashMap<i64, i64>,
    relative_base: i64,
}

impl IntcodeProcessor {
    fn run(
        &mut self,
        pa: &mut HashMap<(i32, i32), i64>,
        rp: &mut (i32, i32),
        mut input: impl FnMut(&HashMap<(i32, i32), i64>, &(i32, i32)) -> i64,
        mut output: impl FnMut(&mut HashMap<(i32, i32), i64>, &mut (i32, i32), i64),
    ) {
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
                    let val = input(pa, rp);
                    self.mem_write(a_mode, a, val);
                    self.instruction_ptr += 2;
                }
                4 => {
                    let a = self.memory[&(self.instruction_ptr + 1)];
                    let a_val = self.mem_read(a_mode, a);
                    output(pa, rp, a_val);
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
    }
}
