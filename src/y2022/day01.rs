use std::collections::BinaryHeap;
use std::fs;

pub(crate) fn day01() {
    let input = fs::read_to_string("input/2022/day01/input.txt").unwrap();
    let mut elves = parse(input.as_str());
    println!("{}", part_a(&elves));
    println!("{}", part_b(&mut elves));
}

fn parse(input: &str) -> BinaryHeap<i32> {
    let elves: Vec<i32> = input.split("\n\n")
        .map(|elfo| elfo.lines().map(|line| line.parse::<i32>().unwrap()).sum()).collect();
    BinaryHeap::from(elves)
}

fn part_a(elves: &BinaryHeap<i32>) -> i32 {
    *elves.peek().unwrap()
}

fn part_b(elves: &mut BinaryHeap<i32>) -> i32 {
    let max1 = elves.pop().unwrap();
    let max2 = elves.pop().unwrap();
    let max3 = elves.pop().unwrap();
    max1 + max2 + max3
}

#[cfg(test)]
mod day01_tests {
    use std::fs;
    use crate::y2022::day01::{parse, part_a, part_b};

    #[test]
    fn test_works() {
        let input = fs::read_to_string("input/2022/day01/test.txt").unwrap();
        let mut elves = parse(input.as_str());
        assert_eq!(24000, part_a(&elves));
        assert_eq!(45000, part_b(&mut elves));
    }

    #[test]
    fn input_works() {
        let input = fs::read_to_string("input/2022/day01/input.txt").unwrap();
        let mut elves = parse(input.as_str());
        assert_eq!(68292, part_a(&elves));
        assert_eq!(203203, part_b(&mut elves));
    }
}
