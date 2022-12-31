use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs;

pub(crate) fn day24() {
    let input = fs::read_to_string("input/2022/day24/input.txt").unwrap();
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let x_max = (map.len() - 2) as i32;
    let y_max = (map[0].len() - 2) as i32;

    let mut blizzards: HashSet<(i32, i32, char)> = HashSet::new();
    for x in 0..map.len() {
        for y in 0..map[x].len() {
            if map[x][y] == '>' || map[x][y] == '<' || map[x][y] == '^' || map[x][y] == 'v' {
                blizzards.insert((x as i32, y as i32, map[x][y]));
            }
        }
    }

    let mut blizzards_per_row: HashMap<i32, Vec<(i32, i32, char)>> = HashMap::new();
    blizzards.iter()
        .filter(|(_, _, c)| *c == '<' || *c == '>')
        .for_each(|blizzard| blizzards_per_row.entry(blizzard.0).or_insert(vec!()).push(*blizzard));

    let mut blizzards_per_col: HashMap<i32, Vec<(i32, i32, char)>> = HashMap::new();
    blizzards.iter()
        .filter(|(_, _, c)| *c == '^' || *c == 'v')
        .for_each(|blizzard| blizzards_per_col.entry(blizzard.1).or_insert(vec!()).push(*blizzard));

    let start_y = map[0].iter().enumerate()
        .filter(|(_, c)| **c == '.')
        .map(|(i, _)| i)
        .min().unwrap() as i32;
    let end_y = map[map.len() - 1].iter().enumerate()
        .filter(|(_, c)| **c == '.')
        .map(|(i, _)| i)
        .max().unwrap() as i32;

    let min_1 = find_shortest(&map, x_max, y_max, &blizzards_per_row, &blizzards_per_col, (0, start_y), (x_max + 1, end_y), 0);
    println!("{}", min_1);

    let min_2 = find_shortest(&map, x_max, y_max, &blizzards_per_row, &blizzards_per_col, (x_max + 1, end_y), (0, start_y), min_1);
    let min_3 = find_shortest(&map, x_max, y_max, &blizzards_per_row, &blizzards_per_col, (0, start_y), (x_max + 1, end_y), min_2);
    println!("{}", min_3);
}

fn find_shortest(map: &Vec<Vec<char>>, x_max: i32, y_max: i32,
                 blizzards_per_row: &HashMap<i32, Vec<(i32, i32, char)>>,
                 blizzards_per_col: &HashMap<i32, Vec<(i32, i32, char)>>,
                 start: (i32, i32), end: (i32, i32), start_time: i32) -> i32 {
    let dirs = [(0, 0), (0, 1), (0, -1), (1, 0), (-1, 0)];

    let mut prio_queue = BinaryHeap::new();
    let mut visited = HashSet::new();
    prio_queue.push(Pos { x: start.0, y: start.1, time: start_time, heuristic: manhattan_dist(start, end) });

    let mut min = i32::MAX;
    while !prio_queue.is_empty() {
        let pos = prio_queue.pop().unwrap();
        if !visited.insert((pos.x, pos.y, pos.time)) { continue; }
        if pos.x == end.0 && pos.y == end.1 {
            if pos.time < min {
                min = pos.time
            }
            break; // Due to BFS nature, the first end reached will also be the minimal, so we can terminate
        }
        dirs.iter()
            .filter(|(dx, dy)| blizzards_per_row.get(&(pos.x + dx)).unwrap_or(&vec!()).iter()
                .all(|(bx, by, dir)| {
                    let new_blizzard = calc_new_blizzard(x_max, y_max, pos.time, bx, by, dir);
                    new_blizzard != (pos.x + *dx, pos.y + *dy)
                }))
            .filter(|(dx, dy)| blizzards_per_col.get(&(pos.y + dy)).unwrap_or(&vec!()).iter()
                .all(|(bx, by, dir)| {
                    let new_blizzard = calc_new_blizzard(x_max, y_max, pos.time, bx, by, dir);
                    new_blizzard != (pos.x + *dx, pos.y + *dy)
                }))
            .filter(|(dx, dy)| {
                pos.x + dx >= 0 && pos.x + dx < map.len() as i32
                    && pos.y + dy >= 0 && pos.y + dy < map[0].len() as i32
                    && map[(pos.x + dx) as usize][(pos.y + dy) as usize] != '#'
            })
            .for_each(|(dx, dy)| {
                let new_x = pos.x + dx;
                let new_y = pos.y + dy;
                let new_time = pos.time + 1;
                prio_queue.push(Pos { x: new_x, y: new_y, time: new_time, heuristic: manhattan_dist((new_x, new_y), end) + new_time });
            });
    }
    min - 1
}

fn manhattan_dist(from: (i32, i32), to: (i32, i32)) -> i32 {
    (from.0 - to.0).abs() + (from.1 - to.1).abs()
}

fn calc_new_blizzard(x_max: i32, y_max: i32, time: i32, bx: &i32, by: &i32, dir: &char) -> (i32, i32) {
    let new_blizzard = match dir {
        '<' => { (*bx, (*by - time - 1).rem_euclid(y_max) + 1) }
        '>' => { (*bx, (*by + time - 1).rem_euclid(y_max) + 1) }
        '^' => { ((*bx - time - 1).rem_euclid(x_max) + 1, *by) }
        'v' => { ((*bx + time - 1).rem_euclid(x_max) + 1, *by) }
        _ => panic!()
    };
    new_blizzard
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Pos {
    x: i32,
    y: i32,
    time: i32,
    heuristic: i32, // A-star
}

impl Ord for Pos {
    fn cmp(&self, other: &Self) -> Ordering {
        other.heuristic.cmp(&self.heuristic)
    }
}

impl PartialOrd for Pos {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod day24_tests {
    #[test]
    fn right_works() {
        let by = 1;
        let y_max = 5;
        assert_eq!(1, (by + 0 - 1 as i32).rem_euclid(y_max) + 1);
        assert_eq!(2, (by + 1 - 1 as i32).rem_euclid(y_max) + 1);
        assert_eq!(3, (by + 2 - 1 as i32).rem_euclid(y_max) + 1);
        assert_eq!(4, (by + 3 - 1 as i32).rem_euclid(y_max) + 1);
        assert_eq!(5, (by + 4 - 1 as i32).rem_euclid(y_max) + 1);
        assert_eq!(1, (by + 5 - 1 as i32).rem_euclid(y_max) + 1);
    }

    #[test]
    fn left_works() {
        let by = 1;
        let y_max = 5;
        assert_eq!(1, (by - 0 - 1 as i32).rem_euclid(y_max) + 1);
        assert_eq!(5, (by - 1 - 1 as i32).rem_euclid(y_max) + 1);
        assert_eq!(4, (by - 2 - 1 as i32).rem_euclid(y_max) + 1);
        assert_eq!(3, (by - 3 - 1 as i32).rem_euclid(y_max) + 1);
        assert_eq!(2, (by - 4 - 1 as i32).rem_euclid(y_max) + 1);
        assert_eq!(1, (by - 5 - 1 as i32).rem_euclid(y_max) + 1);
    }
}
