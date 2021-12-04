use std::fs;

//use advent_of_code::parse_number;
use advent_of_code::parse_commasep_numbers;
use advent_of_code::parse_grid_numbers;
use advent_of_code::parse_whitespacesep_numbers;

use std::collections::HashMap;
use std::slice::IterMut;

pub fn parse_day4_grid(i: &str) -> nom::IResult<&str, Vec<Vec<u64>>> {
    let (i, _) = nom::character::complete::newline(i)?;
    let (i, _) = nom::character::complete::newline(i)?;
    let (i, g) = parse_grid_numbers(i)?;

    Ok((i, g))
}

pub fn parse_day4_grids(i: &str) -> nom::IResult<&str, Vec<Vec<Vec<u64>>>> {
    nom::multi::many1(parse_day4_grid)(i)
}

pub fn parse_day4(i: &str) -> nom::IResult<&str, (Vec<u64>, Vec<Vec<Vec<u64>>>)> {
    let (i, ns) = parse_commasep_numbers(i)?;
    let (i, gs) = parse_day4_grids(i)?;

    Ok((i, (ns, gs)))
}

pub fn parse(filename: &str) -> Result<(Vec<u64>, Vec<Vec<Vec<u64>>>), ()> {
    let text = fs::read_to_string(filename).unwrap();

    match parse_day4(&text) {
        Ok((_, (ns, gs))) => {
            Ok((ns, gs))
        }
        Err(_) => Err(()),
    }
}

fn construct_coordinate_lookup(g1: &Vec<Vec<u64>>) -> HashMap<u64, (usize, usize)> {
    let mut book_reviews = HashMap::new();

    for row in 0..g1.len() {
        for col in 0..g1[0].len() {
            let cell = g1[row][col];
            book_reviews.insert(cell, (row, col));
        }
    }

    book_reviews
}

fn construct_bitgrid(g1: &Vec<Vec<u64>>) -> Vec<Vec<bool>> {
    let mut bitgrid = Vec::new();

    for _ in 0..g1.len() {
        let mut row = Vec::new();
        for _ in 0..g1[0].len() {
            row.push(false);
        }
        bitgrid.push(row);
    }

    bitgrid
}

fn bingo_check(g1: &Vec<Vec<bool>>) -> bool {
    // cols
    for row in 0..g1.len() {
        let mut ok = true;
        for col in 0..g1[0].len() {
            if !g1[row][col] {
                ok = false;
                break;
            }
        }
        if ok {
            return true;
        }
    }

    // rows
    for col in 0..g1[0].len() {
        let mut ok = true;
        for row in 0..g1.len() {
            if !g1[row][col] {
                ok = false;
                break;
            }
        }
        if ok {
            return true;
        }
    }

    false
}

pub fn solve(numbers: &Vec<u64>, grids: &Vec<Vec<Vec<u64>>>) {
    let book_reviews = grids.into_iter().map(|g| construct_coordinate_lookup(g));
    let mut bit_grids = grids.into_iter().map(|g| construct_bitgrid(g));
    let both = book_reviews.zip(bit_grids);

    for bingo in numbers {
        for (book_review, mut bit_grid) in both
        {
            if let Some((row, col)) = book_review.get(bingo) {
                bit_grid[*row][*col] = true;
                if bingo_check(&bit_grid) {
                    println!("bingo {:?}", bit_grid);
                    break;
                }
            }
        }
    }

    //println!("{:?}", book_reviews);
}
