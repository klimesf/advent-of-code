use std::fs;

pub fn day12(print: fn(usize)) {
    print(part_a(
        fs::read_to_string("input/2025/day12/input.txt").unwrap(),
    ));
}

fn part_a(input: String) -> usize {
    let parts = input.split("\n\n").collect::<Vec<&str>>();

    let gifts: Vec<Vec<Vec<bool>>> = parts[0..parts.len() - 1]
        .iter()
        .map(|gift| {
            let lines = gift.lines().collect::<Vec<&str>>();
            lines[1..]
                .iter()
                .map(|line| line.chars().map(|c| c == '#').collect())
                .collect()
        })
        .collect();

    let areas: Vec<(usize, usize, Vec<usize>)> = parts[parts.len() - 1]
        .lines()
        .map(|line| {
            let (l, r) = line.split_once(": ").unwrap();
            let (ll, rr) = l.split_once("x").unwrap();
            let pos = r
                .split_whitespace()
                .map(|c| c.parse::<usize>().unwrap())
                .collect();

            (
                ll.parse::<usize>().unwrap(),
                rr.parse::<usize>().unwrap(),
                pos,
            )
        })
        .collect();

    areas
        .iter()
        .filter(|(max_x, max_y, pos)| {
            let mut required_fields = 0;

            for i in 0..pos.len() {
                if pos[i] == 0 {
                    continue;
                }
                let mut tiles = 0;
                for x in 0..3 {
                    for y in 0..3 {
                        if gifts[i][x][y] {
                            tiles += 1;
                        }
                    }
                }
                required_fields += pos[i] * tiles
            }

            max_x * max_y > required_fields
        })
        .filter(|(max_x, max_y, pos)| {
            let place_for_all = pos.iter().map(|i| i * 3 * 3).sum::<usize>();
            if max_x * max_y >= place_for_all {
                return true;
            }

            let area = vec![vec![false; *max_y]; *max_x];
            let placed = vec![0; gifts.len()];

            let mut stack = vec![(area.clone(), placed.clone())];

            while let Some((area, placed)) = stack.pop() {
                let mut to_place = vec![];
                for i in 0..gifts.len() {
                    if pos[i] > placed[i] {
                        to_place.push(i);
                    }
                }
                if to_place.len() < 1 {
                    return true;
                }

                for i in to_place {
                    let gift = &gifts[i];
                    let mut new_placed = placed.clone();
                    new_placed[i] += 1;
                    for new_area in try_place(&area, &gift) {
                        stack.push((new_area, new_placed.clone()));
                    }
                }
            }
            false
        })
        .count()
}

fn try_place(area: &Vec<Vec<bool>>, gift: &Vec<Vec<bool>>) -> Vec<Vec<Vec<bool>>> {
    let mut placed = vec![];
    let mut rotated_gifts = vec![gift.clone()];

    let mut rot1 = vec![vec![false; gift[0].len()]; gift.len()];
    for x in 0..gift.len() {
        for y in 0..gift[x].len() {
            rot1[y][x] = gift[x][y];
        }
    }
    rotated_gifts.push(rot1.clone());

    let mut rot2 = vec![vec![false; gift[0].len()]; gift.len()];
    for x in 0..rot1.len() {
        for y in 0..rot1[x].len() {
            rot2[y][x] = rot1[x][y];
        }
    }
    rotated_gifts.push(rot2.clone());

    let mut rot3 = vec![vec![false; gift[0].len()]; gift.len()];
    for x in 0..rot2.len() {
        for y in 0..rot2[x].len() {
            rot3[y][x] = rot2[x][y];
        }
    }
    rotated_gifts.push(rot3.clone());

    for rotated_gift in &rotated_gifts {
        for i in 0..area.len() - rotated_gift.len() + 1 {
            for j in 0..area[0].len() - rotated_gift[0].len() + 1 {
                let mut can_be_placed = true;
                for x in 0..rotated_gift.len() {
                    for y in 0..rotated_gift[0].len() {
                        let gift_val = rotated_gift[x][y];
                        let area_val = area[i + x][j + y];
                        if gift_val {
                            can_be_placed = can_be_placed && !area_val;
                        } else {
                        }
                    }
                }
                if can_be_placed {
                    let mut new_area: Vec<Vec<bool>> = area.clone();
                    for x in 0..rotated_gift.len() {
                        for y in 0..rotated_gift[0].len() {
                            if rotated_gift[x][y] {
                                new_area[x][y] = true;
                            }
                        }
                    }
                    placed.push(new_area);
                }
            }
        }
    }
    placed
}

#[cfg(test)]
mod day12_tests {
    use std::fs;

    use crate::y2025::day12::part_a;

    #[test]
    fn test_works() {
        assert_eq!(
            2,
            part_a(fs::read_to_string("input/2025/day12/test.txt").unwrap())
        );
    }

    #[test]
    fn input_works() {
        assert_eq!(
            521,
            part_a(fs::read_to_string("input/2025/day12/input.txt").unwrap())
        );
    }
}
