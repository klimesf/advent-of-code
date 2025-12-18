use std::fs;

pub(crate) fn day02() {
    println!("{}", part_a(fs::read_to_string("input/2015/day02/input.txt").unwrap()));
    println!("{}", part_b(fs::read_to_string("input/2015/day02/input.txt").unwrap()));
}

fn part_a(input: String) -> usize {
    input
        .lines()
        .map(|line| {
            let dims: Vec<usize> = line.splitn(3, "x").map(|n| n.parse().unwrap()).collect();
            let area = 2 * dims[0] * dims[1] + 2 * dims[1] * dims[2] + 2 * dims[0] * dims[2];
            let smallest_side = (dims[0] * dims[1]).min(dims[1] * dims[2]).min(dims[0] * dims[2]);
            area + smallest_side
        })
        .sum()
}

fn part_b(input: String) -> usize {
    input
        .lines()
        .map(|line| {
            let mut dims: Vec<usize> = line.splitn(3, "x").map(|n| n.parse().unwrap()).collect();
            dims.sort();
            let ribbon = 2 * dims[0] + 2 * dims[1];
            let bow = dims[0] * dims[1] * dims[2];
            ribbon + bow
        })
        .sum()
}

#[cfg(test)]
mod day02_tests {
    use std::fs;

    use crate::y2015::day02::{part_a, part_b};

    #[test]
    fn input_works() {
        assert_eq!(1586300, part_a(fs::read_to_string("input/2015/day02/input.txt").unwrap()));
        assert_eq!(3737498, part_b(fs::read_to_string("input/2015/day02/input.txt").unwrap()));
    }
}
