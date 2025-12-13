use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

pub(crate) fn day07() {
    println!(
        "{}",
        part_a(fs::read_to_string("input/2023/day07/input.txt").unwrap())
    );
    println!(
        "{}",
        part_b(fs::read_to_string("input/2023/day07/input.txt").unwrap())
    );
}

fn part_a(input: String) -> i32 {
    let mut high_card = vec![];
    let mut one_pair = vec![];
    let mut two_pair = vec![];
    let mut three_of_a_kind = vec![];
    let mut full_house = vec![];
    let mut four_of_kind = vec![];
    let mut five_of_kind = vec![];

    input.lines().for_each(|line| {
        let (cards_s, bid_s) = line.split_once(" ").unwrap();
        let bid = bid_s.parse::<i32>().unwrap();

        let mut types: HashMap<char, i32> = HashMap::new();
        for c in cards_s.chars() {
            *types.entry(c).or_insert(0) += 1;
        }

        if types.len() == 5 {
            high_card.push((cards_s, bid));
        } else if types.len() == 4 {
            one_pair.push((cards_s, bid));
        } else if types.len() == 3 {
            if *types.values().max().unwrap() == 3 {
                three_of_a_kind.push((cards_s, bid));
            } else {
                two_pair.push((cards_s, bid));
            }
        } else if types.len() == 2 {
            if *types.values().max().unwrap() == 4 {
                four_of_kind.push((cards_s, bid));
            } else {
                full_house.push((cards_s, bid));
            }
        } else if types.len() == 1 {
            five_of_kind.push((cards_s, bid));
        }
    });

    count_total(
        vec![
            high_card,
            one_pair,
            two_pair,
            three_of_a_kind,
            full_house,
            four_of_kind,
            five_of_kind,
        ],
        cmp_cards,
    )
}

fn count_total(stacks: Vec<Vec<(&str, i32)>>, cmp_fn: fn(&str, &str) -> Ordering) -> i32 {
    let mut total = 0;
    let mut rank = 1;

    for mut stack in stacks {
        stack.sort_by(|(card_a, _), (card_b, _)| cmp_fn(card_a, card_b));
        for (_, bid) in stack {
            total += rank * bid;
            rank += 1;
        }
    }

    total
}

fn part_b(input: String) -> i32 {
    let mut high_card = vec![];
    let mut one_pair = vec![];
    let mut two_pair = vec![];
    let mut three_of_a_kind = vec![];
    let mut full_house = vec![];
    let mut four_of_kind = vec![];
    let mut five_of_kind = vec![];

    input.lines().for_each(|line| {
        let (cards_s, bid_s) = line.split_once(" ").unwrap();
        let bid = bid_s.parse::<i32>().unwrap();

        let mut types: HashMap<char, i32> = HashMap::new();
        for c in cards_s.chars() {
            *types.entry(c).or_insert(0) += 1;
        }

        if types.len() == 5 {
            if cards_s.contains('J') {
                one_pair.push((cards_s, bid));
            } else {
                high_card.push((cards_s, bid));
            }
        } else if types.len() == 4 {
            if cards_s.contains('J') {
                three_of_a_kind.push((cards_s, bid));
            } else {
                one_pair.push((cards_s, bid));
            }
        } else if types.len() == 3 {
            if *types.values().max().unwrap() == 3 {
                if cards_s.contains('J') {
                    four_of_kind.push((cards_s, bid));
                } else {
                    three_of_a_kind.push((cards_s, bid));
                }
            } else if cards_s.contains('J') {
                if *types.get(&'J').unwrap() == 2 {
                    four_of_kind.push((cards_s, bid));
                } else {
                    full_house.push((cards_s, bid));
                }
            } else {
                two_pair.push((cards_s, bid));
            }
        } else if types.len() == 1 {
            five_of_kind.push((cards_s, bid));
        } else if types.len() == 2 {
            if cards_s.contains('J') {
                five_of_kind.push((cards_s, bid));
            } else if *types.values().max().unwrap() == 4 {
                four_of_kind.push((cards_s, bid));
            } else {
                full_house.push((cards_s, bid));
            }
        }
    });

    count_total(
        vec![
            high_card,
            one_pair,
            two_pair,
            three_of_a_kind,
            full_house,
            four_of_kind,
            five_of_kind,
        ],
        cmp_cards_part_b,
    )
}

fn cmp_cards(card_a: &str, card_b: &str) -> Ordering {
    for i in 0..card_a.len() {
        let ca = card_a.chars().nth(i).unwrap();
        let cb = card_b.chars().nth(i).unwrap();
        if ca == cb {
            continue;
        }
        return card_rank(ca).cmp(&card_rank(cb));
    }
    Ordering::Equal
}

fn card_rank(card: char) -> i32 {
    match card {
        '2' => 1,
        '3' => 2,
        '4' => 3,
        '5' => 4,
        '6' => 5,
        '7' => 6,
        '8' => 7,
        '9' => 8,
        'T' => 9,
        'J' => 10,
        'Q' => 11,
        'K' => 12,
        'A' => 13,
        _ => panic!("Unknown card {}", card),
    }
}

fn cmp_cards_part_b(card_a: &str, card_b: &str) -> Ordering {
    for i in 0..card_a.len() {
        let ca = card_a.chars().nth(i).unwrap();
        let cb = card_b.chars().nth(i).unwrap();
        if ca == cb {
            continue;
        }
        return card_rank_part_b(ca).cmp(&card_rank_part_b(cb));
    }
    Ordering::Equal
}

fn card_rank_part_b(card: char) -> i32 {
    match card {
        'J' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'Q' => 11,
        'K' => 12,
        'A' => 13,
        _ => panic!("Unknown card {}", card),
    }
}

#[cfg(test)]
mod day07_tests {
    use std::cmp::Ordering;
    use std::fs;

    use crate::y2023::day07::{cmp_cards, part_a, part_b};

    #[test]
    fn ordering_works() {
        assert_eq!(Ordering::Greater, cmp_cards("KK677", "KTJJT"));
        assert_eq!(Ordering::Less, cmp_cards("T55J5", "QQQJA"));
    }

    #[test]
    fn test_works() {
        assert_eq!(
            6440,
            part_a(fs::read_to_string("input/2023/day07/test.txt").unwrap())
        );
        assert_eq!(
            5905,
            part_b(fs::read_to_string("input/2023/day07/test.txt").unwrap())
        );
    }

    #[test]
    fn input_works() {
        assert_eq!(
            254024898,
            part_a(fs::read_to_string("input/2023/day07/input.txt").unwrap())
        );
        assert_eq!(
            254115617,
            part_b(fs::read_to_string("input/2023/day07/input.txt").unwrap())
        );
    }
}
