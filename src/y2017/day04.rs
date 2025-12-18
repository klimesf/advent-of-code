use itertools::Itertools;
use std::fs;

pub(crate) fn day04() {
    let input = fs::read_to_string("input/2017/day04/input.txt").unwrap();
    println!("{}", input.lines().filter(|line| validate_no_duplicates(*line)).count());
    println!("{}", input.lines().filter(|line| validate_no_anagrams(*line)).count());
}

fn validate_no_duplicates(passphrase: &str) -> bool {
    let words: Vec<&str> = passphrase.split_whitespace().collect();
    let unique = words.iter().unique().count();
    words.len() == unique
}

fn validate_no_anagrams(passphrase: &str) -> bool {
    let words: Vec<String> = passphrase
        .split_whitespace()
        .map(|word| word.chars().sorted().collect())
        .collect();
    let unique = words.iter().unique().count();
    words.len() == unique
}

#[cfg(test)]
mod day04_tests {
    use crate::y2017::day04::{validate_no_anagrams, validate_no_duplicates};
    use std::fs;

    #[test]
    fn validate_no_duplicates_works() {
        assert_eq!(true, validate_no_duplicates("aa bb cc dd ee"));
        assert_eq!(false, validate_no_duplicates("aa bb cc dd aa"));
        assert_eq!(true, validate_no_duplicates("aa bb cc dd aaa"));
    }

    #[test]
    fn validate_no_anagrams_works() {
        assert_eq!(true, validate_no_anagrams("abcde fghij"));
        assert_eq!(false, validate_no_anagrams("abcde xyz ecdab"));
        assert_eq!(true, validate_no_anagrams("a ab abc abd abf abj"));
        assert_eq!(true, validate_no_anagrams("iiii oiii ooii oooi oooo"));
        assert_eq!(false, validate_no_anagrams("oiii ioii iioi iiio"));
    }

    #[test]
    fn input_works() {
        let input = fs::read_to_string("input/2017/day04/input.txt").unwrap();
        assert_eq!(325, input.lines().filter(|line| validate_no_duplicates(*line)).count());
        assert_eq!(119, input.lines().filter(|line| validate_no_anagrams(*line)).count());
    }
}
