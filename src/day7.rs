use advent_of_code::parse_commasep_numbers;
use std::fs;
use stats::*;

pub fn parse_day6(i: &str) -> nom::IResult<&str, Vec<i64>> {
    let (i, ns) = parse_commasep_numbers(i)?;

    Ok((i, ns))
}

pub fn parse(filename: &str) -> Result<Vec<i64>, ()> {
    let text = fs::read_to_string(filename).unwrap();

    match parse_day6(&text) {
        Ok((_, ns)) => Ok(ns),
        Err(_) => Err(()),
    }
}

fn triangular(n: i64) -> i64 {
    n * (n + 1) / 2
}

fn part1_solve(data: &Vec<i64>) -> i64 {
    let median = stats::median(data.iter().copied()).unwrap() as i64;
    data.iter().map(|x| (x - median).abs()).sum()
}

fn part2_solve(data: &Vec<i64>) -> i64 {
    let count = data.len() as i64;
    let sum = data.iter().sum();
    let pos = (sum + data.iter().filter(|&&x| x * count > sum).count() as i64)/count;
    data.iter().map(|x| triangular((x - pos).abs())).sum()
}

//

use advent_of_code::implement_day;
#[cfg(test)]
use advent_of_code::implement_test;

const NO: usize = 7;

pub fn day() {
    implement_day(NO, "", parse, part1_solve, part2_solve);
}

#[cfg(test)]
mod tests {
    use crate::day7::*;

    #[test]
    pub fn part1() {
        implement_test(NO, "", parse, part1_solve, 353800);
    }

    #[test]
    pub fn part2() {
        implement_test(NO, "", parse, part2_solve, 98119739);
    }
}
