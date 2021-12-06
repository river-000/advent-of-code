use advent_of_code::parse_number;
use advent_of_code::read_lines;
use std::collections::HashMap;
use std::hash::Hash;
use std::collections::hash_set::HashSet;
use itertools::Itertools;
use std::cmp::Ordering;

#[derive(PartialEq)]
#[derive(Debug)]
enum LineType {
    Horizontal,
    Vertical,
    DiagonalDR,
    DiagonalDL,
}

#[derive(Debug)]
pub struct Line {
    from: (i64, i64),
    to: (i64, i64),
    typ: LineType,
}

impl Line {
    fn new(from: (i64, i64), to: (i64, i64)) -> Line {
        match (from.0.cmp(&to.0), from.1.cmp(&to.1)) {
            (Ordering::Less, Ordering::Less) =>
            Line{
                from: from,
                to: to,
                typ: LineType::DiagonalDR
            },
            (Ordering::Equal, Ordering::Less) =>
            Line{
                from: from,
                to: to,
                typ: LineType::Vertical
            },
            (Ordering::Greater, Ordering::Less) =>
            Line{
                from: from,
                to: to,
                typ: LineType::DiagonalDL
            },

            (Ordering::Less, Ordering::Equal) =>
            Line{
                from: from,
                to: to,
                typ: LineType::Horizontal
            },
            (Ordering::Equal, Ordering::Equal) =>
            Line{
                from: from,
                to: to,
                typ: LineType::Horizontal
            },
            (Ordering::Greater, Ordering::Equal) =>
            Line{
                from: to,
                to: from,
                typ: LineType::Horizontal
            },

            (Ordering::Less, Ordering::Greater) =>
            Line{
                from: to,
                to: from,
                typ: LineType::DiagonalDL
            },
            (Ordering::Equal, Ordering::Greater) =>
            Line{
                from: to,
                to: from,
                typ: LineType::Vertical
            },
            (Ordering::Greater, Ordering::Greater) =>
            Line{
                from: to,
                to: from,
                typ: LineType::DiagonalDR
            },
        }
    }
}

fn is_horizontal_or_vertical(l: &Line) -> bool {
    l.typ == LineType::Horizontal ||
    l.typ == LineType::Vertical
}

fn parse_point(i: &str) -> nom::IResult<&str, (i64, i64)> {
    let (i, a) = parse_number(i)?;
    let (i, _) = nom::bytes::complete::tag(",")(i)?;
    let (i, b) = parse_number(i)?;

    Ok((i, (a, b)))
}

// parse a vector of bits
fn parse_line(i: &str) -> nom::IResult<&str, Line> {
    let (i, a) = parse_point(i)?;
    let (i, _) = nom::character::complete::space1(i)?;
    let (i, _) = nom::bytes::complete::tag("->")(i)?;
    let (i, _) = nom::character::complete::space1(i)?;
    let (i, b) = parse_point(i)?;

    Ok((i, Line::new(a,b)))
}

pub fn parse(filename: &str) -> Result<Vec<Line>, ()> {
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

fn intersect_lines(intpoints: &mut HashSet<(i64,i64)>, a: &Line, b: &Line) {
    intersect_lines_helper(intpoints, a, b, false);
}

fn mirror_if(mirror: bool, a: (i64, i64)) -> (i64, i64) {
    if mirror {
        (a.1, a.0)
    }
    else {
        a
    }
}

fn mirror_line(a: Line) -> Line {
    Line::new(mirror_if(true, a.from), mirror_if(true, a.to))
}

fn intersect_lines_helper(intpoints: &mut HashSet<(i64,i64)>, a: &Line, b: &Line, mirror: bool) {
    //    intpoints.insert(a.from);
    match (&a.typ, &b.typ) {
        (LineType::Horizontal, LineType::Horizontal) => {
            // aaaaaaaaaaaaa
            //    bbbbbbbbbbbbbbb
            //    L        R
            //
            // aaaaaaaaaaaaa
            //      bbbb
            //      L  R
            //
            //       aaaaaaaa
            //   bbbbbbb
            //       L R
            //
            // aaaaa
            //       bbbbbb
            //     R L

            let l = std::cmp::max(a.from.0, b.from.0);
            let r = std::cmp::max(a.to.0, b.to.0);

            for xc in (l..r+1) {
                intpoints.insert(mirror_if(mirror, (xc, a.from.1)));
            }
        },
        (LineType::Horizontal, LineType::Vertical) => {
            //      b
            //      b
            // aaaaaxaaaaa
            //      b
            //      b
            //      b

            if a.from.0 <= b.from.0 && b.from.0 <= a.to.0 {
                if b.from.1 <= a.from.1 && a.from.1 <= a.to.1 {
                    intpoints.insert(mirror_if(mirror, (b.from.0, a.from.1)));
                }
            }
        },
        (LineType::Horizontal, LineType::DiagonalDR) => {
            //      b
            //       b
            // aaaaaaaxaaa
            //        ^b
            //        | b
            //       xc  b  = b.from.0 + 2
            //                  2 = a.from.1 - b.from.1

            let xc = b.from.0 + a.from.1 - b.from.1;

            if a.from.0 <= xc && xc <= a.to.0 {
                if b.from.1 <= a.from.1 && a.from.1 <= a.to.1 {
                    intpoints.insert(mirror_if(mirror, (b.from.0, a.from.1)));
                }
            }
        },
        (LineType::Horizontal, LineType::DiagonalDL) => {
            //      b
            //     b
            // aaaxaaaaaaa
            //   b

            let xc = b.from.0 - (a.from.1 - b.from.1);

            if a.from.0 <= xc && xc <= a.to.0 {
                if b.from.1 <= a.from.1 && a.from.1 <= a.to.1 {
                    intpoints.insert(mirror_if(mirror, (b.from.0, a.from.1)));
                }
            }
        },

        (LineType::Vertical, LineType::Horizontal) => {
            intersect_lines(intpoints, b, a)
        },
        (LineType::Vertical, LineType::Vertical) => {
            // similar to hori hori

            let l = std::cmp::max(a.from.1, b.from.1);
            let r = std::cmp::max(a.to.1, b.to.1);

            for yc in (l..r+1) {
                intpoints.insert((a.from.0, yc));
            }
        },
        (LineType::Vertical, LineType::DiagonalDR) => {
            if mirror { panic!("mirrored twice!"); }
            intersect_lines_helper(intpoints, mirror_line(a), mirror_line(b), true);
        },
        (LineType::Vertical, LineType::DiagonalDL) => {
            if mirror { panic!("mirrored twice!"); }
            intersect_lines_helper(intpoints, mirror_line(a), mirror_line(b), true);
        },

        (LineType::DiagonalDR, LineType::Horizontal) => {
            intersect_lines(intpoints, b, a)
        },
        (LineType::DiagonalDR, LineType::Vertical) => {
            intersect_lines(intpoints, b, a)
        },
        (LineType::DiagonalDR, LineType::DiagonalDR) => {
            
        },
        (LineType::DiagonalDR, LineType::DiagonalDL) => {
            // case 1
            // a   b
            //  a b
            //   x
            //  b a
            //     a
            //
            // case 2
            // a  b
            //  ab
            //  ba
            // b  a
            //     a

            
        },

        (LineType::DiagonalDL, LineType::Horizontal) => {
            intersect_lines(intpoints, b, a)
        },
        (LineType::DiagonalDL, LineType::Vertical) => {
            intersect_lines(intpoints, b, a)
        },
        (LineType::DiagonalDL, LineType::DiagonalDR) => {
            intersect_lines(intpoints, b, a)
        },
        (LineType::DiagonalDL, LineType::DiagonalDL) => {
            
        },
    }
}

fn solve_part1_and_part2(lines: &Vec<Line>) -> (i64, i64) {
    let mut intpoints: HashSet<(i64,i64)> = HashSet::new();

    for (a, b) in lines.into_iter().tuple_combinations() {
        intersect_lines(&mut intpoints, a, b);
    }

    (0, intpoints.len() as i64)
}

fn solve_part1(lines: &Vec<Line>) -> i64 {
    solve_part1_and_part2(lines).0
}

fn solve_part2(lines: &Vec<Line>) -> i64 {
    solve_part1_and_part2(lines).1
}

//

//use advent_of_code::implement_day;
use advent_of_code::implement_day_twoforone;
#[cfg(test)]
use advent_of_code::implement_test;

const NO: usize = 5;

pub fn day() {
    implement_day_twoforone(NO, "", parse, solve_part1_and_part2);
    //implement_day_twoforone(NO, "evil/5-50000-10000000.in", parse, solve_part1_and_part2);
    //implement_day_twoforone(NO, "evil/5-20000-6400-fixed.in", parse, solve_part1_and_part2);
    //implement_day_twoforone(NO, "evil/5-50000-10000.in", parse, solve_part1_and_part2);
    //implement_day_twoforone(NO, "evil/5-50000-10000000.in", parse, solve_part1_and_part2);
}

#[cfg(test)]
mod tests {
    use crate::day5::*;

    #[test]
    pub fn part1() {
        implement_test(NO, "", parse, solve_part1, 4421);
    }

    #[test]
    pub fn part2() {
        implement_test(NO, "", parse, solve_part2, 18674);
    }
}
