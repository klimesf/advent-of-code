extern crate itertools;
extern crate regex;

use colored::Colorize;
use std::env;
use crate::measure;
use crate::y2025::day01::day01;
use crate::y2025::day02::day02;
use crate::y2025::day03::day03;
use crate::y2025::day04::day04;
use crate::y2025::day05::day05;
use crate::y2025::day06::day06;
use crate::y2025::day07::day07;

pub mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;

pub fn y2025() {
    let args: Vec<String> = env::args().collect();
    println!();
    println!("{}", format!("Advent of Code").red());
    println!("{}", format!("        //2025").red());
    println!();

    if args.contains(&"all".to_string()) || args.contains(&"day01".to_string()) {
        println!("{}", format!("--- day01:").underline().green());
        measure!(day01(print_i32));
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
}

#[allow(dead_code)]
fn print_usize(a: usize) {
    println!("{}", a);
}

#[allow(dead_code)]
fn print_i16(a: i16) {
    println!("{}", a);
}

#[allow(dead_code)]
fn print_i32(a: i32) {
    println!("{}", a);
}

#[allow(dead_code)]
fn print_u32(a: u32) {
    println!("{}", a);
}

#[allow(dead_code)]
fn print_i64(a: i64) {
    println!("{}", a);
}

#[allow(dead_code)]
fn print_string(a: String) {
    println!("{}", a);
}
