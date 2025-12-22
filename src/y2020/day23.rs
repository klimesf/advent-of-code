use itertools::Itertools;
use std::collections::VecDeque;
use std::fs;

pub(crate) fn day23() {
    println!("{}", part_a(fs::read_to_string("input/2020/day23/input.txt").unwrap()));
    println!("{}", part_b(fs::read_to_string("input/2020/day23/input.txt").unwrap()));
}

fn part_a(input: String) -> usize {
    let mut cups = input
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<VecDeque<u32>>();

    let len = cups.len() as u32;

    for _ in 0..100 {
        let current = cups.pop_front().unwrap();
        let r1 = cups.pop_front().unwrap();
        let r2 = cups.pop_front().unwrap();
        let r3 = cups.pop_front().unwrap();
        let rs = [r1, r2, r3];

        let mut label = current;
        loop {
            label = label - 1;
            if label == 0 {
                label = len;
            }
            if !rs.contains(&label) {
                break;
            }
        }

        cups.push_front(current);
        let mut rotations = 0;
        while cups[0] != label {
            let a = cups.pop_back().unwrap();
            cups.push_front(a);
            rotations += 1;
        }
        cups.pop_front().unwrap();
        cups.push_front(r3);
        cups.push_front(r2);
        cups.push_front(r1);
        cups.push_front(label);
        for _ in 0..=(rotations + 3) {
            let a = cups.pop_front().unwrap();
            cups.push_back(a);
        }
    }

    while cups[0] != 1 {
        let a = cups.pop_front().unwrap();
        cups.push_back(a);
    }

    cups.iter()
        .skip(1)
        .map(|c| c.to_string())
        .join("")
        .parse::<usize>()
        .unwrap()
}

fn part_b(input: String) -> u32 {
    let mut cups = input
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<VecDeque<u32>>();
    cups.reserve(100_000_000);

    for i in cups.len() + 1..=1_000_000 {
        cups.push_back(i as u32);
    }
    let len = cups.len() as u32;

    for i in 0..10_000_000 {
        if i % 10000 == 0 {
            println!("{}", i);
            let pos = cups.iter().position(|&c| c == 1).unwrap();
            println!("{} {} {}", pos, cups[pos + 1 % cups.len()], cups[pos + 2 % cups.len()]);
        }
        let current = cups.pop_front().unwrap();
        let r1 = cups.pop_front().unwrap();
        let r2 = cups.pop_front().unwrap();
        let r3 = cups.pop_front().unwrap();
        let rs = [r1, r2, r3];

        let mut label = current;
        loop {
            label = label - 1;
            if label == 0 {
                label = len;
            }
            if !rs.contains(&label) {
                break;
            }
        }

        cups.push_front(current);
        let mut rotations = 0;
        while cups[0] != label {
            let a = cups.pop_back().unwrap();
            cups.push_front(a);
            rotations += 1;
        }
        // println!("over! lab:{} i:{} rots:{}", label, i, rotations);
        if i > 100 && rotations != i + 2 {
            panic!("lab:{} i:{} rots:{}", label, i, rotations);
        }
        cups.pop_front().unwrap();
        cups.push_front(r3);
        cups.push_front(r2);
        cups.push_front(r1);
        cups.push_front(label);
        for _ in 0..=(rotations + 3) {
            let a = cups.pop_front().unwrap();
            cups.push_back(a);
        }
    }

    while cups[0] != 1 {
        let a = cups.pop_front().unwrap();
        cups.push_back(a);
    }

    cups[1] * cups[2]
}

#[cfg(test)]
mod day23_tests {
    use std::fs;

    use crate::y2020::day23::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(67384529, part_a(fs::read_to_string("input/2020/day23/test.txt").unwrap()));
        // assert_eq!(0, part_b(fs::read_to_string("input/2020/day23/test.txt").unwrap()));
        // 3683490560
        // 111057672960
    }

    #[test]
    fn input_works() {
        assert_eq!(58427369, part_a(fs::read_to_string("input/2020/day23/input.txt").unwrap()));
        assert_eq!(0, part_b(fs::read_to_string("input/2020/day23/input.txt").unwrap()));
    }
}
