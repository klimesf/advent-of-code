extern crate itertools;
extern crate regex;

use colored::Colorize;
use std::env;
use crate::measure;
use crate::y2023::day01::day01;

mod day01;

pub(crate) fn y2023() {
    let args: Vec<String> = env::args().collect();
    println!();
    println!("{}", format!("Advent of Code").red());
    println!("{}", format!("        //2023").red());
    println!();

    if args.contains(&"all".to_string()) || args.contains(&"day01".to_string()) {
        println!("{}", format!("--- day01:").underline().green());
        measure!(day01());
    }
}
