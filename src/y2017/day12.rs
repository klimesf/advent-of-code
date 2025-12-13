use std::collections::{HashMap, HashSet};
use std::fs;

use itertools::Itertools;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

pub(crate) fn day12() {
    let input = fs::read_to_string("input/2017/day12/input.txt").unwrap();
    let adjacency_list = parse(input.as_str());
    println!("{}", discover("0", &adjacency_list).len());
    println!("{}", discover_all(&adjacency_list));
}

fn parse(input: &str) -> HashMap<&str, Vec<&str>> {
    input
        .lines()
        .map(|line| {
            let (from, to) = line.split_once(" <-> ").unwrap();
            let neighbors: Vec<&str> = to.split(", ").into_iter().collect();
            (from, neighbors)
        })
        .collect()
}

fn discover_all(adjacency_list: &HashMap<&str, Vec<&str>>) -> usize {
    let forest = adjacency_list
        .par_iter()
        .map(|(from, _)| discover(*from, adjacency_list))
        .collect::<Vec<Vec<&str>>>();
    forest.iter().unique().count()
}

fn discover<'a>(from: &'a str, adjacency_list: &HashMap<&'a str, Vec<&'a str>>) -> Vec<&'a str> {
    let mut stack = vec![from];
    let mut visited = HashSet::new();
    while let Some(next) = stack.pop() {
        if !visited.insert(next) {
            continue;
        }
        adjacency_list
            .get(next)
            .unwrap()
            .iter()
            .for_each(|neighbor| stack.push(neighbor));
    }
    visited.iter().map(|s| *s).sorted().collect()
}

#[cfg(test)]
mod day12_tests {
    use std::fs;

    use crate::y2017::day12::{discover, discover_all, parse};

    #[test]
    fn test_works() {
        let input = fs::read_to_string("input/2017/day12/test.txt").unwrap();
        let adjacency_list = parse(input.as_str());
        assert_eq!(6, discover("0", &adjacency_list).len());
        assert_eq!(2, discover_all(&adjacency_list));
    }

    #[test]
    fn input_works() {
        let input = fs::read_to_string("input/2017/day12/input.txt").unwrap();
        let adjacency_list = parse(input.as_str());
        assert_eq!(306, discover("0", &adjacency_list).len());
        assert_eq!(200, discover_all(&adjacency_list));
    }
}
