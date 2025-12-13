use itertools::Itertools;
use std::fs;

pub(crate) fn day24() {
    println!(
        "{}",
        part_a(
            fs::read_to_string("input/2023/day24/input.txt").unwrap(),
            200000000000000.,
            400000000000000.
        )
    );
    println!(
        "{}",
        part_b(fs::read_to_string("input/2023/day24/input.txt").unwrap())
    );
}

fn part_a(input: String, search_min: f64, search_max: f64) -> usize {
    let hailstones: Vec<((f64, f64, f64), (f64, f64, f64))> = input
        .lines()
        .map(|line| {
            let (pos, vel) = line.split_once(" @ ").unwrap();
            let posa: Vec<f64> = pos
                .split(", ")
                .map(|s| s.trim().parse::<f64>().unwrap())
                .collect();
            let vela: Vec<f64> = vel
                .split(", ")
                .map(|s| s.trim().parse::<f64>().unwrap())
                .collect();
            ((posa[0], posa[1], posa[2]), (vela[0], vela[1], vela[2]))
        })
        .collect();

    let mut ans = 0;
    hailstones
        .iter()
        .tuple_combinations()
        .for_each(|(hailstone_a, hailstone_b)| {
            let (ax, ay, ac) = find_equation(*hailstone_a);
            let (bx, by, bc) = find_equation(*hailstone_b);

            if let Some(intersection) = find_intersection((ax, ay, ac), (bx, by, bc)) {
                if intersection.0 >= search_min
                    && intersection.0 <= search_max
                    && intersection.1 >= search_min
                    && intersection.1 <= search_max
                    && !met_in_past(*hailstone_a, *hailstone_b, intersection)
                {
                    ans += 1;
                }
            }
        });

    ans
}

fn find_equation(hailstone: ((f64, f64, f64), (f64, f64, f64))) -> (f64, f64, f64) {
    let ((x, y, _), (vx, vy, _)) = hailstone;
    let pta = (x, y);
    let ptb = (x + vx, y + vy);

    // y - y1 / x - x1 = y2 - y1 / x2 - x1
    let right = (ptb.1 - pta.1) / (ptb.0 - pta.0);
    // y - y1 = right * x - right * x1
    // 0 = right * x - y - (right * x1 + y1)

    (right, -1., right * (-pta.0) + pta.1)
}

fn find_intersection(ha: (f64, f64, f64), hb: (f64, f64, f64)) -> Option<(f64, f64)> {
    let (a1, b1, c1) = ha;
    let (a2, b2, c2) = hb;

    let tl = b1 * c2 - b2 * c1;
    let bl = a1 * b2 - a2 * b1;
    if bl == 0. {
        return None;
    }

    let tr = c1 * a2 - c2 * a1;
    let tb = a1 * b2 - a2 * b1;
    if tb == 0. {
        return None;
    }

    Some((tl / bl, tr / tb))
}

fn met_in_past(
    ha: ((f64, f64, f64), (f64, f64, f64)),
    hb: ((f64, f64, f64), (f64, f64, f64)),
    intersection: (f64, f64),
) -> bool {
    let ((ax, _, _), (avx, _, _)) = ha;
    let ((bx, _, _), (bvx, _, _)) = hb;
    let (x, _) = intersection;

    if (avx < 0.0 && x > ax) || (avx > 0.0 && x < ax) {
        return true;
    }
    if (bvx < 0.0 && x > bx) || (bvx > 0.0 && x < bx) {
        return true;
    }

    false
}

fn part_b(_: String) -> i32 {
    0
}

#[cfg(test)]
mod day24_tests {
    use std::fs;

    use crate::y2023::day24::{find_equation, part_a, part_b};

    #[test]
    fn find_equation_works() {
        assert_eq!(
            (1.5, -1., -3.5),
            find_equation(((-1., -5., 0.), (6., 9., 0.)))
        );
    }

    #[test]
    fn test_works() {
        assert_eq!(
            2,
            part_a(
                fs::read_to_string("input/2023/day24/test.txt").unwrap(),
                7.,
                27.
            )
        );
        assert_eq!(
            0,
            part_b(fs::read_to_string("input/2023/day24/test.txt").unwrap())
        );
    }

    #[test]
    fn input_works() {
        assert_eq!(
            19976,
            part_a(
                fs::read_to_string("input/2023/day24/input.txt").unwrap(),
                200000000000000.,
                400000000000000.
            )
        );
        assert_eq!(
            0,
            part_b(fs::read_to_string("input/2023/day24/input.txt").unwrap())
        );
    }
}
