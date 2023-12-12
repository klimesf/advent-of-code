extern crate itertools;
extern crate regex;

use colored::Colorize;
use std::env;
use crate::measure;
use crate::y2016::day01::day01;
use crate::y2016::day02::day02;
use crate::y2016::day03::day03;

mod day01;
mod day02;
mod day03;

pub(crate) fn y2016() {
    let args: Vec<String> = env::args().collect();
    println!();
    println!("{}", format!("Advent of Code").red());
    println!("{}", format!("        //2016").red());
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
}
