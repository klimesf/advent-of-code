use crate::utils::toolbox::parse_usize;
use regex::Regex;
use std::fs;

pub(crate) fn day15() {
    println!(
        "{}",
        part_a(fs::read_to_string("input/2016/day15/input.txt").unwrap())
    );
    println!(
        "{}",
        part_b(fs::read_to_string("input/2016/day15/input.txt").unwrap())
    );
}

fn part_a(input: String) -> usize {
    let re = Regex::new(
        r"^Disc #[0-9]+ has ([0-9]+) positions; at time=0, it is at position ([0-9]+)\.$",
    )
    .unwrap();
    let discs: Vec<(usize, usize)> = input
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            (parse_usize(caps.get(1)), parse_usize(caps.get(2)))
        })
        .collect();

    let mut time_offset = 0;
    'outer: loop {
        for i in 0..discs.len() {
            let time = i + 1 + time_offset;
            let (positions, start) = discs[i];
            let position = (start + time) % positions;

            if position != 0 {
                time_offset += 1;
                continue 'outer;
            }
        }
        return time_offset;
    }
}

fn part_b(input: String) -> usize {
    let re = Regex::new(
        r"^Disc #[0-9]+ has ([0-9]+) positions; at time=0, it is at position ([0-9]+)\.$",
    )
    .unwrap();
    let mut discs: Vec<(usize, usize)> = input
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            (parse_usize(caps.get(1)), parse_usize(caps.get(2)))
        })
        .collect();
    discs.push((11, 0));

    let mut time_offset = 0;
    'outer: loop {
        for i in 0..discs.len() {
            let time = i + 1 + time_offset;
            let (positions, start) = discs[i];
            let position = (start + time) % positions;

            if position != 0 {
                time_offset += 1;
                continue 'outer;
            }
        }
        return time_offset;
    }
}

#[cfg(test)]
mod day15_tests {
    use std::fs;

    use crate::y2016::day15::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(
            5,
            part_a(fs::read_to_string("input/2016/day15/test.txt").unwrap())
        );
    }

    #[test]
    fn input_works() {
        assert_eq!(
            400589,
            part_a(fs::read_to_string("input/2016/day15/input.txt").unwrap())
        );
        assert_eq!(
            3045959,
            part_b(fs::read_to_string("input/2016/day15/input.txt").unwrap())
        );
    }
}
