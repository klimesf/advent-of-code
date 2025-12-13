extern crate itertools;
extern crate regex;

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
use crate::y2024::day22::day22;
use crate::y2024::day23::day23;
use crate::y2024::day24::day24;
use crate::y2024::day25::day25;
use colored::Colorize;
use std::env;

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
pub mod day22;
pub mod day23;
pub mod day24;
pub mod day25;

pub fn y2024() {
    let args: Vec<String> = env::args().collect();
    println!();
    println!("{}", format!("Advent of Code").red());
    println!("{}", format!("        //2024").red());
    println!();

    if args.contains(&"all".to_string()) || args.contains(&"day01".to_string()) {
        println!("{}", format!("--- day01:").underline().green());
        measure!(day01(print_u32));
    }

    if args.contains(&"all".to_string()) || args.contains(&"day02".to_string()) {
        println!("{}", format!("--- day02:").underline().green());
        measure!(day02(print_usize));
    }

    if args.contains(&"all".to_string()) || args.contains(&"day03".to_string()) {
        println!("{}", format!("--- day03:").underline().green());
        measure!(day03(print_usize));
    }

    if args.contains(&"all".to_string()) || args.contains(&"day04".to_string()) {
        println!("{}", format!("--- day04:").underline().green());
        measure!(day04(print_usize));
    }

    if args.contains(&"all".to_string()) || args.contains(&"day05".to_string()) {
        println!("{}", format!("--- day05:").underline().green());
        measure!(day05(print_usize));
    }

    if args.contains(&"all".to_string()) || args.contains(&"day06".to_string()) {
        println!("{}", format!("--- day06:").underline().green());
        measure!(day06(print_usize));
    }

    if args.contains(&"all".to_string()) || args.contains(&"day07".to_string()) {
        println!("{}", format!("--- day07:").underline().green());
        measure!(day07(print_usize));
    }

    if args.contains(&"all".to_string()) || args.contains(&"day08".to_string()) {
        println!("{}", format!("--- day08:").underline().green());
        measure!(day08(print_usize));
    }

    if args.contains(&"all".to_string()) || args.contains(&"day09".to_string()) {
        println!("{}", format!("--- day09:").underline().green());
        measure!(day09(print_i64));
    }

    if args.contains(&"all".to_string()) || args.contains(&"day10".to_string()) {
        println!("{}", format!("--- day10:").underline().green());
        measure!(day10(print_usize));
    }

    if args.contains(&"all".to_string()) || args.contains(&"day11".to_string()) {
        println!("{}", format!("--- day11:").underline().green());
        measure!(day11(print_usize));
    }

    if args.contains(&"all".to_string()) || args.contains(&"day12".to_string()) {
        println!("{}", format!("--- day12:").underline().green());
        measure!(day12(print_usize));
    }

    if args.contains(&"all".to_string()) || args.contains(&"day13".to_string()) {
        println!("{}", format!("--- day13:").underline().green());
        measure!(day13(print_i64));
    }

    if args.contains(&"all".to_string()) || args.contains(&"day14".to_string()) {
        println!("{}", format!("--- day14:").underline().green());
        measure!(day14(print_usize));
    }

    if args.contains(&"all".to_string()) || args.contains(&"day15".to_string()) {
        println!("{}", format!("--- day15:").underline().green());
        measure!(day15(print_i32));
    }

    if args.contains(&"all".to_string()) || args.contains(&"day16".to_string()) {
        println!("{}", format!("--- day16:").underline().green());
        measure!(day16(print_usize));
    }

    if args.contains(&"all".to_string()) || args.contains(&"day17".to_string()) {
        println!("{}", format!("--- day17:").underline().green());
        measure!(day17(print_string));
    }

    if args.contains(&"all".to_string()) || args.contains(&"day18".to_string()) {
        println!("{}", format!("--- day18:").underline().green());
        measure!(day18(print_string));
    }

    if args.contains(&"all".to_string()) || args.contains(&"day19".to_string()) {
        println!("{}", format!("--- day19:").underline().green());
        measure!(day19(print_usize));
    }

    if args.contains(&"all".to_string()) || args.contains(&"day20".to_string()) {
        println!("{}", format!("--- day20:").underline().green());
        measure!(day20(print_usize));
    }

    if args.contains(&"all".to_string()) || args.contains(&"day21".to_string()) {
        println!("{}", format!("--- day21:").underline().green());
        measure!(day21(print_usize));
    }

    if args.contains(&"all".to_string()) || args.contains(&"day22".to_string()) {
        println!("{}", format!("--- day22:").underline().green());
        measure!(day22(print_usize, print_i16));
    }

    if args.contains(&"all".to_string()) || args.contains(&"day23".to_string()) {
        println!("{}", format!("--- day23:").underline().green());
        measure!(day23(print_usize, print_string));
    }

    if args.contains(&"all".to_string()) || args.contains(&"day24".to_string()) {
        println!("{}", format!("--- day24:").underline().green());
        measure!(day24(print_usize, print_string));
    }

    if args.contains(&"all".to_string()) || args.contains(&"day25".to_string()) {
        println!("{}", format!("--- day25:").underline().green());
        measure!(day25(print_usize));
    }
}

fn print_usize(a: usize) {
    println!("{}", a);
}

fn print_i16(a: i16) {
    println!("{}", a);
}

fn print_i32(a: i32) {
    println!("{}", a);
}

fn print_u32(a: u32) {
    println!("{}", a);
}

fn print_i64(a: i64) {
    println!("{}", a);
}

fn print_string(a: String) {
    println!("{}", a);
}
