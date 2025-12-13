use std::collections::HashMap;
use std::fs;

pub(crate) fn day15() {
    let input = fs::read_to_string("input/2017/day15/input.txt").unwrap();
    let (generator_a, generator_b) = parse(input.as_str());
    println!("{}", part_a(generator_a, generator_b));
    println!("{}", part_b(generator_a, generator_b));
}

fn parse(input: &str) -> (u64, u64) {
    let generators: HashMap<&str, u64> = input
        .lines()
        .into_iter()
        .map(|line| line.split_once(" starts with ").unwrap())
        .map(|(n, s)| (n, s.parse::<u64>().unwrap()))
        .collect();
    (generators["Generator A"], generators["Generator B"])
}

fn part_a(mut gen_a: u64, mut gen_b: u64) -> u64 {
    let mask = 0xffff_u64;
    (0..40000000)
        .map(|_| {
            gen_a = gen(gen_a, 16807);
            gen_b = gen(gen_b, 48271);
            if gen_a & mask == gen_b & mask {
                1
            } else {
                0
            }
        })
        .sum()
}

fn part_b(mut gen_a: u64, mut gen_b: u64) -> u64 {
    let mask = 0xffff_u64;
    (0..5000000)
        .map(|_| {
            gen_a = gen(gen_a, 16807);
            while gen_a & 0b11 != 0 {
                gen_a = gen(gen_a, 16807);
            }
            gen_b = gen(gen_b, 48271);
            while gen_b & 0b111 != 0 {
                gen_b = gen(gen_b, 48271);
            }
            if gen_a & mask == gen_b & mask {
                1
            } else {
                0
            }
        })
        .sum()
}

fn gen(num: u64, factor: u64) -> u64 {
    let mut prod = num * factor;
    prod = (prod & 2147483647) + (prod >> 31);
    if prod >> 31 != 0 {
        prod - 2147483647
    } else {
        prod
    }
}

#[cfg(test)]
mod day15_tests {
    use crate::y2017::day15::{parse, part_a, part_b};
    use std::fs;

    #[test]
    fn test_works() {
        let input = fs::read_to_string("input/2017/day15/test.txt").unwrap();
        let (generator_a, generator_b) = parse(input.as_str());
        assert_eq!(588, part_a(generator_a, generator_b));
        assert_eq!(309, part_b(generator_a, generator_b));
    }

    #[test]
    fn input_works() {
        let input = fs::read_to_string("input/2017/day15/input.txt").unwrap();
        let (generator_a, generator_b) = parse(input.as_str());
        assert_eq!(577, part_a(generator_a, generator_b));
        assert_eq!(316, part_b(generator_a, generator_b));
    }
}
