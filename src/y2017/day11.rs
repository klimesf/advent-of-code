use std::fs;

pub(crate) fn day11() {
    let input = fs::read_to_string("input/2017/day11/input.txt").unwrap();
    let (final_dist, max_dist) = walk_hexagonal(input.trim());
    println!("{}", final_dist);
    println!("{}", max_dist);
}

fn walk_hexagonal(input: &str) -> (i32, i32) {
    let mut pos = (0, 0, 0);
    let mut max = 0;

    input.split(",").into_iter().for_each(|dir| {
        match dir {
            // Cube coordinates (see https://www.redblobgames.com/grids/hexagons/)
            "n" => pos = (pos.0, pos.1 + 1, pos.2 - 1),
            "ne" => pos = (pos.0 + 1, pos.1 + 1, pos.2),
            "nw" => pos = (pos.0 - 1, pos.1, pos.2 - 1),
            "s" => pos = (pos.0, pos.1 - 1, pos.2 + 1),
            "se" => pos = (pos.0 + 1, pos.1, pos.2 + 1),
            "sw" => pos = (pos.0 - 1, pos.1 - 1, pos.2),
            _ => panic!("Unknown dir {}", dir),
        }
        max = max.max(distance(pos));
    });

    (distance(pos), max)
}

fn distance(pos: (i32, i32, i32)) -> i32 {
    (pos.0.abs() + pos.1.abs() + pos.2.abs()) / 2
}

#[cfg(test)]
mod day11_tests {
    use crate::y2017::day11::walk_hexagonal;
    use std::fs;

    #[test]
    fn part_a_works() {
        assert_eq!(3, walk_hexagonal("ne,ne,ne").0);
        assert_eq!(3, walk_hexagonal("n,n,n").0);
        assert_eq!(3, walk_hexagonal("nw,nw,nw").0);
        assert_eq!(0, walk_hexagonal("ne,ne,sw,sw").0);
        assert_eq!(2, walk_hexagonal("ne,ne,s,s").0);
        assert_eq!(3, walk_hexagonal("se,sw,se,sw,sw").0);
    }

    #[test]
    fn input_works() {
        let input = fs::read_to_string("input/2017/day11/input.txt").unwrap();
        assert_eq!((812, 1603), walk_hexagonal(input.trim()));
    }
}
