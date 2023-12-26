use std::fs;
use regex::Regex;
use crate::utils::toolbox::{parse_i64};

pub(crate) fn day15() {
    println!("{}", part_a(fs::read_to_string("input/2015/day15/input.txt").unwrap()));
    println!("{}", part_b(fs::read_to_string("input/2015/day15/input.txt").unwrap()));
}

fn part_a(input: String) -> i64 {
    let mut ingredients = vec!();
    input.lines().for_each(|line| {
        let re = Regex::new("^.+: capacity (-?[0-9]+), durability (-?[0-9]+), flavor (-?[0-9]+), texture (-?[0-9]+), calories (-?[0-9]+)$").unwrap();
        let caps = re.captures(line).unwrap();

        let capacity = parse_i64(caps.get(1));
        let durability = parse_i64(caps.get(2));
        let flavor = parse_i64(caps.get(3));
        let texture = parse_i64(caps.get(4));
        let calories = parse_i64(caps.get(5));

        ingredients.push(vec! { capacity, durability, flavor, texture, calories });
    });

    let mut max = 0;
    for i in 0..=100 {
        for j in 0..=(100 - i) {
            for k in 0..=(100 - i - j) {
                for l in 0..=(100 - i - j - k) {
                    let mut score = 1;
                    for m in 0..4 { // skip calories now
                        let mut property = i * ingredients[0][m] + j * ingredients[1][m] + k * ingredients[2][m] + l * ingredients[3][m];
                        if property < 0 { property = 0 }
                        score *= property;
                    }
                    if score > max { max = score }
                }
            }
        }
    }
    max
}

fn part_b(input: String) -> i64 {
    let mut ingredients = vec!();
    input.lines().for_each(|line| {
        let re = Regex::new("^.+: capacity (-?[0-9]+), durability (-?[0-9]+), flavor (-?[0-9]+), texture (-?[0-9]+), calories (-?[0-9]+)$").unwrap();
        let caps = re.captures(line).unwrap();

        let capacity = parse_i64(caps.get(1));
        let durability = parse_i64(caps.get(2));
        let flavor = parse_i64(caps.get(3));
        let texture = parse_i64(caps.get(4));
        let calories = parse_i64(caps.get(5));

        ingredients.push(vec! { capacity, durability, flavor, texture, calories });
    });

    let mut max = 0;
    for i in 0..=100 {
        for j in 0..=(100 - i) {
            for k in 0..=(100 - i - j) {
                for l in 0..=(100 - i - j - k) {
                    let mut score = 1;
                    let calories = i * ingredients[0][4] + j * ingredients[1][4] + k * ingredients[2][4] + l * ingredients[3][4];
                    if calories != 500 { continue }

                    for m in 0..4 {
                        let mut property = i * ingredients[0][m] + j * ingredients[1][m] + k * ingredients[2][m] + l * ingredients[3][m];
                        if property < 0 { property = 0 }
                        score *= property;
                    }

                    if score > max { max = score }
                }
            }
        }
    }
    max
}

#[cfg(test)]
mod day15_tests {
    use std::fs;

    use crate::y2015::day15::{part_a, part_b};

    #[test]
    fn input_works() {
        assert_eq!(222870, part_a(fs::read_to_string("input/2015/day15/input.txt").unwrap()));
        assert_eq!(117936, part_b(fs::read_to_string("input/2015/day15/input.txt").unwrap()));
    }
}
