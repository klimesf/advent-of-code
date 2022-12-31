use std::{fs, mem};

use itertools::Itertools;

pub(crate) fn day02() {
    let input = parse(fs::read_to_string("input/2017/day02/input.txt").unwrap());
    println!("{}", part_a(&input));
    println!("{}", part_b(&input));
}

fn parse(input: String) -> Vec<Vec<i32>> {
    input.lines().into_iter()
        .map(|line| line.split_whitespace().map(|num| num.parse().unwrap()).collect::<Vec<i32>>())
        .collect()
}

fn part_a(input: &Vec<Vec<i32>>) -> i32 {
    input.iter().map(|line| {
        let max = line.iter().max().unwrap();
        let min = line.iter().min().unwrap();
        max - min
    }).sum()
}

fn part_b(input: &Vec<Vec<i32>>) -> i32 {
    input.iter().map(|line| {
        line.iter().combinations(2).map(|aha| {
            let mut a = *aha[0];
            let mut b = *aha[1];
            if a < b { mem::swap(&mut a, &mut b) }
            if a % b == 0 { a / b } else { 0 }
        }).sum::<i32>()
    }).sum()
}


#[cfg(test)]
mod day02_tests {
    use std::fs;

    use crate::y2017::day02::{parse, part_a, part_b};

    #[test]
    fn part_a_works() {
        let input = parse(fs::read_to_string("input/2017/day02/test.txt").unwrap());
        assert_eq!(18, part_a(&input));
    }

    #[test]
    fn part_b_works() {
        let input = parse(fs::read_to_string("input/2017/day02/test_2.txt").unwrap());
        assert_eq!(9, part_b(&input));
    }

    #[test]
    fn input_works() {
        let input = parse(fs::read_to_string("input/2017/day02/input.txt").unwrap());
        assert_eq!(32020, part_a(&input));
        assert_eq!(236, part_b(&input));
    }
}
