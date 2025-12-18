use std::collections::{HashMap, HashSet};
use std::fs;

pub(crate) fn day17() {
    println!("{}", part_a(fs::read_to_string("input/2018/day17/input.txt").unwrap()));
    println!("{}", part_b(fs::read_to_string("input/2018/day17/input.txt").unwrap()));
}

fn part_a(input: String) -> usize {
    let (min_y, max_y, clay) = parse_input(input);
    let mut water = HashMap::new();
    rec_floodfil(500, 0, max_y, &mut water, &clay);
    water.keys().filter(|(_, y)| *y >= min_y && *y <= max_y).count()
}

fn part_b(input: String) -> usize {
    let (_, max_y, clay) = parse_input(input);
    let mut water = HashMap::new();
    rec_floodfil(500, 0, max_y, &mut water, &clay);
    water.iter().filter(|(_, v)| !**v).count()
}

fn parse_input(input: String) -> (i32, i32, HashSet<(i32, i32)>) {
    let mut min_y = i32::MAX;
    let mut max_y = 0;
    let mut clay = HashSet::new();
    input.lines().for_each(|line| {
        let (l, r) = line.split_once(", ").unwrap();
        let (ll, lr) = l.split_once("=").unwrap();
        let (_, rr) = r.split_once("=").unwrap();
        let (from_s, to_s) = rr.split_once("..").unwrap();

        if ll == "x" {
            let x = lr.parse::<i32>().unwrap();
            let y_from = from_s.parse::<i32>().unwrap();
            let y_to = to_s.parse::<i32>().unwrap();

            min_y = min_y.min(y_from);
            max_y = max_y.max(y_to);

            for y in y_from..=y_to {
                clay.insert((x, y));
            }
        } else {
            let y = lr.parse::<i32>().unwrap();
            let x_from = from_s.parse::<i32>().unwrap();
            let x_to = to_s.parse::<i32>().unwrap();

            min_y = min_y.min(y);
            max_y = max_y.max(y);

            for x in x_from..=x_to {
                clay.insert((x, y));
            }
        }
    });
    (min_y, max_y, clay)
}

fn rec_floodfil(x: i32, y: i32, max_y: i32, water: &mut HashMap<(i32, i32), bool>, clay: &HashSet<(i32, i32)>) -> bool {
    if y > max_y {
        return true;
    }
    if let Some(prev) = water.get(&(x, y)) {
        return *prev;
    }

    if !clay.contains(&(x, y + 1)) {
        let og_dripping = rec_floodfil(x, y + 1, max_y, water, clay);
        let mut dripping_right = og_dripping;
        let mut dripping_left = og_dripping;
        water.insert((x, y), og_dripping);

        if !og_dripping
            && (*water.get(&(x - 1, y + 1)).unwrap_or(&true) == false || clay.contains(&(x - 1, y + 1)))
            && !clay.contains(&(x - 1, y))
        {
            if rec_floodfil(x - 1, y, max_y, water, clay) {
                dripping_right = true;
            }
        }

        if !og_dripping
            && (*water.get(&(x + 1, y + 1)).unwrap_or(&true) == false || clay.contains(&(x + 1, y + 1)))
            && !clay.contains(&(x + 1, y))
        {
            if rec_floodfil(x + 1, y, max_y, water, clay) {
                dripping_left = true;
            }
        }

        if dripping_right {
            // Correct the dripping level if one side is dripping
            let mut pos = (x, y);
            while !clay.contains(&pos) && water.contains_key(&pos) {
                water.insert(pos, true);
                pos = (pos.0 - 1, pos.1);
            }
        }
        if dripping_left {
            // Correct the dripping level if one side is dripping
            let mut pos = (x, y);
            while !clay.contains(&pos) && water.contains_key(&pos) {
                water.insert(pos, true);
                pos = (pos.0 + 1, pos.1);
            }
        }

        water.insert((x, y), dripping_left || dripping_right);
        return dripping_left || dripping_right;
    } else {
        water.insert((x, y), false);
        let mut dripping = false;
        if !clay.contains(&(x + 1, y)) {
            if rec_floodfil(x + 1, y, max_y, water, clay) {
                dripping = true;
            }
        }
        if !clay.contains(&(x - 1, y)) {
            if rec_floodfil(x - 1, y, max_y, water, clay) {
                dripping = true;
            }
        }

        water.insert((x, y), dripping);
        return dripping || dripping;
    }
}

#[cfg(test)]
mod day17_tests {
    use std::fs;

    use crate::y2018::day17::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(57, part_a(fs::read_to_string("input/2018/day17/test.txt").unwrap()));
        assert_eq!(29, part_b(fs::read_to_string("input/2018/day17/test.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!(31641, part_a(fs::read_to_string("input/2018/day17/input.txt").unwrap()));
        assert_eq!(26321, part_b(fs::read_to_string("input/2018/day17/input.txt").unwrap()));
    }
}
