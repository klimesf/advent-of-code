use std::fs;

pub fn day13(print: fn(i64)) {
    print(part_a(fs::read_to_string("input/2024/day13/input.txt").unwrap()));
    print(part_b(fs::read_to_string("input/2024/day13/input.txt").unwrap()));
}

fn part_a(input: String) -> i64 {
    let configurations = parse_input(input);
    let mut ans = 0;
    for (xa, ya, xb, yb, xp, yp) in configurations {
        let a = (xp * yb - yp * xb) / (xa * yb - ya * xb);
        let b = (yp - a * ya) / yb;
        let x = xa * a + xb * b;
        let y = ya * a + yb * b;

        if a > 0 && a <= 100 && b > 0 && b <= 100 && x == xp && y == yp {
            ans += 3 * a + b;
        }
    }
    ans
}

fn part_b(input: String) -> i64 {
    let configurations = parse_input(input);
    let mut ans = 0;
    for (xa, ya, xb, yb, xp, yp) in configurations {
        let xp = xp + 10000000000000;
        let yp = yp + 10000000000000;

        let a = (xp * yb - yp * xb) / (xa * yb - ya * xb);
        let b = (yp - a * ya) / yb;
        let x = xa * a + xb * b;
        let y = ya * a + yb * b;

        if a >= 0 && b >= 0 && x == xp && y == yp {
            ans += 3 * a + b;
        }
    }
    ans
}

fn parse_input(input: String) -> Vec<(i64, i64, i64, i64, i64, i64)> {
    let mut configurations = vec![];
    input.split("\n\n").for_each(|block| {
        let lines = block.lines().collect::<Vec<&str>>();
        let (_, s_a) = lines[0].split_once(": ").unwrap();
        let (_, s_b) = lines[1].split_once(": ").unwrap();
        let (_, s_p) = lines[2].split_once(": ").unwrap();

        let (sxa, sya) = s_a.split_once(", ").unwrap();
        let (_, ssxa) = sxa.split_once("+").unwrap();
        let xa = ssxa.parse::<i64>().unwrap();
        let (_, ssya) = sya.split_once("+").unwrap();
        let ya = ssya.parse::<i64>().unwrap();

        let (sxb, syb) = s_b.split_once(", ").unwrap();
        let (_, ssxb) = sxb.split_once("+").unwrap();
        let xb = ssxb.parse::<i64>().unwrap();
        let (_, ssyb) = syb.split_once("+").unwrap();
        let yb = ssyb.parse::<i64>().unwrap();

        let (sxp, syp) = s_p.split_once(", ").unwrap();
        let (_, ssxp) = sxp.split_once("=").unwrap();
        let xp = ssxp.parse::<i64>().unwrap();
        let (_, ssyp) = syp.split_once("=").unwrap();
        let yp = ssyp.parse::<i64>().unwrap();

        configurations.push((xa, ya, xb, yb, xp, yp));
    });
    configurations
}

// --- Equations:
// a * xa + b * xb == xp
// a * ya + b * yb == yp
//
// b == (yp - a * ya) / yb
//
// a * xa + ((yp - a * ya) / yb) * xb == xp
// a * xa + (((yp - a * ya) * xb) / yb) == xp
// a * xa + ((yp * xb - a * ya * xb) / yb) == xp
// a * xa * yb + yp * xb - a * ya * xb == xp * yb
// a * xa * yb - a * ya * xb == xp * yb - yp * xb
// a * (xa * yb - ya * xb) == xp * yb - yp * xb
// a = (xp * yb - yp * xb) / (xa * yb - ya * xb)

// --- Python solution used for part 2 initially:
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
