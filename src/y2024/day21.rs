use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::fs;

pub(crate) fn day21() {
    println!("{}", part_a(fs::read_to_string("input/2024/day21/test.txt").unwrap()));
    println!("{}", part_b(fs::read_to_string("input/2024/day21/input.txt").unwrap()));
}

fn part_a(input: String) -> usize {

    let codes: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    // +---+---+---+
    // | 7 | 8 | 9 |
    // +---+---+---+
    // | 4 | 5 | 6 |
    // +---+---+---+
    // | 1 | 2 | 3 |
    // +---+---+---+
    //     | 0 | A |
    //     +---+---+
    let numeric_keypad = [['7', '8', '9'], ['4', '5', '6'], ['1', '2', '3'], [' ', '0', 'A']];

    //     +---+---+
    //     | ^ | A |
    // +---+---+---+
    // | < | v | > |
    // +---+---+---+
    let arrow_keypad = [[' ', '^', 'A'], ['<', 'v', '>']];

    let mut ans = 0;
    for code in &codes {
        let mut paths: HashSet<Vec<char>> = HashSet::new();
        let arm_pos = (3, 2);

        let mut path_stack: Vec<((usize, usize), Vec<char>)> = vec![(arm_pos, vec!())];
        for c in code {
            let mut new_paths = vec!();

            for (arm_pos, _) in &path_stack {
                let mut stack = BinaryHeap::new();
                stack.push(Pos { x: arm_pos.0, y: arm_pos.1, dist: 0, path: vec!() });
                let mut min = usize::MAX;

                while let Some(pos) = stack.pop() {
                    if pos.dist > min { break; }
                    if numeric_keypad[pos.x][pos.y] == *c {
                        let mut new_path = pos.path.clone();
                        new_path.push('A');
                        new_paths.push(((pos.x, pos.y), new_path));
                        min = pos.dist;
                        continue;
                    }

                    if pos.x > 0 && (pos.x - 1, pos.y) != (3, 0) {
                        let mut newpath = pos.path.clone();
                        newpath.push('^');
                        stack.push(Pos { x: pos.x - 1, y: pos.y, dist: pos.dist + 1, path: newpath });
                    }
                    if pos.x < 3 && (pos.x + 1, pos.y) != (3, 0) {
                        let mut newpath = pos.path.clone();
                        newpath.push('v');
                        stack.push(Pos { x: pos.x + 1, y: pos.y, dist: pos.dist + 1, path: newpath });
                    }
                    if pos.y > 0 && (pos.x, pos.y - 1) != (3, 0) {
                        let mut newpath = pos.path.clone();
                        newpath.push('<');
                        stack.push(Pos { x: pos.x, y: pos.y - 1, dist: pos.dist + 1, path: newpath });
                    }
                    if pos.y < 2 && (pos.x, pos.y + 1) != (3, 0) {
                        let mut newpath = pos.path.clone();
                        newpath.push('>');
                        stack.push(Pos { x: pos.x, y: pos.y + 1, dist: pos.dist + 1, path: newpath });
                    }
                }
            }

            // println!("{:?}", new_paths);
            let mut new_path_stack = vec!();
            for ps in path_stack {
                for np in &new_paths {
                    let mut extension = ps.1.clone();
                    extension.extend(np.1.clone());
                    new_path_stack.push((np.0, extension));
                }
            }
            path_stack = new_path_stack;
            // println!("{:?}", path_stack);
        }

        path_stack.iter().for_each(|((_x, _y), path)| {
            paths.insert(path.clone());
        });
        println!("{}", paths.len());

        for path in &paths {
            print!("{:?}: ", code);
            print_path(path);
            println!("");
        }

        let mut paths2: HashSet<Vec<char>> = HashSet::new();
        let arm_pos = (0, 2);

        let mut path_stack: HashSet<((usize, usize), Vec<char>)> = HashSet::new();
        path_stack.insert((arm_pos, vec!()));
        for p in paths {
            println!("{:?}", p.clone());
            for c in p {
                let mut new_paths = HashSet::new();
                println!("{} {}", c, path_stack.len());

                for (arm_pos, _) in &path_stack {
                    let mut stack = BinaryHeap::new();
                    stack.push(Pos { x: arm_pos.0, y: arm_pos.1, dist: 0, path: vec!() });
                    let mut min = usize::MAX;

                    while let Some(pos) = stack.pop() {
                        if pos.dist > min { break; }
                        if arrow_keypad[pos.x][pos.y] == c {
                            let mut new_path = pos.path.clone();
                            new_path.push('A');
                            new_paths.insert(((pos.x, pos.y), new_path));
                            min = pos.dist;
                            continue;
                        }

                        if pos.x > 0 && (pos.x - 1, pos.y) != (0, 0) {
                            let mut newpath = pos.path.clone();
                            newpath.push('^');
                            stack.push(Pos { x: pos.x - 1, y: pos.y, dist: pos.dist + 1, path: newpath });
                        }
                        if pos.x < 1 && (pos.x + 1, pos.y) != (0, 0) {
                            let mut newpath = pos.path.clone();
                            newpath.push('v');
                            stack.push(Pos { x: pos.x + 1, y: pos.y, dist: pos.dist + 1, path: newpath });
                        }
                        if pos.y > 0 && (pos.x, pos.y - 1) != (0, 0) {
                            let mut newpath = pos.path.clone();
                            newpath.push('<');
                            stack.push(Pos { x: pos.x, y: pos.y - 1, dist: pos.dist + 1, path: newpath });
                        }
                        if pos.y < 2 && (pos.x, pos.y + 1) != (0, 0) {
                            let mut newpath = pos.path.clone();
                            newpath.push('>');
                            stack.push(Pos { x: pos.x, y: pos.y + 1, dist: pos.dist + 1, path: newpath });
                        }
                    }
                }

                // println!("{:?}", new_paths);
                let mut new_path_stack = HashSet::new();
                for ps in path_stack {
                    for np in &new_paths {
                        let mut extension = ps.1.clone();
                        extension.extend(np.1.clone());
                        new_path_stack.insert((np.0, extension));
                    }
                }
                path_stack = new_path_stack;
                // println!("{:?}", path_stack);
            }
        }

        path_stack.iter().for_each(|((_x, _y), path)| {
            paths2.insert(path.clone());
        });
        println!("{}", paths2.len());

        for path2 in &paths2 {
            print!("{:?}: ", code);
            print_path(path2);
            println!("");
        }

        let mut paths3: HashSet<Vec<char>> = HashSet::new();
        let arm_pos = (0, 2);

        let mut path_stack: HashSet<((usize, usize), Vec<char>)> = HashSet::new();
        path_stack.insert((arm_pos, vec!()));
        for p in paths2 {
            println!("{:?}", p.clone());
            for c in p {
                let mut new_paths = HashSet::new();
                println!("{} {}", c, path_stack.len());

                for (arm_pos, _) in &path_stack {
                    let mut stack = BinaryHeap::new();
                    stack.push(Pos { x: arm_pos.0, y: arm_pos.1, dist: 0, path: vec!() });
                    let mut min = usize::MAX;

                    while let Some(pos) = stack.pop() {
                        if pos.dist > min { break; }
                        if arrow_keypad[pos.x][pos.y] == c {
                            let mut new_path = pos.path.clone();
                            new_path.push('A');
                            new_paths.insert(((pos.x, pos.y), new_path));
                            min = pos.dist;
                            continue;
                        }

                        if pos.x > 0 && (pos.x - 1, pos.y) != (0, 0) {
                            let mut newpath = pos.path.clone();
                            newpath.push('^');
                            stack.push(Pos { x: pos.x - 1, y: pos.y, dist: pos.dist + 1, path: newpath });
                        }
                        if pos.x < 1 && (pos.x + 1, pos.y) != (0, 0) {
                            let mut newpath = pos.path.clone();
                            newpath.push('v');
                            stack.push(Pos { x: pos.x + 1, y: pos.y, dist: pos.dist + 1, path: newpath });
                        }
                        if pos.y > 0 && (pos.x, pos.y - 1) != (0, 0) {
                            let mut newpath = pos.path.clone();
                            newpath.push('<');
                            stack.push(Pos { x: pos.x, y: pos.y - 1, dist: pos.dist + 1, path: newpath });
                        }
                        if pos.y < 2 && (pos.x, pos.y + 1) != (0, 0) {
                            let mut newpath = pos.path.clone();
                            newpath.push('>');
                            stack.push(Pos { x: pos.x, y: pos.y + 1, dist: pos.dist + 1, path: newpath });
                        }
                    }
                }

                // println!("{:?}", new_paths);
                let mut new_path_stack = HashSet::new();
                for ps in path_stack {
                    for np in &new_paths {
                        let mut extension = ps.1.clone();
                        extension.extend(np.1.clone());
                        new_path_stack.insert((np.0, extension));
                    }
                }
                path_stack = new_path_stack;
                // println!("{:?}", path_stack);
            }
        }

        path_stack.iter().for_each(|((_x, _y), path)| {
            paths3.insert(path.clone());
        });
        println!("{}", paths3.len());

        let mut s = String::new();
        for c in code {
            if *c == 'A' { continue }
            s.push(*c);
        }
        let val = s.parse::<usize>().unwrap();
        let min = paths3.iter().min_by(|p, p2| p.len().cmp(&p2.len())).unwrap().len();
        ans += val * min;
    }

    ans
}

fn print_path(path: &Vec<char>) {
    for c in path {
        print!("{}", c);
    }
}

fn part_b(_: String) -> usize {
    0
}


#[derive(Clone, Eq, PartialEq, Debug)]
struct Pos {
    x: usize,
    y: usize,
    dist: usize,
    path: Vec<char>,
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
mod day21_tests {
    use std::fs;

    use crate::y2024::day21::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(126384, part_a(fs::read_to_string("input/2024/day21/test.txt").unwrap()));
        assert_eq!(0, part_b(fs::read_to_string("input/2024/day21/test.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!(0, part_a(fs::read_to_string("input/2024/day21/input.txt").unwrap()));
        assert_eq!(0, part_b(fs::read_to_string("input/2024/day21/input.txt").unwrap()));
    }
}
