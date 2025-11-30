use std::fs;

pub fn day01(print: fn(i32)) {
    print(part_a(fs::read_to_string("input/2025/day01/input.txt").unwrap()));
    print(part_b(fs::read_to_string("input/2025/day01/input.txt").unwrap()));
}

fn part_a(input: String) -> i32 {
    let mut dial = 50;
    let mut ans = 0;
    input.lines().for_each(|line| {
        let num = line[1..].parse::<i32>().unwrap();
        let dir = line[0..1].to_string();

        if dir == "R" {
            dial += num;
            while dial >= 100 {
                dial -= 100;
            }
        } else {
            dial -= num;
            while dial < 0 {
                dial += 100;
            }
        }
        if dial == 0 {
            ans += 1;
        }
    });
    ans
}

fn part_b(input: String) -> i32 {
    let mut dial = 50;
    let mut ans = 0;
    input.lines().for_each(|line| {
        let num = line[1..].parse::<i32>().unwrap();
        let dir = line[0..1].to_string();

        if dir == "R" {
            for _ in 0..num {
                if dial == 99 {
                    dial = 0;
                } else {
                    dial += 1;
                }
                if dial == 0 {
                    ans += 1;
                }
            }
        } else {
            for _ in 0..num {
                if dial == 0 {
                    dial = 99;
                } else {
                    dial -= 1;
                }
                if dial == 0 {
                    ans += 1;
                }
            }
        }
    });
    ans
}

#[cfg(test)]
mod day01_tests {
    use std::fs;
    
    use crate::y2025::day01::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(3, part_a(fs::read_to_string("input/2025/day01/test.txt").unwrap()));
        assert_eq!(6, part_b(fs::read_to_string("input/2025/day01/test.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!(1145, part_a(fs::read_to_string("input/2025/day01/input.txt").unwrap()));
        assert_eq!(6561, part_b(fs::read_to_string("input/2025/day01/input.txt").unwrap()));
    }
}
