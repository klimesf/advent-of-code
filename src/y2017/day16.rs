use std::collections::{HashMap, VecDeque};
use std::fs;

use itertools::Itertools;

pub(crate) fn day16() {
    let input = fs::read_to_string("input/2017/day16/input.txt").unwrap();
    let moves: Vec<DanceMove> = parse(input.trim());
    println!("{}", dance("abcdefghijklmnop", &moves));
    println!("{}", dance_a_lot("abcdefghijklmnop", &moves));
}

enum DanceMove {
    S(usize),
    X(usize, usize),
    P(char, char),
}

fn parse(input: &str) -> Vec<DanceMove> {
    input
        .trim()
        .split(",")
        .map(|m| match m.chars().nth(0).unwrap() {
            's' => {
                let i = m[1..m.len()].parse::<usize>().unwrap();
                DanceMove::S(i)
            }
            'x' => {
                let (from, to) = m[1..m.len()].split_once("/").unwrap();
                DanceMove::X(from.parse().unwrap(), to.parse().unwrap())
            }
            'p' => {
                let first = m.chars().nth(1).unwrap();
                let second = m.chars().nth(3).unwrap();
                DanceMove::P(first, second)
            }
            c => panic!("Unknown move {}", c),
        })
        .collect()
}

fn dance(programs: &str, moves: &Vec<DanceMove>) -> String {
    let mut programs: VecDeque<char> = programs.chars().collect();
    for m in moves {
        match m {
            DanceMove::S(i) => {
                for _ in 0..*i {
                    let temp = programs.pop_back().unwrap();
                    programs.push_front(temp);
                }
            }
            DanceMove::X(from, to) => {
                programs.swap(*from, *to);
            }
            DanceMove::P(first, second) => {
                let pos_first = programs.iter().position(|c| c == first).unwrap();
                let pos_second = programs.iter().position(|c| c == second).unwrap();
                programs.swap(pos_first, pos_second);
            }
        }
    }

    programs.iter().join("")
}

fn dance_a_lot(programs: &str, moves: &Vec<DanceMove>) -> String {
    let mut seen: HashMap<String, usize> = HashMap::new();
    let mut i = 0;
    let mut ans = programs.to_string();
    while i < 1000000000 {
        ans = dance(ans.as_str(), &moves);
        i += 1;

        if let Some(dejavu) = seen.insert(ans.clone(), i) {
            let remaining = 1000000000 - i;
            let skip = remaining / (i - dejavu);
            i += skip * (i - dejavu);
            break;
        }
    }

    while i < 1000000000 {
        ans = dance(ans.as_str(), &moves);
        i += 1;
    }
    ans
}

#[cfg(test)]
mod day16_tests {
    use std::fs;

    use crate::y2017::day16::{dance, dance_a_lot, parse, DanceMove};

    #[test]
    fn test_works() {
        let input = fs::read_to_string("input/2017/day16/test.txt").unwrap();
        let moves: Vec<DanceMove> = parse(input.trim());
        assert_eq!("baedc", dance("abcde", &moves));
        assert_eq!("abcde", dance_a_lot("abcde", &moves));
    }

    #[test]
    fn input_works() {
        let input = fs::read_to_string("input/2017/day16/input.txt").unwrap();
        let moves: Vec<DanceMove> = parse(input.trim());
        assert_eq!("ebjpfdgmihonackl", dance("abcdefghijklmnop", &moves));
        assert_eq!("abocefghijklmndp", dance_a_lot("abcdefghijklmnop", &moves));
    }
}
