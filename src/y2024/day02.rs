use std::fs;

pub fn day02(print: fn(usize)) {
    print(part_a(fs::read_to_string("input/2024/day02/input.txt").unwrap()));
    print(part_b(fs::read_to_string("input/2024/day02/input.txt").unwrap()));
}

fn part_a(input: String) -> usize {
    input
        .lines()
        .filter(|line| {
            let nums: Vec<u32> = line
                .split(" ")
                .map(|num| num.parse::<u32>().unwrap())
                .collect();
            is_correct(nums)
        })
        .count()
}

fn part_b(input: String) -> usize {
    input
        .lines()
        .filter(|line| {
            let nums: Vec<u32> = line
                .split(" ")
                .map(|num| num.parse::<u32>().unwrap())
                .collect();

            if is_correct(nums.clone()) {
                return true;
            }

            for i in 0..nums.len() {
                let new_nums = nums[..i].iter().chain(nums[(i + 1)..].iter())
                    .cloned().collect();
                if is_correct(new_nums) {
                    return true;
                }
            }
            return false;
        })
        .count()
}

fn is_correct(nums: Vec<u32>) -> bool {
    let mut diff = true;
    let mut increases = true;
    let mut decreases = true;

    for i in 0..nums.len() - 1 {
        let a = nums[i];
        let b = nums[i + 1];

        if a < b && decreases {
            decreases = false;
        }
        if a > b && increases {
            increases = false;
        }

        if a.max(b) - a.min(b) > 3 || a.max(b) - a.min(b) < 1 {
            diff = false;
        }
    }

    diff && (increases || decreases)
}

#[cfg(test)]
mod day02_tests {
    use std::fs;

    use crate::y2024::day02::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(2, part_a(fs::read_to_string("input/2024/day02/test.txt").unwrap()));
        assert_eq!(4, part_b(fs::read_to_string("input/2024/day02/test.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!(269, part_a(fs::read_to_string("input/2024/day02/input.txt").unwrap()));
        assert_eq!(337, part_b(fs::read_to_string("input/2024/day02/input.txt").unwrap()));
    }
}
