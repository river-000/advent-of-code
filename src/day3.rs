use advent_of_code::read_lines;

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

pub fn parse(filename: &str) -> Result<Vec<Vec<bool>>, ()> {
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

fn bits_to_number(bits: &Vec<bool>) -> i64 {
    let mut w = 0;

    for bit in bits.iter() {
        w <<= 1;
        if *bit {
            w += 1;
        }
    }

    w
}

pub fn day3_part1_helper(input: &Vec<Vec<bool>>) -> (Vec<bool>, Vec<bool>) {
    let mut mcb_result = Vec::new();
    let mut lcb_result = Vec::new();

    for col in 0..input[0].len() {
        let mut zero_bits = 0;
        let mut one_bits = 0;

        for row in 0..input.len() {
            if input[row][col] {
                one_bits += 1
            } else {
                zero_bits += 1
            }
        }

        let mcb = one_bits > zero_bits;
        let lcb = one_bits < zero_bits;
        mcb_result.push(mcb);
        lcb_result.push(lcb);
    }

    (mcb_result, lcb_result)
}

pub fn part1_solve(input: &Vec<Vec<bool>>) -> i64 {
    let (mcb_result, lcb_result) = day3_part1_helper(&input);
    bits_to_number(&mcb_result) * bits_to_number(&lcb_result)
}

fn count_msb_lsb_in_col(input: &Vec<Vec<bool>>, col: usize) -> (bool, bool) {
    let mut zero_bits = 0;
    let mut one_bits = 0;
    for row in 0..input.len() {
        if input[row][col] {
            one_bits += 1
        } else {
            zero_bits += 1
        }
    }

    (one_bits > zero_bits, one_bits < zero_bits)
}

pub fn part2_solve(input: &Vec<Vec<bool>>) -> i64 {
    let num_cols = input[0].len();
    let mut msb_filtered: Vec<Vec<bool>> = input.clone();
    let mut lsb_filtered: Vec<Vec<bool>> = input.clone();

    for col in 0..num_cols {
        let (msb_in_col, lsb_in_col) = count_msb_lsb_in_col(&msb_filtered, col);
        msb_filtered.retain(|x| {
            if msb_in_col || lsb_in_col {
                x[col] == msb_in_col
            } else {
                x[col] == true
            }
        });
        if msb_filtered.len() == 1 {
            break;
        }
    }

    for col in 0..num_cols {
        let (msb_in_col, lsb_in_col) = count_msb_lsb_in_col(&lsb_filtered, col);
        lsb_filtered.retain(|x| {
            if msb_in_col || lsb_in_col {
                x[col] == lsb_in_col
            } else {
                x[col] == false
            }
        });
        if lsb_filtered.len() == 1 {
            break;
        }
    }

    bits_to_number(&msb_filtered[0]) * bits_to_number(&lsb_filtered[0])
}

//

use advent_of_code::implement_day;
#[cfg(test)]
use advent_of_code::implement_test;

const NO: usize = 3;

pub fn day() {
    implement_day(NO, "", parse, part1_solve, part2_solve);
}

#[cfg(test)]
mod tests {
    use crate::day3::*;

    #[test]
    pub fn part1() {
        implement_test(NO, "", parse, part1_solve, 3374136);
    }

    #[test]
    pub fn part2() {
        implement_test(NO, "", parse, part2_solve, 4432698);
    }
}
