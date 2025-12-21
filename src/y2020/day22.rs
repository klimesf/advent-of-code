use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

pub(crate) fn day22() {
    println!("{}", part_a(fs::read_to_string("input/2020/day22/input.txt").unwrap()));
    println!("{}", part_b(fs::read_to_string("input/2020/day22/input.txt").unwrap()));
}

fn part_a(input: String) -> usize {
    let (p1_deck_str, p2_deck_str) = input.split_once("\n\n").unwrap();
    let mut p1_deck = p1_deck_str
        .lines()
        .skip(1)
        .map(|line| line.parse().unwrap())
        .collect::<VecDeque<usize>>();
    let mut p2_deck = p2_deck_str
        .lines()
        .skip(1)
        .map(|line| line.parse().unwrap())
        .collect::<VecDeque<usize>>();

    while !p1_deck.is_empty() && !p2_deck.is_empty() {
        let p1_card = p1_deck.pop_front().unwrap();
        let p2_card = p2_deck.pop_front().unwrap();

        debug_assert_ne!(p1_card, p2_card);
        if p1_card > p2_card {
            p1_deck.push_back(p1_card);
            p1_deck.push_back(p2_card);
        } else {
            p2_deck.push_back(p2_card);
            p2_deck.push_back(p1_card);
        }
    }

    let winning_deck = if p1_deck.is_empty() { p2_deck } else { p1_deck };
    winning_deck
        .iter()
        .enumerate()
        .fold(0, |acc, (i, &card)| acc + (card * (winning_deck.len() - i)))
}

fn part_b(input: String) -> usize {
    let (p1_deck_str, p2_deck_str) = input.split_once("\n\n").unwrap();
    let p1_deck = p1_deck_str
        .lines()
        .skip(1)
        .map(|line| line.parse().unwrap())
        .collect::<VecDeque<usize>>();
    let p2_deck = p2_deck_str
        .lines()
        .skip(1)
        .map(|line| line.parse().unwrap())
        .collect::<VecDeque<usize>>();

    let mut cache: HashMap<(VecDeque<usize>, VecDeque<usize>), usize> = HashMap::new();
    let (_, winning_deck) = recursive_combat(p1_deck, p2_deck, &mut cache);
    winning_deck
        .iter()
        .enumerate()
        .fold(0, |acc, (i, &card)| acc + (card * (winning_deck.len() - i)))
}

fn recursive_combat(
    mut p1_deck: VecDeque<usize>,
    mut p2_deck: VecDeque<usize>,
    cache: &mut HashMap<(VecDeque<usize>, VecDeque<usize>), usize>,
) -> (usize, VecDeque<usize>) {
    if cache.contains_key(&(p1_deck.clone(), p2_deck.clone())) {
        let winner = cache[&(p1_deck.clone(), p2_deck.clone())];
        return (winner, if winner == 1 { p1_deck } else { p2_deck });
    }
    let start_deck_1 = p1_deck.clone();
    let start_deck_2 = p2_deck.clone();

    let mut visited = HashSet::new();

    while !p1_deck.is_empty() && !p2_deck.is_empty() {
        if !visited.insert((p1_deck.clone(), p2_deck.clone())) {
            cache.insert((start_deck_1.clone(), start_deck_2.clone()), 1);
            return (1, p1_deck);
        }

        let p1_card = p1_deck.pop_front().unwrap();
        let p2_card = p2_deck.pop_front().unwrap();

        if p1_card > p1_deck.len() || p2_card > p2_deck.len() {
            if p1_card > p2_card {
                p1_deck.push_back(p1_card);
                p1_deck.push_back(p2_card);
                continue;
            } else {
                p2_deck.push_back(p2_card);
                p2_deck.push_back(p1_card);
                continue;
            }
        }

        let mut new_p1_deck = VecDeque::with_capacity(p1_card);
        for i in 0..p1_card {
            new_p1_deck.push_back(p1_deck[i]);
        }
        let mut new_p2_deck = VecDeque::with_capacity(p2_card);
        for i in 0..p2_card {
            new_p2_deck.push_back(p2_deck[i]);
        }

        let (winner, _) = recursive_combat(new_p1_deck, new_p2_deck, cache);
        if winner == 1 {
            p1_deck.push_back(p1_card);
            p1_deck.push_back(p2_card);
        } else {
            p2_deck.push_back(p2_card);
            p2_deck.push_back(p1_card);
        }
    }

    if p1_deck.is_empty() {
        cache.insert((start_deck_1, start_deck_2), 2);
        (2, p2_deck)
    } else {
        cache.insert((start_deck_1, start_deck_2), 1);
        (1, p1_deck)
    }
}

#[cfg(test)]
mod day22_tests {
    use std::fs;

    use crate::y2020::day22::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(306, part_a(fs::read_to_string("input/2020/day22/test.txt").unwrap()));
        assert_eq!(291, part_b(fs::read_to_string("input/2020/day22/test.txt").unwrap()));
    }

    #[test]
    fn input_works() {
        assert_eq!(32677, part_a(fs::read_to_string("input/2020/day22/input.txt").unwrap()));
        assert_eq!(33661, part_b(fs::read_to_string("input/2020/day22/input.txt").unwrap()));
    }
}
