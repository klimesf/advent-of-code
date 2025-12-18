use std::fs;

pub(crate) fn day16() {
    println!("{}", part_a(fs::read_to_string("input/2020/day16/input.txt").unwrap()));
    println!("{}", part_b(fs::read_to_string("input/2020/day16/input.txt").unwrap()));
}

fn part_a(input: String) -> usize {
    let parts = input.split("\n\n").collect::<Vec<&str>>();
    let ranges = parts[0]
        .lines()
        .map(|line| {
            let (_, r) = line.split_once(": ").unwrap();
            let (lr, rr) = r.split_once(" or ").unwrap();
            let (llr, rlr) = lr.split_once("-").unwrap();
            let (lrr, rrr) = rr.split_once("-").unwrap();
            (llr.parse().unwrap(), rlr.parse().unwrap(), lrr.parse().unwrap(), rrr.parse().unwrap())
        })
        .collect::<Vec<(usize, usize, usize, usize)>>();

    parts[2]
        .lines()
        .skip(1)
        .map(|line| {
            line.split(',')
                .map(|val| val.parse::<usize>().unwrap())
                .filter(|val| {
                    for (llr, rlr, lrr, rrr) in &ranges {
                        if (val >= llr && val <= rlr) || (val >= lrr && val <= rrr) {
                            return false;
                        }
                    }
                    true
                })
                .collect::<Vec<usize>>()
        })
        .flatten()
        .sum()
}

fn part_b(input: String) -> usize {
    let parts = input.split("\n\n").collect::<Vec<&str>>();
    let ranges = parts[0]
        .lines()
        .map(|line| {
            let (l, r) = line.split_once(": ").unwrap();
            let (lr, rr) = r.split_once(" or ").unwrap();
            let (llr, rlr) = lr.split_once("-").unwrap();
            let (lrr, rrr) = rr.split_once("-").unwrap();
            (l, llr.parse().unwrap(), rlr.parse().unwrap(), lrr.parse().unwrap(), rrr.parse().unwrap())
        })
        .collect::<Vec<(&str, usize, usize, usize, usize)>>();

    let your_tickets = parts[1]
        .lines()
        .skip(1)
        .map(|line| {
            line.split(',')
                .map(|val| val.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();
    let your_ticket = your_tickets.first().unwrap();

    let mut tickets = parts[2]
        .lines()
        .skip(1)
        .map(|line| {
            line.split(',')
                .map(|val| val.parse::<usize>().unwrap())
                .filter(|val| {
                    for (_, llr, rlr, lrr, rrr) in &ranges {
                        if (val >= llr && val <= rlr) || (val >= lrr && val <= rrr) {
                            return true;
                        }
                    }
                    false
                })
                .collect::<Vec<usize>>()
        })
        .filter(|ticket| ticket.len() == ranges.len())
        .collect::<Vec<Vec<usize>>>();
    tickets.push(your_ticket.clone());

    // Map possible positions for the fields
    let mut possible_positions = vec![vec![true; ranges.len()]; ranges.len()];
    for r in 0..ranges.len() {
        let (_, llr, rlr, lrr, rrr) = ranges[r];
        let mut possible_fields = vec![true; ranges.len()];
        'outer: for i in 0..ranges.len() {
            for ticket in &tickets {
                if ticket[i] < llr || (ticket[i] > rlr && ticket[i] < lrr) || ticket[i] > rrr {
                    possible_fields[i] = false;
                    continue 'outer;
                }
            }
        }
        possible_positions[r] = possible_fields.clone();
    }

    // Assume that there is always one field that can be put only onto one position
    // Find it, assign, and remove the position from other fields
    // => assume there will now be another that will have only 1 possible
    // => rinse & repeat
    let mut assignments = vec![usize::MAX; ranges.len()];
    let mut assigned_count = 0;
    'outer: while assigned_count < ranges.len() {
        for r in 0..ranges.len() {
            if assignments[r] != usize::MAX {
                continue;
            }
            let mut poss = vec![];
            for i in 0..ranges.len() {
                if possible_positions[r][i] == true {
                    poss.push(i);
                }
            }
            if poss.len() == 1 {
                let possss = *poss.first().unwrap();
                assignments[r] = possss;
                for s in 0..ranges.len() {
                    possible_positions[s][possss] = false;
                }
                assigned_count += 1;
                continue 'outer;
            }
        }
        panic!("no range with just one possible assignment");
    }

    ranges
        .iter()
        .filter(|(name, _, _, _, _)| name.starts_with("departure"))
        .enumerate()
        .map(|(r, _)| your_ticket[assignments[r]])
        .product()
}

#[cfg(test)]
mod day16_tests {
    use std::fs;

    use crate::y2020::day16::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(71, part_a(fs::read_to_string("input/2020/day16/test.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!(28882, part_a(fs::read_to_string("input/2020/day16/input.txt").unwrap()));
        assert_eq!(1429779530273, part_b(fs::read_to_string("input/2020/day16/input.txt").unwrap()));
    }
}
