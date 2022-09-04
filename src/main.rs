use polars::df;
use polars::lazy::frame::LazyCsvReader;
use polars::prelude::*;
// use polars_io::prelude::*;
// use std::fs::File;
use std::thread::sleep;
use std::time::{Duration, Instant};

// MAIN
// -------------------------------------------------------------
fn main() {
    // time the operation
    let start_time_now: Instant = Instant::now();

    // read the file from the path
    let path = "oracle.csv";
    // let file = File::open(path).expect("Something went wrong");

    // create a dummy df
    // -------------------------------------------------------------
    let df = df![
        "a" => [1,2,3,4,5],
        "b" => [None, Some("alas"), Some("sala"), None, None]
    ];
    println!("{:?}", df);

    // create df with functions
    // -------------------------------------------------------------
    normal_csv_reader(&path);
    lazy_csv_reader(&path);

    // Duration
    // -------------------------------------------------------------
    // create a variable Duration
    let dur_secs: Duration = Duration::new(2, 0);
    // sleep for 5 seconds
    sleep(dur_secs);
    // save the duration for the program to run + the sleep time
    let elapsed_time: Duration = start_time_now.elapsed();
    // print the elapsed time
    println!("time took to run the program: {:?}", elapsed_time);
}

// FUNCTIONS
// -------------------------------------------------------------
fn normal_csv_reader(path: &str) {
    let df1 = CsvReader::from_path(path)
        .unwrap()
        .has_header(true)
        .finish()
        .expect("Error opening file");
    println!("{:?}", df1.head(Some(3)));
}

fn lazy_csv_reader(path: &str) {
    let lf1 = LazyCsvReader::new(path.to_string())
        .has_header(true)
        .finish()
        .expect("Error opening file");
    println!("{:?}", lf1.head(Some(3)));
}
