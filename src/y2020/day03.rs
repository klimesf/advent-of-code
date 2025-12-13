use crate::utils::grid::Grid;
use std::fs;

pub(crate) fn day03() {
    println!(
        "{}",
        part_a(fs::read_to_string("input/2020/day03/input.txt").unwrap())
    );
    println!(
        "{}",
        part_b(fs::read_to_string("input/2020/day03/input.txt").unwrap())
    );
}

fn part_a(input: String) -> usize {
    let grid = Grid::parse(&input.trim());
    let mut pos = (0, 0);
    let inc = (1, 3);

    let mut ans = 0;
    while pos.0 < grid.x_len {
        if grid[pos] == b'#' {
            ans += 1;
        }
        pos = (pos.0 + inc.0, (pos.1 + inc.1) % grid.y_len);
    }
    ans
}

fn part_b(input: String) -> usize {
    let grid = Grid::parse(&input.trim());

    let mut total = 1;
    for inc in [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)] {
        let mut pos = (0, 0);
        let mut ans = 0;
        while pos.0 < grid.x_len {
            if grid[pos] == b'#' {
                ans += 1;
            }
            pos = (pos.0 + inc.0, (pos.1 + inc.1) % grid.y_len);
        }
        total *= ans;
    }
    total
}

#[cfg(test)]
mod day03_tests {
    use std::fs;

    use crate::y2020::day03::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(
            7,
            part_a(fs::read_to_string("input/2020/day03/test.txt").unwrap())
        );
        assert_eq!(
            336,
            part_b(fs::read_to_string("input/2020/day03/test.txt").unwrap())
        );
    }

    #[test]
    fn input_works() {
        assert_eq!(
            278,
            part_a(fs::read_to_string("input/2020/day03/input.txt").unwrap())
        );
        assert_eq!(
            9709761600,
            part_b(fs::read_to_string("input/2020/day03/input.txt").unwrap())
        );
    }
}
