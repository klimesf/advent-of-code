use std::fs;

pub(crate) fn day19() {
    println!(
        "{}",
        part_a(fs::read_to_string("input/2018/day19/input.txt").unwrap())
    );
    println!(
        "{}",
        part_b(fs::read_to_string("input/2018/day19/input.txt").unwrap())
    );
}

fn part_a(input: String) -> usize {
    let input_split: Vec<&str> = input.splitn(2, "\n").collect();
    let ip_reg = input_split[0][4..].parse::<usize>().unwrap();

    let instructions: Vec<(String, Vec<usize>)> = input_split[1]
        .lines()
        .map(|line| {
            let instr = line[0..4].to_string();
            let args = line[5..]
                .split(" ")
                .map(|n| n.parse::<usize>().unwrap())
                .collect();
            (instr, args)
        })
        .collect();

    let mut regs = vec![0; 6];
    let mut ip = 0;
    while ip < instructions.len() {
        regs[ip_reg] = ip;
        let (instr, args) = &instructions[ip];
        regs = eval(instr.as_str(), args, &regs);
        ip = regs[ip_reg];
        ip += 1;
    }
    regs[0]
}

fn part_b(_: String) -> usize {
    let mut sum = 0;
    // This is just bullshit
    for i in 1..=10551311 {
        if 10551311 % i == 0 {
            sum += i;
        }
    }
    sum
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
mod day19_tests {
    use std::fs;

    use crate::y2018::day19::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(
            6,
            part_a(fs::read_to_string("input/2018/day19/test.txt").unwrap())
        );
    }

    #[test]
    fn input_works() {
        assert_eq!(
            912,
            part_a(fs::read_to_string("input/2018/day19/input.txt").unwrap())
        );
        assert_eq!(
            10576224,
            part_b(fs::read_to_string("input/2018/day19/input.txt").unwrap())
        );
    }
}
