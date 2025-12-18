use crate::utils::toolbox::parse_usize;
use regex::Regex;
use std::collections::HashMap;
use std::fs;

pub(crate) fn day14() {
    println!("{}", part_a(fs::read_to_string("input/2020/day14/input.txt").unwrap()));
    println!("{}", part_b(fs::read_to_string("input/2020/day14/input.txt").unwrap()));
}

fn part_a(input: String) -> usize {
    let mut mem = HashMap::new();
    let mut mask: Vec<char> = vec![];
    let re = Regex::new("mem\\[(\\d+)\\] = (\\d+)").unwrap();

    for line in input.lines() {
        if line.starts_with("mask = ") {
            mask = line["mask = ".len()..].trim().chars().collect();
            continue;
        }

        let caps = re.captures(&line).unwrap();
        let addr = parse_usize(caps.get(1));
        let value = parse_usize(caps.get(2));

        let mut value_binary = format!("{:036b}", value).chars().collect::<Vec<char>>();
        for i in 0..36 {
            match mask[i] {
                'X' => {}
                '1' => value_binary[i] = '1',
                '0' => value_binary[i] = '0',
                _ => panic!("unknown mask char: {}", mask[i]),
            }
        }

        let masked_value: usize = value_binary
            .iter()
            .rev()
            .enumerate()
            .map(|(place, bit)| match bit {
                '0' => 0,
                '1' => 2_usize.pow(place as u32),
                _ => panic!("unknown bit: {}", bit),
            })
            .sum();
        mem.insert(addr, masked_value);
    }
    mem.values().sum::<usize>()
}

fn part_b(input: String) -> usize {
    let mut mem = HashMap::new();
    let mut mask: Vec<char> = vec![];
    let re = Regex::new("mem\\[(\\d+)\\] = (\\d+)").unwrap();

    for line in input.lines() {
        if line.starts_with("mask = ") {
            mask = line["mask = ".len()..].trim().chars().collect();
            continue;
        }

        let caps = re.captures(&line).unwrap();
        let addr = parse_usize(caps.get(1));
        let value = parse_usize(caps.get(2));

        let mut addr_binary = format!("{:036b}", addr).chars().collect::<Vec<char>>();
        for i in 0..36 {
            match mask[i] {
                'X' => addr_binary[i] = 'X',
                '1' => addr_binary[i] = '1',
                '0' => {}
                _ => panic!("unknown mask char: {}", mask[i]),
            }
        }

        let mut possible_addrs = vec![addr_binary];
        for i in 0..36 {
            let mut new_possible_addrs = vec![];
            for possible_addr in possible_addrs {
                if possible_addr[i] == 'X' {
                    let mut a = possible_addr.clone();
                    a[i] = '1';
                    new_possible_addrs.push(a);
                    let mut b = possible_addr.clone();
                    b[i] = '0';
                    new_possible_addrs.push(b);
                } else {
                    new_possible_addrs.push(possible_addr.clone());
                }
            }
            possible_addrs = new_possible_addrs;
        }

        for possible_addr in possible_addrs {
            mem.insert(possible_addr, value);
        }
    }
    mem.values().sum::<usize>()
}

#[cfg(test)]
mod day14_tests {
    use std::fs;

    use crate::y2020::day14::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(165, part_a(fs::read_to_string("input/2020/day14/test_a.txt").unwrap()));
        assert_eq!(208, part_b(fs::read_to_string("input/2020/day14/test_b.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!(9879607673316, part_a(fs::read_to_string("input/2020/day14/input.txt").unwrap()));
        assert_eq!(3435342392262, part_b(fs::read_to_string("input/2020/day14/input.txt").unwrap()));
    }
}
