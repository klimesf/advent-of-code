use std::fs;

pub(crate) fn day18() {
    println!(
        "{}",
        solve(
            fs::read_to_string("input/2016/day18/input.txt").unwrap(),
            40
        )
    );
    println!(
        "{}",
        solve(
            fs::read_to_string("input/2016/day18/input.txt").unwrap(),
            400000
        )
    );
}

fn solve(input: String, total_rows: usize) -> usize {
    let mut ans = 0;
    let mut prev_row: Vec<char> = input.chars().collect();
    ans += prev_row.iter().filter(|c| **c == '.').count();

    for _ in 1..total_rows {
        let mut new_row = vec![];
        for i in 0..prev_row.len() {
            let left = if i > 0 { prev_row[i - 1] } else { '.' };
            let center = prev_row[i];
            let right = if i < prev_row.len() - 1 {
                prev_row[i + 1]
            } else {
                '.'
            };

            let is_trap = (left == '^' && center == '^' && right == '.')
                || (left == '.' && center == '^' && right == '^')
                || (left == '^' && center == '.' && right == '.')
                || (left == '.' && center == '.' && right == '^');

            new_row.push(if is_trap { '^' } else { '.' });
        }

        ans += new_row.iter().filter(|c| **c == '.').count();
        prev_row = new_row
    }

    ans
}

#[cfg(test)]
mod day18_tests {
    use std::fs;

    use crate::y2016::day18::solve;

    #[test]
    fn test_works() {
        assert_eq!(
            38,
            solve(fs::read_to_string("input/2016/day18/test.txt").unwrap(), 10)
        );
    }

    #[test]
    fn input_works() {
        assert_eq!(
            2005,
            solve(
                fs::read_to_string("input/2016/day18/input.txt").unwrap(),
                40
            )
        );
        assert_eq!(
            20008491,
            solve(
                fs::read_to_string("input/2016/day18/input.txt").unwrap(),
                400000
            )
        );
    }
}
