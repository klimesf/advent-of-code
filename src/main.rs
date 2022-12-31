use std::env;
use crate::y2022::y2022;

mod y2022;
mod utils { pub mod toolbox; pub mod io; }

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.contains(&"2022".to_string()) {
        y2022();
    }
}
