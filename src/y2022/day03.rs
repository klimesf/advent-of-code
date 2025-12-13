use std::borrow::Borrow;
use std::collections::HashSet;
use std::fs;

pub(crate) fn day03() {
    part_a();
    part_b();
}

fn part_a() {
    let input = fs::read_to_string("input/2022/day03/input.txt").unwrap();
    let mut total = 0;
    for line in input.lines() {
        let c = find_item(line);
        total += score(c);
    }
    println!("{}", total);
}

fn part_b() {
    let input = fs::read_to_string("input/2022/day03/input.txt").unwrap();
    let mut total = 0;
    let mut group = vec![];
    for line in input.lines() {
        group.push(line);
        if group.len() == 3 {
            total += score(find_badge(group[0], group[1], group[2]));
            group.clear()
        }
    }
    println!("{}", total)
}

fn find_item(line: &str) -> char {
    let half = line.len() / 2;
    let mut chars = HashSet::new();
    for (i, c) in line.chars().enumerate() {
        // do something with character `c` and index `i`
        if i < half {
            chars.insert(c);
        } else if chars.contains(c.borrow()) {
            return c;
        }
    }
    panic!("No same items in {}", line);
}

fn find_badge(line1: &str, line2: &str, line3: &str) -> char {
    let mut chars1 = HashSet::new();
    for c in line1.chars() {
        chars1.insert(c);
    }
    let mut chars2 = HashSet::new();
    for c in line2.chars() {
        if chars1.contains(&c) {
            chars2.insert(c);
        }
    }
    for c in line3.chars() {
        if chars2.contains(&c) {
            return c;
        }
    }

    panic!("No common character")
}

fn score(c: char) -> u32 {
    let n = c as u32;
    if n > 96 {
        return n - 96;
    } else if n > 38 {
        return n - 38;
    } else {
        return n;
    }
}

#[cfg(test)]
mod day03_tests {
    use crate::y2022::day03::score;

    #[test]
    fn test_score() {
        assert_eq!(1, score('a'));
        assert_eq!(26, score('z'));
        assert_eq!(27, score('A'));
        assert_eq!(52, score('Z'));
    }
}
