use std::fs;

//use advent_of_code::parse_number;
use advent_of_code::parse_commasep_numbers;
use advent_of_code::parse_grid_numbers;

use std::collections::HashMap;

pub fn parse_day4_grid(i: &str) -> nom::IResult<&str, Vec<Vec<i64>>> {
    let (i, _) = nom::character::complete::newline(i)?;
    let (i, _) = nom::character::complete::newline(i)?;
    let (i, g) = parse_grid_numbers(i)?;

    Ok((i, g))
}

pub fn parse_day4_grids(i: &str) -> nom::IResult<&str, Vec<Vec<Vec<i64>>>> {
    nom::multi::many1(parse_day4_grid)(i)
}

pub fn parse_day4(i: &str) -> nom::IResult<&str, (Vec<i64>, Vec<Vec<Vec<i64>>>)> {
    let (i, ns) = parse_commasep_numbers(i)?;
    let (i, gs) = parse_day4_grids(i)?;

    Ok((i, (ns, gs)))
}

pub fn day4_parse(filename: &str) -> Result<(Vec<i64>, Vec<Vec<Vec<i64>>>), ()> {
    let text = fs::read_to_string(filename).unwrap();

    match parse_day4(&text) {
        Ok((_, (ns, gs))) => Ok((ns, gs)),
        Err(_) => Err(()),
    }
}

fn construct_coordinate_lookup(g1: &Vec<Vec<i64>>) -> HashMap<i64, (usize, usize)> {
    let mut book_reviews = HashMap::new();

    for row in 0..g1.len() {
        for col in 0..g1[0].len() {
            let cell = g1[row][col];
            book_reviews.insert(cell, (row, col));
        }
    }

    book_reviews
}

fn construct_bitgrid(g1: &Vec<Vec<i64>>) -> Vec<Vec<bool>> {
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

fn sum_unmarked_numbers(numbers: &Vec<Vec<i64>>, bits: &Vec<Vec<bool>>) -> i64 {
    numbers
        .into_iter()
        .zip(bits.into_iter())
        .map(|(a, b)| {
            a.into_iter()
                .zip(b.into_iter())
                .map(|(x, y)| if *y { 0 } else { *x })
                .sum::<i64>()
        })
        .sum::<i64>()
}

pub fn day4_part1_solve((numbers, grids): &(Vec<i64>, Vec<Vec<Vec<i64>>>)) -> i64 {
    let book_reviews: Vec<HashMap<i64, (usize, usize)>> = grids
        .into_iter()
        .map(|g| construct_coordinate_lookup(g))
        .collect();
    let mut bit_grids: Vec<Vec<Vec<bool>>> =
        grids.into_iter().map(|g| construct_bitgrid(g)).collect();

    for bingo in numbers {
        for (grid, (book_review, bit_grid)) in grids
            .into_iter()
            .zip(book_reviews.iter().zip(bit_grids.iter_mut()))
        {
            if let Some((row, col)) = book_review.get(bingo) {
                bit_grid[*row][*col] = true;
                if bingo_check(&bit_grid) {
                    return bingo * sum_unmarked_numbers(&grid, &bit_grid);
                }
            }
        }
    }

    return 0;
}

pub fn day4_part2_solve((numbers, grids): &(Vec<i64>, Vec<Vec<Vec<i64>>>)) -> i64 {
    let book_reviews: Vec<HashMap<i64, (usize, usize)>> = grids
        .into_iter()
        .map(|g| construct_coordinate_lookup(g))
        .collect();
    let mut bit_grids: Vec<Vec<Vec<bool>>> =
        grids.into_iter().map(|g| construct_bitgrid(g)).collect();
    
    let mut answers = Vec::new();

    for bingo in numbers {
        for (grid, (book_review, bit_grid)) in grids
            .into_iter()
            .zip(book_reviews.iter().zip(bit_grids.iter_mut()))
        {
            if !bingo_check(&bit_grid) {
                if let Some((row, col)) = book_review.get(bingo) {
                    bit_grid[*row][*col] = true;
                    if bingo_check(&bit_grid) {
                        answers.push(bingo * sum_unmarked_numbers(&grid, &bit_grid));
                    }
                }
            }
        }
    }

    return *answers.last().unwrap();
}

//

use advent_of_code::implement_day;
#[cfg(test)]
use advent_of_code::implement_test;

const NO: usize = 4;

pub fn day() {
    implement_day(NO, "", day4_parse, day4_part1_solve, day4_part2_solve);
}

#[cfg(test)]
mod tests {
    use crate::day4::*;

    #[test]
    pub fn part1() {
        implement_test(NO, "", day4_parse, day4_part1_solve, 58374);
    }

    #[test]
    pub fn part2() {
        implement_test(NO, "", day4_parse, day4_part2_solve, 11377);
    }
}
