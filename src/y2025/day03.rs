use std::fs;

pub fn day03(print: fn(usize)) {
    print(part_a(fs::read_to_string("input/2025/day03/input.txt").unwrap()));
    print(part_b(fs::read_to_string("input/2025/day03/input.txt").unwrap()));
}

fn part_a(input: String) -> usize {
    let mut ans = 0;
    input.lines().for_each(|line| {
        let chars: Vec<usize> = line.chars().map(|c| c.to_digit(10).unwrap() as usize).collect();

        let mut max = 0;
        let mut pos = 0;
        for i in 0..chars.len() - 1 {
            if chars[i] > max {
                max = chars[i];
                pos = i;
            }
        }

        let mut max_2 = 0;
        for i in pos + 1..chars.len() {
            if chars[i] > max_2 {
                max_2 = chars[i];
            }
        }

        ans += max * 10 + max_2;
    });

    ans
}

fn part_b(input: String) -> usize {
    let mut ans = 0;
    input.lines().for_each(|line| {
        let chars: Vec<usize> = line.chars().map(|c| c.to_digit(10).unwrap() as usize).collect();

        let mut pos = 0;
        for j in 1..=12 {
            let mut max = 0;
            for i in pos..(chars.len() - (12 - j)) {
                if chars[i] > max {
                    max = chars[i];
                    pos = i + 1;
                }
            }
            ans += max * 10_usize.pow(12 - j as u32);
        }
    });

    ans
}

#[cfg(test)]
mod day03_tests {
    use std::fs;

    use crate::y2025::day03::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(357, part_a(fs::read_to_string("input/2025/day03/test.txt").unwrap()));
        assert_eq!(3121910778619, part_b(fs::read_to_string("input/2025/day03/test.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!(17412, part_a(fs::read_to_string("input/2025/day03/input.txt").unwrap()));
        assert_eq!(172681562473501, part_b(fs::read_to_string("input/2025/day03/input.txt").unwrap()));
    }
}
