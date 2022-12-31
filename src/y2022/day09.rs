use std::collections::HashSet;
use std::fs;

pub(crate) fn day09() {
    let input = fs::read_to_string("input/2022/day09/input.txt").unwrap();
    let mut rope: Vec<(i32, i32)> = vec!((0, 0); 10);
    let mut visited_a: HashSet<(i32, i32)> = HashSet::new();
    let mut visited_b: HashSet<(i32, i32)> = HashSet::new();
    visited_a.insert(rope[1]);
    visited_b.insert(rope[9]);

    for line in input.lines() {
        let parts = line.split_once(' ').unwrap();
        let dir = parts.0;
        let amount: i32 = parts.1.parse().unwrap();

        for _ in 0..amount {
            match dir {
                "R" => { rope[0].0 += 1; }
                "L" => { rope[0].0 -= 1; }
                "U" => { rope[0].1 += 1; }
                "D" => { rope[0].1 -= 1; }
                _ => panic!("Unknown dir {}", dir)
            }

            for j in 1..10 {
                rope[j] = update_tail(rope[j - 1], rope[j]);
            }

            visited_a.insert(rope[1]);
            visited_b.insert(rope[9]);
        }
    }

    println!("{}", visited_a.len());
    println!("{}", visited_b.len());
}

fn update_tail(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    let dir = (head.0 - tail.0, head.1 - tail.1);
    return if dir.0.abs() > 1 || dir.1.abs() > 1 {
        (tail.0 + to_one(dir.0), tail.1 + to_one(dir.1))
    } else {
        tail
    };
}

fn to_one(i: i32) -> i32 {
    return if i == 0 { 0 } else { i / i.abs() };
}
