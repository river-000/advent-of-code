use std::fs;
use advent_of_code::parse_commasep_numbers;
extern crate nalgebra as na;
use na::base::*;
use histogram::Histogram;

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

pub fn day() {
    let mut histogram = Histogram::new();

    let ns = parse("data/day6.txt").unwrap();
    println!("{:?}", ns);

    let mut v: Vector9<i64> = Vector9::from([0, 1, 1, 2, 1, 0, 0, 0, 0]);

    ns.iter().for_each(|i|{histogram.increment(*i as u64);});
    for bucket in &histogram {
        println!("{:?}", bucket);

        if bucket.value() > 8 {
            break;
        }

        v[bucket.value() as usize] = bucket.count() as i64;
    }

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


    println!("{:?}", m * v);
    println!("{:?}", (m.pow(1 as usize)).unwrap() * v);
    println!("{:?}", (m.pow(2 as usize)).unwrap() * v);
    println!("{:?}", (m.pow(3 as usize)).unwrap() * v);
    println!("{:?}", (m.pow(4 as usize)).unwrap() * v);

    println!("{:?}", (m.pow(80 as usize)).unwrap() * v);
    println!("{:?}", ((m.pow(80-1 as usize)).unwrap() * v).sum());
    println!("{:?}", ((m.pow(256-1 as usize)).unwrap() * v).sum());
}

/*
Initial state: 3,4,3,1,2          0 1 1 2 1 0 0 0 0
After  1 day:  2,3,2,0,1          1 1 2 1 0 0 0 0 0
After  2 days: 1,2,1,6,0,8        1 2 1 0 0 0 1 0 1
After  3 days: 0,1,0,5,6,7,8      2 1 0 0 0 1 1 1 1
After  4 days: 6,0,6,4,5,6,7,8,8  1 0 0 1 1 3 1 2 0

Matrix { data: [[1, 1, 2, 1, 0, 0, 0, 0, 0]] }
Matrix { data: [[1, 2, 1, 0, 0, 0, 1, 0, 1]] }
Matrix { data: [[2, 1, 0, 0, 0, 1, 1, 1, 1]] }
Matrix { data: [[1, 0, 0, 0, 1, 1, 3, 1, 2]] }

*/
