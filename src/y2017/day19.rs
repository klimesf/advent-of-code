use std::collections::HashMap;
use std::fs;

pub(crate) fn day19() {
    let input = fs::read_to_string("input/2017/day19/input.txt").unwrap();
    let (part_a, part_b) = route(input.as_str());
    println!("{}", part_a);
    println!("{}", part_b);
}

fn route(input: &str) -> (String, i32) {
    let map: HashMap<(i32, i32), char> = input
        .lines()
        .enumerate()
        .flat_map(|(x, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c != ' ')
                .map(|(y, c)| ((x as i32, y as i32), c))
                .collect::<Vec<((i32, i32), char)>>()
        })
        .collect();
    let mut pos: (i32, i32) = *map.keys().find(|(x, _)| *x == 0).unwrap();
    let mut ans = String::new();
    let mut steps = 0;
    let mut dir = (1, 0);

    loop {
        match map.get(&pos) {
            Some('|') | Some('-') => {} // Continue
            Some('+') => {
                // Find new dir
                let lookup = if dir.0 != 0 {
                    [(0, 1), (0, -1)]
                } else {
                    [(1, 0), (-1, 0)]
                };
                let new_dir = lookup
                    .iter()
                    .find(|(dx, dy)| map.get(&(pos.0 + dx, pos.1 + dy)).is_some());
                match new_dir {
                    Some(n) => dir = *n,
                    None => {
                        panic!("No new dir found at {:?}", pos)
                    }
                }
            }
            Some(c) => {
                ans.push(*c);
            }
            None => {
                return (ans, steps);
            } // Nowhere else to go
        }
        pos = (pos.0 + dir.0, pos.1 + dir.1);
        steps += 1;
    }
}

#[cfg(test)]
mod day19_tests {
    use std::fs;

    use crate::y2017::day19::route;

    #[test]
    fn test_works() {
        let input = fs::read_to_string("input/2017/day19/test.txt").unwrap();
        let (part_a, part_b) = route(input.as_str());
        assert_eq!("ABCDEF", part_a);
        assert_eq!(38, part_b);
    }

    #[test]
    fn input_works() {
        let input = fs::read_to_string("input/2017/day19/input.txt").unwrap();
        let (part_a, part_b) = route(input.as_str());
        assert_eq!("XYFDJNRCQA", part_a);
        assert_eq!(17450, part_b);
    }
}
