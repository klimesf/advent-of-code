use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs;
use regex::Regex;
use crate::utils::toolbox::parse_i32;

pub(crate) fn day23() {
    println!("{}", part_a(fs::read_to_string("input/2018/day23/input.txt").unwrap()));
    println!("{}", part_b(fs::read_to_string("input/2018/day23/input.txt").unwrap()));
}

fn part_a(input: String) -> usize {
    let nanobots: Vec<(i32, i32, i32, i32)> = input.lines().map(|line| {
        let re = Regex::new("^pos=<(-?[0-9]+),(-?[0-9]+),(-?[0-9]+)>, r=([0-9]+)$").unwrap();
        let caps = re.captures(line).unwrap();
        (parse_i32(caps.get(1)), parse_i32(caps.get(2)), parse_i32(caps.get(3)), parse_i32(caps.get(4)))
    }).collect();

    let max_nanobot = nanobots.iter()
        .max_by(|(_, _, _, r1), (_, _, _, r2)| r1.cmp(r2))
        .unwrap();

    let mut ans = 0;
    for nanobot in &nanobots {
        if manhattan_dist(max_nanobot, &nanobot) <= max_nanobot.3 { ans += 1 }
    }
    ans
}

fn part_b(input: String) -> i32 {
    let nanobots: Vec<(i32, i32, i32, i32)> = input.lines().map(|line| {
        let re = Regex::new("^pos=<(-?[0-9]+),(-?[0-9]+),(-?[0-9]+)>, r=([0-9]+)$").unwrap();
        let caps = re.captures(line).unwrap();
        (parse_i32(caps.get(1)), parse_i32(caps.get(2)), parse_i32(caps.get(3)), parse_i32(caps.get(4)))
    }).collect();

    let mut intervals = BinaryHeap::new();
    let start = &(0, 0, 0, 0);
    for nanobot in &nanobots {
        let d = manhattan_dist(start, nanobot);
        intervals.push(IntervalPt { dist: (d - nanobot.3).max(0), inc: 1 });
        intervals.push(IntervalPt { dist: d + nanobot.3 + 1, inc: -1 });
    }

    let mut count = 0;
    let mut max_count = 0;
    let mut ans = 0;
    while let Some(pt) = intervals.pop() {
        count += pt.inc;
        if count > max_count {
            ans = pt.dist;
            max_count = count;
        }
    }
    ans
}

fn manhattan_dist(a: &(i32, i32, i32, i32), b: &(i32, i32, i32, i32)) -> i32 {
    a.0.max(b.0) - a.0.min(b.0) +
        a.1.max(b.1) - a.1.min(b.1) +
        a.2.max(b.2) - a.2.min(b.2)
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct IntervalPt {
    dist: i32,
    inc: i32,
}

impl Ord for IntervalPt {
    fn cmp(&self, other: &Self) -> Ordering {
        other.dist.cmp(&self.dist)
    }
}

impl PartialOrd for IntervalPt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod day23_tests {
    use std::fs;

    use crate::y2018::day23::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(7, part_a(fs::read_to_string("input/2018/day23/test.txt").unwrap()));
        assert_eq!(36, part_b(fs::read_to_string("input/2018/day23/test.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!(430, part_a(fs::read_to_string("input/2018/day23/input.txt").unwrap()));
        assert_eq!(0, part_b(fs::read_to_string("input/2018/day23/input.txt").unwrap()));
    }
}
