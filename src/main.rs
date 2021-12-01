use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn day1_parse(filename: &str) -> Result<Vec<u64>,()> {
    let mut v = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(ip) = line {
                let my_int: u64 = ip.parse().unwrap();
                v.push(my_int);
            }
        }
    }

    return Ok(v);
}

fn day1_part1_solve(data: &Vec<u64>) -> u64 {
    let mut counter = 0;
    let mut previous_line = data[0];

    for datum in &data[1..] {
        let line = *datum;
        if line > previous_line {
            counter = counter+1;
        }
        previous_line = line;
    }

    return counter;
}

/*
fn day1_part2_solve(data: &Vec<u64>) -> u64 {
    let window_len = 3;

    let mut counter = 0;
    let mut previous_window = &data[0..0+window_len];

    let mut i = 1;
    while (i <= data.len() - window_len) {
        let window = &data[i..i+window_len];
        println!("{:?}", window);
        i = i + 1;
        previous_window = window;
    }

    return 0;
}
*/

fn day1_part2_solve(data: &Vec<u64>) -> u64 {
    let mut counter = 0;

    let window_size = 3;
    let mut window = 0;
    let mut previous_window;

    let mut i = 0;
    while i < 3 {
        window = window + &data[i];
        i = i + 1;
    }

    let mut i = 0;
    while i < data.len() - window_size {
        previous_window = window;

        window = window - &data[i] + &data[i+window_size];

        if window > previous_window {
            counter = counter + 1;
        }

        i = i + 1;
    }

    return counter;
}

fn main() {
    let filename = "data/day1.txt";
    let data = day1_parse(filename).unwrap();

    //println!("{:?}", data);

    let answer = day1_part1_solve(&data);
    println!("{}", answer);

    let answer = day1_part2_solve(&data);
    println!("{}", answer);
}
