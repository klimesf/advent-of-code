use std::fs;

pub(crate) fn day16() {
    println!("{}", part_a(fs::read_to_string("input/2018/day16/input.txt").unwrap()));
    println!("{}", part_b(fs::read_to_string("input/2018/day16/input.txt").unwrap()));
}

fn part_a(input: String) -> usize {
    let (input_a, _) = input.split_once("\n\n\n").unwrap();
    let mut samples = vec![];
    input_a.split("\n\n").for_each(|case| {
        let lines: Vec<&str> = case.splitn(3, "\n").collect();

        let before: Vec<usize> = lines[0][9..19]
            .split(", ")
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        let op: Vec<usize> = lines[1].split(" ").map(|s| s.parse::<usize>().unwrap()).collect();
        let after: Vec<usize> = lines[2][9..19]
            .split(", ")
            .map(|s| s.parse::<usize>().unwrap())
            .collect();

        samples.push((before, op, after));
    });

    let mut ans = 0;
    for (before, op, after) in samples {
        let mut matching = 0;
        let args = op[1..].iter().map(|n| *n).collect();
        for instr in [
            "addr", "addi", "mulr", "muli", "banr", "bani", "borr", "bori", "setr", "seti", "gtir", "gtri", "gtrr",
            "eqir", "eqri", "eqrr",
        ] {
            let res = eval(instr, &args, &before);
            if res == after {
                matching += 1
            }
        }
        if matching >= 3 {
            ans += 1
        }
    }
    ans
}

fn part_b(input: String) -> usize {
    let (input_a, input_b) = input.split_once("\n\n\n\n").unwrap();
    let mut samples = vec![];
    input_a.split("\n\n").for_each(|case| {
        let lines: Vec<&str> = case.splitn(3, "\n").collect();

        let before: Vec<usize> = lines[0][9..19]
            .split(", ")
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        let op: Vec<usize> = lines[1].split(" ").map(|s| s.parse::<usize>().unwrap()).collect();
        let after: Vec<usize> = lines[2][9..19]
            .split(", ")
            .map(|s| s.parse::<usize>().unwrap())
            .collect();

        samples.push((before, op, after));
    });

    // Manual: Create map for opcode => instr
    // let mut instr_ctr = vec! { vec! { 0 ;16 }; 16 };
    // for (before, op, after) in samples {
    //     let args = op[1..].iter().map(|n| *n).collect();
    //     for (i, instr) in ["addr", "addi", "mulr", "muli", "banr", "bani", "borr", "bori", "setr", "seti", "gtir", "gtri", "gtrr", "eqir", "eqri", "eqrr"].iter().enumerate() {
    //         let res = eval(instr, &args, &before);
    //         if res == after {
    //             instr_ctr[op[0]][i] += 1;
    //
    //         }
    //     }
    // }
    // for i in 0..instr_ctr.len() {
    //     println!("{} {:?}", i, instr_ctr[i]);
    // }

    // 0  [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 48, 48]
    // 1  [0, 0, 46, 0, 46, 46, 46, 0, 46, 46, 0, 0, 0, 46, 0, 46]
    // 2  [47, 0, 0, 0, 0, 0, 0, 0, 0, 47, 0, 0, 0, 0, 0, 0]
    // 3  [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 0, 48, 0, 48]
    // 4  [44, 44, 44, 44, 44, 44, 44, 44, 44, 44, 44, 44, 44, 0, 0, 0]
    // 5  [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 58, 58, 58, 58, 58, 58]
    // 6  [0, 0, 57, 0, 57, 57, 0, 0, 0, 57, 57, 57, 0, 0, 57, 57]
    // 7  [0, 0, 0, 0, 52, 0, 0, 0, 0, 0, 52, 52, 52, 52, 52, 52]
    // 8  [0, 48, 48, 0, 48, 0, 48, 48, 48, 48, 0, 48, 0, 48, 0, 48]
    // 9  [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 57, 57]
    // 10 [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48]
    // 11 [0, 0, 0, 0, 43, 43, 0, 0, 0, 0, 43, 43, 43, 43, 43, 43]
    // 12 [0, 0, 0, 0, 0, 0, 0, 0, 43, 0, 43, 0, 0, 0, 0, 0]
    // 13 [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 56, 56, 56, 56, 0]
    // 14 [59, 59, 0, 0, 0, 0, 0, 0, 0, 59, 0, 0, 0, 0, 0, 0]
    // 15 [0, 0, 0, 0, 55, 55, 0, 0, 0, 55, 55, 55, 55, 55, 55, 55]

    let op_to_instr = vec![
        "eqir", // 0
        "borr", // 1
        "addr", // 2
        "gtri", // 3
        "muli", // 4
        "gtir", // 5
        "mulr", // 6
        "banr", // 7
        "bori", // 8
        "eqri", // 9
        "eqrr", // 10
        "bani", // 11
        "setr", // 12
        "gtrr", // 13
        "addi", // 14
        "seti", // 15
    ];

    // Execute program
    let mut regs = vec![0; 4];
    input_b.lines().for_each(|line| {
        let op: Vec<usize> = line.split(" ").map(|s| s.parse::<usize>().unwrap()).collect();
        let args = op[1..].iter().map(|n| *n).collect();
        let instr = op_to_instr[op[0]];
        regs = eval(instr, &args, &regs);
    });
    regs[0]
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
mod day16_tests {
    use std::fs;

    use crate::y2018::day16::{part_a, part_b};

    #[test]
    fn input_works() {
        assert_eq!(614, part_a(fs::read_to_string("input/2018/day16/input.txt").unwrap()));
        assert_eq!(656, part_b(fs::read_to_string("input/2018/day16/input.txt").unwrap()));
    }
}
