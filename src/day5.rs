use std::io::{self, BufRead};
use advent_of_code::read_lines;
use advent_of_code::parse_number;

#[derive(Debug)]
pub struct Line {
    from: (i64, i64),
    to: (i64, i64),
}

fn parse_point(i: &str) -> nom::IResult<&str, (i64, i64)> {
    let (i, a) = parse_number(i)?;
    let (i, _) = nom::bytes::complete::tag(",")(i)?;
    let (i, b) = parse_number(i)?;

    Ok((i, (a,b)))
}

// parse a vector of bits
fn parse_line(i: &str) -> nom::IResult<&str, Line> {
    let (i, a) = parse_point(i)?;
    let (i, _) = nom::character::complete::space1(i)?;
    let (i, _) = nom::bytes::complete::tag("->")(i)?;
    let (i, _) = nom::character::complete::space1(i)?;
    let (i, b) = parse_point(i)?;

    Ok((i, Line { from: a, to: b }))
}

pub fn day5_parse(filename: &str) -> Result<Vec<Line>, ()> {
    let mut result = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(line) = line {
                match parse_line(&line) {
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

//

use advent_of_code::implement_day;
#[cfg(test)]
use advent_of_code::implement_test;

const NO: usize = 5;

pub fn day() {
    let name = advent_of_code::filename(NO, ".example");
    let data = day5_parse(&name).unwrap();
    println!("{:?}", data)
    //implement_day(NO, "", day5_parse, day5_part1_solve, day5_part2_solve);
}

/*
#[cfg(test)]
mod tests {
    use crate::day3::*;

    #[test]
    pub fn part1() {
        implement_test(NO, "", day5_parse, day5_part1_solve, 3374136);
    }

    #[test]
    pub fn part2() {
        implement_test(NO, "", day5_parse, day5_part2_solve, 4432698);
    }
}
*/
