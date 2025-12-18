use std::collections::{HashMap, HashSet};
use std::fs;

pub(crate) fn day15() {
    let input = fs::read_to_string("input/2019/day15/input.txt").unwrap();
    let code: Vec<i64> = input.trim().split(',').map(|c| c.parse().unwrap()).collect();
    let mut intcode = intcode_instance(&code);

    // Part A - walk the whole map in search for the oxygen point, use modified dijsktra to count the shortest
    // path to any given point
    let mut pos = (0, 0);
    let mut oxygen_system_pos = (0, 0);
    let mut dir = 1; // 1 - N, 2 - SOUTH, 3 - WEST, 4 - EAST
    let mut dist = 0;
    let mut map: HashMap<(i32, i32), i32> = HashMap::new();
    let mut exhausted: HashSet<(i32, i32)> = HashSet::new();
    map.insert((0, 0), 0);

    let mut input = |pos: &mut (i32, i32),
                     dir: &mut i32,
                     dist: &mut i32,
                     map: &mut HashMap<(i32, i32), i32>,
                     exhausted: &mut HashSet<(i32, i32)>,
                     output: &Vec<i64>|
     -> i64 {
        if exhausted.len() == map.len() {
            return 0;
        } // We can end

        // Remember how far is the point from the starting pos
        // While backtracking, prefer to go to the point with shorter distance
        // In the map, mark -1 as walls, otherwise mark the dist of the spot
        match output.last() {
            Some(r) => {
                match *r {
                    0 => {
                        // We hit the wall, mark it in the map, then look around and find new direction
                        map.insert(add_dir_to_pos(dir, pos), -1);
                        exhausted.insert(add_dir_to_pos(dir, pos));
                    }
                    1 => {
                        // Update position by the direction and mark it in the map
                        let new_pos = add_dir_to_pos(dir, pos);
                        // Overwrite the value by the lowest onw
                        *dist += 1;
                        let curr_dist = *map.entry(new_pos).or_insert(*dist);
                        *dist = if curr_dist < *dist { curr_dist } else { *dist };
                        map.insert(new_pos, *dist);

                        pos.0 = new_pos.0;
                        pos.1 = new_pos.1;
                    }
                    2 => {
                        // Update position by the direction and mark it in the map
                        let new_pos = add_dir_to_pos(dir, pos);
                        // Overwrite the value by the lowest onw
                        *dist += 1;
                        let curr_dist = *map.entry(new_pos).or_insert(*dist);
                        *dist = if curr_dist < *dist { curr_dist } else { *dist };
                        map.insert(new_pos, *dist);

                        pos.0 = new_pos.0;
                        pos.1 = new_pos.1;
                        oxygen_system_pos = new_pos.clone();
                    }
                    _ => panic!("Unknown output code: {}", r),
                }

                // Prefer the unvisited
                let unvisited = [1, 4, 2, 3]
                    .iter()
                    .find(|new_dir| !map.contains_key(&add_dir_to_pos(&new_dir, pos)));
                if unvisited.is_some() {
                    let new_dir = unvisited.unwrap();
                    *dir = *new_dir;
                    return *dir as i64;
                }

                // In case we hit the visited one, we need to backtrack to the place with least dist
                exhausted.insert(pos.clone()); // We can exhaust the curr pos, since we cannot find a new way from there
                let visited_possible = [1, 4, 2, 3]
                    .iter()
                    .filter(|new_dir| *map.get(&add_dir_to_pos(&new_dir, pos)).unwrap() > -1)
                    .min_by(|new_dir, other_new_dir| {
                        map.get(&add_dir_to_pos(&new_dir, pos))
                            .unwrap()
                            .cmp(map.get(&add_dir_to_pos(&other_new_dir, pos)).unwrap())
                    })
                    .unwrap();

                *dir = *visited_possible;
                return *dir as i64;
            }
            None => {
                return 1;
            } // We haven't sent any instruction yet
        }
    };
    intcode.run(&mut pos, &mut dir, &mut dist, &mut map, &mut exhausted, &mut input);

    println!("It takes {} commands to reach the oxygen systems", map.get(&oxygen_system_pos).unwrap());
    map.insert(oxygen_system_pos, -2); // Mark for printing
    print_map(&map);

    // Part B - use BFS to add all neighbors of the oxygen system, and so on to fill the whole map
    let total_locations: usize = map.values().filter(|v| **v != -1).count();
    let mut filled_locations: HashSet<(i32, i32)> = HashSet::new();
    let mut minutes = 0;
    let mut to_visit = vec![oxygen_system_pos];

    while total_locations > filled_locations.len() {
        let mut neighbors = vec![];
        for v in to_visit {
            let west = (v.0 - 1, v.1);
            let east = (v.0 + 1, v.1);
            let north = (v.0, v.1 - 1);
            let south = (v.0, v.1 + 1);
            if *map.get(&west).unwrap() != -1 && !filled_locations.contains(&west) {
                neighbors.push(west);
            }
            if *map.get(&east).unwrap() != -1 && !filled_locations.contains(&east) {
                neighbors.push((v.0 + 1, v.1));
            }
            if *map.get(&north).unwrap() != -1 && !filled_locations.contains(&north) {
                neighbors.push(north);
            }
            if *map.get(&south).unwrap() != -1 && !filled_locations.contains(&south) {
                neighbors.push(south);
            }
            filled_locations.insert(v);
        }

        to_visit = neighbors;
        minutes += 1;
    }
    println!("It takes {} minutes to fill the area with oxygen", minutes - 1);
}

fn add_dir_to_pos(dir: &i32, pos: &(i32, i32)) -> (i32, i32) {
    match dir {
        1 => (pos.0, pos.1 - 1), // NORTH
        2 => (pos.0, pos.1 + 1), // SOUTH
        3 => (pos.0 - 1, pos.1), // WEST
        4 => (pos.0 + 1, pos.1), // EAST
        _ => panic!("Unknown direction: {}", dir),
    }
}

fn print_map(map: &HashMap<(i32, i32), i32>) {
    let min_x = map.keys().map(|k| k.0).min().unwrap();
    let max_x = map.keys().map(|k| k.0).max().unwrap();
    let min_y = map.keys().map(|k| k.1).min().unwrap();
    let max_y = map.keys().map(|k| k.1).max().unwrap();

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            match map.get(&(x, y)) {
                Some(v) => match v {
                    -2 => print!("X"),
                    -1 => print!("#"),
                    0 => print!("D"),
                    _ => print!("."),
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
    output: Vec<i64>,
    relative_base: i64,
}

impl IntcodeProcessor {
    fn run(
        &mut self,
        pos: &mut (i32, i32),
        dir: &mut i32,
        dist: &mut i32,
        map: &mut HashMap<(i32, i32), i32>,
        exhausted: &mut HashSet<(i32, i32)>,
        input: &mut dyn FnMut(
            &mut (i32, i32),
            &mut i32,
            &mut i32,
            &mut HashMap<(i32, i32), i32>,
            &mut HashSet<(i32, i32)>,
            &Vec<i64>,
        ) -> i64,
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
                    self.mem_write(a_mode, a, input(pos, dir, dist, map, exhausted, &self.output));
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
