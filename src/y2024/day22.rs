use std::collections::HashMap;
use std::fs;
use std::hash::{Hash, Hasher};
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

fn part_b(input: String) -> i32 {
    let secrets = input.lines().map(|l| l.parse().unwrap()).collect::<Vec<i32>>();

    let mut prices: Vec<Vec<i32>> = vec![vec![0; 2001]; secrets.len()];
    let mut map: HashMap<BigBrain, i32> = HashMap::new();

    for i in 0..secrets.len() {
        let mut num = secrets[i];
        for j in 0..=2000 {
            prices[i][j] = num % 10;
            if j >= 4 {
                let change_4 = prices[i][j - 4] - prices[i][j - 3];
                let change_3 = prices[i][j - 3] - prices[i][j - 2];
                let change_2 = prices[i][j - 2] - prices[i][j - 1];
                let change_1 = prices[i][j - 1] - prices[i][j];
                let key = BigBrain { i, a: change_4, b: change_3, c: change_2, d: change_1 };
                if !map.contains_key(&key) {
                    map.insert(key, prices[i][j]);
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
                        let key = BigBrain { i, a, b, c, d };
                        if !map.contains_key(&key) { return 0 }
                        map[&key]
                    }).sum::<i32>();
                    ans = ans.max(ans_sequence);
                });
            });
        });
        ans
    }).max().unwrap()
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct BigBrain {
    a: i32,
    b: i32,
    c: i32,
    d: i32,
    i: usize,
}

impl Hash for BigBrain {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut ans = 0;
        ans += self.a + 10; // -9..9 => 1..19
        ans = ans << 5;
        ans += self.b + 10;
        ans += ans << 5;
        ans += self.c + 10;
        ans += ans << 5;
        ans += self.d + 10;
        ans += ans << 7;
        ans += self.i as i32;
        state.write_i32(ans);
    }
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
