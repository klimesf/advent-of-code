use std::collections::{HashSet, VecDeque};
use std::fs;

pub(crate) fn day06() {
    let input = fs::read_to_string("input/2022/day06/input.txt").unwrap();
    println!("{}", part_a(input.trim()));
    println!("{}", part_b(input.trim()));
}

fn part_a(input: &str) -> usize {
    find_first_n_different(input, 4)
}

fn part_b(input: &str) -> usize {
    find_first_n_different(input, 14)
}

fn find_first_n_different(input: &str, len: usize) -> usize {
    let mut last_n: VecDeque<char> = VecDeque::new();
    for (i, c) in input.chars().enumerate() {
        if last_n.len() == len && all_different(last_n.clone()) {
            return i;
        }
        last_n.push_front(c);
        if last_n.len() > len {
            last_n.pop_back();
        }
    }
    panic!("No unique sequence found")
}

fn all_different(deque: VecDeque<char>) -> bool {
    let mut chars = HashSet::new();
    for c in deque {
        if chars.contains(&c) {
            return false;
        }
        chars.insert(c);
    }
    return true;
}

#[cfg(test)]
mod day06_tests {
    use crate::y2022::day06::{part_a, part_b};

    #[test]
    fn part_a_works() {
        assert_eq!(7, part_a("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
        assert_eq!(5, part_a("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(6, part_a("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(10, part_a("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!(11, part_a("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }

    #[test]
    fn part_b_works() {
        assert_eq!(19, part_b("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
        assert_eq!(23, part_b("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(23, part_b("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(29, part_b("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!(26, part_b("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }
}
