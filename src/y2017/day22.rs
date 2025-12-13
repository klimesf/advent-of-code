use std::collections::{HashMap, HashSet};
use std::fs;

pub(crate) fn day22() {
    let input = fs::read_to_string("input/2017/day22/input.txt").unwrap();
    let (start, map) = parse(input.as_str());
    println!("{}", virus(start, &map));
    println!("{}", evolved_virus(start, &map));
}

fn parse(input: &str) -> ((i32, i32), HashSet<(i32, i32)>) {
    let mut map = HashSet::new();
    let mut start = (0, 0);
    input.lines().enumerate().for_each(|(x, line)| {
        start = ((x / 2) as i32, (x / 2) as i32);
        line.chars().enumerate().for_each(|(y, c)| {
            if c == '#' {
                map.insert((x as i32, y as i32));
            }
        })
    });
    (start, map)
}

fn virus(start: (i32, i32), map: &HashSet<(i32, i32)>) -> usize {
    let mut dir = (-1, 0);
    let mut pos = start;
    let mut infected = map.clone();
    let mut ans = 0;

    for _ in 0..10000 {
        if infected.contains(&pos) {
            infected.remove(&pos);
            dir = turn_right(&dir);
        } else {
            infected.insert(pos);
            ans += 1;
            dir = turn_left(&dir);
        }
        pos = (pos.0 + dir.0, pos.1 + dir.1);
    }
    ans
}

enum Status {
    Infected,
    Weakened,
    Flagged,
}

fn evolved_virus(start: (i32, i32), map: &HashSet<(i32, i32)>) -> usize {
    let mut dir = (-1, 0);
    let mut pos = start;
    let mut statuses: HashMap<(i32, i32), Status> =
        map.iter().map(|pos| (*pos, Status::Infected)).collect();
    let mut ans = 0;

    for _ in 0..10000000 {
        match statuses.get(&pos) {
            Some(Status::Infected) => {
                statuses.insert(pos, Status::Flagged);
                dir = turn_right(&dir);
            }
            Some(Status::Weakened) => {
                statuses.insert(pos, Status::Infected);
                ans += 1;
            }
            Some(Status::Flagged) => {
                statuses.remove(&pos);
                dir = (dir.0 * -1, dir.1 * -1);
            }
            None => {
                statuses.insert(pos, Status::Weakened);
                dir = turn_left(&dir);
            }
        }
        pos = (pos.0 + dir.0, pos.1 + dir.1);
    }
    ans
}

fn turn_left(dir: &(i32, i32)) -> (i32, i32) {
    match dir {
        (-1, 0) => (0, -1),
        (0, -1) => (1, 0),
        (1, 0) => (0, 1),
        (0, 1) => (-1, 0),
        _ => panic!(),
    }
}

fn turn_right(dir: &(i32, i32)) -> (i32, i32) {
    match dir {
        (-1, 0) => (0, 1),
        (0, 1) => (1, 0),
        (1, 0) => (0, -1),
        (0, -1) => (-1, 0),
        _ => panic!(),
    }
}

#[cfg(test)]
mod day22_tests {
    use crate::y2017::day22::{evolved_virus, parse, virus};
    use std::fs;

    #[test]
    fn test_works() {
        let input = fs::read_to_string("input/2017/day22/test.txt").unwrap();
        let (start, map) = parse(input.as_str());
        assert_eq!(5587, virus(start, &map));
        assert_eq!(2511944, evolved_virus(start, &map));
    }

    #[test]
    fn input_works() {
        let input = fs::read_to_string("input/2017/day22/input.txt").unwrap();
        let (start, map) = parse(input.as_str());
        assert_eq!(5565, virus(start, &map));
        assert_eq!(2511978, evolved_virus(start, &map));
    }
}
