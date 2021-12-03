use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// parse a vector of bits
fn parse_zero(i: &str) -> nom::IResult<&str, bool> {
    let (i, _) = nom::bytes::complete::tag("0")(i)?;
    Ok((i, false))
}

fn parse_one(i: &str) -> nom::IResult<&str, bool> {
    let (i, _) = nom::bytes::complete::tag("1")(i)?;
    Ok((i, true))
}

fn parse_command(i: &str) -> nom::IResult<&str, Vec<bool>> {
    let (i, r) = nom::multi::many0(nom::branch::alt((parse_zero, parse_one)))(i)?;
    Ok((i, r))
}

pub fn day3_parse(filename: &str) -> Result<Vec<Vec<bool>>, ()> {
    let mut result = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(line) = line {
                match parse_command(&line) {
                    Ok((_, m)) => {
                        result.push(m);
                    }
                    Err(_) => return Err(()),
                }
            }
        }
    }

    Ok(result)
}

fn bits_to_number(bits: Vec<bool>) -> u64 {
    let mut w = 0;

    for bit in bits.iter() {
        w <<= 1;
        if *bit {
            w += 1;
        }
    }

    w
}

// https://stackoverflow.com/questions/29530011/creating-a-vector-of-zeros-for-a-specific-size
fn zeros(size: usize) -> Vec<u64> {
    let mut zero_vec: Vec<u64> = Vec::with_capacity(size);
    for i in 0..size {
        zero_vec.push(0);
    }
    return zero_vec;
}

pub fn day3_part1_helper(input: &Vec<Vec<bool>>) -> (u64, Vec<u64>) {
    let mut sums = zeros(input[0].len());

    for col in 0..input[0].len() {
        for row in 0..input.len() {
            if input[row][col] {
                sums[col] += 1
            }
        }
    }

    (input.len().try_into().unwrap(), sums)
}

pub fn day3_part1_solve(input: &Vec<Vec<bool>>) -> u64 {
    let (cols, sums) = day3_part1_helper(&input);

    let mut mcb_result = Vec::new();
    let mut lcb_result = Vec::new();

    for sum in sums {
        let mcb = 2 * sum >= cols;

        mcb_result.push(mcb);
        lcb_result.push(!mcb);
    }

    bits_to_number(mcb_result) * bits_to_number(lcb_result)
}

/*
pub fn day3_part1_solve(input: &Vec<Vec<bool>>) -> u64 {
    let (cols, sums) = day3_part1_helper(&input);


}
*/
