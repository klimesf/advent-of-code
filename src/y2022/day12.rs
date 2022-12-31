use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs;

pub(crate) fn day12() {
    let input = fs::read_to_string("input/2022/day12/input.txt").unwrap();
    let map: Vec<Vec<char>> = input.lines().into_iter()
        .map(|line| line.chars().into_iter().collect())
        .collect();
    let mut start = (0, 0);
    for x in 0..map.len() {
        for y in 0..map[x].len() {
            if map[x][y] == 'E' {
                start = (x, y);
                break;
            }
        }
    }
    println!("{}", dijkstra(start, &map, 'S'));
    println!("{}", dijkstra(start, &map, 'a'));
}

fn dijkstra(start: (usize, usize), map: &Vec<Vec<char>>, stop_on: char) -> usize {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut dist: HashMap<(usize, usize), usize> = HashMap::new();
    let mut to_visit: BinaryHeap<Pos> = BinaryHeap::new();

    dist.insert(start, 0);
    to_visit.push(Pos { x: start.0, y: start.1, dist: 0 });

    while let Some(pos) = to_visit.pop() {
        let c = map[pos.x][pos.y];
        if c == stop_on {
            return pos.dist;
        }

        if !visited.insert((pos.x, pos.y)) {
            continue;
        }

        let mut neighbors = vec!();
        if pos.x > 0 && is_within_range(c, map[pos.x - 1][pos.y]) {
            neighbors.push((pos.x - 1, pos.y));
        }
        if pos.x < map.len() - 1 && is_within_range(c, map[pos.x + 1][pos.y]) {
            neighbors.push((pos.x + 1, pos.y));
        }
        if pos.y > 0 && is_within_range(c, map[pos.x][pos.y - 1]) {
            neighbors.push((pos.x, pos.y - 1));
        }
        if pos.y < map[pos.x].len() - 1 && is_within_range(c, map[pos.x][pos.y + 1]) {
            neighbors.push((pos.x, pos.y + 1));
        }

        for neighbor in neighbors {
            if *dist.get(&neighbor).unwrap_or(&usize::MAX) > pos.dist + 1 {
                dist.insert(neighbor, pos.dist + 1);
                to_visit.push(Pos { x: neighbor.0, y: neighbor.1, dist: pos.dist + 1 })
            }
        }
    }
    panic!("Did not find {}", stop_on);
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

fn is_within_range(from: char, to: char) -> bool {
    if to == 'S' {
        return true;
    }
    if from == 'E' {
        return to == 'z' || to == 'y';
    }

    let from_u = from as u32;
    let to_u = to as u32;

    return if from_u > to_u { from_u - to_u == 1 } else { true };
}

#[cfg(test)]
mod day12_tests {
    use crate::y2022::day12::is_within_range;

    #[test]
    fn is_within_range_works() {
        assert_eq!(true, is_within_range('a', 'S'));
        assert_eq!(true, is_within_range('a', 'a'));
        assert_eq!(true, is_within_range('b', 'a'));
        assert_eq!(true, is_within_range('a', 'b'));
        assert_eq!(false, is_within_range('c', 'a'));
        assert_eq!(true, is_within_range('a', 'c'));
        assert_eq!(false, is_within_range('E', 'x'));
        assert_eq!(true, is_within_range('E', 'y'));
        assert_eq!(true, is_within_range('E', 'z'));
    }
}
