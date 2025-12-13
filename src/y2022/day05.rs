use std::collections::VecDeque;
use std::fs;

use regex::{Match, Regex};

pub(crate) fn day05() {
    part_a();
    part_b();
}

fn part_a() {
    //         [F] [Q]         [Q]
    // [B]     [Q] [V] [D]     [S]
    // [S] [P] [T] [R] [M]     [D]
    // [J] [V] [W] [M] [F]     [J]     [J]
    // [Z] [G] [S] [W] [N] [D] [R]     [T]
    // [V] [M] [B] [G] [S] [C] [T] [V] [S]
    // [D] [S] [L] [J] [L] [G] [G] [F] [R]
    // [G] [Z] [C] [H] [C] [R] [H] [P] [D]
    //  1   2   3   4   5   6   7   8   9

    let mut stack1 = VecDeque::from(['B', 'S', 'J', 'Z', 'V', 'D', 'G']);
    let mut stack2 = VecDeque::from(['P', 'V', 'G', 'M', 'S', 'Z']);
    let mut stack3 = VecDeque::from(['F', 'Q', 'T', 'W', 'S', 'B', 'L', 'C']);
    let mut stack4 = VecDeque::from(['Q', 'V', 'R', 'M', 'W', 'G', 'J', 'H']);
    let mut stack5 = VecDeque::from(['D', 'M', 'F', 'N', 'S', 'L', 'C']);
    let mut stack6 = VecDeque::from(['D', 'C', 'G', 'R']);
    let mut stack7 = VecDeque::from(['Q', 'S', 'D', 'J', 'R', 'T', 'G', 'H']);
    let mut stack8 = VecDeque::from(['V', 'F', 'P']);
    let mut stack9 = VecDeque::from(['J', 'T', 'S', 'R', 'D']);
    let mut stacks = vec![
        &mut stack1,
        &mut stack2,
        &mut stack3,
        &mut stack4,
        &mut stack5,
        &mut stack6,
        &mut stack7,
        &mut stack8,
        &mut stack9,
    ];

    let input = fs::read_to_string("input/2022/day05/input.txt").unwrap();
    let re = Regex::new(r"^move ([0-9]+) from ([0-9]+) to ([0-9]+)$").unwrap();
    for line in input.lines() {
        let g = re.captures(line).unwrap();
        let much = parse_usize(g.get(1));
        let from = parse_usize(g.get(2));
        let to = parse_usize(g.get(3));

        for _ in 0..much {
            let c = stacks[from - 1].pop_front().unwrap();
            stacks[to - 1].push_front(c);
        }
    }
    for stack in stacks {
        print!("{}", stack.pop_front().unwrap())
    }
    println!();
}

fn part_b() {
    //         [F] [Q]         [Q]
    // [B]     [Q] [V] [D]     [S]
    // [S] [P] [T] [R] [M]     [D]
    // [J] [V] [W] [M] [F]     [J]     [J]
    // [Z] [G] [S] [W] [N] [D] [R]     [T]
    // [V] [M] [B] [G] [S] [C] [T] [V] [S]
    // [D] [S] [L] [J] [L] [G] [G] [F] [R]
    // [G] [Z] [C] [H] [C] [R] [H] [P] [D]
    //  1   2   3   4   5   6   7   8   9
    let mut stack1 = VecDeque::from(['B', 'S', 'J', 'Z', 'V', 'D', 'G']);
    let mut stack2 = VecDeque::from(['P', 'V', 'G', 'M', 'S', 'Z']);
    let mut stack3 = VecDeque::from(['F', 'Q', 'T', 'W', 'S', 'B', 'L', 'C']);
    let mut stack4 = VecDeque::from(['Q', 'V', 'R', 'M', 'W', 'G', 'J', 'H']);
    let mut stack5 = VecDeque::from(['D', 'M', 'F', 'N', 'S', 'L', 'C']);
    let mut stack6 = VecDeque::from(['D', 'C', 'G', 'R']);
    let mut stack7 = VecDeque::from(['Q', 'S', 'D', 'J', 'R', 'T', 'G', 'H']);
    let mut stack8 = VecDeque::from(['V', 'F', 'P']);
    let mut stack9 = VecDeque::from(['J', 'T', 'S', 'R', 'D']);
    let mut stacks = vec![
        &mut stack1,
        &mut stack2,
        &mut stack3,
        &mut stack4,
        &mut stack5,
        &mut stack6,
        &mut stack7,
        &mut stack8,
        &mut stack9,
    ];

    let input = fs::read_to_string("input/2022/day05/input.txt").unwrap();
    let re = Regex::new(r"^move ([0-9]+) from ([0-9]+) to ([0-9]+)$").unwrap();
    for line in input.lines() {
        let g = re.captures(line).unwrap();
        let much = parse_usize(g.get(1));
        let from = parse_usize(g.get(2));
        let to = parse_usize(g.get(3));

        let mut interim = VecDeque::new();
        for _ in 0..much {
            interim.push_front(stacks[from - 1].pop_front().unwrap());
        }
        for _ in 0..much {
            stacks[to - 1].push_front(interim.pop_front().unwrap());
        }
    }
    for stack in stacks {
        print!("{}", stack.pop_front().unwrap())
    }
    println!();
}

fn parse_usize(g: Option<Match>) -> usize {
    return g.map_or(0, |m| m.as_str().parse().unwrap());
}
