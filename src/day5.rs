use advent_of_code::parse_number;
use advent_of_code::read_lines;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug)]
pub struct Line {
    from: (i64, i64),
    to: (i64, i64),
}

fn is_horizontal_or_vertical(l: &Line) -> bool {
    l.from.0 == l.to.0 || l.from.1 == l.to.1
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

    Ok((i, Line { from: a, to: b }))
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

struct LineIterator {
    first: bool,
    pos: (i64, i64),
    to: (i64, i64),
    dx: i64,
    dy: i64,
}

impl LineIterator {
    fn new(line: &Line) -> Self {
        let mut dx = 0;
        let mut dy = 0;

        if line.from.0 < line.to.0 {
            dx = 1;
        }

        if line.from.1 < line.to.1 {
            dy = 1;
        }

        if line.from.0 > line.to.0 {
            dx = -1;
        }

        if line.from.1 > line.to.1 {
            dy = -1;
        }

        if dx < 0 {
            LineIterator {
                first: true,
                pos: line.to,
                to: line.from,
                dx: -dx,
                dy: -dy,
            }
        } else {
            LineIterator {
                first: true,
                pos: line.from,
                to: line.to,
                dx: dx,
                dy: dy,
            }
        }
    }
}

impl Iterator for LineIterator {
    type Item = (i64, i64);

    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            self.first = false;
            return Some(self.pos);
        }

        if (self.pos.0 == self.to.0) && (self.pos.1 == self.to.1) {
            return None;
        }

        self.pos = (self.pos.0 + self.dx, self.pos.1 + self.dy);
        Some(self.pos)
    }
}

fn map_increment<K>(m: &mut HashMap<K, i64>, k: K) -> ()
where
    K: Hash + Eq,
{
    *m.entry(k).or_default() += 1
    /*
    match m.get(&k) {
        None => m.insert(k, 1),
        Some(x) => m.insert(k, x + 1),
    };
    */
}

fn measure_grid(lines: &Vec<Line>) -> (i64, i64) {
    let mut mx = 0;
    let mut my = 0;

    for line in lines {
        mx = std::cmp::max(mx, line.from.0);
        mx = std::cmp::max(mx, line.to.0);
        my = std::cmp::max(my, line.from.1);
        my = std::cmp::max(my, line.to.1);
    }

    (mx + 1, my + 1)
}

fn solve_part1_and_part2(lines: &Vec<Line>) -> (i64, i64) {
    let (cols, rows) = measure_grid(lines);
    //let mut map1 = advent_of_code::zeros((cols * rows) as u32);
    //let mut map2 = advent_of_code::zeros((cols * rows) as u32);
    let mut map1 = vec![0u8; (cols * rows) as usize];
    let mut map2 = vec![0u8; (cols * rows) as usize];

    let mut ctr1 = 0;
    let mut ctr2 = 0;

    for line in lines {
        for point in LineIterator::new(&line).into_iter() {
            if is_horizontal_or_vertical(&line) {
                map1[(point.0 + cols * point.1) as usize] += 1;
                if map1[(point.0 + cols * point.1) as usize] == 2 {
                    ctr1 += 1;
                }
            }
            map2[(point.0 + cols * point.1) as usize] += 1;
            if map2[(point.0 + cols * point.1) as usize] == 2 {
                ctr2 += 1;
            }
        }
    }

    //(
    //    map1.iter().filter(|&&v| v >= 2).count() as i64,
    //    map2.iter().filter(|&&v| v >= 2).count() as i64,
    //)

    (ctr1, ctr2)
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
