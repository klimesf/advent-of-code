use std::collections::HashSet;
use std::fs;

pub(crate) fn day02(print: fn(usize)) {
    print(part_a(fs::read_to_string("input/2025/day02/input.txt").unwrap()));
    print(part_b(fs::read_to_string("input/2025/day02/input.txt").unwrap()));
}

fn part_a(input: String) -> usize {
    let ranges = input.split(',').collect::<Vec<&str>>();
    let mut ans = 0;
    ranges.iter().for_each(|range| {
        let (l, r) = range.split_once('-').unwrap();
        let start = l.parse::<usize>().unwrap();
        let end = r.parse::<usize>().unwrap();
        (start..=end).for_each(|num| {
            let num_str = num.to_string();

            if num_str.len() % 2 != 0 { return; }
            let mut same = true;
            let half = num_str.len() / 2;
            for i in 0..half {
                if num_str.chars().nth(i).unwrap() != num_str.chars().nth(i + half).unwrap() {
                    same = false;
                    break;
                }
            }

            if same {
                ans += num;
            }
        });
    });

    ans
}

fn part_b(input: String) -> usize {
    let ranges = input.split(',').collect::<Vec<&str>>();
    let mut ans = HashSet::new();
    ranges.iter().for_each(|range| {
        let (l, r) = range.split_once('-').unwrap();
        let start = l.parse::<usize>().unwrap();
        let end = r.parse::<usize>().unwrap();
        (start..=end).for_each(|num| {
            let num_str = num.to_string();
            let half = num_str.len() / 2;
            for len in 1..=half {
                if num_str.len() % len != 0 { continue; }
                let mut same = true;
                for split in 0..((num_str.len() / len) - 1) {
                    for i in (split * len)..((split + 1) * len) {
                        if num_str.chars().nth(i).unwrap() != num_str.chars().nth(i + len).unwrap() {
                            same = false;
                            break;
                        }
                    }

                }

                if same {
                    ans.insert(num);
                }
            }
        });
    });

    ans.iter().sum()
}

#[cfg(test)]
mod day02_tests {
    use std::fs;

    use crate::y2025::day02::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(1227775554, part_a(fs::read_to_string("input/2025/day02/test.txt").unwrap()));
        assert_eq!(4174379265, part_b(fs::read_to_string("input/2025/day02/test.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!(19219508902, part_a(fs::read_to_string("input/2025/day02/input.txt").unwrap()));
        assert_eq!(27180728081, part_b(fs::read_to_string("input/2025/day02/input.txt").unwrap()));
    }
}
