use std::collections::{HashMap, HashSet};
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

    dfs(start, end, 0, &mut HashSet::new(), &matrix)
}

fn dfs(
    pos: (usize, usize),
    end: (usize, usize),
    dist: usize,
    visited: &mut HashSet<(usize, usize)>,
    matrix: &Vec<Vec<char>>,
) -> usize {
    if pos == end {
        return dist;
    }
    if matrix[pos.0][pos.1] == '#' {
        return 0;
    }
    if !visited.insert(pos) {
        return 0;
    }

    let mut neighbors = vec![];
    if matrix[pos.0][pos.1] == '>' {
        neighbors.push((pos.0, pos.1 + 1));
    } else if matrix[pos.0][pos.1] == '<' {
        neighbors.push((pos.0, pos.1 - 1));
    } else if matrix[pos.0][pos.1] == '^' {
        neighbors.push((pos.0 - 1, pos.1));
    } else if matrix[pos.0][pos.1] == 'v' {
        neighbors.push((pos.0 + 1, pos.1));
    } else {
        neighbors.push((pos.0, pos.1 + 1));
        neighbors.push((pos.0, pos.1 - 1));
        if pos.0 > 0 {
            neighbors.push((pos.0 - 1, pos.1))
        }
        neighbors.push((pos.0 + 1, pos.1));
    }

    let max = neighbors
        .iter()
        .map(|new_pos| dfs(*new_pos, end, dist + 1, visited, matrix))
        .max()
        .unwrap();

    visited.remove(&pos);
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
        for c in 1..(matrix[0].len() - 1) {
            if matrix[r][c] == '#' {
                continue;
            }
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
        let mut stack = vec![];
        stack.push((crossroad.0 + 1, crossroad.1, 1));
        stack.push((crossroad.0 - 1, crossroad.1, 1));
        stack.push((crossroad.0, crossroad.1 + 1, 1));
        stack.push((crossroad.0, crossroad.1 - 1, 1));

        while let Some((r, c, dist)) = stack.pop() {
            if matrix[r][c] == '#' {
                continue;
            }
            if !visited.insert((r, c)) {
                continue;
            }
            if r == start.0 && c == start.1 {
                map.entry(*crossroad).or_insert(vec![]).push((r, c, dist));
                map.entry(start)
                    .or_insert(vec![])
                    .push((crossroad.0, crossroad.1, dist));
                continue;
            }
            if r == end.0 && c == end.1 {
                map.entry(*crossroad).or_insert(vec![]).push((r, c, dist));
                map.entry(end).or_insert(vec![]).push((crossroad.0, crossroad.1, dist));
                continue;
            }
            if crossroads.contains(&(r, c)) {
                map.entry(*crossroad).or_insert(vec![]).push((r, c, dist));
                continue;
            }

            stack.push((r - 1, c, dist + 1));
            stack.push((r + 1, c, dist + 1));
            stack.push((r, c + 1, dist + 1));
            stack.push((r, c - 1, dist + 1));
        }
    }

    dfs_b(start, end, 0, &map, &mut HashSet::new())
}

fn dfs_b(
    pos: (usize, usize),
    end: (usize, usize),
    dist: usize,
    map: &HashMap<(usize, usize), Vec<(usize, usize, usize)>>,
    visited: &mut HashSet<(usize, usize)>,
) -> usize {
    if pos == end {
        return dist;
    }

    if !visited.insert(pos) {
        return 0;
    }
    let max = map
        .get(&pos)
        .unwrap()
        .iter()
        .map(|(r, x, dist_to)| dfs_b((*r, *x), end, dist + dist_to, map, visited))
        .max()
        .unwrap();
    visited.remove(&pos);
    max
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
