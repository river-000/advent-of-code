use advent_of_code::parse_commasep_numbers;
use std::fs;
extern crate nalgebra as na;
use histogram::Histogram;
use na::base::*;

type Vector9<T> = Matrix<T, U9, U1, ArrayStorage<T, 9, 1>>;
type Matrix9x9<T> = Matrix<T, U9, U9, ArrayStorage<T, 9, 9>>;

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

fn compute_vector(ns: &Vec<i64>) -> Vector9<i64> {
    let mut histogram = Histogram::new();
    let mut v: Vector9<i64> = Vector9::from([0, 0, 0, 0, 0, 0, 0, 0, 0]);

    ns.iter().for_each(|i| {
        histogram.increment(*i as u64);
    });
    for bucket in &histogram {
        if bucket.value() > 8 {
            break;
        }

        v[bucket.value() as usize] = bucket.count() as i64;
    }
    return v;
}

fn part1_solve(data: &Vec<i64>) -> i64 {
    let v = compute_vector(data);

    let m: Matrix9x9<i64> = Matrix9x9::from([
        [0, 0, 0, 0, 0, 0, 1, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 1, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 1, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 1, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 1, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 1, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 1, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 1, 0],
    ]);

    ((m.pow(80 - 1 as usize)).unwrap() * v).sum()
}

fn part2_solve(data: &Vec<i64>) -> i64 {
    let v = compute_vector(data);

    let m: Matrix9x9<i64> = Matrix9x9::from([
        [0, 0, 0, 0, 0, 0, 1, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 1, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 1, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 1, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 1, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 1, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 1, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 1, 0],
    ]);

    ((m.pow(256 - 1 as usize)).unwrap() * v).sum()
}

//

use advent_of_code::implement_day;
#[cfg(test)]
use advent_of_code::implement_test;

const NO: usize = 6;

pub fn day() {
    implement_day(NO, "", parse, part1_solve, part2_solve);
}

#[cfg(test)]
mod tests {
    use crate::day6::*;

    #[test]
    pub fn part1() {
        implement_test(NO, "", parse, part1_solve, 390011);
    }

    #[test]
    pub fn part2() {
        implement_test(NO, "", parse, part2_solve, 1746710169834);
    }
}
