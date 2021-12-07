mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

use std::time::SystemTime;

fn time_it(n: usize, f: fn() -> ()) {
    println!("day {}", n);
    let start = SystemTime::now();
    f();
    let stop = SystemTime::now();
    let since_the_epoch = stop.duration_since(start).expect("Time went backwards");
    println!("{:?}", since_the_epoch);
    println!("")
}

fn main() {
    time_it(1,day1::day);
    time_it(2,day2::day);
    time_it(3,day3::day);
    time_it(4,day4::day);
    time_it(5,day5::day);
    time_it(6,day6::day);
    time_it(7,day7::day);
}
