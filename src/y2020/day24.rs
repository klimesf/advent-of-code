use std::collections::{HashMap, HashSet};
use std::fs;

pub(crate) fn day24() {
    let ans = solve(fs::read_to_string("input/2020/day24/input.txt").unwrap());
    println!("{}", ans.0);
    println!("{}", ans.1);
}

fn solve(input: String) -> (usize, usize) {
    let mut black_tiles: HashSet<(i32, i32)> = HashSet::new();
    input.lines().for_each(|line| {
        let chars = line.chars().collect::<Vec<char>>();
        let mut pos = (0, 0);
        let mut buf = '_';
        for i in 0..chars.len() {
            let c = chars[i];
            if c == 'n' || c == 's' {
                buf = c;
                continue;
            }

            //    (0, 0)    (0, 1)    (0, 2) ...
            // (1, 0)  (1, 1)   (1, 2)  (1, 3) ...
            //    (2, 0)    (2, 1)    (2, 2) ...
            match (buf, c) {
                ('n', 'e') => {
                    pos = (pos.0 - 1, if pos.0 % 2 == 0 { pos.1 + 1 } else { pos.1 });
                }
                ('n', 'w') => {
                    pos = (pos.0 - 1, if pos.0 % 2 == 0 { pos.1 } else { pos.1 - 1 });
                }
                ('s', 'e') => {
                    pos = (pos.0 + 1, if pos.0 % 2 == 0 { pos.1 + 1 } else { pos.1 });
                }
                ('s', 'w') => {
                    pos = (pos.0 + 1, if pos.0 % 2 == 0 { pos.1 } else { pos.1 - 1 });
                }
                ('_', 'e') => {
                    pos = (pos.0, pos.1 + 1);
                }
                ('_', 'w') => {
                    pos = (pos.0, pos.1 - 1);
                }
                _ => panic!("unknown dir {}{}", buf, c),
            }
            buf = '_'; // reset
        }

        if black_tiles.contains(&pos) {
            black_tiles.remove(&pos);
        } else {
            black_tiles.insert(pos);
        }
    });

    let part_a = black_tiles.len();

    for _ in 0..100 {
        let mut adjacent_tile_count: HashMap<(i32, i32), usize> = HashMap::new();
        for tile in &black_tiles {
            for pos in [
                (tile.0 - 1, if tile.0 % 2 == 0 { tile.1 + 1 } else { tile.1 }), // NE
                (tile.0 - 1, if tile.0 % 2 == 0 { tile.1 } else { tile.1 - 1 }), // NW
                (tile.0 + 1, if tile.0 % 2 == 0 { tile.1 + 1 } else { tile.1 }), // SE
                (tile.0 + 1, if tile.0 % 2 == 0 { tile.1 } else { tile.1 - 1 }), // SW
                (tile.0, tile.1 + 1),                                            // E
                (tile.0, tile.1 - 1),                                            // W
            ] {
                *adjacent_tile_count.entry(pos).or_insert(0) += 1;
            }
        }

        let mut new_black_tiles: HashSet<(i32, i32)> = HashSet::new();
        for (pos, count) in adjacent_tile_count {
            if black_tiles.contains(&pos) {
                if count == 1 || count == 2 {
                    new_black_tiles.insert(pos);
                }
            } else {
                if count == 2 {
                    new_black_tiles.insert(pos);
                }
            }
        }
        black_tiles = new_black_tiles;
    }

    (part_a, black_tiles.len())
}

#[cfg(test)]
mod day24_tests {
    use std::fs;

    use crate::y2020::day24::solve;

    #[test]
    fn test_works() {
        assert_eq!((10, 2208), solve(fs::read_to_string("input/2020/day24/test.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!((382, 3964), solve(fs::read_to_string("input/2020/day24/input.txt").unwrap()));
    }
}
