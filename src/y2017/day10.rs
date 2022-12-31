use std::{fs};

pub(crate) fn day10() {
    let input = fs::read_to_string("input/2017/day10/input.txt").unwrap();
    println!("{}", part_a(256, input.trim()));
    println!("{}", knot_hash(input.trim()));
}

fn part_a(list_len: usize, input: &str) -> usize {
    let lengths = input.trim().split(",").map(|i| i.parse::<usize>().unwrap()).collect();
    let mut list: Vec<usize> = (0..list_len).collect();
    knot_hash_iteration(0, 0, &mut list, &lengths);
    list[0] * list[1]
}

pub(crate) fn knot_hash(input: &str) -> String {
    let mut list: Vec<usize> = (0..256).collect();
    let mut lengths: Vec<usize> = input.chars().map(|c| c as usize).collect();
    [17, 31, 73, 47, 23].iter().for_each(|i| lengths.push(*i));

    let mut pos = 0;
    let mut skip = 0;
    for _ in 0..64 {
        let (new_pos, new_skip) = knot_hash_iteration(pos, skip, &mut list, &lengths);
        pos = new_pos;
        skip = new_skip;
    }

    let mut hash = String::new();
    for i in 0..16 {
        let mut byte = 0;
        for j in 0..16 {
            byte = byte ^ list[16 * i + j];
        }
        hash.push_str(format!("{:02x}", byte).as_str());
    }

    hash
}

fn knot_hash_iteration(mut pos: usize, mut skip: usize, list: &mut Vec<usize>, lengths: &Vec<usize>) -> (usize, usize) {
    let list_len = list.len();
    for len in lengths {
        for i in 0..(len / 2) {
            list.swap((pos + i) % list_len, (pos + len - i - 1) % list_len);
        }
        pos = (pos + len + skip) % list_len;
        skip += 1;
    }
    (pos, skip)
}

#[cfg(test)]
mod day10_tests {
    use std::fs;
    use crate::y2017::day10::{part_a, knot_hash};

    #[test]
    fn part_a_works() {
        assert_eq!(12, part_a(5, "3,4,1,5"));
    }

    #[test]
    fn part_b_works() {
        assert_eq!("a2582a3a0e66e6e86e3812dcb672a272", knot_hash(""));
        assert_eq!("33efeb34ea91902bb2f59c9920caa6cd", knot_hash("AoC 2017"));
        assert_eq!("3efbe78a8d82f29979031a4aa0b16a9d", knot_hash("1,2,3"));
        assert_eq!("63960835bcdc130f0b66d7ff4f6a5a8e", knot_hash("1,2,4"));
    }

    #[test]
    fn input_works() {
        let input = fs::read_to_string("input/2017/day10/input.txt").unwrap();
        assert_eq!(52070, part_a(256, input.trim()));
        assert_eq!("7f94112db4e32e19cf6502073c66f9bb", knot_hash(input.trim()));
    }
}
