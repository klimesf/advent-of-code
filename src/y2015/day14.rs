use std::collections::HashMap;
use std::fs;
use itertools::Itertools;
use regex::Regex;
use crate::utils::toolbox::parse_usize;

pub(crate) fn day14() {
    println!("{}", part_a(fs::read_to_string("input/2015/day14/input.txt").unwrap(), 2503));
    println!("{}", part_b(fs::read_to_string("input/2015/day14/input.txt").unwrap(), 2503));
}

fn part_a(input: String, total_time: usize) -> usize {
    input.lines().map(|line| {
        let re = Regex::new("^(.+) can fly ([0-9]+) km/s for ([0-9]+) seconds, but then must rest for ([0-9]+) seconds\\.$").unwrap();
        let caps = re.captures(line).unwrap();

        let speed = parse_usize(caps.get(2));
        let move_time = parse_usize(caps.get(3));
        let rest_time = parse_usize(caps.get(4));

        let full_cycles = total_time / (move_time + rest_time);
        let last_cycle = (total_time - (full_cycles * (move_time + rest_time))).min(move_time) * speed;

        (full_cycles * speed * move_time) + last_cycle
    }).max().unwrap()
}

fn part_b(input: String, total_time: usize) -> usize {
    let mut reindeers = vec!();
    input.lines().for_each(|line| {
        let re = Regex::new("^(.+) can fly ([0-9]+) km/s for ([0-9]+) seconds, but then must rest for ([0-9]+) seconds\\.$").unwrap();
        let caps = re.captures(line).unwrap();

        let reindeer = caps.get(1).unwrap().as_str();
        let speed = parse_usize(caps.get(2));
        let move_time = parse_usize(caps.get(3));
        let rest_time = parse_usize(caps.get(4));

        reindeers.push((reindeer, speed, move_time, rest_time));
    });

    let mut score = HashMap::new();
    let mut distances = HashMap::new();
    for t in 0..total_time {
        for (reindeer, speed, move_time, rest_time) in &reindeers {
            let tr = t % (move_time + rest_time);
            if tr < *move_time {
                *distances.entry(reindeer).or_insert(0) += speed;
            }
        }

        let leads: Vec<(&str, usize)> = distances.iter()
            .sorted_by(|(_, a), (_, b)| b.cmp(a))
            .map(|(k, v)| (**k, *v))
            .collect();
        let max = leads[0].1;

        for i in 0..leads.len() {
            let (reindeer, dist) = leads[i];
            if dist < max { break };
            *score.entry(reindeer).or_insert(0) += 1;
        }
    }

    *score.values().into_iter().max().unwrap()
}

#[cfg(test)]
mod day14_tests {
    use std::fs;

    use crate::y2015::day14::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(1120, part_a(fs::read_to_string("input/2015/day14/test.txt").unwrap(), 1000));
        assert_eq!(689, part_b(fs::read_to_string("input/2015/day14/test.txt").unwrap(), 1000));
    }

    #[test]
    fn input_works() {
        assert_eq!(2655, part_a(fs::read_to_string("input/2015/day14/input.txt").unwrap(), 2503));
        assert_eq!(1059, part_b(fs::read_to_string("input/2015/day14/input.txt").unwrap(), 2503));
    }
}
