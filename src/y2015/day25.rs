pub(crate) fn day25() {
    println!("{}", part_a(3010, 3019));
}

fn part_a(row: usize, col: usize) -> i64 {
    let mut paper = vec![vec! { 0; col.max(row) * 2 }; col.max(row) * 2];

    let mut i: i64 = 20151125;
    for diagonal in 0..paper.len() {
        let mut pos = (diagonal, 0);
        loop {
            paper[pos.0][pos.1] = i;
            i = (i * 252533).rem_euclid(33554393);
            if pos.0 == 0 {
                break;
            }
            pos = (pos.0 - 1, pos.1 + 1);
        }
    }

    paper[row - 1][col - 1]
}

#[cfg(test)]
mod day25_tests {
    use crate::y2015::day25::part_a;

    #[test]
    fn test_works() {
        assert_eq!(20151125, part_a(1, 1));
        assert_eq!(33511524, part_a(1, 6));
        assert_eq!(33071741, part_a(6, 1));
        assert_eq!(27995004, part_a(6, 6));
    }

    #[test]
    fn input_works() {
        assert_eq!(8997277, part_a(3010, 3019));
    }
}
