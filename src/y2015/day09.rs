use std::collections::{HashMap, HashSet};
use std::fs;
use itertools::Itertools;

pub(crate) fn day09() {
    println!("{}", part_a(fs::read_to_string("input/2015/day09/input.txt").unwrap()));
    println!("{}", part_b(fs::read_to_string("input/2015/day09/input.txt").unwrap()));
}

fn part_a(input: String) -> usize {
    let mut locations = HashSet::new();
    let mut distances = HashMap::new();

    input.lines().for_each(|line| {
        let (l, dist) = line.split_once(" = ").unwrap();
        let (from, to) = l.split_once(" to ").unwrap();
        locations.insert(from);
        locations.insert(to);
        distances.entry(from).or_insert(HashMap::new()).insert(to, dist.parse::<usize>().unwrap());
        distances.entry(to).or_insert(HashMap::new()).insert(from, dist.parse::<usize>().unwrap());
    });

    locations.iter().permutations(locations.len()).unique()
        .map(|perm| {
            perm.iter().tuple_windows().map(|(a, b)| {
                distances.get(*a).unwrap().get(*b).unwrap()
            }).sum()
        })
        .min().unwrap()
}

fn part_b(input: String) -> usize {
    let mut locations = HashSet::new();
    let mut distances = HashMap::new();

    input.lines().for_each(|line| {
        let (l, dist) = line.split_once(" = ").unwrap();
        let (from, to) = l.split_once(" to ").unwrap();
        locations.insert(from);
        locations.insert(to);
        distances.entry(from).or_insert(HashMap::new()).insert(to, dist.parse::<usize>().unwrap());
        distances.entry(to).or_insert(HashMap::new()).insert(from, dist.parse::<usize>().unwrap());
    });

    locations.iter().permutations(locations.len()).unique()
        .map(|perm| {
            perm.iter().tuple_windows().map(|(a, b)| {
                distances.get(*a).unwrap().get(*b).unwrap()
            }).sum()
        })
        .max().unwrap()
}

#[cfg(test)]
mod day09_tests {
    use std::fs;

    use crate::y2015::day09::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(605, part_a(fs::read_to_string("input/2015/day09/test.txt").unwrap()));
        assert_eq!(982, part_b(fs::read_to_string("input/2015/day09/test.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!(251, part_a(fs::read_to_string("input/2015/day09/input.txt").unwrap()));
        assert_eq!(898, part_b(fs::read_to_string("input/2015/day09/input.txt").unwrap()));
    }
}
