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

fn main() {
    let filename = "data/day1.example.txt";
    let data = day1_parse(filename).unwrap();
    println!("{:?}", data);
}
