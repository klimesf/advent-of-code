use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

pub(crate) fn day23() {
    let input = fs::read_to_string("input/2019/day23/input.txt").unwrap();
    let code: Vec<i64> = input.trim().split(',').map(|c| c.parse().unwrap()).collect();

    part_a(&code);
    part_b(&code);
}

fn part_a(code: &Vec<i64>) {
    let mut computers = vec![];
    for _ in 0..50 {
        computers.push(intcode_instance(code));
    }

    let mut inputs = HashMap::new();
    for i in 0..50 {
        let mut input = VecDeque::new();
        input.push_back(i as i64); // Assign address
        inputs.insert(i, input);
    }
    inputs.insert(255, VecDeque::new());

    let mut outputs: HashMap<i64, Vec<i64>> = HashMap::new();
    for i in 0..50 {
        outputs.insert(i, vec![]);
    }

    let input_callback = |from: i64, inputs: &mut HashMap<i64, VecDeque<i64>>| {
        let input = inputs.get_mut(&from).unwrap();
        return match input.pop_front() {
            Some(v) => {
                //println!("{} received packet {}", from, v);
                v
            }
            None => -1,
        };
    };

    let output_callback =
        |from: i64, outputs: &mut HashMap<i64, Vec<i64>>, inputs: &mut HashMap<i64, VecDeque<i64>>, value: i64| {
            let output = outputs.get_mut(&from).unwrap();
            output.push(value);

            if output.len() == 3 {
                let y = output.pop().unwrap();
                let x = output.pop().unwrap();
                let to = output.pop().unwrap();

                //println!("Sending packet ({}, {}) to {}", x, y, to);

                let input = inputs.get_mut(&to).unwrap();
                input.push_back(x);
                input.push_back(y);
            }
        };

    while inputs.get(&255).unwrap().len() < 2 && !all_finished(&computers) {
        for i in 0..50 {
            computers[i].step(i as i64, &mut inputs, &mut outputs, &input_callback, &output_callback);
        }
    }
    println!("The first y packet sent to 255 is {:?}", inputs.get(&255).unwrap()[1]);
}

fn part_b(code: &Vec<i64>) {
    let mut computers = vec![];
    for _ in 0..50 {
        computers.push(intcode_instance(code));
    }

    let mut inputs = HashMap::new();
    for i in 0..50 {
        let mut input = VecDeque::new();
        input.push_back(i as i64); // Assign address
        inputs.insert(i, input);
    }
    inputs.insert(255, VecDeque::new());

    let mut outputs: HashMap<i64, Vec<i64>> = HashMap::new();
    for i in 0..50 {
        outputs.insert(i, vec![]);
    }

    let input_callback = |from: i64, inputs: &mut HashMap<i64, VecDeque<i64>>| {
        let input = inputs.get_mut(&from).unwrap();
        return match input.pop_front() {
            Some(v) => {
                //println!("{} received packet {}", from, v);
                v
            }
            None => -1,
        };
    };

    let output_callback =
        |from: i64, outputs: &mut HashMap<i64, Vec<i64>>, inputs: &mut HashMap<i64, VecDeque<i64>>, value: i64| {
            let output = outputs.get_mut(&from).unwrap();
            output.push(value);

            if output.len() == 3 {
                let y = output.pop().unwrap();
                let x = output.pop().unwrap();
                let to = output.pop().unwrap();

                //println!("Sending packet ({}, {}) to {}", x, y, to);

                let input = inputs.get_mut(&to).unwrap();
                input.push_back(x);
                input.push_back(y);
            }
        };

    let mut idle_ctr = 0;
    let mut sent_y_to_0 = HashSet::new();
    while !all_finished(&computers) {
        for i in 0..50 {
            computers[i].step(i as i64, &mut inputs, &mut outputs, &input_callback, &output_callback);
        }
        if all_idling(&inputs, &outputs) {
            idle_ctr += 1;

            if idle_ctr > 1500 {
                // Don't ask me why wait 1500 idle counts, it just works
                let input_255 = inputs.get_mut(&255).unwrap();
                if input_255.is_empty() {
                    break;
                }
                let last_y = input_255.pop_back().unwrap();
                let last_x = input_255.pop_back().unwrap();

                let input_0 = inputs.get_mut(&0).unwrap();
                input_0.push_back(last_x);
                input_0.push_back(last_y);

                //println!("Sending packet ({}, {}) to 0 from NAT", last_x, last_y);

                if !sent_y_to_0.insert(last_y) {
                    println!("The first y packet to be sent twice from NAT to 0 is {:?}", last_y);
                    return;
                }

                idle_ctr = 0;
            }
        }
    }
}

fn all_finished(computers: &Vec<IntcodeProcessor>) -> bool {
    computers.iter().all(|c| c.finished)
}

fn all_idling(inputs: &HashMap<i64, VecDeque<i64>>, outputs: &HashMap<i64, Vec<i64>>) -> bool {
    (0..50).all(|i| inputs.get(&i).unwrap().is_empty() && outputs.get(&i).unwrap().is_empty())
}

struct IntcodeProcessor {
    instruction_ptr: i64,
    memory: HashMap<i64, i64>,
    relative_base: i64,
    finished: bool,
}

impl IntcodeProcessor {
    fn step(
        &mut self,
        address: i64,
        inputs: &mut HashMap<i64, VecDeque<i64>>,
        outputs: &mut HashMap<i64, Vec<i64>>,
        input: &dyn Fn(i64, &mut HashMap<i64, VecDeque<i64>>) -> i64,
        output: &dyn Fn(i64, &mut HashMap<i64, Vec<i64>>, &mut HashMap<i64, VecDeque<i64>>, i64),
    ) {
        if self.finished {
            return;
        };

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
                let inp_val = input(address, inputs);
                self.mem_write(a_mode, a, inp_val);
                self.instruction_ptr += 2;
            }
            4 => {
                let a = self.memory[&(self.instruction_ptr + 1)];
                let a_val = self.mem_read(a_mode, a);
                output(address, outputs, inputs, a_val);
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
                self.finished = true;
                return;
            }
            _ => {
                panic!("Unknown opcode {} at pos {}", opcode, self.instruction_ptr);
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
        finished: false,
    }
}
