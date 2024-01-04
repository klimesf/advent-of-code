use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::fs;

pub(crate) fn day22() {
    println!("{}", part_a(fs::read_to_string("input/2018/day22/input.txt").unwrap()));
    println!("{}", part_b(fs::read_to_string("input/2018/day22/input.txt").unwrap()));
}

fn part_a(input: String) -> usize {
    let (depth, target) = parse_input(input);
    let erosion_level = get_erosion_level(depth, target);

    let mut sum = 0;
    for y in 0..=target.1 {
        for x in 0..=target.0 {
            sum += erosion_level[y][x] % 3;
        }
    }
    sum
}

fn part_b(input: String) -> usize {
    let (depth, target) = parse_input(input);
    let erosion_level = get_erosion_level(depth, target);

    let mut visited = HashSet::new();
    let mut stack = BinaryHeap::new();
    stack.push(Pos { x: 0, y: 0, time: 0, tool: Tool::Torch });

    while let Some(pos) = stack.pop() {
        if pos.x == target.0 && pos.y == target.1 {
            if pos.tool != Tool::Torch { panic!("fuck off") }
            return pos.time;
        }

        if !visited.insert((pos.x, pos.y, pos.tool)) {
            continue;
        }

        // Move left, right, up, down if possible
        if pos.x > 0 && pos.tool.compatible_with(erosion_level[pos.y][pos.x - 1] % 3, (pos.x - 1, pos.y), target) {
            stack.push(Pos { x: pos.x - 1, y: pos.y, time: pos.time + 1, tool: pos.tool })
        }
        if pos.tool.compatible_with(erosion_level[pos.y][pos.x + 1] % 3, (pos.x + 1, pos.y), target) {
            stack.push(Pos { x: pos.x + 1, y: pos.y, time: pos.time + 1, tool: pos.tool })
        }
        if pos.y > 0 && pos.tool.compatible_with(erosion_level[pos.y - 1][pos.x] % 3, (pos.x, pos.y - 1), target) {
            stack.push(Pos { x: pos.x, y: pos.y - 1, time: pos.time + 1, tool: pos.tool })
        }
        if pos.tool.compatible_with(erosion_level[pos.y + 1][pos.x] % 3, (pos.x, pos.y + 1), target) {
            stack.push(Pos { x: pos.x, y: pos.y + 1, time: pos.time + 1, tool: pos.tool })
        }

        // Switch tools without moving
        if pos.tool != Tool::Torch && Tool::Torch.compatible_with(erosion_level[pos.y][pos.x] % 3, (pos.x, pos.y), target) {
            stack.push(Pos { x: pos.x, y: pos.y, time: pos.time + 7, tool: Tool::Torch })
        }
        if pos.tool != Tool::ClimbingGear && Tool::ClimbingGear.compatible_with(erosion_level[pos.y][pos.x] % 3, (pos.x, pos.y), target) {
            stack.push(Pos { x: pos.x, y: pos.y, time: pos.time + 7, tool: Tool::ClimbingGear })
        }
        if pos.tool != Tool::Neither && Tool::Neither.compatible_with(erosion_level[pos.y][pos.x] % 3, (pos.x, pos.y), target) {
            stack.push(Pos { x: pos.x, y: pos.y, time: pos.time + 7, tool: Tool::Neither })
        }
    }
    panic!("never reached target");
}

fn parse_input(input: String) -> (usize, (usize, usize)) {
    let (ds, ts) = input.split_once("\n").unwrap();
    let (_, d) = ds.split_once(": ").unwrap();
    let depth = d.parse::<usize>().unwrap();
    let (_, t) = ts.split_once(": ").unwrap();
    let (t1, t2) = t.split_once(",").unwrap();
    let target: (usize, usize) = (t1.parse().unwrap(), t2.parse().unwrap());
    (depth, target)
}

fn get_erosion_level(depth: usize, target: (usize, usize)) -> Vec<Vec<usize>> {
    let mut erosion_level = vec! {vec! {0; target.0 * 100}; target.1 * 20};
    for y in 0..target.1 * 20 {
        for x in 0..target.0 * 100 {
            if x == 0 && y == 0 {
                erosion_level[y][x] = 0;
            } else if x == target.0 && y == target.1 {
                erosion_level[y][x] = 0;
            } else if x == 0 {
                erosion_level[y][x] = (y * 48271 + depth) % 20183;
            } else if y == 0 {
                erosion_level[y][x] = (x * 16807 + depth) % 20183;
            } else {
                erosion_level[y][x] = (erosion_level[y - 1][x] * erosion_level[y][x - 1] + depth) % 20183;
            }
        }
    }
    erosion_level
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
struct Pos {
    x: usize,
    y: usize,
    time: usize,
    tool: Tool,
}

impl Ord for Pos {
    fn cmp(&self, other: &Self) -> Ordering {
        other.time.cmp(&self.time)
    }
}

impl PartialOrd for Pos {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
enum Tool {
    Torch,
    ClimbingGear,
    Neither,
}

impl Tool {
    fn compatible_with(&self, terrain: usize, pos: (usize, usize), target: (usize, usize)) -> bool {
        if pos == target {
            return self == &Tool::Torch;
        }

        match terrain {
            // rocky
            0 => { self == &Tool::Torch || self == &Tool::ClimbingGear }
            // wet
            1 => { self == &Tool::ClimbingGear || self == &Tool::Neither }
            // narrow
            2 => { self == &Tool::Torch || self == &Tool::Neither }
            _ => { panic!("unknown terrain {terrain}") }
        }
    }
}

#[cfg(test)]
mod day22_tests {
    use std::fs;

    use crate::y2018::day22::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(114, part_a(fs::read_to_string("input/2018/day22/test.txt").unwrap()));
        assert_eq!(45, part_b(fs::read_to_string("input/2018/day22/test.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!(8090, part_a(fs::read_to_string("input/2018/day22/input.txt").unwrap()));
        assert_eq!(992, part_b(fs::read_to_string("input/2018/day22/input.txt").unwrap()));
    }
}
