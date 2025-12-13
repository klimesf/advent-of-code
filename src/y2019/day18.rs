use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};
use std::fs;

use priority_queue::PriorityQueue;

pub(crate) fn day18() {
    part_a();
    part_b();
}

#[allow(dead_code)]
fn part_a() {
    let input = fs::read_to_string("input/2019/day18/input.txt").unwrap();
    let mut map: Vec<Vec<char>> = vec![];
    for line in input.trim().split("\n") {
        map.push(line.chars().collect());
    }

    let (_, edges) = create_graph(&map);
    let key_cnt = edges.keys().count() - 1; // Without @

    let mut min = usize::MAX;
    let mut prio_queue = PriorityQueue::new();
    prio_queue.push(('@', 0, 0), Reverse(0));
    let mut visited = HashMap::new();

    let mut next_move_cache: HashMap<(char, usize), HashSet<(char, usize)>> = HashMap::new();

    while let Some(((start, keys, start_dist), _)) = prio_queue.pop() {
        if has_all_keys(keys, key_cnt) {
            min = if start_dist < min { start_dist } else { min };
            break; // We are sure we found the min, we can break
        }
        if *visited.entry((start, keys)).or_insert(usize::MAX) < start_dist {
            continue;
        }

        let mut dist = HashMap::new();
        let mut possible_moves = vec![];
        possible_moves.push((start, start_dist));

        if let Some(next_moves) = next_move_cache.get(&(start, keys)) {
            // If we have the next move in cache, add them right away
            add_moves_to_queue(&mut prio_queue, &mut visited, &keys, next_moves);
            continue;
        }

        // We don't have the next moves in cache, run DFS and find the closest unvisited keys
        let mut next_moves = HashSet::new();
        while !possible_moves.is_empty() {
            let (from, from_dist) = possible_moves.pop().unwrap();
            dist.insert(from, from_dist);

            if !has_key(keys, from) {
                next_moves.insert((from, from_dist));
                continue;
            }

            for (to, to_dist, to_keys) in edges.get(&from).unwrap() {
                if !extract_keys(*to_keys).iter().all(|rk| has_key(keys, *rk)) {
                    continue; // Cannot go there, don't have the keys
                }

                if dist.contains_key(to) && *dist.get(&to).unwrap() < from_dist + to_dist {
                    continue; // We've been there, skip
                }

                possible_moves.push((*to, from_dist + to_dist));
            }
        }

        add_moves_to_queue(&mut prio_queue, &mut visited, &keys, &next_moves);
        next_move_cache.insert((start, keys), next_moves.clone());
    }
    println!("It takes at least {} steps to collect all keys", min);
}

fn add_moves_to_queue(
    prio_queue: &mut PriorityQueue<(char, usize, usize), Reverse<usize>>,
    visited: &mut HashMap<(char, usize), usize>,
    keys: &usize,
    next_moves: &HashSet<(char, usize)>,
) {
    for (from, from_dist) in next_moves {
        let new_keys = add_key(*keys, *from);
        let new_state = (*from, new_keys, *from_dist);
        if *visited.entry((*from, new_keys)).or_insert(usize::MAX) > *from_dist {
            visited.insert((*from, new_keys), *from_dist);
            prio_queue.push(new_state, Reverse(new_state.2));
        }
    }
}

#[allow(dead_code)]
fn part_b() {
    let input = fs::read_to_string("input/2019/day18/input_b.txt").unwrap();
    let mut map: Vec<Vec<char>> = vec![];
    for line in input.trim().split("\n") {
        map.push(line.chars().collect());
    }

    let (vertices, edges) = create_graph(&map);
    let key_cnt = vertices
        .iter()
        .filter(|(c, _, _)| *c >= 'a' && *c <= 'z')
        .count();

    let mut prio_queue = PriorityQueue::new();
    prio_queue.push((('1', '2', '3', '4'), 0, 0), Reverse(0));
    let mut visited = HashMap::new();
    let mut min = usize::MAX;

    let mut robot_cache: HashMap<(char, usize), HashSet<(char, usize)>> = HashMap::new();

    while let Some(((start, keys, start_dist), _)) = prio_queue.pop() {
        //println!("{}", prio_queue.len());
        if has_all_keys(keys, key_cnt) {
            min = if start_dist < min { start_dist } else { min };
            break; // We are sure we found the min, we can break
        }
        if *visited.entry((start, keys)).or_insert(usize::MAX) < start_dist {
            continue;
        }

        let mut dist: HashMap<char, usize> = HashMap::new();
        let mut possible_moves = PriorityQueue::new();
        possible_moves.push((start, start_dist), Reverse(start_dist));

        let mut next_moves = HashSet::new();
        while !possible_moves.is_empty() {
            let ((from, from_dist), _) = possible_moves.pop().unwrap();

            if !has_key(keys, from.0)
                || !has_key(keys, from.1)
                || !has_key(keys, from.2)
                || !has_key(keys, from.3)
            {
                next_moves.insert((from, from_dist));
                continue;
            }

            let robots = [from.0, from.1, from.2, from.3];
            for (i, robot) in robots.iter().enumerate() {
                dist.insert(*robot, from_dist);

                let mut robot_moves: HashSet<(char, usize)> = HashSet::new();
                if robot_cache.contains_key(&(*robot, keys)) {
                    robot_moves = robot_cache.get(&(*robot, keys)).unwrap().clone();
                } else {
                    for (to, to_dist, to_keys) in edges.get(robot).or(Some(&vec![])).unwrap() {
                        if !extract_keys(*to_keys).iter().all(|rk| has_key(keys, *rk)) {
                            continue; // Cannot go there, don't have the keys
                        }

                        if dist.contains_key(to) && *dist.get(to).unwrap() < from_dist + to_dist {
                            continue; // We've been there, skip
                        }

                        robot_moves.insert((*to, *to_dist));
                    }
                    robot_cache.insert((*robot, keys), robot_moves.clone());
                }

                robot_moves.iter().for_each(|(to, to_dist)| {
                    let mut next = robots.clone();
                    next[i] = *to;
                    let dest = (next[0], next[1], next[2], next[3]);
                    possible_moves.push((dest, from_dist + to_dist), Reverse(from_dist + to_dist));
                });
            }
        }

        add_moves_to_queue_b(&mut prio_queue, &mut visited, &keys, &next_moves);
    }
    println!(
        "It takes at least {} steps to collect all keys by 4 robots",
        min
    );
}

fn add_moves_to_queue_b(
    prio_queue: &mut PriorityQueue<((char, char, char, char), usize, usize), Reverse<usize>>,
    visited: &mut HashMap<((char, char, char, char), usize), usize>,
    keys: &usize,
    next_moves: &HashSet<((char, char, char, char), usize)>,
) {
    for (from, from_dist) in next_moves {
        let mut new_keys = *keys;
        for new_key in [from.0, from.1, from.2, from.3] {
            if new_key >= 'a' && new_key <= 'z' && !has_key(*keys, new_key) {
                new_keys = add_key(new_keys, new_key);
            }
        }

        let new_state = (*from, new_keys, *from_dist);
        if *visited.entry((*from, new_keys)).or_insert(usize::MAX) > *from_dist {
            visited.insert((*from, new_keys), *from_dist);
            prio_queue.push(new_state, Reverse(new_state.2));
        }
    }
}

fn create_graph(
    map: &Vec<Vec<char>>,
) -> (
    Vec<(char, usize, usize)>,
    HashMap<char, Vec<(char, usize, usize)>>,
) {
    // Find vertices
    let mut vertices = vec![];
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            let c = map[y][x];
            if c == '.' || c == '#' || (c >= 'A' && c <= 'Z') {
                continue;
            }
            vertices.push((c, x, y));
        }
    }

    // Run DFS from each vertex and find edges to neighbors
    let mut edges = HashMap::new();
    for (from, v_x, v_y) in vertices.clone() {
        let mut to_visit = vec![];
        to_visit.push((v_x, v_y, 0, 0));

        let mut dist: HashMap<(usize, usize), usize> = HashMap::new();
        while !to_visit.is_empty() {
            let (x, y, d, mut keys) = to_visit.pop().unwrap();
            *dist.entry((x, y)).or_insert(d) = d;

            let to = map[y][x];
            if to != from && (to >= 'a' && to <= 'z') {
                edges.entry(from).or_insert(vec![]).push((to, d, keys));
                continue;
            }
            if to >= 'A' && to <= 'Z' {
                keys = add_key(keys, (to as u8 + 32) as char);
            }

            if x > 0
                && map[y][x - 1] != '#'
                && *dist.entry((x - 1, y)).or_insert(usize::MAX) > d + 1
            {
                to_visit.push((x - 1, y, d + 1, keys));
            }
            if x < map[y].len() - 1
                && map[y][x + 1] != '#'
                && *dist.entry((x + 1, y)).or_insert(usize::MAX) > d + 1
            {
                to_visit.push((x + 1, y, d + 1, keys));
            }
            if y > 0
                && map[y - 1][x] != '#'
                && *dist.entry((x, y - 1)).or_insert(usize::MAX) > d + 1
            {
                to_visit.push((x, y - 1, d + 1, keys));
            }
            if y < map.len() - 1
                && map[y + 1][x] != '#'
                && *dist.entry((x, y + 1)).or_insert(usize::MAX) > d + 1
            {
                to_visit.push((x, y + 1, d + 1, keys));
            }
        }
    }

    return (vertices, edges);
}

fn has_all_keys(keys: usize, total_keys: usize) -> bool {
    (0..total_keys).all(|k| has_key(keys, (k + 97) as u8 as char))
}

fn add_key(keys: usize, key: char) -> usize {
    let key_val = (key as u32) - 97;
    let inc = 1 << key_val;
    keys + inc
}

fn has_key(keys: usize, key: char) -> bool {
    if key == '@' || (key >= '1' && key <= '4') {
        return true;
    }
    let key_val = (key as u32) - 97;
    let dec = keys >> key_val;
    dec % 2 == 1
}

fn extract_keys(keys: usize) -> Vec<char> {
    ('a'..'z').filter(|c| has_key(keys, *c)).collect()
}

#[cfg(test)]
mod tests {
    use crate::y2019::day18::{add_key, extract_keys, has_all_keys, has_key};

    #[test]
    fn has_key_work() {
        let mut keys = 0;
        assert_eq!(false, has_key(keys, 'a'));

        keys = add_key(keys, 'a');
        keys = add_key(keys, 'b');

        assert_eq!(true, has_key(keys, 'a'));
        assert_eq!(true, has_key(keys, 'b'));
        assert_eq!(false, has_key(keys, 'c'));

        keys = add_key(keys, 'z');
        assert_eq!(true, has_key(keys, 'a'));
        assert_eq!(true, has_key(keys, 'b'));
        assert_eq!(false, has_key(keys, 'c'));
        assert_eq!(false, has_key(keys, 'd'));
        assert_eq!(false, has_key(keys, 'e'));
        assert_eq!(true, has_key(keys, 'z'));
    }

    #[test]
    fn extract_keys_work() {
        let mut keys = 0;

        assert_eq!(vec!() as Vec<char>, extract_keys(keys));

        keys = add_key(keys, 'a');
        assert_eq!(vec!['a'], extract_keys(keys));

        keys = add_key(keys, 'b');
        assert_eq!(vec!['a', 'b'], extract_keys(keys));

        keys = add_key(keys, 'z');
        assert_eq!(vec!['a', 'b', 'z'], extract_keys(keys));
    }

    #[test]
    fn has_all_keys_work() {
        let mut keys = 0;
        let key_ctr = 4;

        assert_eq!(false, has_all_keys(keys, key_ctr));
        keys = add_key(keys, 'a');
        assert_eq!(false, has_all_keys(keys, key_ctr));
        keys = add_key(keys, 'd');
        assert_eq!(false, has_all_keys(keys, key_ctr));
        keys = add_key(keys, 'c');
        assert_eq!(false, has_all_keys(keys, key_ctr));
        keys = add_key(keys, 'b');
        assert_eq!(true, has_all_keys(keys, key_ctr));
    }
}
