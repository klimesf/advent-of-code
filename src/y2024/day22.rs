use std::collections::{HashMap, HashSet};
use std::fs;
use std::hash::{Hash, Hasher};

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

fn part_b(input: String) -> i32 {
    let secrets = input.lines().map(|l| l.parse().unwrap()).collect::<Vec<i32>>();

    let mut map: HashMap<BigBrain, i32> = HashMap::new();

    for i in 0..secrets.len() {
        let mut prices = vec![0; 2001];
        let mut visited = HashSet::new();
        let mut num = secrets[i];
        for j in 0..=2000 {
            prices[j] = num % 10;
            if j >= 4 {
                let a = prices[j - 4] - prices[j - 3];
                let b = prices[j - 3] - prices[j - 2];
                let c = prices[j - 2] - prices[j - 1];
                let d = prices[j - 1] - prices[j];
                let key = BigBrain { a, b, c, d };
                if visited.insert(key) {
                    *map.entry(key).or_insert(0) += prices[j];
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
    }

    *map.values().max().unwrap()
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct BigBrain {
    a: i32,
    b: i32,
    c: i32,
    d: i32,
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
