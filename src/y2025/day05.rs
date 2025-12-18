use std::fs;

pub fn day05(print: fn(usize)) {
    print(part_a(fs::read_to_string("input/2025/day05/input.txt").unwrap()));
    print(part_b(fs::read_to_string("input/2025/day05/input.txt").unwrap()));
}

fn part_a(input: String) -> usize {
    let (i1, i2) = input.split_once("\n\n").unwrap();

    let intervals: Vec<(usize, usize)> = i1
        .lines()
        .map(|l| {
            let (l, r) = l.split_once('-').unwrap();
            let start = l.parse::<usize>().unwrap();
            let end = r.parse::<usize>().unwrap();

            if start > end {
                panic!("invalid input");
            }
            (start, end)
        })
        .collect();

    let mut ans = 0;
    i2.lines().for_each(|l| {
        let num = l.parse::<usize>().unwrap();

        for i in 0..intervals.len() {
            if num >= intervals[i].0 && num <= intervals[i].1 {
                ans += 1;
                break;
            }
        }
    });

    ans
}

fn part_b(input: String) -> usize {
    let (i1, _) = input.split_once("\n\n").unwrap();

    let mut intervals: Vec<(usize, usize)> = i1
        .lines()
        .map(|l| {
            let (l, r) = l.split_once('-').unwrap();
            let start = l.parse::<usize>().unwrap();
            let end = r.parse::<usize>().unwrap();

            if start > end {
                panic!("invalid input");
            }
            (start, end)
        })
        .collect();

    intervals.sort_by(|a, b| {
        if a.0 == b.0 {
            return a.1.cmp(&b.1);
        }
        a.0.cmp(&b.0)
    });

    let mut res = vec![];
    let mut skip: Vec<usize> = vec![];
    for i in 0..intervals.len() {
        if skip.contains(&i) {
            continue;
        }
        let start1 = intervals[i].0;
        let mut end1 = intervals[i].1;
        for j in i + 1..intervals.len() {
            let (start2, end2) = intervals[j];
            if end1 >= start2 {
                end1 = end1.max(end2);
                skip.push(j);
            } else {
                break;
            }
        }
        res.push((start1, end1));
    }

    let mut ans = 0;
    for i in 0..res.len() {
        ans += res[i].1 - res[i].0 + 1;
    }
    ans
}

#[cfg(test)]
mod day05_tests {
    use std::fs;

    use crate::y2025::day05::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(3, part_a(fs::read_to_string("input/2025/day05/test.txt").unwrap()));
        assert_eq!(14, part_b(fs::read_to_string("input/2025/day05/test.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!(811, part_a(fs::read_to_string("input/2025/day05/input.txt").unwrap()));
        assert_eq!(338189277144473, part_b(fs::read_to_string("input/2025/day05/input.txt").unwrap()));
    }
}
