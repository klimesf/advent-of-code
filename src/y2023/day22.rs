use std::collections::{HashMap, HashSet};
use std::fs;

pub(crate) fn day22() {
    println!("{}", part_a(fs::read_to_string("input/2023/day22/input.txt").unwrap()));
    println!("{}", part_b(fs::read_to_string("input/2023/day22/input.txt").unwrap()));
}

fn part_a(input: String) -> usize {
    let mut bricks: Vec<((i32, i32, i32), (i32, i32, i32))> = input
        .lines()
        .map(|line| {
            let (l, r) = line.split_once("~").unwrap();
            let start: Vec<i32> = l.split(",").map(|s| s.parse::<i32>().unwrap()).collect();
            let end: Vec<i32> = r.split(",").map(|s| s.parse::<i32>().unwrap()).collect();
            ((start[0], start[1], start[2]), (end[0], end[1], end[2]))
        })
        .collect();
    bricks.sort_by(|a, b| {
        if a.0 .2 == b.0 .2 {
            a.1 .2.cmp(&b.1 .2)
        } else {
            a.0 .2.cmp(&b.0 .2)
        }
    });

    let (settled_bricks, _, _) = settle(&bricks);

    let mut ans = 0;
    for _i in 0..settled_bricks.len() {
        let bricks_without_i: Vec<((i32, i32, i32), (i32, i32, i32))> = settled_bricks
            .iter()
            .enumerate()
            .filter(|(j, _)| *j != _i)
            .map(|(_, b)| *b)
            .collect();
        let (_, changes, _) = settle(&bricks_without_i);
        if changes == 0 {
            ans += 1
        }
    }
    ans
}

fn part_b(input: String) -> usize {
    let mut bricks: Vec<((i32, i32, i32), (i32, i32, i32))> = input
        .lines()
        .map(|line| {
            let (l, r) = line.split_once("~").unwrap();
            let start: Vec<i32> = l.split(",").map(|s| s.parse::<i32>().unwrap()).collect();
            let end: Vec<i32> = r.split(",").map(|s| s.parse::<i32>().unwrap()).collect();
            ((start[0], start[1], start[2]), (end[0], end[1], end[2]))
        })
        .collect();
    bricks.sort_by(|a, b| {
        if a.0 .2 == b.0 .2 {
            a.1 .2.cmp(&b.1 .2)
        } else {
            a.0 .2.cmp(&b.0 .2)
        }
    });

    let (settled_bricks, _, _) = settle(&bricks);

    let mut ans = 0;
    for _i in 0..settled_bricks.len() {
        let bricks_without_i: Vec<((i32, i32, i32), (i32, i32, i32))> = settled_bricks
            .iter()
            .enumerate()
            .filter(|(j, _)| *j != _i)
            .map(|(_, b)| *b)
            .collect();
        let (_, _, falls) = settle(&bricks_without_i);
        ans += falls
    }
    ans
}

fn settle(
    bricks_input: &Vec<((i32, i32, i32), (i32, i32, i32))>,
) -> (Vec<((i32, i32, i32), (i32, i32, i32))>, usize, usize) {
    let mut collision_space: HashMap<(i32, i32, i32), usize> = HashMap::new();
    let mut collision_space_index: HashMap<usize, Vec<(i32, i32, i32)>> = HashMap::new();
    let mut bricks = bricks_input.clone();
    for bi in 0..bricks.len() {
        let ((x1, y1, z1), (x2, y2, z2)) = bricks[bi];
        let mut s = vec![];
        for x in x1..=x2 {
            for y in y1..=y2 {
                for z in z1..=z2 {
                    collision_space.insert((x, y, z), bi);
                    s.push((x, y, z));
                }
            }
        }
        collision_space_index.insert(bi, s);
    }

    let mut total_changes = 0;
    let mut falls = HashSet::new();
    loop {
        let mut changes = 0;
        let mut new_bricks = vec![];
        for bi in 0..bricks.len() {
            let ((x1, y1, z1), (x2, y2, z2)) = bricks[bi];

            let mut z_fall = 0;
            for dz in 1..z1 {
                let mut collision = false;
                'outer: for x in x1..=x2 {
                    for y in y1..=y2 {
                        for z in (z1 - dz)..=(z2 - dz) {
                            if *collision_space.get(&(x, y, z)).unwrap_or(&bi) != bi {
                                collision = true;
                                break 'outer;
                            }
                        }
                    }
                }
                if !collision {
                    z_fall = dz
                } else {
                    break;
                };
            }

            if z_fall == 0 {
                new_bricks.push(((x1, y1, z1), (x2, y2, z2)));
                continue;
            }

            falls.insert(bi);
            changes += 1;
            for x in x1..=x2 {
                for y in y1..=y2 {
                    for z in z1..=z2 {
                        collision_space.remove(&(x, y, z));
                    }
                    for z in (z1 - z_fall)..=(z2 - z_fall) {
                        collision_space.insert((x, y, z), bi);
                    }
                }
            }

            new_bricks.push(((x1, y1, z1 - z_fall), (x2, y2, z2 - z_fall)));
        }
        total_changes += changes;
        if changes == 0 {
            break;
        }
        bricks = new_bricks;
    }

    (bricks.clone(), total_changes, falls.len())
}

#[cfg(test)]
mod day22_tests {
    use std::fs;

    use crate::y2023::day22::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(5, part_a(fs::read_to_string("input/2023/day22/test.txt").unwrap()));
        assert_eq!(7, part_b(fs::read_to_string("input/2023/day22/test.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!(527, part_a(fs::read_to_string("input/2023/day22/input.txt").unwrap()));
        assert_eq!(100376, part_b(fs::read_to_string("input/2023/day22/input.txt").unwrap()));
    }
}
