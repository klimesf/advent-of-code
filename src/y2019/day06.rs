use std::collections::{HashMap, VecDeque};
use std::fs;

pub(crate) fn day06() {
    let input = fs::read_to_string("input/2019/day06/input.txt").unwrap();
    let paths: HashMap<_, _> = input
        .trim()
        .split('\n')
        .map(|s| s.split_once(')').unwrap())
        .map(|(to, from)| (from, to))
        .collect();

    part_a(&paths);
    part_b(&paths);
}

fn part_a(paths: &HashMap<&str, &str>) {
    // Run "DFS" from each node and count total orbits
    // The DFS can be simplified, since a planet can orbit only one other planet
    let mut total = 0;
    for planet in paths.keys() {
        let mut from = planet;
        loop {
            match paths.get(from) {
                Some(to) => {
                    from = to;
                    total += 1;
                }
                None => {
                    break;
                }
            }
        }
    }
    println!("{}", total);
}

fn part_b(paths: &HashMap<&str, &str>) {
    // Run "DFS" from YOU and from SAN, find the point where they cross and then calculate the orbital transfers required
    let mut path_from_you: VecDeque<&str> = VecDeque::new();
    let mut from = "YOU";
    loop {
        match paths.get(from) {
            Some(to) => {
                path_from_you.push_back(to);
                from = to;
            }
            None => {
                break;
            }
        }
    }

    let mut path_from_san: VecDeque<&str> = VecDeque::new();
    from = "SAN";
    loop {
        match paths.get(from) {
            Some(to) => {
                path_from_san.push_back(to);
                from = to;
            }
            None => {
                break;
            }
        }
    }

    while !path_from_you.is_empty() && !path_from_san.is_empty() {
        let py = path_from_you.pop_back().unwrap();
        let ps = path_from_san.pop_back().unwrap();
        if py != ps {
            path_from_you.push_back(py);
            path_from_san.push_back(ps);
            break;
        }
    }

    println!("{}", path_from_you.len() + path_from_san.len());
}
