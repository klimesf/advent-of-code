use std::collections::HashMap;
use std::fs;

pub(crate) fn day15() {
    println!(
        "{}",
        part_a(fs::read_to_string("input/2020/day15/input.txt").unwrap())
    );
    println!(
        "{}",
        part_b(fs::read_to_string("input/2020/day15/input.txt").unwrap())
    );
}

fn part_a(input: String) -> usize {
    let mut nums = input
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let mut i = nums.len() - 1;
    while i < 2019 {
        let last = nums[i];

        let mut prev_seen = 0;
        let mut seen = false;
        for j in (0..i).rev() {
            if nums[j] == last {
                prev_seen = j;
                seen = true;
                break;
            }
        }

        if seen {
            nums.push(i - prev_seen);
        } else {
            nums.push(0);
        }
        i += 1;
    }
    nums[nums.len() - 1]
}

fn part_b(input: String) -> usize {
    let mut nums = input
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let mut seen_1 = HashMap::new();
    let mut seen_2 = HashMap::new();
    for i in 0..nums.len() - 1 {
        let num = nums[i];
        seen_1.insert(num, i);
    }

    let mut i = nums.len() - 1;
    while i < 29999999 {
        let last = nums[i];

        if seen_1.contains_key(&last) {
            let last_seen_1 = *seen_1.get(&last).unwrap();
            seen_2.insert(last, last_seen_1);
            nums.push(i - last_seen_1);
            seen_1.insert(last, i);
        } else {
            seen_1.insert(last, i);
            nums.push(0);
        }
        i += 1;
    }
    nums[nums.len() - 1]
}

#[cfg(test)]
mod day15_tests {
    use std::fs;

    use crate::y2020::day15::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(
            436,
            part_a(fs::read_to_string("input/2020/day15/test.txt").unwrap())
        );
        assert_eq!(
            175594,
            part_b(fs::read_to_string("input/2020/day15/test.txt").unwrap())
        );
    }

    #[test]
    fn input_works() {
        assert_eq!(
            492,
            part_a(fs::read_to_string("input/2020/day15/input.txt").unwrap())
        );
        assert_eq!(
            63644,
            part_b(fs::read_to_string("input/2020/day15/input.txt").unwrap())
        );
    }
}
