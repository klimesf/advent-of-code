use std::collections::{VecDeque};
use std::fs;

pub(crate) fn day05() {
    let input = fs::read_to_string("input/2018/day05/input.txt").unwrap();

    let len_after_reacting = react_polymer(input.clone());
    println!("{} units remain after fully reacting the polymer", len_after_reacting);

    let shortest = ('a'..'z').map(|c| {
        let mut omitted = input.clone();
        omitted = omitted.replace(c, "");
        omitted = omitted.replace((c as u8 - 32) as char, "");
        react_polymer(omitted)
    }).min().unwrap();
    println!("{} is the shortest polymer we can produce by omitting units", shortest);
}

fn react_polymer(input: String) -> usize {
    let mut polymer: VecDeque<char> = VecDeque::with_capacity(input.len());
    input.trim().chars().for_each(|c| polymer.push_back(c));

    let mut new_polymer: VecDeque<char> = VecDeque::with_capacity(input.len());
    new_polymer.push_back(polymer.pop_front().unwrap());

    while !polymer.is_empty() {
        let left = new_polymer.pop_back().unwrap();
        let right = polymer.pop_front().unwrap();

        if is_opposite_polarity(left, right) {
            if polymer.is_empty() { break; }
            if new_polymer.is_empty() {
                new_polymer.push_back(polymer.pop_front().unwrap());
            }
        } else {
            new_polymer.push_back(left);
            new_polymer.push_back(right);
        }
    }

    new_polymer.len()
}

fn is_opposite_polarity(left: char, right: char) -> bool {
    let uppercase = left.min(right);
    let lowercase = left.max(right);

    uppercase as u8 == lowercase as u8 - 32
}

#[cfg(test)]
mod day05_tests {
    use crate::y2018::day05::is_opposite_polarity;

    #[test]
    fn is_opposite_polarity_works() {
        assert_eq!(true, is_opposite_polarity('c', 'C'));
        assert_eq!(true, is_opposite_polarity('C', 'c'));
        assert_eq!(false, is_opposite_polarity('C', 'C'));
        assert_eq!(false, is_opposite_polarity('c', 'c'));
        assert_eq!(false, is_opposite_polarity('a', 'b'));
        assert_eq!(false, is_opposite_polarity('a', 'B'));
        assert_eq!(false, is_opposite_polarity('A', 'b'));
    }
}
