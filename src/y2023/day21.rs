use std::collections::{HashSet};
use std::fs;

pub(crate) fn day21() {
    println!("{}", part_a(fs::read_to_string("input/2023/day21/input.txt").unwrap(), 64));
    println!("{}", part_b(fs::read_to_string("input/2023/day21/input.txt").unwrap(), 26501365));
}

fn part_a(input: String, max_steps: usize) -> usize {
    let matrix: Vec<Vec<char>> = input.lines().map(|line| {
        line.chars().collect()
    }).collect();

    let mut start: (usize, usize) = (0, 0);
    'outer: for r in 0..matrix.len() {
        for c in 0..matrix.len() {
            if matrix[r][c] == 'S' {
                start = (r, c);
                break 'outer;
            }
        }
    }

    let mut positions = HashSet::new();
    positions.insert(start);

    for _ in 0..max_steps {
        let mut new_positions = HashSet::new();
        for pos in &positions {
            if matrix[pos.0][pos.1] == '#' { continue; }
            if pos.0 > 0 { new_positions.insert((pos.0 - 1, pos.1)); }
            if pos.0 < matrix.len() - 1 { new_positions.insert((pos.0 + 1, pos.1)); }
            if pos.1 > 0 { new_positions.insert((pos.0, pos.1 - 1)); }
            if pos.1 < matrix[pos.0].len() - 1 { new_positions.insert((pos.0, pos.1 + 1)); }
        }
        positions = new_positions
    }

    positions.iter().filter(|(r, c)| matrix[*r][*c] != '#').count()
}

fn part_b(_input: String, max_steps: usize) -> usize {
    // let matrix: Vec<Vec<char>> = _input.lines().map(|line| {
    //     line.chars().collect()
    // }).collect();
    //
    // let mut start: (i32, i32) = (0, 0);
    // 'outer: for r in 0..matrix.len() {
    //     for c in 0..matrix.len() {
    //         if matrix[r][c] == 'S' {
    //             start = (r as i32, c as i32);
    //             break 'outer;
    //         }
    //     }
    // }
    //
    // let mut positions = HashSet::new();
    // positions.insert(start);
    // let mut nums = vec!();
    //
    // for i in 1..max_steps {
    //     let mut new_positions = HashSet::new();
    //     for pos in &positions {
    //         if matrix[wrap(pos.0 - 1, matrix.len())][wrap(pos.1, matrix[0].len())] != '#' { new_positions.insert((pos.0 - 1, pos.1)); }
    //         if matrix[wrap(pos.0 + 1, matrix.len())][wrap(pos.1, matrix[0].len())] != '#' { new_positions.insert((pos.0 + 1, pos.1)); }
    //         if matrix[wrap(pos.0, matrix.len())][wrap(pos.1 - 1, matrix[0].len())] != '#' { new_positions.insert((pos.0, pos.1 - 1)); }
    //         if matrix[wrap(pos.0, matrix.len())][wrap(pos.1 + 1, matrix[0].len())] != '#' { new_positions.insert((pos.0, pos.1 + 1)); }
    //     }
    //     positions = new_positions;
    //
    //     // 131 is the length of the matrix, 65 is the starting pos
    //     if i % 131 == 65 {
    //         nums.push(positions.len());
    //         if nums.len() > 3 { break }
    //     }
    // }


    // // This finds that the diff between diffs is 30188, so this is a quadratic polynomial
    // let mut nums: Vec<i32> = vec!(3835, 34125, 94603);
    // while nums.iter().any(|a| *a > 0) {
    //     let new_nums: Vec<i32> = nums.into_iter().tuple_windows().map(|(a, b)| b - a).collect();
    //     for num in &new_nums {
    //         print!("{} ", *num);
    //     }
    //     println!();
    //     nums = new_nums;
    // }

    let nums = vec!(3835, 34125, 94603);
    let n = max_steps / 131;

    let n0 = nums[0];
    let n1 = nums[1];
    let n2 = nums[2];

    let d0 = n0;
    let d1 = n1 - n0;
    let d2 = n2 - n1;

    d0 + (d1 * n) + ((n * (n - 1) / 2) * (d2 - d1))
}

#[allow(dead_code)]
fn wrap(coord: i32, max: usize) -> usize {
    coord.rem_euclid(max as i32) as usize
}

#[cfg(test)]
mod day21_tests {
    use std::fs;

    use crate::y2023::day21::{part_a, part_b, wrap};

    #[test]
    fn test_works() {
        assert_eq!(16, part_a(fs::read_to_string("input/2023/day21/test.txt").unwrap(), 6));
        assert_eq!(4, wrap(-1, 5));
        // assert_eq!(1594, part_b(fs::read_to_string("input/2023/day21/test.txt").unwrap(), 50));
        // assert_eq!(6536, part_b(fs::read_to_string("input/2023/day21/test.txt").unwrap(), 100));
    }

    #[test]
    fn input_works() {
        assert_eq!(3733, part_a(fs::read_to_string("input/2023/day21/input.txt").unwrap(), 64));
        assert_eq!(617729401414635, part_b(fs::read_to_string("input/2023/day21/input.txt").unwrap(), 26501365));
    }
}
