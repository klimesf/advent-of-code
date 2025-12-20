use adventofcode::utils::grid::{Grid, DOWN, LEFT, P, RIGHT, UP};
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

pub(crate) fn day20() {
    let (a, b) = solve(fs::read_to_string("input/2020/day20/input.txt").unwrap());
    println!("{}", a);
    println!("{}", b);
}

fn solve(input: String) -> (usize, usize) {
    let tiles: HashMap<usize, Grid<u8>> = input
        .split("\n\n")
        .map(|tile| {
            let lines = tile.lines().collect::<Vec<&str>>();
            let num: usize = lines[0].chars().filter(|c| c.is_numeric()).join("").parse().unwrap();
            let grid = Grid::parse(lines[1..].join("\n").as_str());
            (num, grid)
        })
        .collect();
    let side_len = tiles.len().isqrt() as i32;

    let map = build_map(&tiles);

    let min_x = map.keys().map(|p| p.x).min().unwrap();
    let min_y = map.keys().map(|p| p.y).min().unwrap();
    let top_left = P::new(min_x, min_y);
    let top_right = P::new(min_x, min_y + side_len - 1);
    let bot_left = P::new(min_x + side_len - 1, min_y);
    let bot_right = P::new(min_x + side_len - 1, min_y + side_len - 1);
    let ans_a = map[&top_left].1 * map[&top_right].1 * map[&bot_left].1 * map[&bot_right].1;

    let mut image = Grid::new(side_len * 8, side_len * 8, b'_');
    for pos_x in 0..side_len {
        for pos_y in 0..side_len {
            let pos = P::new(min_x + pos_x, min_y + pos_y);
            let (tile, _) = &map[&pos];
            for x in 1..9 {
                for y in 1..9 {
                    image[P::new(x - 1 + pos_x * 8, y - 1 + pos_y * 8)] = tile[P::new(x, y)]
                }
            }
        }
    }

    let monster = Grid::parse("                  # \n#    ##    ##    ###\n #  #  #  #  #  #   ");
    for i in 0..8 {
        let mut match_ctr = 0;
        for x in 0..image.x_len - monster.x_len {
            'outer: for y in 0..image.y_len - monster.y_len {
                let mut matched = vec![];
                for dx in 0..monster.x_len {
                    for dy in 0..monster.y_len {
                        match monster[(dx, dy)] {
                            b'#' => {
                                if image[(x + dx, y + dy)] != b'#' {
                                    continue 'outer;
                                }
                                matched.push((x + dx, y + dy));
                            }
                            _ => {}
                        }
                    }
                }

                match_ctr += 1;
                for (dx, dy) in matched {
                    image[(dx, dy)] = b'O';
                }
            }
        }

        if match_ctr > 0 {
            break;
        }

        image.rotate_left();
        if i == 3 {
            image.flip_x();
        }
    }

    let mut ans_b = 0;
    for x in 0..image.x_len {
        for y in 0..image.y_len {
            if image[(x, y)] == b'#' {
                ans_b += 1;
            }
        }
    }

    (ans_a, ans_b)
}

fn build_map(tiles: &HashMap<usize, Grid<u8>>) -> HashMap<P, (Grid<u8>, usize)> {
    // Build the whole image starting from a random square tile.
    // Try to add a tile in each direction, taking from the pool of tiles that are not yet placed.
    // The good thing is each edge matches exactly zero or one other tile, not more.
    let mut queue = VecDeque::new();
    queue.push_back((P::new(0, 0), UP));
    queue.push_back((P::new(0, 0), DOWN));
    queue.push_back((P::new(0, 0), LEFT));
    queue.push_back((P::new(0, 0), RIGHT));

    let start_square_tile = *tiles.keys().last().unwrap();
    let mut available_tiles = tiles.keys().clone().into_iter().map(|key| *key).collect::<HashSet<_>>();
    available_tiles.remove(&start_square_tile);

    let mut map: HashMap<P, (Grid<u8>, usize)> = HashMap::new();
    map.insert(P::new(0, 0), (tiles[&start_square_tile].clone(), start_square_tile));

    while let Some((pos, dir)) = queue.pop_front() {
        if available_tiles.len() < 1 {
            continue;
        }
        if map.contains_key(&(pos + dir)) {
            continue;
        }
        let (tile, _) = map[&pos].clone();

        'outer: for other_tile_id in available_tiles.clone() {
            let mut other_tile = tiles[&other_tile_id].clone();
            for i in 0..8 {
                let matches = match dir {
                    UP => (0..tile.y_len).all(|y| tile[(0, y)] == other_tile[(other_tile.x_len - 1, y)]),
                    DOWN => (0..tile.y_len).all(|y| tile[(tile.x_len - 1, y)] == other_tile[(0, y)]),
                    LEFT => (0..tile.x_len).all(|x| tile[(x, 0)] == other_tile[(x, other_tile.y_len - 1)]),
                    RIGHT => (0..tile.x_len).all(|x| tile[(x, tile.y_len - 1)] == other_tile[(x, 0)]),
                    _ => unreachable!(),
                };
                if matches {
                    map.insert(pos + dir, (other_tile.clone(), other_tile_id));
                    available_tiles.remove(&other_tile_id);
                    queue.push_back((pos + dir, UP));
                    queue.push_back((pos + dir, DOWN));
                    queue.push_back((pos + dir, LEFT));
                    queue.push_back((pos + dir, RIGHT));
                    break 'outer;
                }
                other_tile.rotate_left();
                if i == 3 {
                    other_tile.flip_x();
                }
            }
        }
    }
    map
}

#[cfg(test)]
mod day20_tests {
    use std::fs;

    use crate::y2020::day20::solve;

    #[test]
    fn test_works() {
        assert_eq!((20899048083289, 273), solve(fs::read_to_string("input/2020/day20/test.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!((63187742854073, 2152), solve(fs::read_to_string("input/2020/day20/input.txt").unwrap()));
    }
}
