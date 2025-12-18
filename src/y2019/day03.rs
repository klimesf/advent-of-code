use std::collections::{HashMap, HashSet};
use std::fs;

pub(crate) fn day03() {
    let input = fs::read_to_string("input/2019/day03/input.txt").unwrap();
    let (wire_a, wire_b) = input.split_once('\n').unwrap();

    let points_a = get_points(wire_a.to_string());
    let points_b = get_points(wire_b.to_string());

    // Part A
    let points_a_set: HashSet<(i32, i32)> = points_a.keys().cloned().collect();
    let points_b_set: HashSet<(i32, i32)> = points_b.keys().cloned().collect();
    let intersections = points_a_set.intersection(&points_b_set);
    let closest_intersection = intersections.into_iter().map(|i| manhattan_dist(*i)).min();
    println!("{}", closest_intersection.unwrap_or(0));

    // Part B
    let points_a_set: HashSet<(i32, i32)> = points_a.keys().cloned().collect();
    let points_b_set: HashSet<(i32, i32)> = points_b.keys().cloned().collect();
    let intersections = points_a_set.intersection(&points_b_set);
    let closest_intersection_2 = intersections.into_iter().map(|i| points_a[i] + points_b[i]).min();
    println!("{}", closest_intersection_2.unwrap_or(0));
}

fn get_points(wire: String) -> HashMap<(i32, i32), i32> {
    let mut points = HashMap::new();
    let mut pos = (0, 0);
    let mut step = 0;

    for ins in wire.trim().split(',') {
        let direction = &ins[..1];
        let num: i32 = ins[1..].parse().unwrap();
        match direction {
            "R" => {
                for _ in 0..num {
                    pos.0 += 1;
                    step += 1;
                    points.insert(pos, step);
                }
            }
            "L" => {
                for _ in 0..num {
                    pos.0 -= 1;
                    step += 1;
                    points.insert(pos, step);
                }
            }
            "U" => {
                for _ in 0..num {
                    pos.1 += 1;
                    step += 1;
                    points.insert(pos, step);
                }
            }
            "D" => {
                for _ in 0..num {
                    pos.1 -= 1;
                    step += 1;
                    points.insert(pos, step);
                }
            }
            _ => {
                panic!("Unknown direction: {}", direction)
            }
        }
    }
    return points;
}

fn manhattan_dist(p: (i32, i32)) -> i32 {
    p.0.abs() + p.1.abs()
}
