use std::fs;

pub(crate) fn day11() {
    println!("{}", solve(fs::read_to_string("input/2023/day11/input.txt").unwrap(), 1));
    println!("{}", solve(fs::read_to_string("input/2023/day11/input.txt").unwrap(), 1000000));
}

fn solve(input: String, mut m: usize) -> usize {
    if m > 1 {
        m = m - 1
    }
    let mut x_max = 0;
    let mut y_max = 0;
    let mut stars = vec![];
    input.lines().enumerate().for_each(|(x, line)| {
        line.chars().enumerate().for_each(|(y, c)| {
            if c == '#' {
                stars.push((x, y));
            }
            if y > y_max {
                y_max = y
            }
        });
        if x > x_max {
            x_max = x
        }
    });

    let mut empty_rows = vec![];
    let mut empty_cols = vec![];
    for x in 0..=x_max {
        if stars.iter().all(|(sx, _)| *sx != x) {
            empty_rows.push(x);
        }
    }
    for y in 0..=y_max {
        if stars.iter().all(|(_, sy)| *sy != y) {
            empty_cols.push(y);
        }
    }

    let mut shifted_stars = vec![];
    for (sx, sy) in &stars {
        shifted_stars.push((
            sx + (empty_rows.iter().filter(|ex| sx > *ex).count() * m),
            sy + (empty_cols.iter().filter(|ey| sy > ey).count() * m),
        ))
    }

    let mut ans = 0;
    for (i, (x1, y1)) in shifted_stars.iter().enumerate() {
        for (x2, y2) in shifted_stars[i + 1..].iter() {
            ans += (x1.max(x2) - x1.min(x2)) + (y1.max(y2) - y1.min(y2));
        }
    }

    ans
}

#[cfg(test)]
mod day11_tests {
    use std::fs;

    use crate::y2023::day11::solve;

    #[test]
    fn test_works() {
        assert_eq!(374, solve(fs::read_to_string("input/2023/day11/test.txt").unwrap(), 1));
        assert_eq!(1030, solve(fs::read_to_string("input/2023/day11/test.txt").unwrap(), 10));
        assert_eq!(8410, solve(fs::read_to_string("input/2023/day11/test.txt").unwrap(), 100));
    }

    #[test]
    fn input_works() {
        assert_eq!(9233514, solve(fs::read_to_string("input/2023/day11/input.txt").unwrap(), 1));
        assert_eq!(363293506944, solve(fs::read_to_string("input/2023/day11/input.txt").unwrap(), 1000000));
    }
}
