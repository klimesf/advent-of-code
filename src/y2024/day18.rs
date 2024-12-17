use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::fs;

pub(crate) fn day18() {
    println!("{}", part_a(fs::read_to_string("input/2024/day18/input.txt").unwrap(), 70, 1024));
    println!("{:?}", part_b(fs::read_to_string("input/2024/day18/input.txt").unwrap(), 70));
}

fn part_a(input: String, dim: usize, sim: usize) -> usize {
    let bytes: Vec<(usize, usize)> = input.lines().map(|line| {
        let (sx, sy) = line.split_once(',').unwrap();
        let x = sx.parse::<usize>().unwrap();
        let y = sy.parse::<usize>().unwrap();
        (x, y)
    }).collect();

    let mut fallen = HashSet::new();
    for i in 0..sim {
        fallen.insert(bytes[i]);
    }

    let mut stack = BinaryHeap::new();
    let mut visited = HashSet::new();
    stack.push(Pos { x: 0, y: 0, dist: 0 });

    while let Some(pos) = stack.pop() {
        if fallen.contains(&pos.coords()) { continue; }
        if !visited.insert(pos.coords()) { continue; }
        if pos.x == dim && pos.y == dim { return pos.dist; }

        if pos.x > 0 { stack.push(Pos { x: pos.x - 1, y: pos.y, dist: pos.dist + 1 }); }
        if pos.x < dim { stack.push(Pos { x: pos.x + 1, y: pos.y, dist: pos.dist + 1 }); }
        if pos.y > 0 { stack.push(Pos { x: pos.x, y: pos.y - 1, dist: pos.dist + 1 }); }
        if pos.y < dim { stack.push(Pos { x: pos.x, y: pos.y + 1, dist: pos.dist + 1 }); }
    }

    0
}

fn part_b(input: String, dim: usize) -> (usize, usize) {
    let bytes: Vec<(usize, usize)> = input.lines().map(|line| {
        let (sx, sy) = line.split_once(',').unwrap();
        let x = sx.parse::<usize>().unwrap();
        let y = sy.parse::<usize>().unwrap();
        (x, y)
    }).collect();

    let mut fallen = HashSet::new();
    'outer: for i in 0..bytes.len() {
        fallen.insert(bytes[i]);

        let mut stack = BinaryHeap::new();
        let mut visited = HashSet::new();
        stack.push(Pos { x: 0, y: 0, dist: 0 });

        while let Some(pos) = stack.pop() {
            if fallen.contains(&pos.coords()) { continue; }
            if !visited.insert(pos.coords()) { continue; }
            if pos.x == dim && pos.y == dim { continue 'outer; }

            if pos.x > 0 { stack.push(Pos { x: pos.x - 1, y: pos.y, dist: pos.dist + 1 }); }
            if pos.x < dim { stack.push(Pos { x: pos.x + 1, y: pos.y, dist: pos.dist + 1 }); }
            if pos.y > 0 { stack.push(Pos { x: pos.x, y: pos.y - 1, dist: pos.dist + 1 }); }
            if pos.y < dim { stack.push(Pos { x: pos.x, y: pos.y + 1, dist: pos.dist + 1 }); }
        }

        return (bytes[i].0, bytes[i].1)
    }
    panic!()
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Pos {
    x: usize,
    y: usize,
    dist: usize,
}

impl Pos {
    #[inline]
    pub fn coords(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}

impl Ord for Pos {
    fn cmp(&self, other: &Self) -> Ordering {
        other.dist.cmp(&self.dist)
    }
}

impl PartialOrd for Pos {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod day18_tests {
    use std::fs;

    use crate::y2024::day18::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(22, part_a(fs::read_to_string("input/2024/day18/test.txt").unwrap(), 6, 12));
        assert_eq!((6, 1), part_b(fs::read_to_string("input/2024/day18/test.txt").unwrap(), 6));
    }

    #[test]
    fn input_works() {
        assert_eq!(374, part_a(fs::read_to_string("input/2024/day18/input.txt").unwrap(), 70, 1024));
        assert_eq!((30, 12), part_b(fs::read_to_string("input/2024/day18/input.txt").unwrap(), 70));
    }
}
