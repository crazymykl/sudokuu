mod board;
mod possibilites;
mod programmed_iterator;
mod square;

use crate::board::Board;
use std::io::{self, BufRead};
use std::str::FromStr;

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        solve_str(&line.expect("Can't read stdin!"))
    }
}

fn solve_str(arg: &str) {
    match Board::from_str(&arg) {
        Ok(x) => println!(
            "{}\n{}",
            x,
            x.solve()
                .map(|b| b.to_string())
                .unwrap_or_else(|| "No solution".into())
        ),
        e => println!("{:?}", &e),
    }
}
