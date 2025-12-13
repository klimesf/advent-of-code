use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet, VecDeque};

pub(crate) fn day13() {
    println!("{}", part_a(1364, (39, 31)));
    println!("{}", part_b(1364));
}

fn part_a(designer: usize, dest: (usize, usize)) -> usize {
    let (dest_y, dest_x) = dest;

    let mut visited = HashSet::new();
    let mut stack: BinaryHeap<Pos> = BinaryHeap::new();
    stack.push(Pos {
        x: 1,
        y: 1,
        steps: 0,
        a_star: (dest_y as i32 - 1).abs() + (dest_x as i32 - 1).abs(),
    });

    while let Some(pos) = stack.pop() {
        if !visited.insert((pos.x, pos.y)) {
            continue;
        }
        if is_wall((pos.x, pos.y), designer) {
            continue;
        }
        if pos.x == dest_x && pos.y == dest_y {
            return pos.steps;
        }

        if pos.x > 0 {
            stack.push(Pos {
                x: pos.x - 1,
                y: pos.y,
                steps: pos.steps + 1,
                a_star: manhattan_dist((pos.y, pos.x - 1), (dest_y, dest_x)),
            })
        }
        stack.push(Pos {
            x: pos.x + 1,
            y: pos.y,
            steps: pos.steps + 1,
            a_star: manhattan_dist((pos.y, pos.x + 1), (dest_y, dest_x)),
        });
        if pos.y > 0 {
            stack.push(Pos {
                x: pos.x,
                y: pos.y - 1,
                steps: pos.steps + 1,
                a_star: manhattan_dist((pos.y - 1, pos.x), (dest_y, dest_x)),
            })
        }
        stack.push(Pos {
            x: pos.x,
            y: pos.y + 1,
            steps: pos.steps + 1,
            a_star: manhattan_dist((pos.y + 1, pos.x), (dest_y, dest_x)),
        })
    }

    panic!("did not reach {},{}", dest_y, dest_x)
}

fn part_b(designer: usize) -> usize {
    let mut visited = HashSet::new();
    let mut stack = VecDeque::new();
    stack.push_back(Pos {
        x: 1,
        y: 1,
        steps: 0,
        a_star: 0,
    });

    while let Some(pos) = stack.pop_front() {
        if is_wall((pos.x, pos.y), designer) {
            continue;
        }
        if pos.steps > 50 {
            continue;
        }
        if !visited.insert((pos.x, pos.y)) {
            continue;
        }

        if pos.x > 0 {
            stack.push_back(Pos {
                x: pos.x - 1,
                y: pos.y,
                steps: pos.steps + 1,
                a_star: 0,
            })
        }
        stack.push_back(Pos {
            x: pos.x + 1,
            y: pos.y,
            steps: pos.steps + 1,
            a_star: 0,
        });
        if pos.y > 0 {
            stack.push_back(Pos {
                x: pos.x,
                y: pos.y - 1,
                steps: pos.steps + 1,
                a_star: 0,
            })
        }
        stack.push_back(Pos {
            x: pos.x,
            y: pos.y + 1,
            steps: pos.steps + 1,
            a_star: 0,
        })
    }

    visited.len()
}

#[derive(Clone, Eq, PartialEq)]
struct Pos {
    x: usize,
    y: usize,
    steps: usize,
    a_star: i32,
}

impl Ord for Pos {
    fn cmp(&self, other: &Self) -> Ordering {
        other.a_star.cmp(&self.a_star)
    }
}

impl PartialOrd for Pos {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn is_wall(pos: (usize, usize), designer: usize) -> bool {
    let (x, y) = pos;
    let val = ((x * x) + (3 * x) + (2 * x * y) + y + (y * y)) + designer;
    format!("{val:b}").chars().filter(|c| *c == '1').count() % 2 == 1
}

fn manhattan_dist(pos: (usize, usize), dest: (usize, usize)) -> i32 {
    let (pos_y, pos_x) = pos;
    let (dest_y, dest_x) = dest;
    (dest_y as i32 - pos_y as i32).abs() + (dest_x as i32 - pos_x as i32).abs()
}

#[cfg(test)]
mod day13_tests {
    use crate::y2016::day13::{is_wall, part_a, part_b};

    #[test]
    fn is_wall_works() {
        assert_eq!(false, is_wall((0, 0), 10));
        assert_eq!(true, is_wall((1, 0), 10));
    }

    #[test]
    fn test_works() {
        assert_eq!(11, part_a(10, (4, 7)));
    }

    #[test]
    fn input_works() {
        assert_eq!(86, part_a(1364, (39, 31)));
        assert_eq!(127, part_b(1364));
    }
}
