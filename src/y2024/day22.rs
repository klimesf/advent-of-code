use std::fs;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

pub fn day22() {
    println!("{}", part_a(fs::read_to_string("input/2024/day22/input.txt").unwrap()));
    println!("{}", part_b(fs::read_to_string("input/2024/day22/input.txt").unwrap()));
}

fn part_a(input: String) -> usize {
    let secrets = input.lines().map(|l| l.parse().unwrap()).collect::<Vec<usize>>();

    let mut ans = 0;
    for secret in &secrets {
        let mut num = secret.clone();
        for _ in 0..2000 {
            let mut new_num = num << 6;
            num = new_num ^ num;
            num = num & 0b111111111111111111111111;

            new_num = num >> 5;
            num = new_num ^ num;
            num = num & 0b111111111111111111111111;

            new_num = num << 11;
            num = new_num ^ num;
            num = num & 0b111111111111111111111111;
        }
        ans += num;
    }
    ans
}

fn part_b(input: String) -> i16 {
    let secrets = input.lines().map(|l| l.parse().unwrap()).collect::<Vec<i32>>();

    let vals: Vec<(usize, i16)> = secrets.par_iter().map(|s| {
        let mut res = vec!();
        let mut num = *s;
        let mut prices = vec![0; 2001];
        let mut visited = [false; 0b11111111111111111111];

        for j in 0..=2000 {
            prices[j] = (num % 10) as i16;
            if j >= 4 {
                let a = prices[j - 4] - prices[j - 3];
                let b = prices[j - 3] - prices[j - 2];
                let c = prices[j - 2] - prices[j - 1];
                let d = prices[j - 1] - prices[j];
                let key = hash(a, b, c, d);
                if !visited[key] {
                    res.push((key, prices[j]));
                    visited[key] = true;
                }
            }

            let mut new_num = num << 6;
            num = new_num ^ num;
            num = num & 0b111111111111111111111111;

            new_num = num >> 5;
            num = new_num ^ num;
            num = num & 0b111111111111111111111111;

            new_num = num << 11;
            num = new_num ^ num;
            num = num & 0b111111111111111111111111;
        }
        res
    }).flatten().collect();

    let mut map = [0; 0b11111111111111111111];
    vals.iter().map(|(key, value)| {
        map[*key] += value;
        map[*key]
    }).max().unwrap()
}

fn hash(a: i16, b: i16, c: i16, d: i16) -> usize {
    let mut ans = 0;
    ans += (a + 10) as usize; // -9..9 => 1..19
    ans = ans << 5;           // max 19 = 0b10011, i.e. shift left by 5
    ans += (b + 10) as usize;
    ans += ans << 5;
    ans += (c + 10) as usize;
    ans += ans << 5;
    ans += (d + 10) as usize;
    ans // total 20 bits
}

#[cfg(test)]
mod day22_tests {
    use std::fs;

    use crate::y2024::day22::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(37327623, part_a(fs::read_to_string("input/2024/day22/test.txt").unwrap()));
        assert_eq!(23, part_b(fs::read_to_string("input/2024/day22/test_2.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!(17612566393, part_a(fs::read_to_string("input/2024/day22/input.txt").unwrap()));
        assert_eq!(1968, part_b(fs::read_to_string("input/2024/day22/input.txt").unwrap()));
    }
}
