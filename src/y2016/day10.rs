use crate::utils::toolbox::parse_usize;
use regex::Regex;
use std::collections::HashMap;
use std::fs;

pub(crate) fn day10() {
    println!(
        "{}",
        part_a(
            fs::read_to_string("input/2016/day10/input.txt").unwrap(),
            17,
            61
        )
    );
    println!(
        "{}",
        part_b(fs::read_to_string("input/2016/day10/input.txt").unwrap())
    );
}

fn part_a(input: String, ilow: usize, ihigh: usize) -> usize {
    let mut vals: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut configuration: HashMap<usize, (usize, &str, usize, &str)> = HashMap::new();
    let re_val = Regex::new(r"^value (\d+) goes to bot (\d+)$").unwrap();
    let low_high_val =
        Regex::new(r"bot (\d+) gives low to (bot|output) (\d+) and high to (bot|output) (\d+)$")
            .unwrap();

    let mut goes_to = vec![];
    input.lines().for_each(|line| {
        if let Some(caps) = re_val.captures(line) {
            let val = parse_usize(caps.get(1));
            let bot = parse_usize(caps.get(2));
            goes_to.push((bot, val));
        }
        if let Some(caps) = low_high_val.captures(line) {
            let bot = parse_usize(caps.get(1));
            let low_type = caps.get(2).unwrap().as_str();
            let low = parse_usize(caps.get(3));
            let high_type = caps.get(4).unwrap().as_str();
            let high = parse_usize(caps.get(5));
            configuration.insert(bot, (low, low_type, high, high_type));
        }
    });

    for (bot, val) in goes_to {
        let bot_vals = vals.entry(bot).or_insert(vec![]);
        bot_vals.push(val);
        if bot_vals.len() == 2 {
            if let Some(ans) = process(
                bot,
                &configuration,
                bot_vals.clone(),
                &mut vals,
                ilow,
                ihigh,
            ) {
                return ans;
            }
        }
    }

    panic!("no bot compared {} and {}", ilow, ihigh)
}

fn part_b(input: String) -> usize {
    let mut vals: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut output: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut configuration: HashMap<usize, (usize, &str, usize, &str)> = HashMap::new();
    let re_val = Regex::new(r"^value (\d+) goes to bot (\d+)$").unwrap();
    let low_high_val =
        Regex::new(r"bot (\d+) gives low to (bot|output) (\d+) and high to (bot|output) (\d+)$")
            .unwrap();

    let mut goes_to = vec![];
    input.lines().for_each(|line| {
        if let Some(caps) = re_val.captures(line) {
            let val = parse_usize(caps.get(1));
            let bot = parse_usize(caps.get(2));
            goes_to.push((bot, val));
        }
        if let Some(caps) = low_high_val.captures(line) {
            let bot = parse_usize(caps.get(1));
            let low_type = caps.get(2).unwrap().as_str();
            let low = parse_usize(caps.get(3));
            let high_type = caps.get(4).unwrap().as_str();
            let high = parse_usize(caps.get(5));
            configuration.insert(bot, (low, low_type, high, high_type));
        }
    });

    for (bot, val) in goes_to {
        let bot_vals = vals.entry(bot).or_insert(vec![]);
        bot_vals.push(val);
        if bot_vals.len() == 2 {
            process_b(
                bot,
                &configuration,
                bot_vals.clone(),
                &mut vals,
                &mut output,
            );
        }
    }

    output.get(&0).unwrap()[0] * output.get(&1).unwrap()[0] * output.get(&2).unwrap()[0]
}

fn process(
    bot: usize,
    configuration: &HashMap<usize, (usize, &str, usize, &str)>,
    bvals: Vec<usize>,
    mut vals: &mut HashMap<usize, Vec<usize>>,
    ilow: usize,
    ihigh: usize,
) -> Option<usize> {
    let (low, low_type, high, high_type) = configuration.get(&bot).unwrap();

    if (bvals[0] == ilow && bvals[1] == ihigh) || (bvals[0] == ihigh && bvals[1] == ilow) {
        return Some(bot);
    }

    if low_type == &"bot" {
        let bot_vals = vals.entry(*low).or_insert(vec![]);
        let val = bvals[0].min(bvals[1]);
        bot_vals.push(val);
        if bot_vals.len() == 2 {
            if let Some(ans) = process(
                *low,
                &configuration,
                bot_vals.clone(),
                &mut vals,
                ilow,
                ihigh,
            ) {
                return Some(ans);
            }
        }
    }

    if high_type == &"bot" {
        let bot_vals = vals.entry(*high).or_insert(vec![]);
        let val = bvals[0].max(bvals[1]);
        bot_vals.push(val);
        if bot_vals.len() == 2 {
            if let Some(ans) = process(
                *high,
                &configuration,
                bot_vals.clone(),
                &mut vals,
                ilow,
                ihigh,
            ) {
                return Some(ans);
            }
        }
    }

    None
}
fn process_b(
    bot: usize,
    configuration: &HashMap<usize, (usize, &str, usize, &str)>,
    bvals: Vec<usize>,
    mut vals: &mut HashMap<usize, Vec<usize>>,
    output: &mut HashMap<usize, Vec<usize>>,
) {
    let (low, low_type, high, high_type) = configuration.get(&bot).unwrap();

    if low_type == &"bot" {
        let bot_vals = vals.entry(*low).or_insert(vec![]);
        let val = bvals[0].min(bvals[1]);
        bot_vals.push(val);
        if bot_vals.len() == 2 {
            process_b(*low, &configuration, bot_vals.clone(), &mut vals, output)
        }
    } else {
        let val = bvals[0].min(bvals[1]);
        output.entry(*low).or_insert(vec![]).push(val);
    }

    if high_type == &"bot" {
        let bot_vals = vals.entry(*high).or_insert(vec![]);
        let val = bvals[0].max(bvals[1]);
        bot_vals.push(val);
        if bot_vals.len() == 2 {
            process_b(*high, &configuration, bot_vals.clone(), &mut vals, output)
        }
    } else {
        let val = bvals[0].max(bvals[1]);
        output.entry(*high).or_insert(vec![]).push(val);
    }
}

#[cfg(test)]
mod day10_tests {
    use std::fs;

    use crate::y2016::day10::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(
            2,
            part_a(
                fs::read_to_string("input/2016/day10/test.txt").unwrap(),
                2,
                5
            )
        );
        assert_eq!(
            30,
            part_b(fs::read_to_string("input/2016/day10/test.txt").unwrap())
        );
    }

    #[test]
    fn input_works() {
        assert_eq!(
            116,
            part_a(
                fs::read_to_string("input/2016/day10/input.txt").unwrap(),
                17,
                61
            )
        );
        assert_eq!(
            23903,
            part_b(fs::read_to_string("input/2016/day10/input.txt").unwrap())
        );
    }
}
