use std::collections::HashSet;
use std::fs;

pub(crate) fn day02(print: fn(usize)) {
    print(part_a(
        fs::read_to_string("input/2025/day02/input.txt").unwrap(),
    ));
    print(part_b(
        fs::read_to_string("input/2025/day02/input.txt").unwrap(),
    ));
}

fn part_a(input: String) -> usize {
    let ranges = input.split(',').collect::<Vec<&str>>();
    let mut ans = 0;
    ranges.iter().for_each(|range| {
        let (l, r) = range.split_once('-').unwrap();
        let start = l.parse::<usize>().unwrap();
        let end = r.parse::<usize>().unwrap();

        for n in start..=end {
            let len = n.checked_ilog10().unwrap_or(0) + 1;
            if len % 2 != 0 {
                continue;
            }
            let div = 10_usize.pow(len / 2);
            if n % div == n / div {
                ans += n;
            }
        }
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

        for n in start..=end {
            let len = n.checked_ilog10().unwrap_or(0) + 1;
            for parts in 2..=len {
                if len % parts != 0 {
                    continue;
                }

                let div = 10_usize.pow(len / parts);

                let mut last = n % div;
                let mut part: usize;
                let mut rem = n / div;
                let mut same = true;
                while rem > 0 {
                    part = rem % div;
                    rem = rem / div;
                    if part != last {
                        same = false;
                        break;
                    }
                    last = part;
                }
                if same {
                    ans.insert(n);
                }
            }
        }
    });

    ans.iter().sum()
}

#[cfg(test)]
mod day02_tests {
    use std::fs;

    use crate::y2025::day02::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(
            1227775554,
            part_a(fs::read_to_string("input/2025/day02/test.txt").unwrap())
        );
        assert_eq!(
            4174379265,
            part_b(fs::read_to_string("input/2025/day02/test.txt").unwrap())
        );
    }

    #[test]
    fn input_works() {
        assert_eq!(
            19219508902,
            part_a(fs::read_to_string("input/2025/day02/input.txt").unwrap())
        );
        assert_eq!(
            27180728081,
            part_b(fs::read_to_string("input/2025/day02/input.txt").unwrap())
        );
    }
}
