use std::env;
use soda::Soda;
#[path="../src/models/Soda.rs"]
pub mod soda;

fn main() {
    let args: Vec<String> = env::args.collect();
}