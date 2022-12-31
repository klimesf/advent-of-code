use std::collections::HashMap;
use std::fs;

pub(crate) fn day07() {
    let input = fs::read_to_string("input/2022/day07/input.txt").unwrap();

    let mut current_path: Vec<&str> = vec!();
    let mut dir_sizes: HashMap<String, u64> = HashMap::new();

    for line in input.lines() {
        if line.starts_with("$ ls") {
            // do nothing
        } else if line.starts_with("$ cd") {
            let (_, name) = line.split_at(5);
            if name == ".." {
                let final_size = *dir_sizes.get(&*current_path.join("/")).unwrap_or(&0);
                current_path.pop();
                *dir_sizes.entry(current_path.join("/")).or_insert(0) += final_size;
            } else {
                current_path.push(name);
            }
        } else if line.starts_with("dir") {
            // do nothing
        } else { // handle file
            let (size, _) = line.split_once(' ').unwrap();
            *dir_sizes.entry(current_path.join("/")).or_insert(0) += size.parse::<u64>().unwrap();
        }
    }

    // traverse back to root
    while !current_path.is_empty() {
        let final_size = *dir_sizes.get(&*current_path.join("/")).unwrap_or(&0);
        current_path.pop();
        *dir_sizes.entry(current_path.join("/")).or_insert(0) += final_size;
    }

    let total: u64 = dir_sizes.values().into_iter()
        .filter(|size| **size < 100000)
        .sum();
    println!("{}", total);

    let unused = 70000000 - *dir_sizes.get(&*"/").unwrap_or(&0);
    let min = 30000000 - unused;
    let to_delete = dir_sizes.values().into_iter()
        .filter(|size| **size >= min)
        .min();
    println!("{}", to_delete.unwrap())
}
