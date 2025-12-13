use std::collections::HashSet;
use std::fs;

use regex::Regex;

type Star = (i32, i32, i32, i32);

pub(crate) fn day10() {
    let input = fs::read_to_string("input/2018/day10/input.txt").unwrap();
    let mut stars: Vec<Star> = input.trim().split("\n").map(|s| parse_row(s)).collect();

    let mut seconds = 0;
    while !print_sky(&stars) {
        stars = stars.iter().map(|star| increment_star(star)).collect();
        seconds += 1;
    }
    println!("It took {} seconds for the message to appear", seconds);
}

fn parse_row(s: &str) -> Star {
    let re = Regex::new(r"^position=<([\s\d\-]+),([\s\d\-]+)> velocity=<([\s\d\-]+),([\s\d\-]+)>$")
        .unwrap();
    let c = re.captures(s).unwrap();
    (
        c.get(1).unwrap().as_str().trim().parse::<i32>().unwrap(),
        c.get(2).unwrap().as_str().trim().parse::<i32>().unwrap(),
        c.get(3).unwrap().as_str().trim().parse::<i32>().unwrap(),
        c.get(4).unwrap().as_str().trim().parse::<i32>().unwrap(),
    )
}

fn increment_star(star: &Star) -> Star {
    (star.0 + star.2, star.1 + star.3, star.2, star.3)
}

fn print_sky(stars: &Vec<Star>) -> bool {
    let min_x = stars.iter().map(|star| star.0).min().unwrap();
    let max_x = stars.iter().map(|star| star.0).max().unwrap();
    let min_y = stars.iter().map(|star| star.1).min().unwrap();
    let max_y = stars.iter().map(|star| star.1).max().unwrap();

    if max_y - min_y > 10 {
        return false;
    }

    let mut sky = HashSet::new();
    stars.iter().for_each(|star| {
        sky.insert((star.0, star.1));
    });

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            print!("{}", if sky.contains(&(x, y)) { '#' } else { '.' })
        }
        println!();
    }
    return true;
}

#[cfg(test)]
mod day10_tests {
    use crate::y2018::day10::{increment_star, parse_row};

    #[test]
    fn parse_row_works() {
        assert_eq!(
            (9, 1, 0, 2),
            parse_row("position=< 9,  1> velocity=< 0,  2>")
        );
        assert_eq!(
            (-6, 10, 2, -2),
            parse_row("position=<-6, 10> velocity=< 2, -2>")
        );
    }

    #[test]
    fn increment_star_works() {
        assert_eq!((9, 3, 0, 2), increment_star(&(9, 1, 0, 2)));
        assert_eq!((-4, 8, 2, -2), increment_star(&(-6, 10, 2, -2)));
    }
}
