use std::fs;

pub(crate) fn day20() {
    println!(
        "{}",
        part_a(fs::read_to_string("input/2016/day20/input.txt").unwrap())
    );
    println!(
        "{}",
        part_b(
            fs::read_to_string("input/2016/day20/input.txt").unwrap(),
            4294967295
        )
    );
}

fn part_a(input: String) -> usize {
    let blacklist: Vec<(usize, usize)> = input
        .lines()
        .map(|line| {
            let (start, end) = line.split_once("-").unwrap();
            (
                start.parse::<usize>().unwrap(),
                end.parse::<usize>().unwrap(),
            )
        })
        .collect();

    let mut min = 0;
    let mut changes = 1;
    while changes > 0 {
        changes = 0;
        for i in 0..blacklist.len() {
            let (start, end) = blacklist[i];
            if min >= start && min <= end {
                min = end + 1;
                changes += 1;
                break;
            }
        }
    }
    min
}

fn part_b(input: String, range: usize) -> usize {
    let mut allowlist = vec![(0, range)];

    let blacklist: Vec<(usize, usize)> = input
        .lines()
        .map(|line| {
            let (start, end) = line.split_once("-").unwrap();
            (
                start.parse::<usize>().unwrap(),
                end.parse::<usize>().unwrap(),
            )
        })
        .collect();

    for (bl_start, bl_end) in &blacklist {
        let mut new_allowlist = vec![];

        allowlist.iter().for_each(|(al_start, al_end)| {
            if al_start >= bl_start && al_start <= bl_end && al_end > bl_end {
                new_allowlist.push((bl_end + 1, *al_end));
            } else if al_start < bl_start && al_end >= bl_start && al_end <= bl_end {
                new_allowlist.push((*al_start, bl_start - 1));
            } else if al_start < bl_start && al_end > bl_end {
                new_allowlist.push((*al_start, bl_start - 1));
                new_allowlist.push((bl_end + 1, *al_end));
            } else if al_end < bl_start || al_start > bl_end {
                new_allowlist.push((*al_start, *al_end));
            }
        });

        allowlist = new_allowlist
    }

    allowlist.iter().map(|(start, end)| *end - *start + 1).sum()
}

#[cfg(test)]
mod day20_tests {
    use std::fs;

    use crate::y2016::day20::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(
            3,
            part_a(fs::read_to_string("input/2016/day20/test.txt").unwrap())
        );
        assert_eq!(
            2,
            part_b(fs::read_to_string("input/2016/day20/test.txt").unwrap(), 9)
        );
    }

    #[test]
    fn input_works() {
        assert_eq!(
            19449262,
            part_a(fs::read_to_string("input/2016/day20/input.txt").unwrap())
        );
        assert_eq!(
            119,
            part_b(
                fs::read_to_string("input/2016/day20/input.txt").unwrap(),
                4294967295
            )
        );
    }
}
