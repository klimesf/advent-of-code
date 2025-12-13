use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use regex::{Match, Regex};
use std::fs;

pub(crate) fn day15() {
    println!("{}", part_a("input/2022/day15/input.txt", 2000000));
    println!("{}", part_b("input/2022/day15/input.txt", 4000000));
}

fn part_a(filename: &str, y: i64) -> usize {
    let signals = read_signals(filename);
    calculate_no_beacon_slow(signals.clone(), y)
}

fn part_b(filename: &str, max: i64) -> i64 {
    let signals = read_signals(filename);
    let (x, y) = find_uncovered_coords(signals.clone(), max);
    (x * 4000000) + y
}

fn calculate_no_beacon_slow(signals: Vec<(i64, i64, i64, i64)>, y: i64) -> usize {
    let sig: Vec<(i64, i64, i64)> = signals
        .iter()
        .map(|(xs, ys, xb, yb)| (*xs, *ys, manhattan_dist((*xs, *ys), (*xb, *yb))))
        .collect();
    return (-100000..=5000000)
        .into_par_iter()
        .filter(|x| {
            sig.iter()
                .any(|(xs_2, ys_2, dist_2)| manhattan_dist((*xs_2, *ys_2), (*x, y)) <= *dist_2)
        })
        .count()
        - 1;
}

fn find_uncovered_coords(signals: Vec<(i64, i64, i64, i64)>, max: i64) -> (i64, i64) {
    let sig: Vec<(i64, i64, i64)> = signals
        .iter()
        .map(|(xs, ys, xb, yb)| (*xs, *ys, manhattan_dist((*xs, *ys), (*xb, *yb))))
        .collect();
    for (xs, ys, dist) in sig.iter() {
        for dx in 0..=dist + 1 {
            let dy = dist + 1 - dx;
            for (signx, signy) in vec![(-1, -1), (-1, 1), (1, -1), (1, 1)] {
                let x = xs + (dx * signx);
                let y = ys + (dy * signy);
                if 0 > x || x > max || 0 > y || y > max {
                    continue;
                }

                if sig
                    .iter()
                    .all(|(xs_2, ys_2, dist_2)| manhattan_dist((*xs_2, *ys_2), (x, y)) > *dist_2)
                {
                    return (x, y);
                }
            }
        }
    }
    panic!()
}

fn manhattan_dist(from: (i64, i64), to: (i64, i64)) -> i64 {
    (from.0 - to.0).abs() + (from.1 - to.1).abs()
}

fn parse_i64(g: Option<Match>) -> i64 {
    return g.map_or(0, |m| m.as_str().parse().unwrap());
}

fn read_signals(filename: &str) -> Vec<(i64, i64, i64, i64)> {
    let input = fs::read_to_string(filename).unwrap();
    let re = Regex::new(
        r"^Sensor at x=([\-0-9]+), y=([\-0-9]+): closest beacon is at x=([\-0-9]+), y=([\-0-9]+)$",
    )
    .unwrap();
    let mut signals = vec![];
    for line in input.lines().into_iter() {
        let g = re.captures(line).unwrap();
        let signal = (
            parse_i64(g.get(1)),
            parse_i64(g.get(2)),
            parse_i64(g.get(3)),
            parse_i64(g.get(4)),
        );
        signals.push(signal);
    }
    signals
}

#[cfg(test)]
mod day15_tests {
    use crate::y2022::day15::part_a;

    #[test]
    fn part_a_works() {
        assert_eq!(26, part_a("input/2022/day15/test.txt", 10));
    }
}
