use std::collections::HashMap;
use std::fs;

pub(crate) fn day17() {
    let input = fs::read_to_string("input/2022/day17/input.txt").unwrap();
    println!("{}", tetrisize(input.as_str(), 2022));
    println!("{}", tetrisize(input.as_str(), 1000000000000));
}

fn tetrisize(input: &str, rocks: usize) -> usize {
    let shapes: Vec<[u8; 4]> = vec![
        [0b00111100, 0b0, 0b0, 0b0],               // -
        [0b00010000, 0b00111000, 0b00010000, 0b0], // +
        [0b00111000, 0b00001000, 0b00001000, 0b0], // ⅃
        [0b00100000; 4],                           // |
        [0b00110000, 0b00110000, 0b0, 0b0],        // □
    ];

    let jets: Vec<char> = input.trim().chars().collect();
    let mut rows = vec![0b0; 8];
    let mut rock = 0;
    let mut jet_ctr = 0;
    let mut h = 3;
    let mut cache: HashMap<(usize, usize, [usize; 7]), (usize, usize)> = HashMap::new();
    let mut skipped = 0;

    while rock < rocks {
        let cache_key = (jet_ctr % jets.len(), rock % 5, birdseye_view(h, &rows));
        if cache.contains_key(&cache_key) {
            let (old_rock, old_height) = cache.get(&cache_key).unwrap();
            let repeat = (rocks - rock) / (rock - old_rock);
            rock += (rock - old_rock) * repeat;
            skipped += (h - old_height) * repeat;
        } else {
            cache.insert(cache_key, (rock, h));
        }

        let mut shape = shapes[rock % 5];
        loop {
            match jets[jet_ctr % jets.len()] {
                '<' => shift_left(h, &mut shape, &rows),
                '>' => shift_right(h, &mut shape, &rows),
                _ => panic!("Unknown jet"),
            }
            jet_ctr += 1;

            if h > 0 && can_move_down(h - 1, &shape, &rows) {
                h -= 1;
            } else {
                break;
            }
        }

        insert_shape(h, &shape, &mut rows);
        h = get_height(&rows) + 3;
        rock += 1;
    }

    // print(&rows);
    get_height(&rows) + skipped
}

#[allow(dead_code)]
fn print(rows: &Vec<u8>) {
    for i in (0..rows.len()).rev() {
        print!("|");
        let mut mask = 0b10000000;
        for _ in 0..7 {
            let c = if mask & rows[i] > 0 { '#' } else { '.' };
            print!("{}", c);
            mask = mask >> 1;
        }
        print!("|");
        println!();
    }
    println!("+-------+");
}

fn birdseye_view(h: usize, rows: &Vec<u8>) -> [usize; 7] {
    let mut ans = [rows.len(); 7];
    for i in (0..rows.len()).rev() {
        let mut mask = 0b10000000;
        for j in 0..7 {
            if mask & rows[i] > 0 && i < ans[j] {
                ans[j] = h - i
            };
            mask = mask >> 1;
        }
        if ans.iter().all(|x| *x < rows.len()) {
            break;
        }
    }
    return ans;
}

fn insert_shape(h: usize, shape: &[u8; 4], rows: &mut Vec<u8>) {
    for _ in rows.len()..(h + 4) {
        rows.push(0)
    }
    for i in 0..4 {
        rows[h + i] = rows[h + i] | shape[i];
    }
}

fn shift_left(h: usize, shape: &mut [u8; 4], rows: &Vec<u8>) {
    if (0..4).any(|i| {
        !shape[i] | !0b10000000 != 0b11111111 || (h + i < rows.len() && !(shape[i] << 1) | !rows[h + i] != 0b11111111)
    }) {
        return;
    }
    for i in 0..4 {
        shape[i] = shape[i] << 1;
    }
}

fn shift_right(h: usize, shape: &mut [u8; 4], rows: &Vec<u8>) {
    if (0..4).any(|i| {
        !shape[i] | !0b11 != 0b11111111 || (h + i < rows.len() && !(shape[i] >> 1) | !rows[h + i] != 0b11111111)
    }) {
        return;
    }
    for i in 0..4 {
        shape[i] = shape[i] >> 1;
    }
}

fn can_move_down(h: usize, shape: &[u8; 4], rows: &Vec<u8>) -> bool {
    (0..4).into_iter().all(|i| {
        if rows.len() <= h + i {
            true
        } else {
            !shape[i] | !rows[h + i] == 0b11111111
        }
    })
}

fn get_height(rows: &Vec<u8>) -> usize {
    let mut empty = 0;
    for i in (0..rows.len()).rev() {
        if rows[i] > 1 {
            break;
        }
        empty += 1;
    }
    rows.len() - empty
}

#[cfg(test)]
mod day17_tests {
    use crate::y2022::day17::{birdseye_view, can_move_down, get_height, shift_left, shift_right, tetrisize};

    #[test]
    fn shift_left_works() {
        let mut shape = [0b01000000; 4];
        let rows = vec![0; 4];

        shift_left(0, &mut shape, &rows); // This moves it left
        for i in 0..4 {
            assert_eq!(0b10000000, shape[i])
        }

        shift_left(0, &mut shape, &rows); // This does not, it's completely on the left already
        for i in 0..4 {
            assert_eq!(0b10000000, shape[i])
        }
    }

    #[test]
    fn shift_left_works_with_obstacle() {
        let mut shape = [0b0100_0000; 4];
        let rows = vec![0b1000_0000; 4];

        shift_left(3, &mut shape, &rows);
        for i in 0..4 {
            assert_eq!(0b01000000, shape[i])
        }
    }

    #[test]
    fn shift_right_works() {
        let mut shape = [0b100; 4];
        let rows = vec![0; 4];

        shift_right(0, &mut shape, &rows); // This moves it right
        for i in 0..4 {
            assert_eq!(0b10, shape[i])
        }

        shift_right(0, &mut shape, &rows); // This does not, it's completely on the right already
        for i in 0..4 {
            assert_eq!(0b10, shape[i])
        }
    }

    #[test]
    fn shift_right_works_2() {
        let mut shape = [0b0111_1000; 4];
        let rows = vec![0; 4];

        shift_right(0, &mut shape, &rows);
        for i in 0..4 {
            assert_eq!(0b0011_1100, shape[i])
        }

        shift_right(0, &mut shape, &rows);
        for i in 0..4 {
            assert_eq!(0b0001_1110, shape[i])
        }

        shift_right(0, &mut shape, &rows);
        for i in 0..4 {
            assert_eq!(0b0001_1110, shape[i])
        }
    }

    #[test]
    fn shift_right_works_with_obstacle() {
        let mut shape = [0b100; 4];
        let rows = vec![0b10; 4];

        shift_right(3, &mut shape, &rows);
        for i in 0..4 {
            assert_eq!(0b100, shape[i])
        }
    }

    #[test]
    fn can_move_down_works() {
        let shape: [u8; 4] = [0b00100000; 4];
        let rows = vec![0b11100000, 0b00000000];

        assert_eq!(true, can_move_down(1, &shape, &rows));
        assert_eq!(false, can_move_down(0, &shape, &rows));
    }

    #[test]
    fn get_height_works() {
        assert_eq!(0, get_height(&vec![0b0; 8]));
        assert_eq!(0, get_height(&vec![0b1; 8])); // Rightmost bit is disregarded
        assert_eq!(2, get_height(&vec![0b10000, 0b11110000, 0b00000000, 0b00000000, 0b00000000]));
        assert_eq!(5, get_height(&vec![0b10000, 0b11110000, 0b00000000, 0b00000000, 0b01000000]));
    }

    #[test]
    fn birdseye_view_works() {
        let rows = vec![0b11111110, 0b11111110, 0b00010000, 0b10000010, 0b01000100, 0b00101000];
        assert_eq!([2, 1, 0, 3, 0, 1, 2], birdseye_view(5, &rows));
    }

    #[test]
    fn part_a_works() {
        assert_eq!(3068, tetrisize(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>", 2022));
    }

    #[test]
    fn part_b_works() {
        assert_eq!(1514285714288, tetrisize(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>", 1000000000000));
    }
}
