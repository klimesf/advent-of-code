use std::fs;
use rayon::iter::ParallelIterator;
use rayon::iter::ParallelBridge;

pub fn day07(print: fn(usize)) {
    print(part_a(fs::read_to_string("input/2024/day07/input.txt").unwrap()));
    print(part_b(fs::read_to_string("input/2024/day07/input.txt").unwrap()));
}

fn part_a(input: String) -> usize {
    input.lines().par_bridge().map(|line| {
        let (l, r) = line.split_once(": ").unwrap();
        let target = l.parse::<usize>().unwrap();
        let digits: Vec<usize> = r.split(" ").map(|x| x.parse::<usize>().unwrap()).collect();

        let mut stack: Vec<(usize, usize)> = vec!();
        stack.push((digits[0], 1));

        while !stack.is_empty() {
            let (sum, pos) = stack.pop().unwrap();
            if sum == target && pos >= digits.len() { return target; }
            if sum > target { continue; }
            if pos >= digits.len() { continue; }

            stack.push((sum + digits[pos], pos + 1));
            stack.push((sum * digits[pos], pos + 1));
        }
        0
    }).sum()
}

fn part_b(input: String) -> usize {
    input.lines().par_bridge().map(|line| {
        let (l, r) = line.split_once(": ").unwrap();
        let target = l.parse::<usize>().unwrap();
        let digits: Vec<usize> = r.split(" ").map(|x| x.parse::<usize>().unwrap()).collect();

        let mut stack: Vec<(usize, usize)> = vec!();
        stack.push((digits[0], 1));

        while !stack.is_empty() {
            let (sum, pos) = stack.pop().unwrap();
            if sum == target && pos >= digits.len() { return target; }
            if sum > target { continue; }
            if pos >= digits.len() { continue; }
            let n = (0..).take_while(|digit| 10_usize.pow(*digit) <= digits[pos]).count();

            stack.push((sum + digits[pos], pos + 1));
            stack.push((sum * digits[pos], pos + 1));
            stack.push((sum * 10_usize.pow(n as u32) + digits[pos], pos + 1));
        }
        0
    }).sum()
}

#[cfg(test)]
mod day07_tests {
    use std::fs;

    use crate::y2024::day07::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(3749, part_a(fs::read_to_string("input/2024/day07/test.txt").unwrap()));
        assert_eq!(11387 - 3749, part_b(fs::read_to_string("input/2024/day07/test2.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!(20665830408335, part_a(fs::read_to_string("input/2024/day07/input.txt").unwrap()));
        assert_eq!(354060705047464, part_b(fs::read_to_string("input/2024/day07/input.txt").unwrap()));
    }
}
