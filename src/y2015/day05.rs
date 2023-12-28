use std::fs;

pub(crate) fn day05() {
    println!("{}", part_a(fs::read_to_string("input/2015/day05/input.txt").unwrap()));
    println!("{}", part_b(fs::read_to_string("input/2015/day05/input.txt").unwrap()));
}

fn part_a(input: String) -> usize {
    input.lines().filter(|line| is_nice(line)).count()
}

fn part_b(input: String) -> usize {
    input.lines().filter(|line| is_nice_better(line)).count()
}

fn is_nice(str: &str) -> bool {
    contains_3_vowels(str) && contains_two_letters_in_a_row(str) && !contains_forbidden(str)
}

fn contains_3_vowels(str: &str) -> bool {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    str.chars().filter(|c| vowels.contains(c)).count() >= 3
}

fn contains_two_letters_in_a_row(str: &str) -> bool {
    let mut last = '_';
    for c in str.chars() {
        if c == last { return true; }
        last = c;
    }
    false
}

fn contains_forbidden(str: &str) -> bool {
    return str.contains("ab") || str.contains("cd") || str.contains("pq") || str.contains("xy");
}

fn is_nice_better(str: &str) -> bool {
    contains_two_pairs(str) && contins_two_letters_with_one_in_between(str)
}

fn contains_two_pairs(str: &str) -> bool {
    let chars: Vec<char> = str.chars().collect();
    for i in 1..chars.len() {
        let curr_pair = format!("{}{}", chars[i - 1], chars[i]);
        for j in i + 2..chars.len() {
            let cmp_pair = format!("{}{}", chars[j - 1], chars[j]);
            if curr_pair == cmp_pair { return true; }
        }
    }

    false
}

fn contins_two_letters_with_one_in_between(str: &str) -> bool {
    let mut prev_prev = '_';
    let mut prev = '_';
    for c in str.chars() {
        if c == prev_prev { return true; }
        prev_prev = prev;
        prev = c;
    }
    false
}

#[cfg(test)]
mod day05_tests {
    use std::fs;

    use crate::y2015::day05::{is_nice_better, part_a, part_b};

    #[test]
    fn is_nice_better_works() {
        assert_eq!(true, is_nice_better("qjhvhtzxzqqjkmpb"));
        assert_eq!(true, is_nice_better("xxyxx"));
        assert_eq!(false, is_nice_better("uurcxstgmygtbstg"));
        assert_eq!(false, is_nice_better("ieodomkazucvgmuy"));
    }

    #[test]
    fn input_works() {
        assert_eq!(255, part_a(fs::read_to_string("input/2015/day05/input.txt").unwrap()));
        assert_eq!(55, part_b(fs::read_to_string("input/2015/day05/input.txt").unwrap()));
    }
}
