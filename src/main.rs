use std::env;
use soda::Soda;
use bill::Bill;
use coin::Coin;

#[path="../src/models/soda.rs"]
pub mod soda;

#[path="../src/models/coin.rs"]
pub mod coin;

#[path="../src/models/bill.rs"]
pub mod bill;

fn main() {
    let args: Vec<String> = env::args.collect();
}