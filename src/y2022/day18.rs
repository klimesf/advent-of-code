use std::collections::{HashSet, VecDeque};
use std::fs;

pub(crate) fn day18() {
    let input = fs::read_to_string("input/2022/day18/input.txt").unwrap();
    let droplets: HashSet<(i32, i32, i32)> = HashSet::from_iter(input.lines().into_iter().map(|line| {
        let coords: Vec<&str> = line.split(",").collect();
        let x = coords[0].parse::<i32>().unwrap();
        let y = coords[1].parse::<i32>().unwrap();
        let z = coords[2].parse::<i32>().unwrap();
        (x, y, z)
    }));

    let dirs = vec![
        (-1, 0, 0), // Left
        (1, 0, 0),  // Right
        (0, -1, 0), // Down
        (0, 1, 0),  // Up
        (0, 0, -1), // Back
        (0, 0, 1),  // Forth
    ];

    let ans_a: usize = droplets
        .iter()
        .map(|(x, y, z)| {
            dirs.iter()
                .filter(|(dx, dy, dz)| !droplets.contains(&(x + dx, y + dy, z + dz)))
                .count()
        })
        .sum();
    println!("{}", ans_a);

    let max_x = *droplets.iter().map(|(x, _, _)| x).max().unwrap();
    let max_y = *droplets.iter().map(|(_, y, _)| y).max().unwrap();
    let max_z = *droplets.iter().map(|(_, _, z)| z).max().unwrap();
    let min_x = *droplets.iter().map(|(x, _, _)| x).min().unwrap();
    let min_y = *droplets.iter().map(|(_, y, _)| y).min().unwrap();
    let min_z = *droplets.iter().map(|(_, _, z)| z).min().unwrap();

    let mut deque = VecDeque::new();
    let mut reachable = HashSet::new();
    deque.push_back((min_x - 1, min_y - 1, min_z - 1));
    while !deque.is_empty() {
        let (x, y, z) = deque.pop_front().unwrap();
        if reachable.contains(&(x, y, z))
            || droplets.contains(&(x, y, z))
            || x < min_x - 1
            || x > max_x + 1
            || y < min_y - 1
            || y > max_y + 1
            || z < min_z - 1
            || z > max_z + 1
        {
            continue;
        }
        reachable.insert((x, y, z));
        dirs.iter().for_each(|(dx, dy, dz)| {
            deque.push_back((x + dx, y + dy, z + dz));
        });
    }

    let ans_b: usize = droplets
        .iter()
        .map(|(x, y, z)| {
            dirs.iter()
                .filter(|(dx, dy, dz)| reachable.contains(&(x + dx, y + dy, z + dz)))
                .count()
        })
        .sum();
    println!("{}", ans_b);
}
