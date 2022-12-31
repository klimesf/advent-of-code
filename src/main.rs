use std::env;
use crate::y2017::y2017;
use crate::y2022::y2022;

mod y2017;
mod y2022;

mod utils { pub mod toolbox; }

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.contains(&"2017".to_string()) {
        y2017();
    }
    if args.contains(&"2022".to_string()) {
        y2022();
    }
}
