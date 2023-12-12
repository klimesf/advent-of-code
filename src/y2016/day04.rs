use std::collections::HashMap;
use std::fs;
use itertools::Itertools;
use regex::Regex;

pub(crate) fn day04() {
    println!("{}", part_a(fs::read_to_string("input/2016/day04/input.txt").unwrap()));
    println!("{}", part_b(fs::read_to_string("input/2016/day04/input.txt").unwrap()));
}

fn part_a(input: String) -> i32 {
    let re = Regex::new(r"^(.+)\-(\d+)\[(.+)\]$").unwrap();
    input.lines().map(|line| {
        let g = re.captures(line).unwrap();
        let name = g.get(1).unwrap().as_str();
        let id = g.get(2).unwrap().as_str().parse::<i32>().unwrap();
        let checksum = g.get(3).unwrap().as_str();

        let mut ctr = HashMap::new();
        for c in name.chars() {
            *ctr.entry(c).or_insert(0) += 1;
        }
        ctr.remove(&'-');

        let mut check = String::new();
        ctr.iter().sorted_by(|(ca, sa), (cb, sb)| {
            if sa == sb {
                ca.cmp(cb)
            } else {
                sb.cmp(sa)
            }
        }).take(5).for_each(|(c, _)| check.push(*c));

        if check == checksum { id } else { 0 }
    }).sum()
}

fn part_b(input: String) -> i32 {
    let re = Regex::new(r"^(.+)\-(\d+)\[(.+)\]$").unwrap();
    for line in input.lines() {
        let g = re.captures(line).unwrap();
        let name = g.get(1).unwrap().as_str();
        let id = g.get(2).unwrap().as_str().parse::<i32>().unwrap();
        let _ = g.get(3).unwrap().as_str();

        if decrypt(name, id) == "northpole object storage" {
            return id;
        }
    }
    panic!("North pole object storage not found")
}

fn decrypt(s: &str, i: i32) -> String {
    let mut ans = String::new();
    let a = 'a' as i32;
    let z = 'z' as i32;
    for c in s.chars() {
        if c == '-' { ans.push(' '); continue }
        let cd = (((c as i32 - a) + i) % (z - a + 1)) + a;
        ans.push(cd as u8 as char);
    }
    ans
}

#[cfg(test)]
mod day04_tests {
    use std::fs;

    use crate::y2016::day04::{part_a, part_b, decrypt};

    #[test]
    fn test_works() {
        assert_eq!(1514, part_a(fs::read_to_string("input/2016/day04/test.txt").unwrap()));
        assert_eq!("very encrypted name", decrypt("qzmt-zixmtkozy-ivhz", 343));
    }

    #[test]
    fn input_works() {
        assert_eq!(278221, part_a(fs::read_to_string("input/2016/day04/input.txt").unwrap()));
        assert_eq!(267, part_b(fs::read_to_string("input/2016/day04/input.txt").unwrap()));
    }
}
