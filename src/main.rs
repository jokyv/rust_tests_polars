use polars::prelude::*;
use std::fs::File;

fn main() {
    println!("Hello, Coder");
    let path = "/home/jokyv/projects/trading_backtesting/backtrader/data/oracle.csv";
    let file = File::open(path).expect("Something went wrong");
    let lf1 = CsvReader::new(file).has_header(true).finish();
    println!("{:?}", lf1);
}
