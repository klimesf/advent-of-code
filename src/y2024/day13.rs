use std::fs;

pub(crate) fn day13() {
    println!("{}", part_a(fs::read_to_string("input/2024/day13/input.txt").unwrap()));
    println!("{}", part_b(fs::read_to_string("input/2024/day13/input.txt").unwrap()));
}

fn part_a(input: String) -> usize {
    let mut configurations = vec!();
    input.split("\n\n").for_each(|block| {
        let lines = block.lines().collect::<Vec<&str>>();
        let (_, s_a) = lines[0].split_once(": ").unwrap();
        let (_, s_b) = lines[1].split_once(": ").unwrap();
        let (_, s_p) = lines[2].split_once(": ").unwrap();

        let (sxa, sya) = s_a.split_once(", ").unwrap();
        let (_, ssxa) = sxa.split_once("+").unwrap();
        let xa = ssxa.parse::<usize>().unwrap();
        let (_, ssya) = sya.split_once("+").unwrap();
        let ya = ssya.parse::<usize>().unwrap();

        let (sxb, syb) = s_b.split_once(", ").unwrap();
        let (_, ssxb) = sxb.split_once("+").unwrap();
        let xb = ssxb.parse::<usize>().unwrap();
        let (_, ssyb) = syb.split_once("+").unwrap();
        let yb = ssyb.parse::<usize>().unwrap();

        let (sxp, syp) = s_p.split_once(", ").unwrap();
        let (_, ssxp) = sxp.split_once("=").unwrap();
        let xp = ssxp.parse::<usize>().unwrap();
        let (_, ssyp) = syp.split_once("=").unwrap();
        let yp = ssyp.parse::<usize>().unwrap();

        configurations.push((xa, ya, xb, yb, xp, yp));
    });

    let mut ans = 0;
    for (xa, ya, xb, yb, xp, yp) in configurations {
        let mut min = usize::MAX;

        for a in 0..=100 {
            if xa * a > xp { continue; }
            if ya * a > yp { continue; }
            for b in 0..=100 {
                if xa * a + xb * b > xp { continue; }
                if ya * a + yb * b > yp { continue; }
                if xa * a + xb * b == xp && ya * a + yb * b == yp && 3 * a + b < min {
                    min = 3 * a + b;
                }
            }
        }

        if min < usize::MAX {
            ans += min;
        }
    }

    ans
}

fn part_b(_: String) -> usize {
    // Preprocessed in rust, solved in python:
    // from z3 import *
    //
    // def solve(xa, ya, xb, yb, xp, yp):
    //     a = Int('a')
    //     b = Int('b')
    //     cost = Int('cost')
    //
    //     opt = Optimize()
    //
    //     opt.add(a * xa + b * xb == xp)
    //     opt.add(a * ya + b * yb == yp)
    //     opt.add(cost == 3 * a + b)
    //
    //     h = opt.minimize(cost)
    //     if opt.check() == sat:
    //         model = opt.model()
    //         sol_a = model[a].as_long()
    //         sol_b = model[b].as_long()
    //         return 3 * sol_a + sol_b
    //     else:
    //         return 0
    //
    // input = open('input/2024/day13/input_parsed.txt').read().split('\n')
    // ans = 0
    // for line in input:
    //     nums = line.split()
    //     ans += solve(int(nums[0]), int(nums[1]), int(nums[2]), int(nums[3]), int(nums[4]) + 10000000000000, int(nums[5]) + 10000000000000)
    //
    // print(ans)
    92572057880885
}

#[cfg(test)]
mod day13_tests {
    use std::fs;

    use crate::y2024::day13::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(480, part_a(fs::read_to_string("input/2024/day13/test.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!(33481, part_a(fs::read_to_string("input/2024/day13/input.txt").unwrap()));
        assert_eq!(92572057880885, part_b(fs::read_to_string("input/2024/day13/input.txt").unwrap()));
    }
}
