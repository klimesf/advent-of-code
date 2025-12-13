use std::fs;

pub(crate) fn day01() {
    println!(
        "{}",
        part_a(fs::read_to_string("input/2020/day01/input.txt").unwrap())
    );
    println!(
        "{}",
        part_b(fs::read_to_string("input/2020/day01/input.txt").unwrap())
    );
}

fn part_a(input: String) -> usize {
    let nums = input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<usize>>();

    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            if nums[i] + nums[j] == 2020 {
                return nums[i] * nums[j];
            }
        }
    }
    panic!("No solution found!");
}

fn part_b(input: String) -> usize {
    let nums = input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<usize>>();

    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            for k in j + 1..nums.len() {
                if nums[i] + nums[j] + nums[k] == 2020 {
                    return nums[i] * nums[j] * nums[k];
                }
            }
        }
    }
    panic!("No solution found!");
}

#[cfg(test)]
mod day01_tests {
    use std::fs;

    use crate::y2020::day01::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(
            514579,
            part_a(fs::read_to_string("input/2020/day01/test.txt").unwrap())
        );
        assert_eq!(
            241861950,
            part_b(fs::read_to_string("input/2020/day01/test.txt").unwrap())
        );
    }

    #[test]
    fn input_works() {
        assert_eq!(
            1014624,
            part_a(fs::read_to_string("input/2020/day01/input.txt").unwrap())
        );
        assert_eq!(
            80072256,
            part_b(fs::read_to_string("input/2020/day01/input.txt").unwrap())
        );
    }
}
