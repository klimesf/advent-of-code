use std::fs;

pub(crate) fn day03() {
    println!("{}", part_a(fs::read_to_string("input/2016/day03/input.txt").unwrap()));
    println!("{}", part_b(fs::read_to_string("input/2016/day03/input.txt").unwrap()));
}

fn part_a(input: String) -> usize {
    input.lines().filter(|line| {
        let mut triangle: Vec<i32> = line.trim().split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect();

        triangle.sort();
        triangle[0] + triangle[1] > triangle[2]
    }).count()
}

fn part_b(input: String) -> i32 {
    let mut cols = vec! {vec!(), vec!(), vec!()};
    let mut ans = 0;

    for line in input.lines() {
        let nums: Vec<i32> = line.trim().split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect();

        for i in 0..3 {
            cols[i].push(nums[i]);
            if cols[i].len() == 3 {
                cols[i].sort();
                if cols[i][0] + cols[i][1] > cols[i][2] {
                    ans += 1
                }
                cols[i].clear()
            }
        }
    }

    ans
}

#[cfg(test)]
mod day03_tests {
    use std::fs;

    use crate::y2016::day03::{part_a, part_b};

    #[test]
    fn input_works() {
        assert_eq!(917, part_a(fs::read_to_string("input/2016/day03/input.txt").unwrap()));
        assert_eq!(1649, part_b(fs::read_to_string("input/2016/day03/input.txt").unwrap()));
    }
}
