use std::collections::HashMap;
use std::fs;

pub(crate) fn day16() {
    println!("{}", part_a(fs::read_to_string("input/2015/day16/input.txt").unwrap()));
    println!("{}", part_b(fs::read_to_string("input/2015/day16/input.txt").unwrap()));
}

fn part_a(input: String) -> usize {
    let mut sues = HashMap::new();
    input.lines().for_each(|line| {
        let (sue_s, rs) = line.split_once(": ").unwrap();
        let mut sue = HashMap::new();
        rs.split(", ").for_each(|data| {
            let (l, r) = data.split_once(": ").unwrap();
            sue.insert(l, r.parse::<usize>().unwrap());
        });
        sues.insert(sue_s[4..].parse::<usize>().unwrap(), sue);
    });

    for (n, sue) in sues {
        if sue.contains_key("children") && *sue.get("children").unwrap() != 3 {
            continue;
        }
        if sue.contains_key("cats") && *sue.get("cats").unwrap() != 7 {
            continue;
        }
        if sue.contains_key("samoyeds") && *sue.get("samoyeds").unwrap() != 2 {
            continue;
        }
        if sue.contains_key("pomeranians") && *sue.get("pomeranians").unwrap() != 3 {
            continue;
        }
        if sue.contains_key("akitas") && *sue.get("akitas").unwrap() != 0 {
            continue;
        }
        if sue.contains_key("vizslas") && *sue.get("vizslas").unwrap() != 0 {
            continue;
        }
        if sue.contains_key("goldfish") && *sue.get("goldfish").unwrap() != 5 {
            continue;
        }
        if sue.contains_key("trees") && *sue.get("trees").unwrap() != 3 {
            continue;
        }
        if sue.contains_key("cars") && *sue.get("cars").unwrap() != 2 {
            continue;
        }
        if sue.contains_key("perfumes") && *sue.get("perfumes").unwrap() != 1 {
            continue;
        }
        return n;
    }
    panic!("no aunt Sue matched")
}

fn part_b(input: String) -> usize {
    let mut sues = HashMap::new();
    input.lines().for_each(|line| {
        let (sue_s, rs) = line.split_once(": ").unwrap();
        let mut sue = HashMap::new();
        rs.split(", ").for_each(|data| {
            let (l, r) = data.split_once(": ").unwrap();
            sue.insert(l, r.parse::<usize>().unwrap());
        });
        sues.insert(sue_s[4..].parse::<usize>().unwrap(), sue);
    });

    for (n, sue) in sues {
        if sue.contains_key("children") && *sue.get("children").unwrap() != 3 {
            continue;
        }
        if sue.contains_key("cats") && *sue.get("cats").unwrap() <= 7 {
            continue;
        }
        if sue.contains_key("samoyeds") && *sue.get("samoyeds").unwrap() != 2 {
            continue;
        }
        if sue.contains_key("pomeranians") && *sue.get("pomeranians").unwrap() >= 3 {
            continue;
        }
        if sue.contains_key("akitas") && *sue.get("akitas").unwrap() != 0 {
            continue;
        }
        if sue.contains_key("vizslas") && *sue.get("vizslas").unwrap() != 0 {
            continue;
        }
        if sue.contains_key("goldfish") && *sue.get("goldfish").unwrap() >= 5 {
            continue;
        }
        if sue.contains_key("trees") && *sue.get("trees").unwrap() <= 3 {
            continue;
        }
        if sue.contains_key("cars") && *sue.get("cars").unwrap() != 2 {
            continue;
        }
        if sue.contains_key("perfumes") && *sue.get("perfumes").unwrap() != 1 {
            continue;
        }
        return n;
    }
    panic!("no aunt Sue matched")
}

#[cfg(test)]
mod day16_tests {
    use std::fs;

    use crate::y2015::day16::{part_a, part_b};

    #[test]
    fn input_works() {
        assert_eq!(373, part_a(fs::read_to_string("input/2015/day16/input.txt").unwrap()));
        assert_eq!(260, part_b(fs::read_to_string("input/2015/day16/input.txt").unwrap()));
    }
}
