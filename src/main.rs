use polars::lazy::frame::LazyCsvReader;
use polars::prelude::*;
// use polars_io::prelude::*;
// use std::fs::File;
use std::thread::sleep;
use std::time::{Duration, Instant};

fn main() {
    // time the operation
    let start_time_now: Instant = Instant::now();

    // read the file from the path
    let path = "/home/jokyv/projects/trading_backtesting/backtrader/data/oracle.csv";
    // let file = File::open(path).expect("Something went wrong");

    // normal_csv_reader(&path);
    lazy_csv_reader(&path);

    // create a variable Duration
    let dur_5_secs: Duration = Duration::new(2, 0);
    // sleep for 5 seconds
    sleep(dur_5_secs);
    // save the duration for the program to run + the sleep time
    let elapsed_time: Duration = start_time_now.elapsed();
    // print the elapsed time
    println!("time took to run the program: {:?}", elapsed_time);
}

// fn normal_csv_reader(path: &str) {
//     let df1 = CsvReader::new(path.into())
//         .has_header(true)
//         .finish()
//         .expect("Error opening file");
//     println!("{:?}", df1.head(Some(3)));
// }

fn lazy_csv_reader(path: &str) {
    let lf1 = LazyCsvReader::new(path.into())
        .has_header(true)
        .finish()
        .expect("Error opening file")
        .collect()
        .expect("error");
    let count: f64 = 10.0;
    let lf2 = lf1
        .filter(col("Close").gt_eq(lit(count)))
        .collect()
        .unwrap();
    // get when close is above 10
    println!("{:?}", lf2);
    // print the head, takes Some
    println!("{:?}", lf1.head(Some(3)));
    // print the shape of the df
    println!("{:?}", lf1.shape());
}
