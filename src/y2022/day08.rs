use std::collections::HashSet;
use std::fs;

pub(crate) fn day08() {
    let input = fs::read_to_string("input/2022/day08/input.txt").unwrap();
    let mut trees: Vec<Vec<i32>> = vec!();

    for line in input.lines() {
        let mut row = vec!();
        for c in line.chars() {
            row.push(c.to_digit(10).unwrap() as i32)
        }
        trees.push(row);
    }

    part_a(trees.clone());
    part_b(trees.clone());
}

fn part_a(trees: Vec<Vec<i32>>) {
    let mut visible: HashSet<(i32, i32)> = HashSet::new();
    // Rows from left
    for x in 0..trees.len() {
        let mut max = -1;
        for y in 0..trees.len() {
            if trees[x][y] > max {
                visible.insert((x as i32, y as i32));
                max = trees[x][y];
            }
        }
    }
    // Rows from right
    for x in 0..trees.len() {
        let mut max = -1;
        for y in (0..trees.len()).rev() {
            if trees[x][y] > max {
                visible.insert((x as i32, y as i32));
                max = trees[x][y];
            }
        }
    }
    // Cols from top
    for y in 0..trees.len() {
        let mut max = -1;
        for x in 0..trees.len() {
            if trees[x][y] > max {
                visible.insert((x as i32, y as i32));
                max = trees[x][y];
            }
        }
    }
    // Cols from bottom
    for y in 0..trees.len() {
        let mut max = -1;
        for x in (0..trees.len()).rev() {
            if trees[x][y] > max {
                visible.insert((x as i32, y as i32));
                max = trees[x][y];
            }
        }
    }
    println!("{}", visible.len());
}

fn part_b(trees: Vec<Vec<i32>>) {
    let mut max_viewing_score = 0;
    for x in 1..(trees.len() - 1) {
        for y in 1..(trees.len() - 1) {
            let max = trees[x][y];

            let mut bottom = 1;
            'inner1: for x1 in (x + 1)..(trees.len() - 1) {
                if trees[x1][y] >= max {
                    break 'inner1;
                }
                bottom += 1;
            }

            let mut up = 1;
            'inner2: for x2 in (1..=(x - 1)).rev() {
                if trees[x2][y] >= max {
                    break 'inner2;
                }
                up += 1;
            }

            let mut right = 1;
            'inner3: for y1 in (y + 1)..(trees.len() - 1) {
                if trees[x][y1] >= max {
                    break 'inner3;
                }
                right += 1;
            }

            let mut left = 1;
            'inner4: for y2 in (1..=(y - 1)).rev() {
                if trees[x][y2] >= max {
                    break 'inner4;
                }
                left += 1;
            }

            let viewing_score = bottom * up * right * left;
            if viewing_score > max_viewing_score {
                max_viewing_score = viewing_score;
            }
        }
    }
    println!("{}", max_viewing_score);
}
