use std::fs;

//use advent_of_code::parse_number;
use advent_of_code::parse_commasep_numbers;
use advent_of_code::parse_grid_numbers;

use std::collections::HashMap;

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

pub fn day4_parse(filename: &str) -> Result<(Vec<u64>, Vec<Vec<Vec<u64>>>), ()> {
    let text = fs::read_to_string(filename).unwrap();

    match parse_day4(&text) {
        Ok((_, (ns, gs))) => Ok((ns, gs)),
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

fn sum_unmarked_numbers(numbers: &Vec<Vec<u64>>, bits: &Vec<Vec<bool>>) -> u64 {
    numbers
        .into_iter()
        .zip(bits.into_iter())
        .map(|(a, b)| {
            a.into_iter()
                .zip(b.into_iter())
                .map(|(x, y)| if *y { 0 } else { *x })
                .sum::<u64>()
        })
        .sum::<u64>()
}

pub fn day4_part1_solve((numbers, grids): &(Vec<u64>, Vec<Vec<Vec<u64>>>)) -> u64 {
    let book_reviews: Vec<HashMap<u64, (usize, usize)>> = grids
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

pub fn day4_part2_solve((numbers, grids): &(Vec<u64>, Vec<Vec<Vec<u64>>>)) -> u64 {
    let book_reviews: Vec<HashMap<u64, (usize, usize)>> = grids
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

pub fn day() {
    let filename = "data/day4.txt";
    let data = day4_parse(filename).unwrap();
    let pt1 = day4_part1_solve(&data);
    let pt2 = day4_part2_solve(&data);
    println!("{} {}", pt1, pt2);
}

#[cfg(test)]
mod tests {
    use crate::day4::*;

    #[test]
    pub fn part1() {
        let filename = "data/day4.txt";
        let data = day4_parse(filename).unwrap();
        let answer = day4_part1_solve(&data);
        assert_eq!(answer, 58374);
    }

    #[test]
    pub fn part2() {
        let filename = "data/day4.txt";
        let data = day4_parse(filename).unwrap();
        let answer = day4_part2_solve(&data);
        assert_eq!(answer, 11377);
    }
}
