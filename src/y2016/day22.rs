use crate::utils::toolbox::parse_usize;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs;

pub(crate) fn day22() {
    println!("{}", part_a(fs::read_to_string("input/2016/day22/input.txt").unwrap()));
    println!("{}", part_b(fs::read_to_string("input/2016/day22/input.txt").unwrap()));
}

fn part_a(input: String) -> usize {
    let mut devices: Vec<(usize, usize, usize, usize, usize)> = vec![];
    input.lines().for_each(|line| {
        if line.starts_with("root") || line.starts_with("Filesystem") {
            return;
        }
        let re =
            Regex::new("^/dev/grid/node\\-x([0-9]+)\\-y([0-9]+)\\s+([0-9]+)T\\s+([0-9]+)T\\s+([0-9]+)T\\s+[0-9]+%$")
                .unwrap();
        let caps = re.captures(line).unwrap();

        devices.push((
            parse_usize(caps.get(1)), // x
            parse_usize(caps.get(2)), // y
            parse_usize(caps.get(3)), // size
            parse_usize(caps.get(4)), // used
            parse_usize(caps.get(5)), // avail
        ))
    });

    let mut ans = 0;
    for i in 0..devices.len() {
        let a = devices[i];
        if a.3 == 0 {
            continue;
        }
        for j in 0..devices.len() {
            if i == j {
                continue;
            }
            let b = devices[j];
            if b.4 >= a.3 {
                ans += 1
            }
        }
    }
    ans
}

fn part_b(input: String) -> usize {
    let mut devices: HashMap<(usize, usize), (usize, usize, usize)> = HashMap::new();
    let mut x_max = 0;
    let mut y_max = 0;
    input.lines().for_each(|line| {
        if line.starts_with("root") || line.starts_with("Filesystem") {
            return;
        }
        let re =
            Regex::new("^/dev/grid/node\\-x([0-9]+)\\-y([0-9]+)\\s+([0-9]+)T\\s+([0-9]+)T\\s+([0-9]+)T\\s+[0-9]+%$")
                .unwrap();
        let caps = re.captures(line).unwrap();

        let x = parse_usize(caps.get(1));
        if x > x_max {
            x_max = x
        }
        let y = parse_usize(caps.get(2));
        if y > x_max {
            y_max = y
        }
        devices.insert(
            (x, y),
            (
                parse_usize(caps.get(3)), // size
                parse_usize(caps.get(4)), // used
                parse_usize(caps.get(5)), // avail
            ),
        );
    });

    // Find used=0 (empty device)
    let mut empty = (0, 0);
    'outer: for x in 0..=x_max {
        for y in 0..=y_max {
            let (_, used, _) = devices.get(&(x, y)).unwrap();
            if *used == 0 {
                empty = (x, y);
                break 'outer;
            }
        }
    }
    if empty == (0, 0) {
        panic!("did not find empty device")
    }

    // Find shortest path from empty to (x_max - 1, 0) - field before G
    let mut stack = BinaryHeap::new();
    let mut visited = HashSet::new();
    stack.push(Pos {
        x: empty.0,
        y: empty.1,
        dist: 0,
    });
    let mut dist_from_empty = 0;
    while let Some(pos) = stack.pop() {
        if !visited.insert((pos.x, pos.y)) {
            continue;
        }
        if pos.x == x_max - 1 && pos.y == 0 {
            dist_from_empty = pos.dist;
            break;
        }
        let (a_size, _, _) = devices.get(&(pos.x, pos.y)).unwrap();

        // Left
        if pos.x > 0 {
            let (_, b_used, _) = devices.get(&(pos.x - 1, pos.y)).unwrap();
            if b_used <= a_size {
                stack.push(Pos {
                    x: pos.x - 1,
                    y: pos.y,
                    dist: pos.dist + 1,
                })
            }
        }

        // Right
        if pos.x < x_max {
            let (_, b_used, _) = devices.get(&(pos.x + 1, pos.y)).unwrap();
            if b_used <= a_size {
                stack.push(Pos {
                    x: pos.x + 1,
                    y: pos.y,
                    dist: pos.dist + 1,
                })
            }
        }

        // Up
        if pos.y > 0 {
            let (_, b_used, _) = devices.get(&(pos.x, pos.y - 1)).unwrap();
            if b_used <= a_size {
                stack.push(Pos {
                    x: pos.x,
                    y: pos.y - 1,
                    dist: pos.dist + 1,
                })
            }
        }

        // Down
        if pos.y < y_max {
            let (_, b_used, _) = devices.get(&(pos.x, pos.y + 1)).unwrap();
            if b_used <= a_size {
                stack.push(Pos {
                    x: pos.x,
                    y: pos.y + 1,
                    dist: pos.dist + 1,
                })
            }
        }
    }

    // 5 moves for shifting G left 1 pos, plus one to the final 0,0
    dist_from_empty + 5 * (x_max - 1) + 1
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Pos {
    x: usize,
    y: usize,
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
mod day22_tests {
    use std::fs;

    use crate::y2016::day22::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(7, part_b(fs::read_to_string("input/2016/day22/test.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!(981, part_a(fs::read_to_string("input/2016/day22/input.txt").unwrap()));
        assert_eq!(233, part_b(fs::read_to_string("input/2016/day22/input.txt").unwrap()));
    }
}
