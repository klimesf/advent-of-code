use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use std::collections::VecDeque;
use std::fs;
use z3::{ast::Int, Optimize, SatResult};

pub fn day10(print: fn(usize)) {
    print(part_a(
        fs::read_to_string("input/2025/day10/input.txt").unwrap(),
    ));
    print(part_b(
        fs::read_to_string("input/2025/day10/input.txt").unwrap(),
    ));
}

fn part_a(input: String) -> usize {
    let lines: Vec<&str> = input.lines().collect();
    lines
        .par_iter()
        .map(|line| {
            let parts = line.split_whitespace().collect::<Vec<&str>>();

            let target_chars = parts[0][1..parts[0].len() - 1]
                .chars()
                .collect::<Vec<char>>();
            let mut target = 0;
            for i in 0..target_chars.len() {
                if target_chars[i] != '#' {
                    continue;
                }
                target |= 0b1 << i;
            }

            let buttons = parts[1..parts.len() - 1]
                .iter()
                .map(|part| {
                    part[1..part.len() - 1]
                        .split(",")
                        .map(|part| part.parse::<usize>().unwrap())
                        .map(|num| 0b1 << num)
                        .reduce(|a, b| a | b)
                        .unwrap_or(0)
                })
                .collect::<Vec<usize>>();

            solve_a(target, buttons)
        })
        .sum()
}

fn solve_a(target: usize, buttons: Vec<usize>) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back((0b0, 0));

    while let Some((state, presses)) = queue.pop_front() {
        if state == target {
            return presses;
        }
        for button in buttons.iter() {
            queue.push_back((state ^ button, presses + 1));
        }
    }
    0
}

fn part_b(input: String) -> usize {
    let lines: Vec<&str> = input.lines().collect();
    lines
        .par_iter()
        .map(|line| {
            let parts = line.split_whitespace().collect::<Vec<&str>>();

            let buttons = parts[1..parts.len() - 1]
                .iter()
                .map(|part| {
                    part[1..part.len() - 1]
                        .split(",")
                        .map(|part| part.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>()
                })
                .collect::<Vec<Vec<usize>>>();

            let joltage_str =
                parts[parts.len() - 1][1..parts[parts.len() - 1].len() - 1].to_string();
            let joltage = joltage_str
                .split(",")
                .map(|part| part.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            solve_b(joltage, buttons)
        })
        .sum()
}

// a * (p1, p2, p3, ...) + b * (p1, p2, p3, ...) + c * (p1, p2, p3, ...) + ... = (P1, P2, P3, ...)
// a * p1 + b * p1 + ... = P1
// a * p2 + b * p2 + ... = P2
//
// a >= 0, b >= 0, ...
//
// MIN(a + b + c + ... )
fn solve_b(target: Vec<usize>, buttons: Vec<Vec<usize>>) -> usize {
    let opt = Optimize::new();

    let mut button_clicks = vec![];
    for i in 0..buttons.len() {
        let variable = Int::fresh_const(format!("button{}", i).as_str());
        button_clicks.push(variable);
    }

    for i in 0..buttons.len() {
        opt.assert(&button_clicks[i].ge(0));
    }

    for i in 0..target.len() {
        // a * pi + b * pi + ... = Pi
        let left_side = buttons
            .iter()
            .enumerate()
            .map(|(j, button)| {
                if button.contains(&i) {
                    Int::from_u64(1) * button_clicks[j].clone()
                } else {
                    Int::from_u64(0) * button_clicks[j].clone()
                }
            })
            .reduce(|a, b| a + b)
            .unwrap();
        let right_side = Int::from_u64(target[i] as u64);
        let equation = left_side.eq(&right_side);
        opt.assert(&equation);
    }

    let objective = button_clicks
        .iter()
        .map(|click| click.clone())
        .reduce(|a, b| a + b)
        .unwrap();
    opt.minimize(&objective);

    match opt.check(&[]) {
        SatResult::Sat => {
            let mut total = 0;
            let model = opt.get_model().unwrap();
            for i in 0..button_clicks.len() {
                let click = &button_clicks[i];
                total += model.eval(click, true).unwrap().as_i64().unwrap();
            }
            total as usize
        }
        SatResult::Unknown => 0,
        SatResult::Unsat => 0,
    }
}

#[cfg(test)]
mod day10_tests {
    use std::fs;

    use crate::y2025::day10::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(
            7,
            part_a(fs::read_to_string("input/2025/day10/test.txt").unwrap())
        );
        assert_eq!(
            33,
            part_b(fs::read_to_string("input/2025/day10/test.txt").unwrap())
        );
    }

    #[test]
    fn input_works() {
        assert_eq!(
            498,
            part_a(fs::read_to_string("input/2025/day10/input.txt").unwrap())
        );
        assert_eq!(
            17133,
            part_b(fs::read_to_string("input/2025/day10/input.txt").unwrap())
        );
    }
}
