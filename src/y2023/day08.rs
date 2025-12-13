use crate::utils::toolbox::lcm_64;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::collections::HashMap;
use std::fs;

pub(crate) fn day08() {
    println!(
        "{}",
        part_a(fs::read_to_string("input/2023/day08/input.txt").unwrap())
    );
    println!(
        "{}",
        part_b(fs::read_to_string("input/2023/day08/input.txt").unwrap())
    );
}

fn part_a(input: String) -> usize {
    let (instructions, map_str) = input.split_once("\n\n").unwrap();

    let mut map = HashMap::new();
    map_str.lines().for_each(|line| {
        let (l, r) = line.split_once(" = ").unwrap();
        let (rl, rr) = r[1..r.len() - 1].split_once(", ").unwrap();
        map.insert(l, (rl, rr));
    });

    let mut pos = "AAA";
    let mut steps: usize = 0;
    loop {
        if pos == "ZZZ" {
            return steps;
        }

        let instruction = instructions
            .chars()
            .nth(steps % instructions.len())
            .unwrap();
        let (l, r) = map.get(pos).unwrap();
        if instruction == 'L' {
            pos = l;
        } else {
            pos = r;
        }
        steps += 1;
    }
}

fn part_b(input: String) -> i64 {
    let (instructions, map_str) = input.split_once("\n\n").unwrap();

    let mut map = HashMap::new();
    let mut start_points = vec![];
    map_str.lines().for_each(|line| {
        let (l, r) = line.split_once(" = ").unwrap();
        let (rl, rr) = r[1..r.len() - 1].split_once(", ").unwrap();
        map.insert(l, (rl, rr));
        if l.ends_with("A") {
            start_points.push(l);
        }
    });

    let mut all: Vec<i64> = start_points
        .par_iter()
        .map(|ghost| {
            let mut pos = ghost;
            let mut steps: i64 = 0;
            loop {
                if pos.ends_with("Z") {
                    return steps;
                }

                let instruction = instructions
                    .chars()
                    .nth(steps as usize % instructions.len())
                    .unwrap();
                let (l, r) = map.get(pos).unwrap();
                if instruction == 'L' {
                    pos = l;
                } else {
                    pos = r;
                }
                steps += 1;
            }
        })
        .collect();

    let mut res = all.pop().unwrap();
    for i in 0..all.len() {
        res = lcm_64(res, all[i]);
    }
    res
}

#[cfg(test)]
mod day08_tests {
    use std::fs;

    use crate::y2023::day08::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(
            2,
            part_a(fs::read_to_string("input/2023/day08/test.txt").unwrap())
        );
        assert_eq!(
            6,
            part_b(fs::read_to_string("input/2023/day08/test_b.txt").unwrap())
        );
    }

    #[test]
    fn input_works() {
        assert_eq!(
            21389,
            part_a(fs::read_to_string("input/2023/day08/input.txt").unwrap())
        );
        assert_eq!(
            21083806112641,
            part_b(fs::read_to_string("input/2023/day08/input.txt").unwrap())
        );
    }
}
