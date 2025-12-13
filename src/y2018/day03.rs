use std::fs;

use regex::Regex;

pub(crate) fn day03() {
    let input = fs::read_to_string("input/2018/day03/input.txt").unwrap();
    let re = Regex::new(r"^#(\d+)\s@\s(\d+),(\d+):\s(\d+)x(\d+)$").unwrap();
    let claims: Vec<(usize, usize, usize, usize, usize)> = input
        .trim()
        .split("\n")
        .map(|claim| {
            let captures = re.captures(claim).unwrap();
            let claim_id: usize = captures.get(1).unwrap().as_str().parse().unwrap();
            let start_x: usize = captures.get(2).unwrap().as_str().parse().unwrap();
            let start_y: usize = captures.get(3).unwrap().as_str().parse().unwrap();
            let size_x: usize = captures.get(4).unwrap().as_str().parse().unwrap();
            let size_y: usize = captures.get(5).unwrap().as_str().parse().unwrap();
            (claim_id, start_x, start_y, size_x, size_y)
        })
        .collect();

    let mut fabric = [[0; 1000]; 1000];
    for (_, start_x, start_y, size_x, size_y) in &claims {
        for y in 0..*size_y {
            for x in 0..*size_x {
                fabric[start_y + y][start_x + x] += 1;
            }
        }
    }

    println!(
        "There are {} overlapping inches",
        (0..1000)
            .map(|y| (0..1000).filter(|x| fabric[y][*x] > 1).count())
            .sum::<usize>()
    );

    for (id, start_x, start_y, size_x, size_y) in &claims {
        let mut valid = true;
        for y in 0..*size_y {
            for x in 0..*size_x {
                if fabric[start_y + y][start_x + x] > 1 {
                    valid = false;
                    break;
                }
            }
        }
        if valid {
            println!("Claim {} is intact", id);
            break;
        }
    }
}
