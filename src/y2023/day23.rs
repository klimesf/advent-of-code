use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

pub(crate) fn day23() {
    println!("{}", part_a(fs::read_to_string("input/2023/day23/input.txt").unwrap()));
    println!("{}", part_b(fs::read_to_string("input/2023/day23/input.txt").unwrap()));
}

fn part_a(input: String) -> usize {
    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut start = (0, 0);
    let mut end = (0, 0);
    for c in 0..matrix[0].len() {
        if matrix[0][c] == '.' {
            start = (0, c);
        }
        if matrix[matrix.len() - 1][c] == '.' {
            end = (matrix.len() - 1, c);
        }
    }

    let mut stack = VecDeque::new();
    stack.push_back(Pos { r: start.0, c: start.1, dist: 0, visited: HashSet::new() });

    let mut max = 0;

    while let Some(mut pos) = stack.pop_back() {
        if matrix[pos.r][pos.c] == '#' { continue }

        if !pos.visited.insert((pos.r, pos.c)) { continue }
        if pos.r == end.0 && pos.c == end.1 {
            if pos.dist > max { max = pos.dist }
            continue
        }

        if matrix[pos.r][pos.c] == '>' {
            stack.push_back(Pos { r: pos.r, c: pos.c + 1, dist: pos.dist + 1, visited: pos.visited.clone() });
            continue
        } else if matrix[pos.r][pos.c] == '<' {
            stack.push_back(Pos { r: pos.r, c: pos.c - 1, dist: pos.dist + 1, visited: pos.visited.clone() });
            continue
        } else if matrix[pos.r][pos.c] == '^' {
            stack.push_back(Pos { r: pos.r - 1, c: pos.c, dist: pos.dist + 1, visited: pos.visited.clone() });
            continue
        } else if matrix[pos.r][pos.c] == 'v' {
            stack.push_back(Pos { r: pos.r + 1, c: pos.c, dist: pos.dist + 1, visited: pos.visited.clone() });
            continue
        } else {
            stack.push_back(Pos { r: pos.r, c: pos.c + 1, dist: pos.dist + 1, visited: pos.visited.clone() });
            stack.push_back(Pos { r: pos.r, c: pos.c - 1, dist: pos.dist + 1, visited: pos.visited.clone() });
            if pos.r > 0 { stack.push_back(Pos { r: pos.r - 1, c: pos.c, dist: pos.dist + 1, visited: pos.visited.clone() }) }
            stack.push_back(Pos { r: pos.r + 1, c: pos.c, dist: pos.dist + 1, visited: pos.visited.clone() });
        }
    }

    max
}

fn part_b(input: String) -> usize {
    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut start = (0, 0);
    let mut end = (0, 0);
    for c in 0..matrix[0].len() {
        if matrix[0][c] == '.' {
            start = (0, c);
        }
        if matrix[matrix.len() - 1][c] == '.' {
            end = (matrix.len() - 1, c);
        }
    }

    let mut crossroads = HashSet::new();

    for r in 1..(matrix.len() - 1) {
        for c in 1.. (matrix[0].len() - 1) {
            if matrix[r][c] == '#' { continue }
            let mut neighbors = 0;
            if matrix[r - 1][c] != '#' {
                neighbors += 1;
            }
            if matrix[r + 1][c] != '#' {
                neighbors += 1;
            }
            if matrix[r][c - 1] != '#' {
                neighbors += 1;
            }
            if matrix[r][c + 1] != '#' {
                neighbors += 1;
            }
            if neighbors > 2 {
                crossroads.insert((r, c));
            }
        }
    }

    let mut map: HashMap<(usize, usize), Vec<(usize, usize, usize)>> = HashMap::new();

    for crossroad in &crossroads {
        let mut visited = HashSet::new();
        let mut stack = vec!();
        stack.push((crossroad.0 + 1, crossroad.1, 1));
        stack.push((crossroad.0 - 1, crossroad.1, 1));
        stack.push((crossroad.0, crossroad.1 + 1, 1));
        stack.push((crossroad.0, crossroad.1 - 1, 1));

        while let Some((r, c, dist)) = stack.pop() {
            if matrix[r][c] == '#' { continue }
            if !visited.insert((r, c)) { continue }
            if r == start.0 && c == start.1 {
                map.entry(*crossroad).or_insert(vec!()).push((r, c, dist));
                map.entry(start).or_insert(vec!()).push((crossroad.0, crossroad.1, dist));
                continue;
            }
            if r == end.0 && c == end.1 {
                map.entry(*crossroad).or_insert(vec!()).push((r, c, dist));
                map.entry(end).or_insert(vec!()).push((crossroad.0, crossroad.1, dist));
                continue;
            }
            if crossroads.contains(&(r, c)) {
                map.entry(*crossroad).or_insert(vec!()).push((r, c, dist));
                continue;
            }

            stack.push((r - 1, c, dist + 1));
            stack.push((r + 1, c, dist + 1));
            stack.push((r, c + 1, dist + 1));
            stack.push((r, c - 1, dist + 1));
        }
    }

    let mut max = 0;
    let mut stack = vec!();
    let mut visited = HashSet::new();
    visited.insert(start);
    stack.push(Pos { r: start.0, c: start.1, dist: 0, visited: visited });

    while let Some(mut pos) = stack.pop() {
        if pos.r == end.0 && pos.c == end.1 {
            if pos.dist > max { max = pos.dist }
            continue
        }

        pos.visited.insert((pos.r, pos.c));
        let edges = map.get(&(pos.r, pos.c)).unwrap();
        for (r, c, dist_to) in edges {
            if !pos.visited.contains(&(*r, *c)) {
                stack.push(Pos { r: *r, c: *c, dist: pos.dist + dist_to, visited: pos.visited.clone() })
            }
        }
    }

    max
}

#[derive(Clone, Eq, PartialEq)]
struct Pos {
    r: usize,
    c: usize,
    dist: usize,
    visited: HashSet<(usize, usize)>,
}

#[cfg(test)]
mod day23_tests {
    use std::fs;

    use crate::y2023::day23::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(94, part_a(fs::read_to_string("input/2023/day23/test.txt").unwrap()));
        assert_eq!(154, part_b(fs::read_to_string("input/2023/day23/test.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!(2186, part_a(fs::read_to_string("input/2023/day23/input.txt").unwrap()));
        assert_eq!(6802, part_b(fs::read_to_string("input/2023/day23/input.txt").unwrap()));
    }
}
