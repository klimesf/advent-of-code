extern crate itertools;
extern crate regex;

use colored::Colorize;
use std::env;
use crate::measure;
use crate::y2018::day01::day01;
use crate::y2018::day02::day02;
use crate::y2018::day03::day03;
use crate::y2018::day04::day04;
use crate::y2018::day05::day05;
use crate::y2018::day06::day06;
use crate::y2018::day07::day07;
use crate::y2018::day08::day08;
use crate::y2018::day09::day09;
use crate::y2018::day10::day10;
use crate::y2018::day11::day11;
use crate::y2018::day12::day12;
use crate::y2018::day13::day13;
use crate::y2018::day14::day14;
use crate::y2018::day16::day16;
use crate::y2018::day18::day18;
use crate::y2018::day19::day19;
use crate::y2018::day21::day21;
use crate::y2018::day22::day22;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day16;
mod day18;
mod day19;
mod day21;
mod day22;

pub(crate) fn y2018() {
    let args: Vec<String> = env::args().collect();
    println!();
    println!("{}", format!("Advent of Code").red());
    println!("{}", format!("        //2018").red());
    println!();

    if args.contains(&"all".to_string()) || args.contains(&"day01".to_string()) {
        println!("{}", format!("--- day01:").underline().green());
        measure!(day01());
    }

    if args.contains(&"all".to_string()) || args.contains(&"day02".to_string()) {
        println!("{}", format!("--- day02:").underline().green());
        measure!(day02());
    }

    if args.contains(&"all".to_string()) || args.contains(&"day03".to_string()) {
        println!("{}", format!("--- day03:").underline().green());
        measure!(day03());
    }

    if args.contains(&"all".to_string()) || args.contains(&"day04".to_string()) {
        println!("{}", format!("--- day04:").underline().green());
        measure!(day04());
    }

    if args.contains(&"all".to_string()) || args.contains(&"day05".to_string()) {
        println!("{}", format!("--- day05:").underline().green());
        measure!(day05());
    }

    if args.contains(&"all".to_string()) || args.contains(&"day06".to_string()) {
        println!("{}", format!("--- day06:").underline().green());
        measure!(day06());
    }

    if args.contains(&"all".to_string()) || args.contains(&"day07".to_string()) {
        println!("{}", format!("--- day07:").underline().green());
        measure!(day07());
    }

    if args.contains(&"all".to_string()) || args.contains(&"day08".to_string()) {
        println!("{}", format!("--- day08:").underline().green());
        measure!(day08());
    }

    if args.contains(&"all".to_string()) || args.contains(&"day09".to_string()) {
        println!("{}", format!("--- day09:").underline().green());
        measure!(day09());
    }

    if args.contains(&"all".to_string()) || args.contains(&"day10".to_string()) {
        println!("{}", format!("--- day10:").underline().green());
        measure!(day10());
    }

    if args.contains(&"all".to_string()) || args.contains(&"day11".to_string()) {
        println!("{}", format!("--- day11:").underline().green());
        measure!(day11());
    }

    if args.contains(&"all".to_string()) || args.contains(&"day12".to_string()) {
        println!("{}", format!("--- day12:").underline().green());
        measure!(day12());
    }

    if args.contains(&"all".to_string()) || args.contains(&"day13".to_string()) {
        println!("{}", format!("--- day13:").underline().green());
        measure!(day13());
    }

    if args.contains(&"all".to_string()) || args.contains(&"day14".to_string()) {
        println!("{}", format!("--- day14:").underline().green());
        measure!(day14());
    }

    if args.contains(&"all".to_string()) || args.contains(&"day16".to_string()) {
        println!("{}", format!("--- day16:").underline().green());
        measure!(day16());
    }

    if args.contains(&"all".to_string()) || args.contains(&"day18".to_string()) {
        println!("{}", format!("--- day18:").underline().green());
        measure!(day18());
    }

    if args.contains(&"all".to_string()) || args.contains(&"day19".to_string()) {
        println!("{}", format!("--- day19:").underline().green());
        measure!(day19());
    }

    if args.contains(&"all".to_string()) || args.contains(&"day21".to_string()) {
        println!("{}", format!("--- day21:").underline().green());
        measure!(day21());
    }

    if args.contains(&"all".to_string()) || args.contains(&"day22".to_string()) {
        println!("{}", format!("--- day22:").underline().green());
        measure!(day22());
    }
}
