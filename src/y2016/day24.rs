use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs;
use itertools::Itertools;

pub(crate) fn day24() {
    let ans = solve(fs::read_to_string("input/2016/day24/input.txt").unwrap());
    println!("{}", ans.0);
    println!("{}", ans.1);
}

fn solve(input: String) -> (usize, usize) {
    let matrix: Vec<Vec<char>> = input.lines().map(|line| {
        line.chars().collect()
    }).collect();

    let mut starting_positions = vec!();
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j].is_ascii_digit() {
                starting_positions.push((i, j, matrix[i][j]));
            }
        }
    }

    let mut distances: HashMap<char, HashMap<char, usize>> = HashMap::new();
    for (i, j, c) in &starting_positions {
        // Run dijkstra from each point into the others and find distances
        let mut stack = BinaryHeap::new();
        let mut visited = HashSet::new();
        stack.push(Pos { x: *i, y: *j, dist: 0 });
        let mut dests = HashMap::new();

        while let Some(pos) = stack.pop() {
            if matrix[pos.x][pos.y] == '#' { continue }
            if !visited.insert((pos.x, pos.y)) { continue }
            if matrix[pos.x][pos.y].is_ascii_digit() {
                dests.insert(matrix[pos.x][pos.y], pos.dist);
            }

            if pos.x > 0 { stack.push(Pos { x: pos.x - 1, y: pos.y, dist: pos.dist + 1 }) }
            if pos.x < matrix.len() - 1 { stack.push(Pos { x: pos.x + 1, y: pos.y, dist: pos.dist + 1 }) }
            if pos.y > 0 { stack.push(Pos { x: pos.x, y: pos.y - 1, dist: pos.dist + 1 }) }
            if pos.y < matrix[0].len() - 1 { stack.push(Pos { x: pos.x, y: pos.y + 1, dist: pos.dist + 1 }) }
        }

        distances.insert(*c, dests);
    }

    let others: Vec<char> = starting_positions.iter()
        .map(|(_, _, c)| *c)
        .filter(|c| *c != '0')
        .collect();

    let mut min_a = usize::MAX;
    let mut min_b = usize::MAX;
    for perm in others.iter().permutations(others.len()).unique() {
        let dist_a = perm.iter().tuple_windows().map(|(a, b)| {
            distances.get(a).unwrap().get(b).unwrap()
        }).sum::<usize>() + distances.get(&'0').unwrap().get(perm[0]).unwrap();
        let dist_b = perm.iter().tuple_windows().map(|(a, b)| {
            distances.get(a).unwrap().get(b).unwrap()
        }).sum::<usize>()
            + distances.get(&'0').unwrap().get(perm[0]).unwrap()
            + distances.get(perm[perm.len() - 1]).unwrap().get(&'0').unwrap();
        if dist_a < min_a { min_a = dist_a }
        if dist_b < min_b { min_b = dist_b }
    }

    (min_a, min_b)
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Pos {
    x: usize,
    y: usize,
    dist: usize,
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
mod day24_tests {
    use std::fs;

    use crate::y2016::day24::{solve};

    #[test]
    fn test_works() {
        assert_eq!((14, 20), solve(fs::read_to_string("input/2016/day24/test.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!((502, 724), solve(fs::read_to_string("input/2016/day24/input.txt").unwrap()));
    }
}
