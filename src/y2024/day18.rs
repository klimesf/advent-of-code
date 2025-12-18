use crate::utils::toolbox::manhattan_dist;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs;

pub fn day18(print: fn(String)) {
    print(format!("{}", part_a(fs::read_to_string("input/2024/day18/input.txt").unwrap(), 70, 1024)));
    print(format!("{:?}", part_b(fs::read_to_string("input/2024/day18/input.txt").unwrap(), 70)));
}

fn part_a(input: String, dim: usize, sim: usize) -> usize {
    let bytes: Vec<(usize, usize)> = input
        .lines()
        .map(|line| {
            let (sx, sy) = line.split_once(',').unwrap();
            let x = sx.parse::<usize>().unwrap();
            let y = sy.parse::<usize>().unwrap();
            (x, y)
        })
        .collect();

    let mut fallen = HashSet::new();
    for i in 0..sim {
        fallen.insert(bytes[i]);
    }

    let mut stack = BinaryHeap::new();
    let mut visited = HashSet::new();
    stack.push(Pos {
        x: 0,
        y: 0,
        dist: 0,
        a_star: 0,
    });

    while let Some(pos) = stack.pop() {
        if fallen.contains(&pos.coords()) {
            continue;
        }
        if !visited.insert(pos.coords()) {
            continue;
        }
        if pos.x == dim && pos.y == dim {
            return pos.dist;
        }

        if pos.x > 0 {
            stack.push(Pos {
                x: pos.x - 1,
                y: pos.y,
                dist: pos.dist + 1,
                a_star: manhattan_dist((pos.x - 1, pos.y), (dim, dim)) + pos.dist + 1,
            });
        }
        if pos.x < dim {
            stack.push(Pos {
                x: pos.x + 1,
                y: pos.y,
                dist: pos.dist + 1,
                a_star: manhattan_dist((pos.x + 1, pos.y), (dim, dim)) + pos.dist + 1,
            });
        }
        if pos.y > 0 {
            stack.push(Pos {
                x: pos.x,
                y: pos.y - 1,
                dist: pos.dist + 1,
                a_star: manhattan_dist((pos.x, pos.y - 1), (dim, dim)) + pos.dist + 1,
            });
        }
        if pos.y < dim {
            stack.push(Pos {
                x: pos.x,
                y: pos.y + 1,
                dist: pos.dist + 1,
                a_star: manhattan_dist((pos.x, pos.y + 1), (dim, dim)) + pos.dist + 1,
            });
        }
    }

    panic!("No solution found");
}

fn part_b(input: String, dim: usize) -> (usize, usize) {
    let bytes: Vec<(usize, usize)> = input
        .lines()
        .map(|line| {
            let (sx, sy) = line.split_once(',').unwrap();
            let x = sx.parse::<usize>().unwrap();
            let y = sy.parse::<usize>().unwrap();
            (x, y)
        })
        .collect();

    // Add all bytes first and do union-find for each vertex in the dim x dim matrix
    let mut fallen: HashSet<(usize, usize)> = HashSet::new();
    bytes.iter().for_each(|byte| {
        fallen.insert(*byte);
    });

    let mut parents: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    for x in 0..=dim {
        for y in 0..=dim {
            parents.insert((x, y), (x, y)); // First each vertex is the representative of itself
        }
    }

    // Build unions by adding an edge for each vertex's neighbors, if there is no byte blocking them
    for x in 0..=dim {
        for y in 0..=dim {
            if fallen.contains(&(x, y)) {
                continue;
            }
            if x > 0 && !fallen.contains(&(x - 1, y)) {
                union((x, y), (x - 1, y), &mut parents);
            }
            if x < dim && !fallen.contains(&(x + 1, y)) {
                union((x, y), (x + 1, y), &mut parents);
            }
            if y > 0 && !fallen.contains(&(x, y - 1)) {
                union((x, y), (x, y - 1), &mut parents);
            }
            if y < dim && !fallen.contains(&(x, y + 1)) {
                union((x, y), (x, y + 1), &mut parents);
            }
        }
    }

    // Remove bytes one by one until S and E are in the same group
    for i in (0..bytes.len()).rev() {
        let byte = bytes[i];
        fallen.remove(&byte);
        let (x, y) = byte;

        if x > 0 && !fallen.contains(&(x - 1, y)) {
            union((x, y), (x - 1, y), &mut parents);
        }
        if x < dim && !fallen.contains(&(x + 1, y)) {
            union((x, y), (x + 1, y), &mut parents);
        }
        if y > 0 && !fallen.contains(&(x, y - 1)) {
            union((x, y), (x, y - 1), &mut parents);
        }
        if y < dim && !fallen.contains(&(x, y + 1)) {
            union((x, y), (x, y + 1), &mut parents);
        }

        if find((0, 0), &parents) == find((dim, dim), &parents) {
            return byte;
        }
    }

    panic!("There is no path from start to end even without all bytes removed")
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Pos {
    x: usize,
    y: usize,
    dist: usize,
    a_star: usize,
}

impl Pos {
    #[inline]
    pub fn coords(&self) -> (usize, usize) {
        (self.x, self.y)
    }
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

fn union(a: (usize, usize), b: (usize, usize), parents: &mut HashMap<(usize, usize), (usize, usize)>) {
    let parent_a = find(a, parents);
    let parent_b = find(b, parents);
    parents.insert(parent_a, parent_b);
}

fn find(a: (usize, usize), parents: &HashMap<(usize, usize), (usize, usize)>) -> (usize, usize) {
    let parent = *parents.get(&a).unwrap();
    if parent == a {
        a
    } else {
        find(parent, parents)
    }
}

#[cfg(test)]
mod day18_tests {
    use std::fs;

    use crate::y2024::day18::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(22, part_a(fs::read_to_string("input/2024/day18/test.txt").unwrap(), 6, 12));
        assert_eq!((6, 1), part_b(fs::read_to_string("input/2024/day18/test.txt").unwrap(), 6));
    }

    #[test]
    fn input_works() {
        assert_eq!(374, part_a(fs::read_to_string("input/2024/day18/input.txt").unwrap(), 70, 1024));
        assert_eq!((30, 12), part_b(fs::read_to_string("input/2024/day18/input.txt").unwrap(), 70));
    }
}
