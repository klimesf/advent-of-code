use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs;

pub(crate) fn day16() {
    println!("{}", part_a(fs::read_to_string("input/2024/day16/input.txt").unwrap()));
    println!("{}", part_b(fs::read_to_string("input/2024/day16/input.txt").unwrap()));
}

fn part_a(input: String) -> usize {
    let map: Vec<Vec<char>> = input.lines().map(|line| {
        line.chars().collect()
    }).collect();

    let mut starting_position = (0, 0);
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 'S' {
                starting_position = (i, j);
            }
        }
    }

    let mut stack = BinaryHeap::new();
    let mut visited = HashSet::new();
    stack.push(Pos { x: starting_position.0, y: starting_position.1, dir: 1, dist: 0 });

    while let Some(pos) = stack.pop() {
        if map[pos.x][pos.y] == 'E' {
            return pos.dist;
        }
        if map[pos.x][pos.y] == '#' { continue }
        if !visited.insert((pos.x, pos.y, pos.dir)) { continue }

        match pos.dir {
            0 => {
                if pos.x > 0 { stack.push(Pos { x: pos.x - 1, y: pos.y, dir: pos.dir, dist: pos.dist + 1 }) }
                stack.push(Pos { x: pos.x, y: pos.y, dir: 1, dist: pos.dist + 1000 });
                stack.push(Pos { x: pos.x, y: pos.y, dir: 3, dist: pos.dist + 1000 });
            }
            1 => {
                if pos.y < map[0].len() - 1 { stack.push(Pos { x: pos.x, y: pos.y + 1, dir: pos.dir, dist: pos.dist + 1 }) }
                stack.push(Pos { x: pos.x, y: pos.y, dir: 0, dist: pos.dist + 1000 });
                stack.push(Pos { x: pos.x, y: pos.y, dir: 2, dist: pos.dist + 1000 });
            }
            2 => {
                if pos.x < map.len() - 1 { stack.push(Pos { x: pos.x + 1, y: pos.y, dir: pos.dir, dist: pos.dist + 1 }) }
                stack.push(Pos { x: pos.x, y: pos.y, dir: 1, dist: pos.dist + 1000 });
                stack.push(Pos { x: pos.x, y: pos.y, dir: 3, dist: pos.dist + 1000 });
            }
            3 => {
                if pos.y > 0 { stack.push(Pos { x: pos.x, y: pos.y - 1, dir: pos.dir, dist: pos.dist + 1 }) }
                stack.push(Pos { x: pos.x, y: pos.y, dir: 2, dist: pos.dist + 1000 });
                stack.push(Pos { x: pos.x, y: pos.y, dir: 0, dist: pos.dist + 1000 });
            }
            _ => panic!()
        }
    }
    0
}

fn part_b(input: String) -> usize {
    let map: Vec<Vec<char>> = input.lines().map(|line| {
        line.chars().collect()
    }).collect();

    let mut starting_position = (0, 0);
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 'S' {
                starting_position = (i, j);
            }
        }
    }

    let mut stack = BinaryHeap::new();
    let mut distances = HashMap::new();
    let mut final_dist = usize::MAX;
    let mut visited = HashSet::new();
    stack.push(Pos2 { x: starting_position.0, y: starting_position.1, dir: 1, dist: 0, visited: vec!() });

    while let Some(pos) = stack.pop() {
        if map[pos.x][pos.y] == '#' { continue }
        if pos.dist > final_dist { continue }
        let dist = distances.entry((pos.x, pos.y, pos.dir)).or_insert(usize::MAX);
        if pos.dist > *dist { continue }
        *dist = pos.dist;
        if map[pos.x][pos.y] == 'E' {
            if *dist <= final_dist {
                final_dist = *dist;
                visited.extend(pos.visited);
            }
            continue
        }

        match pos.dir {
            0 => {
                if pos.x > 0 {
                    let mut new_visited = pos.visited.clone();
                    new_visited.push((pos.x, pos.y));
                    stack.push(Pos2 { visited: new_visited, x: pos.x - 1, y: pos.y, dir: pos.dir, dist: pos.dist + 1 });
                }
                stack.push(Pos2 { visited: pos.visited.clone(), x: pos.x, y: pos.y, dir: 1, dist: pos.dist + 1000 });
                stack.push(Pos2 { visited: pos.visited.clone(), x: pos.x, y: pos.y, dir: 3, dist: pos.dist + 1000 });
            }
            1 => {
                if pos.y < map[0].len() - 1 {
                    let mut new_visited = pos.visited.clone();
                    new_visited.push((pos.x, pos.y));
                    stack.push(Pos2 { visited: new_visited, x: pos.x, y: pos.y + 1, dir: pos.dir, dist: pos.dist + 1 });
                }
                stack.push(Pos2 { visited: pos.visited.clone(), x: pos.x, y: pos.y, dir: 0, dist: pos.dist + 1000 });
                stack.push(Pos2 { visited: pos.visited.clone(), x: pos.x, y: pos.y, dir: 2, dist: pos.dist + 1000 });
            }
            2 => {
                if pos.x < map.len() - 1 {
                    let mut new_visited = pos.visited.clone();
                    new_visited.push((pos.x, pos.y));
                    stack.push(Pos2 { visited: new_visited, x: pos.x + 1, y: pos.y, dir: pos.dir, dist: pos.dist + 1 });
                }
                stack.push(Pos2 { visited: pos.visited.clone(), x: pos.x, y: pos.y, dir: 1, dist: pos.dist + 1000 });
                stack.push(Pos2 { visited: pos.visited.clone(), x: pos.x, y: pos.y, dir: 3, dist: pos.dist + 1000 });
            }
            3 => {
                if pos.y > 0 {
                    let mut new_visited = pos.visited.clone();
                    new_visited.push((pos.x, pos.y));
                    stack.push(Pos2 { visited: new_visited, x: pos.x, y: pos.y - 1, dir: pos.dir, dist: pos.dist + 1 });
                }
                stack.push(Pos2 { visited: pos.visited.clone(), x: pos.x, y: pos.y, dir: 2, dist: pos.dist + 1000 });
                stack.push(Pos2 { visited: pos.visited.clone(), x: pos.x, y: pos.y, dir: 0, dist: pos.dist + 1000 });
            }
            _ => panic!()
        }
    }

    visited.len() + 1
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Pos {
    x: usize,
    y: usize,
    dir: usize,
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

#[derive(Clone, Eq, PartialEq, Debug)]
struct Pos2 {
    x: usize,
    y: usize,
    dir: usize,
    dist: usize,
    visited: Vec<(usize, usize)>,
}

impl Ord for Pos2 {
    fn cmp(&self, other: &Self) -> Ordering {
        other.dist.cmp(&self.dist)
    }
}

impl PartialOrd for Pos2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod day16_tests {
    use std::fs;

    use crate::y2024::day16::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(7036, part_a(fs::read_to_string("input/2024/day16/test.txt").unwrap()));
        assert_eq!(45, part_b(fs::read_to_string("input/2024/day16/test.txt").unwrap()));
        assert_eq!(64, part_b(fs::read_to_string("input/2024/day16/test_2.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!(83444, part_a(fs::read_to_string("input/2024/day16/input.txt").unwrap()));
        assert_eq!(483, part_b(fs::read_to_string("input/2024/day16/input.txt").unwrap()));
    }
}
