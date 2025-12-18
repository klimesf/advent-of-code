use std::fs;

pub fn day09(print: fn(usize)) {
    print(part_a(fs::read_to_string("input/2025/day09/input.txt").unwrap()));
    print(part_b(fs::read_to_string("input/2025/day09/input.txt").unwrap()));
}

fn part_a(input: String) -> usize {
    let dots: Vec<(usize, usize)> = input
        .lines()
        .map(|line| {
            let (l, r) = line.split_once(',').unwrap();
            (l.parse::<usize>().unwrap(), r.parse::<usize>().unwrap())
        })
        .collect();

    let mut max = 0;
    for i in 0..dots.len() {
        let a = dots[i];
        for j in 0..dots.len() {
            let b = dots[j];

            let top_left = (a.0.min(b.0), a.1.min(b.1));
            let top_right = (a.0.min(b.0), a.1.max(b.1));
            let bottom_left = (a.0.max(b.0), a.1.min(b.1));

            let len_top = top_right.1 - top_left.1;
            let len_bot = bottom_left.0 - top_left.0;

            let size = (len_top + 1) * (len_bot + 1);

            if size > max {
                max = size;
            }
        }
    }

    max
}

fn part_b(input: String) -> usize {
    let red_tiles: Vec<(usize, usize)> = input
        .lines()
        .map(|line| {
            let (l, r) = line.split_once(',').unwrap();
            (l.parse::<usize>().unwrap(), r.parse::<usize>().unwrap())
        })
        .collect();

    let mut edges = Vec::<((usize, usize), (usize, usize))>::new();
    for i in 0..red_tiles.len() {
        let a = red_tiles[i];
        let b = red_tiles[(i + 1) % red_tiles.len()];

        let y1 = a.0.min(b.0);
        let y2 = a.0.max(b.0);
        let x1 = a.1.min(b.1);
        let x2 = a.1.max(b.1);

        edges.push(((y1, x1), (y2, x2)));
    }

    let mut max = 0;
    for i in 0..red_tiles.len() {
        let a = red_tiles[i];
        for j in i + 1..red_tiles.len() {
            let b = red_tiles[j];

            let left = a.0.min(b.0);
            let right = a.0.max(b.0);
            let top = a.1.min(b.1);
            let bottom = a.1.max(b.1);
            let area = (right - left + 1) * (bottom - top + 1);

            if area <= max {
                continue;
            }

            let mut intersects = false;

            for k in 0..edges.len() {
                let ((y1, x1), (y2, x2)) = edges[k];

                if !(right <= y1 || left >= y2 || bottom <= x1 || top >= x2) {
                    intersects = true;
                    break;
                }
            }

            if !intersects && area > max {
                max = area;
            }
        }
    }
    max
}

#[cfg(test)]
mod day09_tests {
    use std::fs;

    use crate::y2025::day09::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(50, part_a(fs::read_to_string("input/2025/day09/test.txt").unwrap()));
        assert_eq!(24, part_b(fs::read_to_string("input/2025/day09/test.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!(4782151432, part_a(fs::read_to_string("input/2025/day09/input.txt").unwrap()));
        assert_eq!(1450414119, part_b(fs::read_to_string("input/2025/day09/input.txt").unwrap()));
    }
}
