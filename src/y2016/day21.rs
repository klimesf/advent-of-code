use crate::utils::toolbox::{parse_char, parse_usize};
use itertools::Itertools;
use regex::Regex;
use std::fs;

pub(crate) fn day21() {
    println!("{}", part_a(fs::read_to_string("input/2016/day21/input.txt").unwrap(), "abcdefgh"));
    println!("{}", part_b(fs::read_to_string("input/2016/day21/input.txt").unwrap(), "fbgdceah"));
}

fn part_a(input: String, input_pass: &str) -> String {
    let instructions: Vec<Op> = input
        .lines()
        .map(|line| {
            if line.starts_with("swap position") {
                let re = Regex::new("swap position ([0-9]+) with position ([0-9]+)").unwrap();
                let caps = re.captures(line).unwrap();
                Op::SwapPositions(parse_usize(caps.get(1)), parse_usize(caps.get(2)))
            } else if line.starts_with("swap letter") {
                let re = Regex::new("swap letter (.) with letter (.)").unwrap();
                let caps = re.captures(line).unwrap();
                Op::SwapLetters(parse_char(caps.get(1)), parse_char(caps.get(2)))
            } else if line.starts_with("rotate left") {
                let re = Regex::new("rotate left ([0-9]+) step").unwrap();
                let caps = re.captures(line).unwrap();
                Op::RotateLeft(parse_usize(caps.get(1)))
            } else if line.starts_with("rotate right") {
                let re = Regex::new("rotate right ([0-9]+) step").unwrap();
                let caps = re.captures(line).unwrap();
                Op::RotateRight(parse_usize(caps.get(1)))
            } else if line.starts_with("rotate based") {
                let re = Regex::new("rotate based on position of letter (.)").unwrap();
                let caps = re.captures(line).unwrap();
                Op::RotateLetter(parse_char(caps.get(1)))
            } else if line.starts_with("reverse") {
                let re = Regex::new("reverse positions ([0-9]+) through ([0-9]+)").unwrap();
                let caps = re.captures(line).unwrap();
                Op::Reverse(parse_usize(caps.get(1)), parse_usize(caps.get(2)))
            } else {
                let re = Regex::new("move position ([0-9]+) to position ([0-9]+)").unwrap();
                let caps = re.captures(line).unwrap();
                Op::Move(parse_usize(caps.get(1)), parse_usize(caps.get(2)))
            }
        })
        .collect();

    let mut pass: Vec<char> = input_pass.chars().collect();
    for op in instructions {
        match op {
            Op::SwapPositions(a, b) => swap_pos(a, b, &mut pass),
            Op::SwapLetters(a, b) => swap_letters(a, b, &mut pass),
            Op::RotateLeft(a) => rot_left(a, &mut pass),
            Op::RotateRight(a) => rot_right(a, &mut pass),
            Op::RotateLetter(a) => rot_letter(a, &mut pass),
            Op::Reverse(a, b) => reverse(a, b, &mut pass),
            Op::Move(a, b) => movee(a, b, &mut pass),
        }
    }

    let mut ans = String::new();
    for c in pass {
        ans.push(c);
    }
    ans
}

fn part_b(input: String, input_pass: &str) -> String {
    let instructions: Vec<Op> = input
        .lines()
        .map(|line| {
            if line.starts_with("swap position") {
                let re = Regex::new("swap position ([0-9]+) with position ([0-9]+)").unwrap();
                let caps = re.captures(line).unwrap();
                Op::SwapPositions(parse_usize(caps.get(1)), parse_usize(caps.get(2)))
            } else if line.starts_with("swap letter") {
                let re = Regex::new("swap letter (.) with letter (.)").unwrap();
                let caps = re.captures(line).unwrap();
                Op::SwapLetters(parse_char(caps.get(1)), parse_char(caps.get(2)))
            } else if line.starts_with("rotate left") {
                let re = Regex::new("rotate left ([0-9]+) step").unwrap();
                let caps = re.captures(line).unwrap();
                Op::RotateLeft(parse_usize(caps.get(1)))
            } else if line.starts_with("rotate right") {
                let re = Regex::new("rotate right ([0-9]+) step").unwrap();
                let caps = re.captures(line).unwrap();
                Op::RotateRight(parse_usize(caps.get(1)))
            } else if line.starts_with("rotate based") {
                let re = Regex::new("rotate based on position of letter (.)").unwrap();
                let caps = re.captures(line).unwrap();
                Op::RotateLetter(parse_char(caps.get(1)))
            } else if line.starts_with("reverse") {
                let re = Regex::new("reverse positions ([0-9]+) through ([0-9]+)").unwrap();
                let caps = re.captures(line).unwrap();
                Op::Reverse(parse_usize(caps.get(1)), parse_usize(caps.get(2)))
            } else {
                let re = Regex::new("move position ([0-9]+) to position ([0-9]+)").unwrap();
                let caps = re.captures(line).unwrap();
                Op::Move(parse_usize(caps.get(1)), parse_usize(caps.get(2)))
            }
        })
        .collect();

    let scrambled: Vec<char> = input_pass.chars().collect();

    for perm in scrambled.iter().permutations(scrambled.len()).unique() {
        let unscrambled = perm.clone();
        let mut s = perm.iter().map(|c| **c).collect();
        for op in &instructions {
            match *op {
                Op::SwapPositions(a, b) => swap_pos(a, b, &mut s),
                Op::SwapLetters(a, b) => swap_letters(a, b, &mut s),
                Op::RotateLeft(a) => rot_left(a, &mut s),
                Op::RotateRight(a) => rot_right(a, &mut s),
                Op::RotateLetter(a) => rot_letter(a, &mut s),
                Op::Reverse(a, b) => reverse(a, b, &mut s),
                Op::Move(a, b) => movee(a, b, &mut s),
            }
        }
        if s == scrambled {
            let mut ans = String::new();
            for c in unscrambled {
                ans.push(*c);
            }
            return ans;
        }
    }

    panic!("never found unscrambled")
}

#[allow(unused)]
#[derive(Copy, Clone, Debug)]
enum Op {
    SwapPositions(usize, usize),
    SwapLetters(char, char),
    RotateLeft(usize),
    RotateRight(usize),
    RotateLetter(char),
    Reverse(usize, usize),
    Move(usize, usize),
}

fn swap_pos(a: usize, b: usize, s: &mut Vec<char>) {
    let tmp = s[a];
    s[a] = s[b];
    s[b] = tmp;
}

fn swap_letters(a: char, b: char, s: &mut Vec<char>) {
    let x = s.iter().find_position(|c| **c == a).unwrap().0;
    let y = s.iter().find_position(|c| **c == b).unwrap().0;
    swap_pos(x, y, s)
}

fn rot_left(a: usize, s: &mut Vec<char>) {
    for _ in 0..a {
        let tmp = s[0];
        let s_len = s.len();
        for i in 0..(s_len - 1) {
            s[i] = s[i + 1];
        }
        s[s_len - 1] = tmp;
    }
}

fn rot_right(a: usize, s: &mut Vec<char>) {
    for _ in 0..a {
        let s_len = s.len();
        let tmp = s[s_len - 1];
        for i in (0..(s_len - 1)).rev() {
            s[i + 1] = s[i];
        }
        s[0] = tmp;
    }
}

fn rot_letter(a: char, s: &mut Vec<char>) {
    let mut x = s.iter().find_position(|c| **c == a).unwrap().0;
    if x >= 4 {
        x += 1;
    }
    rot_right(x + 1, s)
}

fn reverse(a: usize, b: usize, s: &mut Vec<char>) {
    let diff = (b - a) / 2;
    for i in 0..=diff {
        swap_pos(a + i, b - i, s)
    }
}

fn movee(a: usize, b: usize, s: &mut Vec<char>) {
    if a > b {
        let temp = s[a];
        for i in (b..a).rev() {
            s[i + 1] = s[i];
        }
        s[b] = temp;
    } else {
        let temp = s[a];
        for i in a..b {
            s[i] = s[i + 1];
        }
        s[b] = temp;
    }
}

#[cfg(test)]
mod day21_tests {
    use std::fs;

    use crate::y2016::day21::{
        movee, part_a, part_b, reverse, rot_left, rot_letter, rot_right, swap_letters, swap_pos,
    };

    #[test]
    fn swap_pos_works() {
        let mut ans = vec!['a', 'b', 'c', 'd', 'e'];
        swap_pos(4, 0, &mut ans);
        assert_eq!(vec! {'e', 'b', 'c', 'd', 'a'}, ans);
    }

    #[test]
    fn swap_letters_works() {
        let mut ans = vec!['e', 'b', 'c', 'd', 'a'];
        swap_letters('d', 'b', &mut ans);
        assert_eq!(vec! {'e', 'd', 'c', 'b', 'a'}, ans);
    }

    #[test]
    fn rot_left_works() {
        let mut ans = vec!['a', 'b', 'c', 'd', 'e'];
        rot_left(2, &mut ans);
        assert_eq!(vec! {'c', 'd', 'e', 'a', 'b'}, ans);
    }

    #[test]
    fn rot_right_works() {
        let mut ans = vec!['a', 'b', 'c', 'd'];
        rot_right(1, &mut ans);
        assert_eq!(vec! {'d', 'a', 'b', 'c'}, ans);
    }

    #[test]
    fn rot_letter_works() {
        let mut ans = vec!['a', 'b', 'd', 'e', 'c'];
        rot_letter('b', &mut ans);
        assert_eq!(vec! {'e', 'c', 'a', 'b', 'd'}, ans);
        rot_letter('d', &mut ans);
        assert_eq!(vec! {'d', 'e', 'c', 'a', 'b'}, ans);
    }

    #[test]
    fn reverse_works() {
        let mut ans = vec!['e', 'd', 'c', 'b', 'a'];
        reverse(0, 4, &mut ans);
        assert_eq!(vec! {'a', 'b', 'c', 'd', 'e'}, ans);
        reverse(0, 1, &mut ans);
        assert_eq!(vec! {'b', 'a', 'c', 'd', 'e'}, ans);
        reverse(1, 2, &mut ans);
        assert_eq!(vec! {'b', 'c', 'a', 'd', 'e'}, ans);
    }

    #[test]
    fn movee_works() {
        let mut ans = vec!['b', 'c', 'd', 'e', 'a'];
        movee(1, 4, &mut ans);
        assert_eq!(vec! {'b', 'd', 'e', 'a', 'c'}, ans);
        movee(3, 0, &mut ans);
        assert_eq!(vec! {'a', 'b', 'd', 'e', 'c'}, ans);
    }

    #[test]
    fn test_works() {
        assert_eq!("decab", part_a(fs::read_to_string("input/2016/day21/test.txt").unwrap(), "abcde"));
        assert_eq!("abcde", part_b(fs::read_to_string("input/2016/day21/test.txt").unwrap(), "decab"));
    }

    #[test]
    fn input_works() {
        assert_eq!("bgfacdeh", part_a(fs::read_to_string("input/2016/day21/input.txt").unwrap(), "abcdefgh"));
        assert_eq!("bdgheacf", part_b(fs::read_to_string("input/2016/day21/input.txt").unwrap(), "fbgdceah"));
    }
}
