use crate::utils::toolbox::parse_i32;
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;

pub(crate) fn day13() {
    println!(
        "{}",
        part_a(fs::read_to_string("input/2015/day13/input.txt").unwrap())
    );
    println!(
        "{}",
        part_b(fs::read_to_string("input/2015/day13/input.txt").unwrap())
    );
}

fn part_a(input: String) -> i32 {
    let mut happiness = HashMap::new();
    let mut people = HashSet::new();
    input.lines().for_each(|line| {
        let re = Regex::new(
            "^(.+) would (gain|lose) ([0-9]+) happiness units by sitting next to (.+)\\.$",
        );
        let caps = re.unwrap().captures(line).unwrap();
        let from = caps.get(1).unwrap().as_str();
        let minus = caps.get(2).unwrap().as_str() == "lose";
        let mut val = parse_i32(caps.get(3));
        let to = caps.get(4).unwrap().as_str();

        if minus {
            val *= -1;
        }
        people.insert(from);
        people.insert(to);
        happiness
            .entry(from)
            .or_insert(HashMap::new())
            .insert(to, val);
    });

    people
        .iter()
        .permutations(people.len())
        .unique()
        .map(|perm| {
            perm.iter()
                .circular_tuple_windows()
                .map(|(a, b)| happiness.get(*a).unwrap().get(*b).unwrap())
                .sum::<i32>()
                + perm
                    .iter()
                    .rev()
                    .circular_tuple_windows()
                    .map(|(a, b)| happiness.get(*a).unwrap().get(*b).unwrap())
                    .sum::<i32>()
        })
        .max()
        .unwrap()
}

fn part_b(input: String) -> i32 {
    let mut happiness = HashMap::new();
    let mut people = HashSet::new();
    input.lines().for_each(|line| {
        let re = Regex::new(
            "^(.+) would (gain|lose) ([0-9]+) happiness units by sitting next to (.+)\\.$",
        );
        let caps = re.unwrap().captures(line).unwrap();
        let from = caps.get(1).unwrap().as_str();
        let minus = caps.get(2).unwrap().as_str() == "lose";
        let mut val = parse_i32(caps.get(3));
        let to = caps.get(4).unwrap().as_str();

        if minus {
            val *= -1;
        }
        people.insert(from);
        people.insert(to);
        happiness
            .entry(from)
            .or_insert(HashMap::new())
            .insert(to, val);
    });

    let mut happiness_me = HashMap::new();
    for person in &people {
        happiness
            .entry(person)
            .or_insert(HashMap::new())
            .insert("me", 0);
        happiness_me.insert(*person, 0);
    }
    happiness.insert("me", happiness_me);
    people.insert("me");

    people
        .iter()
        .permutations(people.len())
        .unique()
        .map(|perm| {
            perm.iter()
                .circular_tuple_windows()
                .map(|(a, b)| happiness.get(*a).unwrap().get(*b).unwrap())
                .sum::<i32>()
                + perm
                    .iter()
                    .rev()
                    .circular_tuple_windows()
                    .map(|(a, b)| happiness.get(*a).unwrap().get(*b).unwrap())
                    .sum::<i32>()
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod day13_tests {
    use std::fs;

    use crate::y2015::day13::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(
            330,
            part_a(fs::read_to_string("input/2015/day13/test.txt").unwrap())
        );
    }

    #[test]
    fn input_works() {
        assert_eq!(
            664,
            part_a(fs::read_to_string("input/2015/day13/input.txt").unwrap())
        );
        assert_eq!(
            640,
            part_b(fs::read_to_string("input/2015/day13/input.txt").unwrap())
        );
    }
}
