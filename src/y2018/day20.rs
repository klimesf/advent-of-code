use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs;

pub(crate) fn day20() {
    let (part_a, part_b) = solve(fs::read_to_string("input/2018/day20/input.txt").unwrap());
    println!("{}", part_a);
    println!("{}", part_b);
}

fn solve(input: String) -> (usize, usize) {
    let mut graph = HashMap::new();
    let mut positions = HashSet::new();
    positions.insert((0, 0));
    gen_map(input[1..input.len() - 1].to_string(), &positions, &mut graph);

    let mut stack = BinaryHeap::new();
    let mut distances: HashMap<(i32, i32), usize> = HashMap::new();
    stack.push(Pos { r: 0, c: 0, dist: 0 });

    while let Some(pos) = stack.pop() {
        if distances.contains_key(&(pos.r, pos.c)) { continue }
        distances.insert((pos.r, pos.c), pos.dist);

        let empty_set = HashSet::new();
        let edges = graph.get(&(pos.r, pos.c)).unwrap_or(&empty_set);
        if edges.contains(&(pos.r - 1, pos.c)) { stack.push( Pos { r: pos.r - 1, c: pos.c, dist: pos.dist + 1 } ) }
        if edges.contains(&(pos.r + 1, pos.c)) { stack.push( Pos { r: pos.r + 1, c: pos.c, dist: pos.dist + 1 } ) }
        if edges.contains(&(pos.r, pos.c - 1)) { stack.push( Pos { r: pos.r, c: pos.c - 1, dist: pos.dist + 1 } ) }
        if edges.contains(&(pos.r, pos.c + 1)) { stack.push( Pos { r: pos.r, c: pos.c + 1, dist: pos.dist + 1 } ) }
    }

    let part_a = *distances.values().max().unwrap();
    let part_b = distances.values()
        .filter(|v| **v >= 1000)
        .count();
    (part_a, part_b)
}

fn gen_map(
    input: String,
    initial_positions: &HashSet<(i32, i32)>,
    graph: &mut HashMap<(i32, i32), HashSet<(i32, i32)>>,
) -> HashSet<(i32, i32)> {
    let c: Vec<char> = input.chars().collect();
    let mut positions = initial_positions.clone();
    let mut stack = vec!();

    let mut bufs: Vec<String> = vec!();
    let mut curr_buf = String::new();
    for i in 0..c.len() {
        match c[i] {
            '(' => {
                if stack.len() > 0 {
                    curr_buf.push('(');
                }
                stack.push('(');
            }
            ')' => {
                stack.pop();
                if stack.is_empty() {
                    bufs.push(curr_buf.clone());
                    curr_buf.clear();

                    let mut new_positions = HashSet::new();
                    bufs.iter().for_each(|s| {
                        for pos in gen_map(s.clone(), &positions, graph) {
                            new_positions.insert(pos);
                        }
                    });
                    positions = new_positions;

                    bufs.clear();
                } else {
                    curr_buf.push(')')
                }
            }
            '|' => {
                if stack.len() == 1 {
                    bufs.push(curr_buf.clone());
                    curr_buf.clear();
                } else {
                    curr_buf.push(c[i])
                }
            }
            'N' => {
                if stack.is_empty() {
                    let mut new_positions = HashSet::new();
                    for pos in positions {
                        let new_pos = (pos.0 - 1, pos.1);
                        graph.entry(pos).or_insert(HashSet::new()).insert(new_pos);
                        graph.entry(new_pos).or_insert(HashSet::new()).insert(pos);
                        new_positions.insert(new_pos);
                    }
                    positions = new_positions;
                } else {
                    curr_buf.push(c[i])
                }
            }
            'S' => {
                if stack.is_empty() {
                    let mut new_positions = HashSet::new();
                    for pos in positions {
                        let new_pos = (pos.0 + 1, pos.1);
                        graph.entry(pos).or_insert(HashSet::new()).insert(new_pos);
                        graph.entry(new_pos).or_insert(HashSet::new()).insert(pos);
                        new_positions.insert(new_pos);
                    }
                    positions = new_positions;
                } else {
                    curr_buf.push(c[i])
                }
            }
            'E' => {
                if stack.is_empty() {
                    let mut new_positions = HashSet::new();
                    for pos in positions {
                        let new_pos = (pos.0, pos.1 + 1);
                        graph.entry(pos).or_insert(HashSet::new()).insert(new_pos);
                        graph.entry(new_pos).or_insert(HashSet::new()).insert(pos);
                        new_positions.insert(new_pos);
                    }
                    positions = new_positions;
                } else {
                    curr_buf.push(c[i])
                }
            }
            'W' => {
                if stack.is_empty() {
                    let mut new_positions = HashSet::new();
                    for pos in positions {
                        let new_pos = (pos.0, pos.1 - 1);
                        graph.entry(pos).or_insert(HashSet::new()).insert(new_pos);
                        graph.entry(new_pos).or_insert(HashSet::new()).insert(pos);
                        new_positions.insert(new_pos);
                    }
                    positions = new_positions;
                } else {
                    curr_buf.push(c[i])
                }
            }
            x => { panic! {"unknown {}", x} }
        }
    }

    positions
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Pos {
    r: i32,
    c: i32,
    dist: usize,
}

impl Ord for Pos {
    fn cmp(&self, other: &Self) -> Ordering {
        other.dist.cmp(&self.dist)
    }
}

impl PartialOrd for Pos {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod day20_tests {
    use std::fs;

    use crate::y2018::day20::{solve};

    #[test]
    fn test_works() {
        assert_eq!(10, solve(fs::read_to_string("input/2018/day20/test_10.txt").unwrap()).0);
        assert_eq!(18, solve(fs::read_to_string("input/2018/day20/test_18.txt").unwrap()).0);
        assert_eq!(23, solve(fs::read_to_string("input/2018/day20/test_23.txt").unwrap()).0);
        assert_eq!(31, solve(fs::read_to_string("input/2018/day20/test_31.txt").unwrap()).0);
    }

    #[test]
    fn input_works() {
        let (part_a, part_b) = solve(fs::read_to_string("input/2018/day20/input.txt").unwrap());
        assert_eq!(3721, part_a);
        assert_eq!(8613, part_b);
    }
}
