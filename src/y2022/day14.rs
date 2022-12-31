use std::collections::{HashSet, VecDeque};
use std::fs;

pub(crate) fn day14() {
    let map = build_map();
    let bottom = map.iter().map(|(_, y)| *y).max().unwrap() + 2;
    part_a(map.clone(), bottom);
    part_b(map.clone(), bottom);
}

fn part_a(mut map: HashSet<(i32, i32)>, bottom: i32) {
    let rocks = map.len();
    while spawn_sand_a((500, 0), &mut map, bottom) {}
    println!("{}", map.len() - rocks);
}

fn part_b(map: HashSet<(i32, i32)>, bottom: i32) {
    println!("{}", bfs_sand((500, 0), &map, bottom));
}

fn build_map() -> HashSet<(i32, i32)> {
    let mut map: HashSet<(i32, i32)> = HashSet::new();
    fs::read_to_string("input/2022/day14/input.txt").unwrap().lines()
        .into_iter()
        .map(|line| line.split(" -> ")
            .into_iter()
            .map(|path| path.split_once(',').unwrap())
            .map(|(x, y)| (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()))
            .collect())
        .for_each(|mut line: Vec<(i32, i32)>| {
            let mut start = line.pop().unwrap();
            let mut from = line.pop().unwrap();

            loop {
                if start.0 == from.0 {
                    for i in start.1.min(from.1)..=start.1.max(from.1) {
                        map.insert((start.0, i));
                    }
                } else if start.1 == from.1 {
                    for i in start.0.min(from.0)..=start.0.max(from.0) {
                        map.insert((i, start.1));
                    }
                } else {
                    panic!("uh oh, diagonal");
                }

                if line.is_empty() { break; }
                start = from;
                from = line.pop().unwrap();
            }
        });
    map
}

fn spawn_sand_a(spawn_point: (i32, i32), map: &mut HashSet<(i32, i32)>, edge_of_abyss: i32) -> bool {
    let mut pos = spawn_point;
    loop {
        while !map.contains(&(pos.0, pos.1 + 1)) {
            pos = (pos.0, pos.1 + 1);
            if pos.1 > edge_of_abyss {
                return false;
            }
        }
        if !map.contains(&(pos.0 - 1, pos.1 + 1)) {
            pos = (pos.0 - 1, pos.1 + 1);
            continue;
        }
        if !map.contains(&(pos.0 + 1, pos.1 + 1)) {
            pos = (pos.0 + 1, pos.1 + 1);
            continue;
        }
        return map.insert(pos); // Can't go anywhere else
    }
}

fn bfs_sand(spawn_point: (i32, i32), map: &HashSet<(i32, i32)>, bottom: i32) -> usize {
    let mut deque = VecDeque::new();
    let mut visited = HashSet::new();
    deque.push_back(spawn_point);
    while !deque.is_empty() {
        let pos = deque.pop_front().unwrap();
        if visited.contains(&pos) {
            continue;
        }
        visited.insert(pos);

        if !map.contains(&(pos.0, pos.1 + 1)) && pos.1 + 1 < bottom {
            deque.push_back((pos.0, pos.1 + 1));
        }
        if !map.contains(&(pos.0 - 1, pos.1 + 1)) && pos.1 + 1 < bottom {
            deque.push_back((pos.0 - 1, pos.1 + 1));
        }
        if !map.contains(&(pos.0 + 1, pos.1 + 1)) && pos.1 + 1 < bottom {
            deque.push_back((pos.0 + 1, pos.1 + 1));
        }
    }
    return visited.len();
}
