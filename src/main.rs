mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

use std::time::{SystemTime, UNIX_EPOCH};

fn time_it(f: fn() -> ()) {
    let start = SystemTime::now();
    f();
    let stop = SystemTime::now();
    let since_the_epoch = stop.duration_since(start).expect("Time went backwards");
    println!("{:?}", since_the_epoch);
    println!("")
}

fn main() {
    time_it(day1::day);
    time_it(day2::day);
    time_it(day3::day);
    time_it(day4::day);
    time_it(day5::day);
}
