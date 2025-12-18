use std::collections::{HashMap, HashSet};
use std::fs;

pub(crate) fn day04() {
    println!("{}", part_a(fs::read_to_string("input/2020/day04/input.txt").unwrap()));
    println!("{}", part_b(fs::read_to_string("input/2020/day04/input.txt").unwrap()));
}

fn part_a(input: String) -> usize {
    input
        .split("\n\n")
        .filter(|p| {
            let fields = p.split_whitespace().collect::<Vec<&str>>();
            let keys = fields
                .iter()
                .map(|value| {
                    let (key, _) = value.split_once(":").unwrap();
                    key
                })
                .collect::<HashSet<&str>>();

            ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
                .iter()
                .all(|k| keys.contains(k))
        })
        .count()
}

fn part_b(input: String) -> usize {
    input
        .split("\n\n")
        .filter(|p| {
            let fields = p.split_whitespace().collect::<Vec<&str>>();
            let passport = fields
                .iter()
                .map(|value| value.split_once(":").unwrap())
                .collect::<HashMap<&str, &str>>();

            if ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
                .iter()
                .any(|k| !passport.contains_key(k))
            {
                return false;
            }

            let byr = passport["byr"].parse::<usize>().unwrap();
            if byr < 1920 || byr > 2002 {
                return false;
            }

            let iyr = passport["iyr"].parse::<usize>().unwrap();
            if iyr < 2010 || iyr > 2020 {
                return false;
            }

            let eyr = passport["eyr"].parse::<usize>().unwrap();
            if eyr < 2020 || eyr > 2030 {
                return false;
            }

            let hgt = passport["hgt"];
            if hgt[hgt.len() - 2..hgt.len()].to_string() == "cm" {
                let hgt_val = hgt[..hgt.len() - 2].parse::<usize>().unwrap();
                if hgt_val < 150 || hgt_val > 193 {
                    return false;
                }
            } else if hgt[hgt.len() - 2..hgt.len()].to_string() == "in" {
                let hgt_val = hgt[..hgt.len() - 2].parse::<usize>().unwrap();
                if hgt_val < 59 || hgt_val > 76 {
                    return false;
                }
            } else {
                return false;
            }

            let hcl = passport["hcl"];
            if !hcl[0..1].chars().all(|c| c == '#') || !hcl[1..].chars().all(|c| c.is_ascii_hexdigit()) {
                return false;
            }

            let ecl = passport["ecl"];
            if !["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&ecl) {
                return false;
            }

            let pid = passport["pid"];
            if pid.len() != 9 || !pid.chars().all(|c| c.is_ascii_digit()) {
                return false;
            }

            true
        })
        .count()
}

#[cfg(test)]
mod day04_tests {
    use std::fs;

    use crate::y2020::day04::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(2, part_a(fs::read_to_string("input/2020/day04/test.txt").unwrap()));
        assert_eq!(2, part_b(fs::read_to_string("input/2020/day04/test.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!(254, part_a(fs::read_to_string("input/2020/day04/input.txt").unwrap()));
        assert_eq!(184, part_b(fs::read_to_string("input/2020/day04/input.txt").unwrap()));
    }
}
