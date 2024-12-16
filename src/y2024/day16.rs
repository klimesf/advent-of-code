use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::fs;

pub(crate) fn day16() {
    println!("{}", part_a(fs::read_to_string("input/2024/day16/input.txt").unwrap()));
    println!("{}", part_b(fs::read_to_string("input/2024/day16/input.txt").unwrap()));
}

fn part_a(input: String) -> usize {
    let map: Vec<Vec<char>> = input.lines().map(|line| {
        line.chars().collect()
    }).collect();

    let mut starting_position = (0, 0);
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 'S' {
                starting_position = (i, j);
            }
        }
    }

    let mut stack = BinaryHeap::new();
    let mut visited = HashSet::new();
    stack.push(Pos { x: starting_position.0, y: starting_position.1, dir: 1, dist: 0 });

    while let Some(pos) = stack.pop() {
        if map[pos.x][pos.y] == 'E' {
            return pos.dist;
        }
        if map[pos.x][pos.y] == '#' { continue; }
        if !visited.insert((pos.x, pos.y, pos.dir)) { continue; }

        match pos.dir {
            0 => {
                if pos.x > 0 { stack.push(Pos { x: pos.x - 1, y: pos.y, dir: pos.dir, dist: pos.dist + 1 }) }
                stack.push(Pos { x: pos.x, y: pos.y, dir: 1, dist: pos.dist + 1000 });
                stack.push(Pos { x: pos.x, y: pos.y, dir: 3, dist: pos.dist + 1000 });
            }
            1 => {
                if pos.y < map[0].len() - 1 { stack.push(Pos { x: pos.x, y: pos.y + 1, dir: pos.dir, dist: pos.dist + 1 }) }
                stack.push(Pos { x: pos.x, y: pos.y, dir: 0, dist: pos.dist + 1000 });
                stack.push(Pos { x: pos.x, y: pos.y, dir: 2, dist: pos.dist + 1000 });
            }
            2 => {
                if pos.x < map.len() - 1 { stack.push(Pos { x: pos.x + 1, y: pos.y, dir: pos.dir, dist: pos.dist + 1 }) }
                stack.push(Pos { x: pos.x, y: pos.y, dir: 1, dist: pos.dist + 1000 });
                stack.push(Pos { x: pos.x, y: pos.y, dir: 3, dist: pos.dist + 1000 });
            }
            3 => {
                if pos.y > 0 { stack.push(Pos { x: pos.x, y: pos.y - 1, dir: pos.dir, dist: pos.dist + 1 }) }
                stack.push(Pos { x: pos.x, y: pos.y, dir: 2, dist: pos.dist + 1000 });
                stack.push(Pos { x: pos.x, y: pos.y, dir: 0, dist: pos.dist + 1000 });
            }
            _ => panic!()
        }
    }
    0
}

fn part_b(input: String) -> usize {
    let map: Vec<Vec<char>> = input.lines().map(|line| {
        line.chars().collect()
    }).collect();

    let mut starting_position = (0, 0);
    let mut finish = (0, 0);
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 'S' {
                starting_position = (i, j);
            }
            if map[i][j] == 'E' {
                finish = (i, j);
            }
        }
    }

    // Run modified dijkstra, where we remember the shortest distance to the point from each dir
    // This allows us to remember all the shortest paths
    let mut stack = BinaryHeap::new();
    let mut distances = vec![vec![vec![usize::MAX; 4]; map[0].len()]; map.len()];
    let mut min_e = usize::MAX;
    stack.push(Pos { x: starting_position.0, y: starting_position.1, dir: 1, dist: 0 });

    while let Some(pos) = stack.pop() {
        if pos.dist > min_e { continue; }
        if map[pos.x][pos.y] == 'E' {
            distances[pos.x][pos.y][pos.dir] = distances[pos.x][pos.y][pos.dir].min(pos.dist);
            min_e = distances[pos.x][pos.y][pos.dir];
            continue;
        }
        if map[pos.x][pos.y] == '#' { continue; }
        if distances[pos.x][pos.y][pos.dir] < pos.dist { continue; }
        distances[pos.x][pos.y][pos.dir] = distances[pos.x][pos.y][pos.dir].min(pos.dist);

        match pos.dir {
            // Pattern:
            // - Move in dir
            // - Rotate counterclockwise if there is no obstacle ahead in the new dir
            // - Rotate clockwise if there is no obstacle ahead in the new dir
            0 => {
                if map[pos.x - 1][pos.y] != '#' { stack.push(Pos { x: pos.x - 1, y: pos.y, dir: pos.dir, dist: pos.dist + 1 }) }
                if map[pos.x][pos.y + 1] != '#' { stack.push(Pos { x: pos.x, y: pos.y, dir: 1, dist: pos.dist + 1000 }) }
                if map[pos.x][pos.y - 1] != '#' { stack.push(Pos { x: pos.x, y: pos.y, dir: 3, dist: pos.dist + 1000 }) }
            }
            1 => {
                if map[pos.x][pos.y + 1] != '#' { stack.push(Pos { x: pos.x, y: pos.y + 1, dir: pos.dir, dist: pos.dist + 1 }) }
                if map[pos.x - 1][pos.y] != '#' { stack.push(Pos { x: pos.x, y: pos.y, dir: 0, dist: pos.dist + 1000 }) }
                if map[pos.x + 1][pos.y] != '#' { stack.push(Pos { x: pos.x, y: pos.y, dir: 2, dist: pos.dist + 1000 }) }
            }
            2 => {
                if map[pos.x + 1][pos.y] != '#' { stack.push(Pos { x: pos.x + 1, y: pos.y, dir: pos.dir, dist: pos.dist + 1 }) }
                if map[pos.x][pos.y + 1] != '#' { stack.push(Pos { x: pos.x, y: pos.y, dir: 1, dist: pos.dist + 1000 }) }
                if map[pos.x][pos.y - 1] != '#' { stack.push(Pos { x: pos.x, y: pos.y, dir: 3, dist: pos.dist + 1000 }) }
            }
            3 => {
                if map[pos.x][pos.y - 1] != '#' { stack.push(Pos { x: pos.x, y: pos.y - 1, dir: pos.dir, dist: pos.dist + 1 }) }
                if map[pos.x - 1][pos.y] != '#' { stack.push(Pos { x: pos.x, y: pos.y, dir: 0, dist: pos.dist + 1000 }) }
                if map[pos.x + 1][pos.y] != '#' { stack.push(Pos { x: pos.x, y: pos.y, dir: 2, dist: pos.dist + 1000 }) }
            }
            _ => panic!()
        }
    }

    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    // Run DFS from end to start and visit shortest paths
    // For each point P
    //  - either continue in the direction to P' if dist(S -> P') == dist(S -> P) - 1
    //  - or turn in any direction and continue to P' if dist(S -> P') == dist(S -> P) - 1001
    // All visited vertices will be on the shortest path
    let mut stack = vec!();
    for dir in 0..4 {
        if distances[finish.0][finish.1][dir] == min_e {
            stack.push(Pos { x: finish.0, y: finish.1, dir, dist: min_e });
        }
    }

    while let Some(pos) = stack.pop() {
        if map[pos.x][pos.y] == '#' { continue; }
        if !visited.insert((pos.x, pos.y)) { continue; }
        if map[pos.x][pos.y] == 'S' { continue; }

        for dir in 0..4 {
            let diff = if dir == pos.dir { 1 } else { 1001 };
            match dir {
                0 => {
                    if pos.dist >= diff && distances[pos.x + 1][pos.y][dir] == pos.dist - diff {
                        stack.push(Pos { x: pos.x + 1, y: pos.y, dir: dir, dist: pos.dist - diff });
                    }
                }
                1 => {
                    if pos.dist >= diff && distances[pos.x][pos.y - 1][dir] == pos.dist - diff {
                        stack.push(Pos { x: pos.x, y: pos.y - 1, dir: dir, dist: pos.dist - diff });
                    }
                }
                2 => {
                    if pos.dist >= diff && distances[pos.x - 1][pos.y][dir] == pos.dist - diff {
                        stack.push(Pos { x: pos.x - 1, y: pos.y, dir: dir, dist: pos.dist - diff });
                    }
                }
                3 => {
                    if pos.dist >= diff && distances[pos.x][pos.y + 1][dir] == pos.dist - diff {
                        stack.push(Pos { x: pos.x, y: pos.y + 1, dir: dir, dist: pos.dist - diff });
                    }
                }
                _ => panic!()
            }
        }
    }

    visited.len()
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Pos {
    x: usize,
    y: usize,
    dir: usize,
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
mod day16_tests {
    use std::fs;

    use crate::y2024::day16::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(7036, part_a(fs::read_to_string("input/2024/day16/test.txt").unwrap()));
        assert_eq!(45, part_b(fs::read_to_string("input/2024/day16/test.txt").unwrap()));
        assert_eq!(64, part_b(fs::read_to_string("input/2024/day16/test_2.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!(83444, part_a(fs::read_to_string("input/2024/day16/input.txt").unwrap()));
        assert_eq!(483, part_b(fs::read_to_string("input/2024/day16/input.txt").unwrap()));
    }
}
