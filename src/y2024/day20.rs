use std::fs;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use crate::utils::grid::{Grid, DOWN, LEFT, P, RIGHT, UP};

pub fn day20(print: fn(usize)) {
    let (part_a, part_b) = solve(fs::read_to_string("input/2024/day20/input.txt").unwrap(), 100);
    print(part_a);
    print(part_b);
}

fn solve(input: String, target_saving: usize) -> (usize, usize) {
    let map = Grid::parse(input.trim());

    let start = map.find_first(b'E').unwrap();

    // Run DFS to find the path from E -> S (there is only one).
    // Plus, mark the distance to E from each point
    let mut distances = Grid::new_usize(map.x_len, map.y_len, usize::MAX);
    let mut stack = vec!();
    let mut path = vec!();
    stack.push((start, 0));

    while let Some((pos, dist)) = stack.pop() {
        if map[pos] == b'#' { continue; }
        if distances[pos] < dist { continue; }
        path.push(pos);
        distances[pos] = dist;
        if map[pos] == b'S' { break; }

        // No need to check boundaries, the map is bordered by #
        stack.push((pos + UP, dist + 1));
        stack.push((pos + DOWN, dist + 1));
        stack.push((pos + LEFT, dist + 1));
        stack.push((pos + RIGHT, dist + 1));
    }

    // Go through each point in the path and try to find shortcuts within cheat distance
    path.par_iter().map(|p| {
        let mut ans_a = 0;
        let mut ans_b = 0;
        for x in -20..=20_i32 {
            for y in -20..=20_i32 {
                if x == 0 && y == 0 { continue; }
                let cheat_len = x.abs() + y.abs();
                if cheat_len > 20 { continue; }
                if distances[*p] < target_saving { continue; } // No point in cheating if we are close to the E
                let jump = P { x: p.x + x, y: p.y + y };
                if !map.contains(&jump) { continue; }
                if distances[*p] <= distances[jump] { continue; } // No point in cheating to a point that is furter from E
                let saving = distances[*p] - distances[jump] - p.manhattan_distance(&jump) as usize;
                if saving >= target_saving {
                    if cheat_len == 2 { ans_a += 1 }
                    ans_b += 1;
                }
            }
        }
        (ans_a, ans_b)
    }).reduce(|| (0, 0), |a, b| (a.0 + b.0, a.1 + b.1))
}

#[cfg(test)]
mod day20_tests {
    use std::fs;

    use crate::y2024::day20::{solve};

    #[test]
    fn test_works() {
        assert_eq!((5, 1449), solve(fs::read_to_string("input/2024/day20/test.txt").unwrap(), 20));
    }

    #[test]
    fn input_works() {
        assert_eq!((1409, 1012821), solve(fs::read_to_string("input/2024/day20/input.txt").unwrap(), 100));
    }
}
