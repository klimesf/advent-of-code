use std::collections::HashSet;
use std::fs;

pub(crate) fn day21() {
    println!("{}", part_a(fs::read_to_string("input/2018/day21/input.txt").unwrap()));
    println!("{}", part_b(fs::read_to_string("input/2018/day21/input.txt").unwrap()));
}

fn part_a(input: String) -> usize {
    let input_split: Vec<&str> = input.splitn(2, "\n").collect();
    let ip_reg = input_split[0][4..].parse::<usize>().unwrap();

    let instructions: Vec<(String, Vec<usize>)> = input_split[1]
        .lines()
        .map(|line| {
            let instr = line[0..4].to_string();
            let args = line[5..].split(" ").map(|n| n.parse::<usize>().unwrap()).collect();
            (instr, args)
        })
        .collect();

    loop {
        let mut regs = vec![0; 6];
        let mut ip = 0;
        while ip < instructions.len() {
            if ip == 28 {
                return regs[5];
            }

            regs[ip_reg] = ip;
            let (instr, args) = &instructions[ip];
            regs = eval(instr.as_str(), args, &regs);
            ip = regs[ip_reg];
            ip += 1;
        }
    }
}

fn part_b(input: String) -> usize {
    let input_split: Vec<&str> = input.splitn(2, "\n").collect();
    let ip_reg = input_split[0][4..].parse::<usize>().unwrap();

    let instructions: Vec<(String, Vec<usize>)> = input_split[1]
        .lines()
        .map(|line| {
            let instr = line[0..4].to_string();
            let args = line[5..].split(" ").map(|n| n.parse::<usize>().unwrap()).collect();
            (instr, args)
        })
        .collect();

    let mut seen = HashSet::new();
    let mut last = 0;
    'outer: loop {
        let mut regs = vec![0; 6];
        let mut ip = 0;
        while ip < instructions.len() {
            if ip == 28 {
                if !seen.insert(regs[5]) {
                    break 'outer;
                }
                last = regs[5];
            }

            regs[ip_reg] = ip;
            let (instr, args) = &instructions[ip];
            regs = eval(instr.as_str(), args, &regs);
            ip = regs[ip_reg];
            ip += 1;
        }
    }
    last
}

fn eval(instr: &str, args: &Vec<usize>, regs: &Vec<usize>) -> Vec<usize> {
    let mut ans = regs.clone();
    match instr {
        "addr" => ans[args[2]] = regs[args[0]] + regs[args[1]],
        "addi" => ans[args[2]] = regs[args[0]] + args[1],
        "mulr" => ans[args[2]] = regs[args[0]] * regs[args[1]],
        "muli" => ans[args[2]] = regs[args[0]] * args[1],
        "banr" => ans[args[2]] = regs[args[0]] & regs[args[1]],
        "bani" => ans[args[2]] = regs[args[0]] & args[1],
        "borr" => ans[args[2]] = regs[args[0]] | regs[args[1]],
        "bori" => ans[args[2]] = regs[args[0]] | args[1],
        "setr" => ans[args[2]] = regs[args[0]],
        "seti" => ans[args[2]] = args[0],
        "gtir" => ans[args[2]] = if args[0] > regs[args[1]] { 1 } else { 0 },
        "gtri" => ans[args[2]] = if regs[args[0]] > args[1] { 1 } else { 0 },
        "gtrr" => ans[args[2]] = if regs[args[0]] > regs[args[1]] { 1 } else { 0 },
        "eqir" => ans[args[2]] = if args[0] == regs[args[1]] { 1 } else { 0 },
        "eqri" => ans[args[2]] = if regs[args[0]] == args[1] { 1 } else { 0 },
        "eqrr" => ans[args[2]] = if regs[args[0]] == regs[args[1]] { 1 } else { 0 },
        s => {
            panic!("unknown instruction {}", s)
        }
    }
    ans
}

#[cfg(test)]
mod day21_tests {
    use std::fs;

    use crate::y2018::day21::{part_a, part_b};

    #[test]
    fn input_works() {
        assert_eq!(6619857, part_a(fs::read_to_string("input/2018/day21/input.txt").unwrap()));
        assert_eq!(0, part_b(fs::read_to_string("input/2018/day21/input.txt").unwrap()));
    }
}
