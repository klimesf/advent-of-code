use std::collections::HashSet;
use std::fs;

pub(crate) fn day01() {
    let input = fs::read_to_string("input/2018/day01/input.txt").unwrap();
    let changes: Vec<i32> = input.trim().split("\n").map(|s| s.parse::<i32>().unwrap()).collect();
    println!("Resulting frequency is: {}", changes.iter().sum::<i32>());

    let mut visited: HashSet<i32> = HashSet::new();
    let mut frequency = 0;
    let mut i = 0;
    loop {
        if !visited.insert(frequency) {
            break;
        }
        frequency += changes[i % changes.len()];
        i += 1;
    }
    println!("First frequency reached twice is: {}", frequency);
}
