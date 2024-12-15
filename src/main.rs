use std::env;
use crate::y2015::y2015;
use crate::y2016::y2016;
use crate::y2017::y2017;
use crate::y2018::y2018;
use crate::y2019::y2019;
use crate::y2022::y2022;
use crate::y2023::y2023;
use crate::y2024::y2024;

mod y2015;
mod y2016;
mod y2017;
mod y2018;
mod y2019;
mod y2022;
mod y2023;
mod y2024;

mod utils {
    pub mod grid;
    pub mod toolbox;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.contains(&"2015".to_string()) {
        y2015();
    }
    if args.contains(&"2016".to_string()) {
        y2016();
    }
    if args.contains(&"2017".to_string()) {
        y2017();
    }
    if args.contains(&"2018".to_string()) {
        y2018();
    }
    if args.contains(&"2019".to_string()) {
        y2019();
    }
    if args.contains(&"2022".to_string()) {
        y2022();
    }
    if args.contains(&"2023".to_string()) {
        y2023();
    }
    if args.contains(&"2024".to_string()) {
        y2024();
    }
}
