use std::fs;

pub(crate) fn day08() {
    println!(
        "{}",
        solve(fs::read_to_string("input/2016/day08/input.txt").unwrap())
    );
}

fn solve(input: String) -> usize {
    let mut screen = vec![vec! {'.'; 50}; 6];
    input.lines().for_each(|line| {
        if line.starts_with("rect ") {
            let (l, r) = line[5..].split_once("x").unwrap();
            for c in 0..l.parse::<usize>().unwrap() {
                for r in 0..r.parse::<usize>().unwrap() {
                    screen[r][c] = '#'
                }
            }
        } else if line.starts_with("rotate row y=") {
            let (l, r) = line[13..].split_once(" by ").unwrap();
            let row = l.parse::<usize>().unwrap();
            let by = r.parse().unwrap();
            for _ in 0..by {
                let last = screen[row][screen[row].len() - 1];
                for c in (1..(screen[row].len() - 1)).rev() {
                    let left = screen[row][c - 1];
                    let middle = screen[row][c];
                    screen[row][c + 1] = middle;
                    screen[row][c] = left;
                }
                screen[row][0] = last;
            }
        } else if line.starts_with("rotate column x=") {
            let (l, r) = line[16..].split_once(" by ").unwrap();
            let col = l.parse::<usize>().unwrap();
            let by = r.parse().unwrap();
            for _ in 0..by {
                let last = screen[screen.len() - 1][col];
                for row in (1..(screen.len() - 1)).rev() {
                    let left = screen[row - 1][col];
                    let middle = screen[row][col];
                    screen[row + 1][col] = middle;
                    screen[row][col] = left;
                }
                screen[0][col] = last;
            }
        }
    });

    for r in 0..screen.len() {
        for c in 0..screen[r].len() {
            print!("{}", screen[r][c]);
        }
        println!();
    }

    screen
        .iter()
        .map(|row| (*row).iter().filter(|col| **col == '#').count())
        .sum()
}

#[cfg(test)]
mod day08_tests {
    use std::fs;

    use crate::y2016::day08::solve;

    #[test]
    fn input_works() {
        assert_eq!(
            116,
            solve(fs::read_to_string("input/2016/day08/input.txt").unwrap())
        );
    }
}
