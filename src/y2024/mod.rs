extern crate itertools;
extern crate regex;

use colored::Colorize;
use std::env;
use crate::measure;
use crate::y2024::day01::day01;
use crate::y2024::day02::day02;
use crate::y2024::day03::day03;
use crate::y2024::day04::day04;
use crate::y2024::day05::day05;
use crate::y2024::day06::day06;
use crate::y2024::day07::day07;
use crate::y2024::day08::day08;
use crate::y2024::day09::day09;
use crate::y2024::day10::day10;
use crate::y2024::day11::day11;
use crate::y2024::day12::day12;
use crate::y2024::day13::day13;
use crate::y2024::day14::day14;
use crate::y2024::day15::day15;
use crate::y2024::day16::day16;
use crate::y2024::day17::day17;
use crate::y2024::day18::day18;
use crate::y2024::day19::day19;
use crate::y2024::day20::day20;
use crate::y2024::day21::day21;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
pub mod day21;

pub fn y2024() {
    let args: Vec<String> = env::args().collect();
    println!();
    println!("{}", format!("Advent of Code").red());
    println!("{}", format!("        //2024").red());
    println!();

    if args.contains(&"all".to_string()) || args.contains(&"day01".to_string()) {
        println!("{}", format!("--- day01:").underline().green());
        measure!(day01(|s| println!("{}", s)));
    }

    if args.contains(&"all".to_string()) || args.contains(&"day02".to_string()) {
        println!("{}", format!("--- day02:").underline().green());
        measure!(day02(|s| println!("{}", s)));
    }

    if args.contains(&"all".to_string()) || args.contains(&"day03".to_string()) {
        println!("{}", format!("--- day03:").underline().green());
        measure!(day03(|s| println!("{}", s)));
    }

    if args.contains(&"all".to_string()) || args.contains(&"day04".to_string()) {
        println!("{}", format!("--- day04:").underline().green());
        measure!(day04(|s| println!("{}", s)));
    }

    if args.contains(&"all".to_string()) || args.contains(&"day05".to_string()) {
        println!("{}", format!("--- day05:").underline().green());
        measure!(day05(|s| println!("{}", s)));
    }

    if args.contains(&"all".to_string()) || args.contains(&"day06".to_string()) {
        println!("{}", format!("--- day06:").underline().green());
        measure!(day06(|s| println!("{}", s)));
    }

    if args.contains(&"all".to_string()) || args.contains(&"day07".to_string()) {
        println!("{}", format!("--- day07:").underline().green());
        measure!(day07(|s| println!("{}", s)));
    }

    if args.contains(&"all".to_string()) || args.contains(&"day08".to_string()) {
        println!("{}", format!("--- day08:").underline().green());
        measure!(day08(|s| println!("{}", s)));
    }

    if args.contains(&"all".to_string()) || args.contains(&"day09".to_string()) {
        println!("{}", format!("--- day09:").underline().green());
        measure!(day09(|s| println!("{}", s)));
    }

    if args.contains(&"all".to_string()) || args.contains(&"day10".to_string()) {
        println!("{}", format!("--- day10:").underline().green());
        measure!(day10(|s| println!("{}", s)));
    }

    if args.contains(&"all".to_string()) || args.contains(&"day11".to_string()) {
        println!("{}", format!("--- day11:").underline().green());
        measure!(day11(|s| println!("{}", s)));
    }

    if args.contains(&"all".to_string()) || args.contains(&"day12".to_string()) {
        println!("{}", format!("--- day12:").underline().green());
        measure!(day12(|s| println!("{}", s)));
    }

    if args.contains(&"all".to_string()) || args.contains(&"day13".to_string()) {
        println!("{}", format!("--- day13:").underline().green());
        measure!(day13(|s| println!("{}", s)));
    }

    if args.contains(&"all".to_string()) || args.contains(&"day14".to_string()) {
        println!("{}", format!("--- day14:").underline().green());
        measure!(day14(|s| println!("{}", s)));
    }

    if args.contains(&"all".to_string()) || args.contains(&"day15".to_string()) {
        println!("{}", format!("--- day15:").underline().green());
        measure!(day15(|s| println!("{}", s)));
    }

    if args.contains(&"all".to_string()) || args.contains(&"day16".to_string()) {
        println!("{}", format!("--- day16:").underline().green());
        measure!(day16(|s| println!("{}", s)));
    }

    if args.contains(&"all".to_string()) || args.contains(&"day17".to_string()) {
        println!("{}", format!("--- day17:").underline().green());
        measure!(day17(|s| println!("{}", s)));
    }

    if args.contains(&"all".to_string()) || args.contains(&"day18".to_string()) {
        println!("{}", format!("--- day18:").underline().green());
        measure!(day18(|s| println!("{}", s)));
    }

    if args.contains(&"all".to_string()) || args.contains(&"day19".to_string()) {
        println!("{}", format!("--- day19:").underline().green());
        measure!(day19(|s| println!("{}", s)));
    }

    if args.contains(&"all".to_string()) || args.contains(&"day20".to_string()) {
        println!("{}", format!("--- day20:").underline().green());
        measure!(day20(|s| println!("{}", s)));
    }

    if args.contains(&"all".to_string()) || args.contains(&"day21".to_string()) {
        println!("{}", format!("--- day21:").underline().green());
        measure!(day21(|s| println!("{}", s)));
    }
}
