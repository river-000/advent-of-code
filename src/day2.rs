use std::io::BufRead;
use advent_of_code::parse_number;

#[derive(Debug)]
pub enum Movement {
    Forward { x: i64 },
    Down { x: i64 },
}

fn parse_forward(i: &str) -> nom::IResult<&str, Movement> {
    let (i, _) = nom::bytes::complete::tag("forward")(i)?;
    let (i, _) = nom::character::complete::space1(i)?;
    let (i, n) = parse_number(i)?;

    Ok((i, Movement::Forward { x: n }))
}

fn parse_up(i: &str) -> nom::IResult<&str, Movement> {
    let (i, _) = nom::bytes::complete::tag("up")(i)?;
    let (i, _) = nom::character::complete::space1(i)?;
    let (i, n) = parse_number(i)?;

    Ok((i, Movement::Down { x: -n }))
}

fn parse_down(i: &str) -> nom::IResult<&str, Movement> {
    let (i, _) = nom::bytes::complete::tag("down")(i)?;
    let (i, _) = nom::character::complete::space1(i)?;
    let (i, n) = parse_number(i)?;

    Ok((i, Movement::Down { x: n }))
}

fn parse_command(i: &str) -> nom::IResult<&str, Movement> {
    let (i, r) = nom::branch::alt((parse_forward, parse_up, parse_down))(i)?;
    Ok((i, r))
}

pub fn parse(filename: &str) -> Result<Vec<Movement>, ()> {
    let mut result: Vec<Movement> = Vec::new();

    let file = std::fs::File::open(filename).unwrap();
    let buf_reader = std::io::BufReader::new(file);
    for line in buf_reader.lines() {
        match parse_command(&line.unwrap()) {
            Ok((_, m)) => {
                result.push(m);
            }
            Err(_) => return Err(()),
        }
    }

    Ok(result)
}

//

pub fn part1_solve(v: &Vec<Movement>) -> i64 {
    let mut hori = 0;
    let mut depth = 0;

    for movement in v {
        match movement {
            Movement::Forward { x } => hori += x,
            Movement::Down { x } => depth += x,
        }
    }

    hori * depth
}

pub fn part2_solve(v: &Vec<Movement>) -> i64 {
    let mut hori = 0;
    let mut depth = 0;
    let mut aim = 0;

    for movement in v {
        match movement {
            Movement::Down { x } => aim += x,
            Movement::Forward { x } => {
                hori += x;
                depth += aim * x;
            },
        }
    }

    hori * depth
}

//

use advent_of_code::implement_day;
#[cfg(test)]
use advent_of_code::implement_test;

const NO: usize = 2;

pub fn day() {
    implement_day(NO, "", parse, part1_solve, part2_solve);
}

#[cfg(test)]
mod tests {
    use crate::day2::*;

    #[test]
    pub fn part1() {
        implement_test(NO, "", parse, part1_solve, 2272262);
    }

    #[test]
    pub fn part2() {
        implement_test(NO, "", parse, part2_solve, 2134882034);
    }
}
