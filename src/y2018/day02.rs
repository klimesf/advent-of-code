use std::fs;

pub(crate) fn day02() {
    let input = fs::read_to_string("input/2018/day02/input.txt").unwrap();
    let words: Vec<&str> = input.trim().split("\n").collect();

    let w2 = words
        .iter()
        .filter(|w| contains_n_same_letters(**w, 2))
        .count();
    let w3 = words
        .iter()
        .filter(|w| contains_n_same_letters(**w, 3))
        .count();
    println!("Checksum: {} * {} = {}", w2, w3, w2 * w3);

    for i in 0..words.len() {
        for j in 0..words.len() {
            if differs_in_1_pos(words[i], words[j]) {
                println!("{}", words[i]);
                println!("{}", words[j]);
                return;
            }
        }
    }
}

fn contains_n_same_letters(w: &str, n: usize) -> bool {
    let mut counts = [0; 26];
    for c in w.chars() {
        counts[(c as usize) - 97] += 1;
    }
    counts.iter().any(|cnt| *cnt == n)
}

fn differs_in_1_pos(w1: &str, w2: &str) -> bool {
    let mut diff = 0;
    let w1_bytes = w1.as_bytes();
    let w2_bytes = w2.as_bytes();
    for i in 0..w1_bytes.len() {
        if w1_bytes[i] != w2_bytes[i] {
            diff += 1;
        }
    }
    diff == 1
}
