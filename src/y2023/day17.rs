use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs;

pub(crate) fn day17() {
    println!("{}", part_a(fs::read_to_string("input/2023/day17/input.txt").unwrap()));
    println!("{}", part_b(fs::read_to_string("input/2023/day17/input.txt").unwrap()));
}

fn part_a(input: String) -> u32 {
    let matrix: Vec<Vec<u32>> = input.lines().map(|line| {
        line.chars().map(|c| c.to_digit(10).unwrap()).collect()
    }).collect();
    dijkstra(&matrix, 1, 3)
}

fn part_b(input: String) -> u32 {
    let matrix: Vec<Vec<u32>> = input.lines().map(|line| {
        line.chars().map(|c| c.to_digit(10).unwrap()).collect()
    }).collect();
    dijkstra(&matrix, 4, 10)
}

fn dijkstra(matrix: &Vec<Vec<u32>>, i_min: usize, i_max: usize) -> u32 {
    let fin = (matrix.len() - 1, matrix[0].len() - 1);

    let mut stack: BinaryHeap<Pos> = BinaryHeap::new();
    let mut dist: HashMap<(usize, usize, usize), u32> = HashMap::new();
    let mut visited: HashSet<(usize, usize, usize)> = HashSet::new();

    stack.push(Pos { x: 0, y: 0, loss: 0, dir: 69 }); // we need a different dir in the start
    while let Some(pos) = stack.pop() {
        if (pos.x, pos.y) == fin {
            return pos.loss;
        }
        if *dist.get(&(pos.x, pos.y, pos.dir)).unwrap_or(&u32::MAX) < pos.loss {
            continue;
        }
        if !visited.insert((pos.x, pos.y, pos.dir)) {
            continue;
        }

        for dir in [0, 90, 180, 270] {
            if dir == pos.dir || dir == (pos.dir + 180) % 360 { continue; }
            let mut loss = pos.loss;
            for i in 1..=i_max {
                let next_pos = match dir {
                    90 => {
                        if pos.y < matrix[pos.x].len() - i {
                            Some((pos.x, pos.y + i))
                        } else {
                            None
                        }
                    }
                    180 => {
                        if pos.x < matrix.len() - i {
                            Some((pos.x + i, pos.y))
                        } else {
                            None
                        }
                    }
                    270 => {
                        if pos.y >= i {
                            Some((pos.x, pos.y - i))
                        } else {
                            None
                        }
                    }
                    0 => {
                        if pos.x >= i {
                            Some((pos.x - i, pos.y))
                        } else {
                            None
                        }
                    }
                    _ => panic!("unknown dir {}", dir)
                };

                if let Some((x, y)) = next_pos {
                    loss += matrix[x][y];
                    if i >= i_min && *dist.get(&(x, y, dir)).unwrap_or(&u32::MAX) > loss {
                        stack.push(Pos { x, y, dir, loss });
                        dist.insert((x, y, dir), loss);
                    }
                }
            }
        }
    }

    panic!("destination not found");
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Pos {
    x: usize,
    y: usize,
    loss: u32,
    dir: usize,
}

impl Ord for Pos {
    fn cmp(&self, other: &Self) -> Ordering {
        other.loss.cmp(&self.loss)
    }
}

impl PartialOrd for Pos {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod day17_tests {
    use std::fs;

    use crate::y2023::day17::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(102, part_a(fs::read_to_string("input/2023/day17/test.txt").unwrap()));
        assert_eq!(94, part_b(fs::read_to_string("input/2023/day17/test.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!(686, part_a(fs::read_to_string("input/2023/day17/input.txt").unwrap()));
        assert_eq!(801, part_b(fs::read_to_string("input/2023/day17/input.txt").unwrap()));
    }
}
