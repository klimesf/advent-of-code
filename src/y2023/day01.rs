use std::fs;

pub(crate) fn day01() {
    println!("{}", part_a(fs::read_to_string("input/2023/day01/input.txt").unwrap()));
    println!("{}", part_b(fs::read_to_string("input/2023/day01/input.txt").unwrap()));
}

fn part_a(elves: String) -> u32 {
    return elves.lines().map(|line| {
        let mut has_first = false;
        let mut first = 0;
        let mut last = 0;

        for x in line.chars() {
            if x.is_digit(10) {
                let z = x.to_digit(10).unwrap();
                if !has_first {
                    has_first = true;
                    first = z;
                }
                last = z;
            }
        }

        first * 10 + last
    }).sum();
}

fn part_b(elves: String) -> u32 {
    return elves.lines().map(|l| {
        let mut line = l;
        let mut nums = vec!();

        while line.len() > 0 {
            if line.starts_with("1") || line.starts_with("one") {
                nums.push(1);
            }
            if line.starts_with("2") || line.starts_with("two") {
                nums.push(2);
            }
            if line.starts_with("3") || line.starts_with("three") {
                nums.push(3);
            }
            if line.starts_with("4") || line.starts_with("four") {
                nums.push(4);
            }
            if line.starts_with("5") || line.starts_with("five") {
                nums.push(5);
            }
            if line.starts_with("6") || line.starts_with("six") {
                nums.push(6);
            }
            if line.starts_with("7") || line.starts_with("seven") {
                nums.push(7);
            }
            if line.starts_with("8") || line.starts_with("eight") {
                nums.push(8);
            }
            if line.starts_with("9") || line.starts_with("nine") {
                nums.push(9);
            }
            line = &line[1..]
        }

        10 * nums.first().unwrap() + nums.last().unwrap()
    }).sum();
}

#[cfg(test)]
mod day01_tests {
    use std::fs;

    use crate::y2023::day01::{part_a, part_b};

    #[test]
    fn test_works() {
        let input_a = fs::read_to_string("input/2023/day01/test.txt").unwrap();
        assert_eq!(142, part_a(input_a));
        let input_b = fs::read_to_string("input/2023/day01/test-b.txt").unwrap();
        assert_eq!(281, part_b(input_b));
    }

    #[test]
    fn input_works() {
        assert_eq!(54990, part_a(fs::read_to_string("input/2023/day01/input.txt").unwrap()));
        assert_eq!(54473, part_b(fs::read_to_string("input/2023/day01/input.txt").unwrap()));
    }
}
