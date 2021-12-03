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

pub fn day3_part1_solve(input: &Vec<Vec<bool>>) -> u64 {
    let mut mcb_result = Vec::new();
    let mut lcb_result = Vec::new();

    for col in 0..input[0].len() {
        let mut zero_bits = 0;
        let mut one_bits = 0;

        for row in 0..input.len() {
            if input[row][col] {
                one_bits += 1
            }
            else {
                zero_bits += 1
            }
        }

        let mcb = one_bits > zero_bits;
        mcb_result.push(mcb);
        lcb_result.push(!mcb);
    }

    bits_to_number(mcb_result) * bits_to_number(lcb_result)
}
