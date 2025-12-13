use std::collections::HashMap;
use std::fs;

pub(crate) fn day13() {
    let input = fs::read_to_string("input/2017/day13/input.txt").unwrap();
    let layers = parse(input);
    println!("{}", severity(0, &layers));
    println!("{}", find_delay(&layers));
}

fn parse(input: String) -> HashMap<i32, i32> {
    let layers: HashMap<i32, i32> = input
        .lines()
        .map(|line| {
            let (depth, range) = line.split_once(": ").unwrap();
            (depth.parse().unwrap(), range.parse().unwrap())
        })
        .collect();
    layers
}

fn severity(delay: i32, layers: &HashMap<i32, i32>) -> i32 {
    layers
        .iter()
        .map(|(depth, range)| {
            let scanner_pos = scanner_pos(delay + depth, *range);
            if scanner_pos == 0 {
                depth * range
            } else {
                0
            }
        })
        .sum()
}

fn scanner_pos(picosecond: i32, range: i32) -> i32 {
    let mut pos = picosecond % ((range * 2) - 2);
    if pos >= range {
        pos = 2 * range - pos - 2
    };
    pos
}

fn find_delay(layers: &HashMap<i32, i32>) -> i32 {
    let mut delay = 0;
    loop {
        if layers.iter().any(|(depth, range)| {
            let scanner_pos = scanner_pos(delay + depth, *range);
            scanner_pos == 0
        }) {
            delay += 1;
            continue;
        }
        return delay;
    }
}

#[cfg(test)]
mod day13_tests {
    use std::fs;

    use crate::y2017::day13::{find_delay, parse, scanner_pos, severity};

    #[test]
    fn scanner_pos_works() {
        assert_eq!(0, scanner_pos(0, 3));
        assert_eq!(1, scanner_pos(1, 3));
        assert_eq!(2, scanner_pos(2, 3));
        assert_eq!(1, scanner_pos(3, 3));
        assert_eq!(0, scanner_pos(4, 3));
        assert_eq!(1, scanner_pos(5, 3));
        assert_eq!(2, scanner_pos(6, 3));
        assert_eq!(2, scanner_pos(10, 3));
        assert_eq!(2, scanner_pos(10, 4));
        assert_eq!(0, scanner_pos(10, 2));
    }

    #[test]
    fn test_works() {
        let input = fs::read_to_string("input/2017/day13/test.txt").unwrap();
        let layers = parse(input);
        assert_eq!(24, severity(0, &layers));
        assert_eq!(10, find_delay(&layers));
    }

    #[test]
    fn input_works() {
        let input = fs::read_to_string("input/2017/day13/input.txt").unwrap();
        let layers = parse(input);
        assert_eq!(2164, severity(0, &layers));
        assert_eq!(3861798, find_delay(&layers));
    }
}
