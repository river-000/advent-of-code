use std::io::BufRead;
use std::str::FromStr;

// https://github.com/Geal/nom/blob/main/doc/choosing_a_combinator.md
use nom::{
    character::complete::{char, one_of},
    combinator::recognize,
    multi::{many0, many1},
    sequence::terminated,
};

// https://github.com/benkay86/nom-tutorial#alternative-final-parser
/// Type-erased errors.
/*
pub type BoxError = std::boxed::Box<
    dyn std::error::Error // must implement Error to satisfy ?
        + std::marker::Send // needed for threads
        + std::marker::Sync, // needed for threads
>;
*/

#[derive(Debug)]
pub enum Movement {
    Forward { x: i64 },
    Down { x: i64 },
}

// https://github.com/Geal/nom/blob/main/doc/nom_recipes.md
fn decimal(input: &str) -> nom::IResult<&str, i64> {
    let (i, s) = recognize(many1(terminated(one_of("0123456789"), many0(char('_')))))(input)?;

    let n = i64::from_str(s).unwrap(); // TODO

    Ok((i, n))
}

fn parse_forward(i: &str) -> nom::IResult<&str, Movement> {
    let (i, _) = nom::bytes::complete::tag("forward")(i)?;
    let (i, _) = nom::character::complete::space1(i)?;
    let (i, n) = decimal(i)?;

    Ok((i, Movement::Forward { x: n }))
}

fn parse_up(i: &str) -> nom::IResult<&str, Movement> {
    let (i, _) = nom::bytes::complete::tag("up")(i)?;
    let (i, _) = nom::character::complete::space1(i)?;
    let (i, n) = decimal(i)?;

    Ok((i, Movement::Down { x: -n }))
}

fn parse_down(i: &str) -> nom::IResult<&str, Movement> {
    let (i, _) = nom::bytes::complete::tag("down")(i)?;
    let (i, _) = nom::character::complete::space1(i)?;
    let (i, n) = decimal(i)?;

    Ok((i, Movement::Down { x: n }))
}

fn parse_command(i: &str) -> nom::IResult<&str, Movement> {
    let (i, r) = nom::branch::alt((parse_forward, parse_up, parse_down))(i)?;
    Ok((i, r))
}

pub fn day2_parse(filename: &str) -> Result<Vec<Movement>, ()> {
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

pub fn day2_part1_solve(v: &[Movement]) -> i64 {
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

pub fn day2_part2_solve(v: &[Movement]) -> i64 {
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
