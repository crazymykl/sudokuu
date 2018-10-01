use std::env;

mod board;
mod programmed_iterator;
mod square;

use self::board::Board;
use std::str::FromStr;

fn main() {
    for arg in env::args().skip(1) {
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
}