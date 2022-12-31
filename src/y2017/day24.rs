use std::collections::{HashMap, HashSet};
use std::fs;

pub(crate) fn day24() {
    let input = fs::read_to_string("input/2017/day24/input.txt").unwrap();
    let components = parse(input.as_str());
    println!("{}", find_strongest(&components));
    println!("{}", find_longest(&components));
}

fn parse(input: &str) -> HashMap<i64, (usize, usize)> {
    input.lines()
        .map(|line| line.split_once("/").unwrap())
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .enumerate()
        .map(|(i, p)| (0b1 << i, p)) // We can do this because there is < 64 parts in input
        .collect()
}

fn find_strongest(components: &HashMap<i64, (usize, usize)>) -> usize {
    let mut available = components.keys().sum();
    let mut duplicates = HashSet::new();
    components.iter()
        .filter(|(_, (a, b))| a == b)
        .for_each(|(k, (a, _))| {
            available -= k;
            duplicates.insert(a);
        });

    let mut stack: Vec<(usize, usize, i64)> = vec!();
    stack.push((0, 0, available));
    let mut max = usize::MIN;

    while let Some(state) = stack.pop() {
        let (pins, mut strength, available) = state;
        if duplicates.contains(&pins) {
            strength += pins * 2;
        }
        if strength > max { max = strength }

        components.iter()
            .filter(|(k, _)| available & **k > 0)
            .filter(|(_, (a, b))| *a == pins || *b == pins)
            .for_each(|(k, (a, b))| {
                let new_pins = if *a == pins { *b } else { *a };
                stack.push((new_pins, strength + *a + *b, available - k));
            });
    }
    max
}

fn find_longest(components: &HashMap<i64, (usize, usize)>) -> usize {
    let mut available = components.keys().sum();
    let mut duplicates = HashSet::new();
    components.iter()
        .filter(|(_, (a, b))| a == b)
        .for_each(|(k, (a, _))| {
            available -= k;
            duplicates.insert(a);
        });

    let mut stack: Vec<(usize, usize, usize, i64)> = vec!();
    stack.push((0, 0, 0, available));
    let mut max = (usize::MIN, usize::MIN);

    while let Some(state) = stack.pop() {
        let (pins, mut length, mut strength, available) = state;
        if duplicates.contains(&pins) {
            strength += pins * 2;
            length += 1;
        }
        if length > max.0 || (length == max.0 && strength > max.1) { max = (length, strength) }

        components.iter()
            .filter(|(k, _)| available & **k > 0)
            .filter(|(_, (a, b))| *a == pins || *b == pins)
            .for_each(|(k, (a, b))| {
                let new_pins = if *a == pins { *b } else { *a };
                stack.push((new_pins, length + 1, strength + *a + *b, available - k));
            });
    }
    max.1
}

#[cfg(test)]
mod day24_tests {
    use std::fs;
    use crate::y2017::day24::{find_longest, find_strongest, parse};

    #[test]
    fn test_works() {
        let input = fs::read_to_string("input/2017/day24/test.txt").unwrap();
        let components = parse(input.as_str());
        assert_eq!(31, find_strongest(&components));
        assert_eq!(19, find_longest(&components));
    }

    #[test]
    fn input_works() {
        let input = fs::read_to_string("input/2017/day24/input.txt").unwrap();
        let components = parse(input.as_str());
        assert_eq!(1940, find_strongest(&components));
        assert_eq!(1928, find_longest(&components));
    }
}
