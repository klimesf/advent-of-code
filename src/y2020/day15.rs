use std::fs;

pub(crate) fn day15() {
    println!("{}", part_a(fs::read_to_string("input/2020/day15/input.txt").unwrap()));
    println!("{}", part_b(fs::read_to_string("input/2020/day15/input.txt").unwrap()));
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
    let nums = input
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let mut seen = vec![usize::MAX; 30_000_000];
    for i in 0..nums.len() - 1 {
        let num = nums[i];
        seen[num] = i;
    }

    let mut i = nums.len() - 1;
    let mut last = nums[i];
    while i < 30_000_000 - 1 {
        if seen[last] != usize::MAX {
            let last_seen = seen[last];
            seen[last] = i;
            last = i - last_seen;
        } else {
            seen[last] = i;
            last = 0;
        }
        i += 1;
    }
    last
}

#[cfg(test)]
mod day15_tests {
    use std::fs;

    use crate::y2020::day15::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(436, part_a(fs::read_to_string("input/2020/day15/test.txt").unwrap()));
        assert_eq!(175594, part_b(fs::read_to_string("input/2020/day15/test.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!(492, part_a(fs::read_to_string("input/2020/day15/input.txt").unwrap()));
        assert_eq!(63644, part_b(fs::read_to_string("input/2020/day15/input.txt").unwrap()));
    }
}
