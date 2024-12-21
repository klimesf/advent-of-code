use std::collections::HashMap;
use std::fs;
use rayon::iter::ParallelBridge;
use rayon::iter::ParallelIterator;

pub(crate) fn day22() {
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
            new_num = new_num ^ num;
            new_num = new_num & 0b111111111111111111111111;

            let new_num2 = new_num >> 5;
            new_num = new_num2 ^ new_num;
            new_num = new_num & 0b111111111111111111111111;

            let new_num3 = new_num << 11;
            new_num = new_num3 ^ new_num;
            new_num = new_num & 0b111111111111111111111111;

            num = new_num;
        }
        ans += num;
    }
    ans
}

fn part_b(input: String) -> i64 {
    let secrets = input.lines().map(|l| l.parse().unwrap()).collect::<Vec<i64>>();

    let mut prices: Vec<Vec<i64>> = vec![vec![0; 2001]; secrets.len()];
    let mut map: HashMap<(usize, i64, i64, i64, i64), i64> = HashMap::new();

    for i in 0..secrets.len() {
        let mut num = secrets[i];
        for j in 0..=2000 {
            prices[i][j] = num % 10;
            if j >= 4 {
                let change_4 = prices[i][j - 4] - prices[i][j - 3];
                let change_3 = prices[i][j - 3] - prices[i][j - 2];
                let change_2 = prices[i][j - 2] - prices[i][j - 1];
                let change_1 = prices[i][j - 1] - prices[i][j];
                if !map.contains_key(&(i, change_4, change_3, change_2, change_1)) {
                    map.insert((i, change_4, change_3, change_2, change_1), prices[i][j]);
                }
            }

            let mut new_num = num << 6;
            new_num = new_num ^ num;
            new_num = new_num & 0b111111111111111111111111;

            let new_num2 = new_num >> 5;
            new_num = new_num2 ^ new_num;
            new_num = new_num & 0b111111111111111111111111;

            let new_num3 = new_num << 11;
            new_num = new_num3 ^ new_num;
            new_num = new_num & 0b111111111111111111111111;

            num = new_num;
        }
    }

    (-9..9).par_bridge().map(|a| {
        let mut ans = 0;
        (-9..=9).for_each(|b| {
            (-9..=9).for_each(|c| {
                (-9..=9).for_each(|d| {
                    let ans_sequence = (0..secrets.len()).map(|i| {
                        if !map.contains_key(&(i, d, c, b, a)) { return 0 }
                        map[&(i, d, c, b, a)]
                    }).sum::<i64>();
                    ans = ans.max(ans_sequence);
                });
            });
        });
        ans
    }).max().unwrap()
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
